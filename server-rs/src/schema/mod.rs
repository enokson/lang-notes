use crate::error_msg;
use postgres::Client;
use std::sync::MutexGuard;

pub mod clusters;
pub mod definitions;
pub mod examples;
pub mod languages;
pub mod translations;
pub mod word_groups;

type Db<'a> = MutexGuard<'a, Client>;
pub type Int = i32;

pub struct ParamIndexer {
    count: u8
}
impl ParamIndexer {
    pub fn new() -> ParamIndexer {
        ParamIndexer { count: 0 }
    }
    pub fn next(&mut self) -> String {
        self.count += 1;
        format!("${}", &self.count)
    }
    pub fn last(&self) -> String {
        format!("${}", &self.count)
    }
    pub fn params(&mut self, count: &u8) -> String {
        let mut params = vec![];
        for _ in 0..*count {
            params.push(self.next())
        }
        params.join(",")
    }
}

pub fn create_tables(db: &mut Db) -> Result<(), String> {
    error_msg!(db.query(clusters::get_create_table_sql().as_str(), &[]))?;
    error_msg!(db.query(definitions::get_create_table_sql().as_str(), &[]))?;
    error_msg!(db.query(examples::get_create_table_sql().as_str(), &[]))?;
    error_msg!(db.query(languages::get_create_table_sql().as_str(), &[]))?;
    error_msg!(db.query(translations::get_create_table_sql().as_str(), &[]))?;
    error_msg!(db.query(word_groups::get_create_table_sql().as_str(), &[]))?;
    Ok(())
}
