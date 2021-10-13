use postgres::Client;
use serde::{Deserialize, Serialize};

use std::sync::MutexGuard;

pub mod clusters;
pub mod definitions;
pub mod examples;
pub mod languages;
pub mod translations;
pub mod word_groups;

type Db<'a> = MutexGuard<'a, Client>;
type Int = i32;

pub struct ParamIndexer {
    count: u8
}
impl ParamIndexer {
    pub fn new() -> ParamIndexer {
        ParamIndexer { count: 1 }
    }
    pub fn next(&mut self) -> String {
        let count = self.count;
        self.count += 1;
        format!("${}", count)
    }
    pub fn params(&mut self, count: &u8) -> String {
        let mut params = vec![];
        for _ in 0..*count {
            params.push(self.next())
        }
        params.join(",")
    }
}

pub fn create_tables(_db: &mut Db) -> Result<(), String> {

    // error_msg!(Table::new("clusters").create_table(db, vec![
    //     (clusters::K::Id, "serial not null unique primary key")
    // ]))?;

    // error_msg!(Table::new("examples").create_table(db, vec![
    //     (examples::K::Id, "serial not null unique primary key"),
    //     (examples::K::ParentType, "int not null"),
    //     (examples::K::ParentId, "int not null"),
    //     (examples::K::Example, "varchar(256) not null")
    // ]))?;

    Ok(())
}
