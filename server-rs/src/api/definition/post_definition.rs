use actix_web::{
    web::{Data, Json},
    HttpResponse
};
use postgres::types::ToSql;
use serde::{ Serialize };
use crate::{
    AppData,
    api::{
        words_group::{
            get_words_group::get_words_group,
            post_words_group::post_words_group
        }
    },
    schema::{
        Db,
        ParamIndexer,
        definitions::{table, RowI},
        word_groups::{
            RowI as WordsGroupRowI,
            NewDefinition
        }
    },
    error_msg
};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Reply {
    Ok { id: i32 },
    Err { error: String }
}

pub const COULD_NOT_FIND_WORDS_GROUP_ERROR: &'static str = "Could not find words-group";

pub fn post_definition(db: &mut Db, body: &RowI) -> Result<i32, String> {
    
    let words_group_id = match &body.words_group {
        NewDefinition::Existing{ id } => match error_msg!(get_words_group(db, &id)) {
            Ok(option) => match option {
                Some(words_group) => Ok(words_group.id),
                None => Err(format!("{}: {}.", COULD_NOT_FIND_WORDS_GROUP_ERROR, id))
            },
            Err(error) => Err(error)
        },
        NewDefinition::New{ name } => error_msg!(post_words_group(db, &WordsGroupRowI { name: name.to_string() }))
    }?;
    let mut indexer = ParamIndexer::new();
    let sql = vec![
        "insert", "into", table::TABLE_NAME,
        &format!("({})", vec![
            table::WORDS_GROUP_ID,
            table::CLUSTER_ID,
            table::PRONOUNCIATION,
            table::WORD,
            table::PREFIXES,
            table::SUFFIXES,
            table::DEFINITION
        ].join(",")),
        "values", &format!("({})", indexer.params(&7)),
        "returning", table::ID
    ].join(" ");
    let params: [&(dyn ToSql + Sync); 7] = [
        &words_group_id, 
        &body.cluster_id, 
        &body.pronounciation,
        &body.word,
        &body.prefixes,
        &body.suffixes,
        &body.definition
    ];
    let id = match error_msg!(db.query(sql.as_str(), &params)) {
        Ok(rows) => match rows.get(0) {
            Some(row) => match error_msg!(row.try_get("id")) {
                Ok(id) => id,
                Err(error) => {
                    return Err(error);
                }
            },
            None => {
                return Err(format!("{}::{} Could not find row.", file!(), line!()));
            }
        },
        Err(error) => {
            return Err(error);
        }
    };
    return Ok(id);
}

pub fn post(data: Data<AppData>, body: Json<RowI>) -> HttpResponse {
    match error_msg!(data.db.try_lock()) {
        Ok(mut db) => match error_msg!(post_definition(&mut db, &body.into_inner())) {
            Ok(id) => {
                return HttpResponse::Ok().json(Reply::Ok{ id });
            },
            Err(error) => {
                if error.contains(COULD_NOT_FIND_WORDS_GROUP_ERROR) {
                    return HttpResponse::Forbidden().json(Reply::Err{ error } );
                }
                println!("{}", error);
                return HttpResponse::InternalServerError().finish();
            }
        },
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    };
}
