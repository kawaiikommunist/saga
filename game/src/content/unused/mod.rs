use bevy::platform::collections::HashMap;

use bevy::prelude::*;
use rules::pack;

fn tst() {
    println!("hi");
}

pub struct Pack {
    pub name: String,
    pub desc: Option<String>,
    pub active: bool,
    pub init: for<'a> fn(&'a mut crate::stage::Stage),
}

pub struct Packlist(HashMap<String, Pack>);

pack! {
    turn "everything to do with the turn-passing" :{
         steps {
             pub struct jaja;
         }
    }
}
