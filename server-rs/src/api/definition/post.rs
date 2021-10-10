use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub enum WordGroupProp {
    Id(u32),
    Name(String)
}

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub word_group: WordGroupProp,
    pub cluster_id: u32,
    pub pronounciation: Option<String>,
    pub word: String,
    pub prefixes: Option<String>,
    pub suffixes: Option<String>,
    pub definition: String
}
