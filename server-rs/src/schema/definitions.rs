
use super::*;

#[derive(Debug, Clone, Copy)]
pub enum K {
    Id,
    WordGroupId,
    ClusterId,
    Pronounciation,
    Word,
    Prefixes,
    Suffixes,
    Definition
}
impl Columnist<V> for K {
    fn get_key(&self) -> String {
        match &self {
            Self::Id => format!("id"),
            Self::WordGroupId => format!("word_group_id"),
            Self::ClusterId => format!("cluster_id"),
            Self::Pronounciation => format!("pronounciation"),
            Self::Word => format!("word"),
            Self::Prefixes => format!("prefixes"),
            Self::Suffixes => format!("suffixes"),
            Self::Definition => format!("definition")
        }
    }
    fn get_value_from_row(&self, row: &postgres::Row) -> Result<V, String> {
        match &self {
            Self::Id => Ok(V::Id(get_value_from_row(row, &self.get_key())?)),
            Self::WordGroupId => Ok(V::WordGroupId(get_value_from_row(row, &self.get_key())?)),
            Self::ClusterId => Ok(V::ClusterId(get_value_from_row(row, &self.get_key())?)),
            Self::Pronounciation => Ok(V::Pronounciation(get_value_from_row(row, &self.get_key())?)),
            Self::Word => Ok(V::Word(get_value_from_row(row, &self.get_key())?)),
            Self::Prefixes => Ok(V::Prefixes(get_value_from_row(row, &self.get_key())?)),
            Self::Suffixes => Ok(V::Suffixes(get_value_from_row(row, &self.get_key())?)),
            Self::Definition => Ok(V::Definition(get_value_from_row(row, &self.get_key())?))
        }
    }
}

#[derive(Debug, Clone)]
pub enum V {
    Id(u32),
    WordGroupId(u32),
    ClusterId(u32),
    Pronounciation(Option<String>),
    Word(String),
    Prefixes(Option<String>),
    Suffixes(Option<String>),
    Definition(String)
}
impl Valuable for V {
    fn get_value(&self) -> String {
        match &self {
            Self::Id(id) => id.to_string(),
            Self::WordGroupId(id) => id.to_string(),
            Self::ClusterId(id) => id.to_string(),
            Self::Pronounciation(opt_pronounciation) => match opt_pronounciation {
                Some(pronounciation) => pronounciation.to_string(),
                None => set_as_null()
            },
            Self::Word(word) => word.to_string(),
            Self::Prefixes(opt_prefixes) => match opt_prefixes {
                Some(prefixes) => prefixes.to_string(),
                None => set_as_null()
            },
            Self::Suffixes(opt_suffixes) => match opt_suffixes {
                Some(suffixes) => suffixes.to_string(),
                None => set_as_null()
            },
            Self::Definition(definition) => definition.to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: u32,
    pub word_group_id: u32,
    pub cluster_id: u32,
    pub pronounciation: Option<String>,
    pub word: String,
    pub prefixes: Option<String>,
    pub suffixes: Option<String>,
    pub definition: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewRow {
    pub words_group_id: word_groups::NewDefinition,
    pub cluster_id: u32,
    pub pronounciation: Option<String>,
    pub word: String,
    pub prefixes: Option<String>,
    pub suffixes: Option<String>,
    pub definition: String
}
impl NewRow {
    pub fn to_kv(&self, words_group_id: u32) -> Vec<KV<K, V>> {
        vec![
            KV::new(K::WordGroupId, V::WordGroupId(words_group_id)),
            KV::new(K::ClusterId, V::ClusterId(self.cluster_id)),
            KV::new(K::Pronounciation, V::Pronounciation(self.pronounciation.clone())),
            KV::new(K::Word, V::Word(self.word.clone())),
            KV::new(K::Prefixes, V::Prefixes(self.prefixes.clone())),
            KV::new(K::Suffixes, V::Suffixes(self.suffixes.clone())),
            KV::new(K::Definition, V::Definition(self.definition.clone()))
        ]
        
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Definition {
    pub value: Row,
    pub word_group: word_groups::Row,
    pub translations: Vec<translations::Translation>,
    pub examples: Vec<examples::Row>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewDefinition {
    pub value: NewRow,
    pub translations: Option<Vec<translations::NewTranslation>>,
    pub examples: Option<Vec<examples::NewRow>>
}
