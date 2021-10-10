use postgres::{Client, Row, types::FromSql};
use serde::{Deserialize, Serialize};
use std::{
    marker::{ Copy, PhantomData},
    sync::{ Mutex, MutexGuard }
};

pub mod clusters;
pub mod definitions;
pub mod examples;
pub mod languages;
pub mod translations;
pub mod word_groups;

type Db<'a> = MutexGuard<'a, Client>;

pub trait Valuable: Clone {
    fn get_value(&self) -> String;
}

pub trait Columnist<V: Valuable>: Copy {
    fn get_key(&self) -> String;
    fn get_value_from_row(&self, row: &Row) -> Result<V, String>;
}

pub struct KV<K: Columnist<V>, V: Valuable>(K, V);
impl<K: Columnist<V>, V: Valuable> KV<K, V> {
    pub fn new(k: K, v: V) -> KV<K, V> {
        KV(k, v)
    }
    pub fn get_kv_str_pair(&self) -> (String, String) {
        (self.0.get_key(), self.1.get_value())
    }
    pub fn get_key_values_for_insert(key_values: &Vec<KV<K, V>>) -> (String, String) {
        let mut keys = vec![];
        let mut values = vec![];
        for kv in key_values.iter() {
            keys.push(kv.get_key());
            values.push(kv.get_value());
        }
        (keys.join(","), values.join(","))
    }
    pub fn get_key(&self) -> String {
        self.0.get_key()
    }
    pub fn get_value(&self) -> String {
        self.1.get_value()
    }
    pub fn get_kv_from_row(k: K, v: &Row) -> Result<KV<K, V>, String> {
        Ok(KV(k, K::get_value_from_row(&k, v)?))
    }
}

pub enum Condition<K: Columnist<V>, V: Valuable> {
    Eq(KV<K, V>),
    Gt(KV<K, V>),
    Lt(KV<K, V>),
    Gte(KV<K, V>),
    Lte(KV<K, V>),
    Ne(KV<K, V>),
    And(Vec<Condition<K, V>>),
    Or(Vec<Condition<K, V>>)
}
impl<K: Columnist<V>, V: Valuable> Condition<K, V> {
    fn map_condition(conditions: &Vec<Self>) -> Vec<String> {
        conditions
            .iter()
            .map(|condition| condition.to_sql())
            .collect::<Vec<String>>()
    }
    fn group(join: &str, conditions: &Vec<Self>) -> String {
        format!("({})", Condition::map_condition(conditions).join(&format!(" {} ", join)))
    }
    pub fn to_sql(&self) -> String {
        match &self {
            Self::Eq(kv) => format!("{} = {}", kv.get_key(), kv.get_value()),
            Self::Gt(kv)  => format!("{} > {}", kv.get_key(), kv.get_value()),
            Self::Lt(kv)  => format!("{} < {}", kv.get_key(), kv.get_value()),
            Self::Gte(kv)  => format!("{} >= {}", kv.get_key(), kv.get_value()),
            Self::Lte(kv)  => format!("{} <= {}", kv.get_key(), kv.get_value()),
            Self::Ne(kv)  => format!("{} <> {}", kv.get_key(), kv.get_value()),
            Self::And(conditions) => Condition::group("and", conditions),
            Self::Or(conditions) => Condition::group("or", conditions)            
        }
    }
}
pub struct Conditions<K: Columnist<V>, V: Valuable>(Vec<Condition<K, V>>);
impl<K: Columnist<V>, V: Valuable> Conditions<K, V> {
    pub fn get_where(&self) -> String {
        let conditions = self.0.iter().map(|condition| condition.to_sql()).collect::<Vec<String>>().join(" ");
        format!("where {}", conditions)
    }
    pub fn get_set(&self) -> String {
        self.0
            .iter()
            .map(|condition| { 
                format!("set {}", condition.to_sql())
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
}

pub trait FromSchema {
    type Column;
    fn to_vec(&self) -> Vec<Self::Column>;
}

#[derive(Debug, Clone)]
pub enum Projection<K: Columnist<V>, V: Valuable> {
    Some(Vec<K>),
    All(Vec<K>),
    PhantomData(PhantomData<V>)
}

pub struct FindOpts<K: Columnist<V>, V: Valuable> {
    filter: Conditions<K, V>,
    projection: Projection<K, V>,
    limit: Option<u32>
}

pub struct Table<K: Columnist<V>, V: Valuable> {
    name: String,
    key_type: PhantomData<K>,
    value_type: PhantomData<V>
}
impl<K: Columnist<V>, V: Valuable> Table<K, V> {

    pub fn new(name: &str) -> Table<K, V> {
        Table { 
            name: name.to_string(), 
            key_type: PhantomData, 
            value_type: PhantomData
        }
    }

    fn get_insert_string(&self, columns: &Vec<KV<K, V>>) -> String {
        let (keys, values) = KV::get_key_values_for_insert(&columns);
        format!("insert into {} ({}) values ({}) returning id;", &self.name, keys, values)
    }
    
    pub fn insert_one(&self, db: &mut Db, columns: &Vec<KV<K, V>>) -> Result<u32, String> {
        let sql = self.get_insert_string(columns);
        match db.query(sql.as_str(), &[ ]) {
            Ok(rows) => get_id(&rows),
            Err(error) => Err(error.to_string())
        }
    }
    
    pub fn insert_many(&self, db: &mut Db, rows: &Vec<Vec<KV<K, V>>>) -> Vec<Result<u32, String>> {
        let mut ids: Vec<Result<u32, String>> = vec![];
        for row in rows.iter() {
            ids.push(self.insert_one(db, row));
        }
        ids
    }
    
    fn get_update_string(&self, filter: &Conditions<K, V>, updates: &Conditions<K, V>) -> String {
        format!("update {} {} {}", &self.name, filter.get_where(), updates.get_set())
    }
    
    pub fn update(&self, db: &mut Db, filter: &Conditions<K, V>, updates: &Conditions<K, V>) -> Result<(), String> {
        let sql = self.get_update_string(filter, updates);
        match db.query(sql.as_str(), &[ ]) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }
    
    pub fn delete(&self, db: &mut Db, filter: &Conditions<K, V>) -> Result<(), String> {
        let sql = format!("delete from {} {}", &self.name, filter.get_where());
        match db.query(sql.as_str(), &[]) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }

    pub fn get_find_sql(&self, options: &FindOpts<K, V>) -> String {
    
        let projection_str = {
            match &options.projection {
                Projection::Some(keys) => keys.iter().map(|key| key.get_key()).collect::<Vec<String>>().join(","),
                Projection::All(_keys) => "*".to_string(),
                Projection::PhantomData(_) => "*".to_string()
            }
        };
        let filter_str = options.filter.get_where();
        let limit_str = {
            if let Some(limit) = options.limit {
                format!("limit {}", limit)
            } else {
                "".to_string()
            }
        };
        format!("select ({}) from {} {}{}", projection_str, &self.name, filter_str, limit_str)
    }
    
    pub fn find(&self, db: &mut Db, options: &FindOpts<K, V>) -> Result<Vec<Vec<V>>, String> {
        let sql = self.get_find_sql(options);
        let column_list: Vec<K> = {
            match &options.projection {
                Projection::All(keys) => {
                    keys.iter().map(|k| k.clone()).collect::<Vec<K>>()
                },
                Projection::Some(keys) => keys.iter().map(|k| k.clone()).collect::<Vec<K>>(),
                Projection::PhantomData(_) => vec![]
            }
        };
        match db.query(sql.as_str(), &[]) {
            Ok(rows) => {
                rows.iter().map(|row| -> Result<Vec<V>, String> {
                    let mut key_values: Vec<V> = vec![];
                    for k in column_list.iter() {
                        key_values.push(k.get_value_from_row(row)?)
                    }
                    Ok(key_values)
                })
                .collect::<Result<Vec<Vec<V>>, String>>()?;
                Ok(vec![])
            },
            Err(error) => Err(error.to_string())
        }
    }
    
}

pub fn get_db<'a>(db: &'a Mutex<Client>) -> Result<Db<'a>, String> {
    match db.try_lock() {
        Ok(db) => Ok(db),
        Err(error) => Err(error.to_string())
    }
}

pub fn get_value_from_row<'a, T: FromSql<'a>>(row: &'a Row, column: &str) -> Result<T, String> {
    match row.try_get(column) {
        Ok(v) => Ok(v),
        Err(error) => Err(error.to_string())
    }
}

pub fn get_id_from_row(row: &Row) -> Result<u32, String> {
    get_value_from_row(row, "id")
}

pub fn get_id(rows: &Vec<Row>) -> Result<u32, String> {
    match rows.get(0) {
        Some(row) => get_id_from_row(row),
        None => Err("Could not get id. Could not find row.".to_string())
    }
}

pub fn escape(str: &str) -> String {
    format!("'{}'", str.replace("'", "''"))
}

pub fn create_table<K: Columnist<V>, V: Valuable>(db: &mut Db, name: &str, columns: Vec<(K, &str)>) -> Result<(), String> {
    let sql = format!(
        "create table if not exists {} ({})",
        name,
        columns
            .iter()
            .map(|(col, params)| format!("{} {}", col.get_key(), params))
            .collect::<Vec<String>>()
            .join(",")
    );
    match db.query(sql.as_str(), &[]) {
        Ok(_rows) => Ok(()),
        Err(error) => Err(error.to_string())
    }
}

pub fn set_as_null() -> String { "null".to_string() }
