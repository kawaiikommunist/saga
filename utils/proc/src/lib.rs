use proc_macro::TokenStream;
use proc_macro2::{Group, Punct, Spacing, Span, TokenStream as Tks, TokenTree};
use quote::{ToTokens, quote};
use syn::{
    Ident, LitStr, Path, PathSegment, Stmt, Token, braced, bracketed,
    parse::{Nothing, Parse},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::{Brace, Bracket},
    visit_mut::VisitMut,
};

struct Requalifier {
    prefix: Vec<Ident>,
}

impl Requalifier {
    fn new() -> Self {
        Self { prefix: Vec::new() }
    }

    pub fn update_paths(&self, stmt: &Stmt) -> syn::Result<Stmt> {
        let input = stmt.to_token_stream();
        let output = self.replace_in_stream(input);
        syn::parse2::<Stmt>(output)
    }

    pub fn replace_in_stream(&self, stream: Tks) -> Tks {
        let mut tokens = stream.into_iter().peekable();
        let mut out = Tks::new();

        while let Some(tt) = tokens.next() {
            match tt {
                // 1. Recurse into groups (e.g. `{ ... }`, `(...)`, `[...]`, macros)
                TokenTree::Group(group) => {
                    let inner = self.replace_in_stream(group.stream());
                    let mut new_group = Group::new(group.delimiter(), inner);
                    new_group.set_span(group.span());
                    out.extend(std::iter::once(TokenTree::Group(new_group)));
                }

                // 2. Check for `self` followed by `::`
                TokenTree::Ident(ident) => {
                    if (ident == "self" && self.is_followed_by_colon_colon(&mut tokens))
                        || ident == "super"
                    {
                        // Consume the `::` from the peekable iterator
                        tokens.next(); // First ':'
                        tokens.next(); // Second ':'

                        // Emit `nested::module::`
                        let mut segs = self.prefix.iter().enumerate();
                        if ident == "super" {
                            segs.next_back();
                            loop {
                                match tokens.peek() {
                                    None => {
                                        break;
                                    }
                                    Some(tt) => {
                                        if let TokenTree::Ident(ident) = tt {
                                            if ident == "super" {
                                                tokens.next();
                                                segs.next_back();
                                                if self.is_followed_by_colon_colon(&mut tokens) {
                                                    tokens.next();
                                                    tokens.next();
                                                } else {
                                                    break;
                                                }
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        for (i, seg) in segs {
                            out.extend(std::iter::once(TokenTree::Ident(seg.clone())));

                            // Emit `::` after each segment, including the last one
                            // (since we consumed the original `::`)
                            out.extend(std::iter::once(TokenTree::Punct(Punct::new(
                                ':',
                                Spacing::Joint,
                            ))));
                            out.extend(std::iter::once(TokenTree::Punct(Punct::new(
                                ':',
                                Spacing::Alone,
                            ))));
                        }
                    } else {
                        // It's a bare `self` (e.g., `fn bar(&self)`), don't touch it
                        out.extend(std::iter::once(TokenTree::Ident(ident)));
                    }
                }

                // 3. Pass everything else through untouched
                other => out.extend(std::iter::once(other)),
            }
        }

        out
    }

    fn is_followed_by_colon_colon(
        &self,
        tokens: &mut std::iter::Peekable<proc_macro2::token_stream::IntoIter>,
    ) -> bool {
        let mut clone = tokens.clone();

        let first_is_colon = matches!(
            clone.next(),
            Some(TokenTree::Punct(p)) if p.as_char() == ':' && p.spacing() == Spacing::Joint
        );
        let second_is_colon = matches!(
            clone.next(),
            Some(TokenTree::Punct(p)) if p.as_char() == ':'
        );

        first_is_colon && second_is_colon
    }
}

impl VisitMut for Requalifier {
    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        // Delegate to inner nodes first so sub-expressions/arguments get visited
        syn::visit_mut::visit_path_mut(self, path);

        // Skip leading colons, self, super, crate
        if path.leading_colon.is_some() {
            return;
        }

        if let Some(first) = path.segments.first() {
            let add_super = if first.ident == "self" {
                false
            } else if first.ident == "super" {
                true
            } else {
                println!("aborted: {:?}", first);
                return;
            };

            let mut new_segments = syn::punctuated::Punctuated::new();

            for seg in &mut self.prefix {
                seg.set_span(Span::call_site());
                new_segments.push(PathSegment {
                    ident: seg.clone(),
                    arguments: syn::PathArguments::None,
                });
            }

            if add_super {
                new_segments.push(PathSegment {
                    ident: Ident::new("super", Span::call_site()),
                    arguments: syn::PathArguments::None,
                });
            }

            for seg in &path.segments {
                new_segments.push(seg.clone());
            }

            path.segments = new_segments;
        }
    }
}

struct PackSet {
    items: Punctuated<ParPack, Nothing>,
}

struct SetItem(Box<(Brace, PackSet)>);

impl Parse for PackSet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(PackSet {
            items: Punctuated::parse_terminated(input)?,
        })
    }
}

impl PackSet {
    pub fn recursive_parse(
        self,
        packs: &mut Vec<Tks>,
        mods: &mut Vec<Tks>,
        req: &mut Requalifier,
    ) -> syn::Result<()> {
        for pack in self.items {
            pack.fmt_out(packs, mods, req)?;
        }
        Ok(())
    }
}

struct ParPack {
    name: Ident,
    desc: Option<LitStr>,
    init: Option<(Bracket, Vec<Stmt>)>,
    block: Option<(Brace, Vec<Stmt>)>,
    set: Option<(Token![:], SetItem)>,
}

impl Parse for ParPack {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let desc = if input.peek(LitStr) {
            Some(input.parse::<LitStr>()?)
        } else {
            None
        };
        let init = if input.peek(Bracket) {
            let content;
            let bracket_token = bracketed!(content in input);
            let mut stmts = content.call(syn::Block::parse_within)?;

            stmts.insert(
                0,
                parse_quote! {
                    use self::*;
                },
            );

            Some((bracket_token, stmts))
        } else {
            None
        };
        let block = if input.peek(Brace) {
            let content;
            let brace_token = braced!(content in input);
            let stmts = content.call(syn::Block::parse_within)?;
            Some((brace_token, stmts))
        } else {
            None
        };
        let set = if input.peek(Token![:]) {
            let colon_token = input.parse::<Token![:]>()?;
            let content;
            let brace_token = braced!(content in input);
            let pkset = content.parse::<PackSet>()?;
            Some((colon_token, SetItem(Box::from((brace_token, pkset)))))
        } else {
            None
        };
        Ok(Self {
            name,
            desc,
            init,
            block,
            set,
        })
    }
}

impl ParPack {
    pub fn fmt_out(
        self,
        packs: &mut Vec<Tks>,
        mods: &mut Vec<Tks>,
        req: &mut Requalifier,
    ) -> syn::Result<()> {
        let ident = self.name;
        let name = ident.to_string();
        let block = self.block;

        req.prefix.push(ident.clone());

        let desc = match self.desc {
            Some(str) => quote! { Some( String::from(#str) ) },
            None => quote! { None },
        };
        let init = match self.init {
            Some((_bracket, mut stmts)) => {
                for stmt in &mut stmts {
                    *stmt = req.update_paths(&stmt)?;
                }
                quote! {
                        |stage| { #( #stmts )* }
                }
            }
            None => quote! {
                |stage| {}
            },
        };

        packs.push(quote! {
            crate::content::Pack{ name: #name.to_string(), desc: #desc, active: true, init: #init}
        });

        let mut inner_mod = Vec::new();

        if let Some((_tkn, set)) = self.set {
            set.0.1.recursive_parse(packs, &mut inner_mod, req)?;
        }

        if let Some((_tkn, blck)) = block {
            mods.push(quote! {
                pub mod #ident { #(#blck)* #(#inner_mod)* }
            });
        } else {
            mods.push(quote! {
                pub mod #ident { #(#inner_mod)* }
            });
        }

        req.prefix.pop();

        Ok(())
    }
}

#[proc_macro]
pub fn pack(input: TokenStream) -> TokenStream {
    let packlist = parse_macro_input!(input as PackSet);

    let mut req = Requalifier::new();
    let mut mods = Vec::new();
    let mut packs = Vec::new();

    for pack in packlist.items {
        pack.fmt_out(&mut packs, &mut mods, &mut req);
    }

    let expanded = quote! {
        #[allow(unused_imports)]
        pub fn init_packs() -> Vec<crate::content::Pack> {
            Vec::from([ #( #packs ),* ])
        }

        #( #mods )*
    };
    TokenStream::from(expanded)
}
