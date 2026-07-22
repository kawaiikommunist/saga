use crate::cut::*;

pub struct Book {
    pub pages: Vec<Page>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PgNum(pub usize);

pub struct Page;
