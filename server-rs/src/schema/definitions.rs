use serde::{Deserialize, Serialize};
use super::{
    Int,
    examples::{
        Row as ExampleRow,
        RowI as ExampleRowI
    },
    translations::{
        Row as TranslationRow,
        RowI as TranslationRowI
    },
    word_groups::{
        Row as WordsGroupRow,
        NewDefinition
    }
};

pub mod table {
    pub const TABLE_NAME: &'static str = "definitions";
    pub const ID: &'static str = "id";
    pub const WORDS_GROUP_ID: &'static str = "words_group_id";
    pub const CLUSTER_ID: &'static str = "cluster_id";
    pub const PRONOUNCIATION: &'static str = "pronounciation";
    pub const WORD: &'static str = "word";
    pub const PREFIXES: &'static str = "prefixes";
    pub const SUFFIXES: &'static str = "suffixes";
    pub const DEFINITION: &'static str = "definition";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: Int,
    pub words_group_id: Int,
    pub cluster_id: Int,
    pub pronounciation: Option<String>,
    pub word: String,
    pub prefixes: Option<String>,
    pub suffixes: Option<String>,
    pub definition: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RowI {
    pub words_group: NewDefinition,
    pub cluster_id: Int,
    pub pronounciation: Option<String>,
    pub word: String,
    pub prefixes: Option<String>,
    pub suffixes: Option<String>,
    pub definition: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Definition {
    pub value: Row,
    pub words_group: WordsGroupRow,
    pub translations: Vec<TranslationRow>,
    pub examples: Vec<ExampleRow>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DefinitionI {
    pub value: RowI,
    // pub words_group: WordsGroupRowI, this is not needed id it is already being handled in the value prop
    pub translations: Option<Vec<TranslationRowI>>,
    pub examples: Option<Vec<ExampleRowI>>
}
