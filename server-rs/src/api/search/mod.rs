use actix_web::{ 
    web::{ Data, Query }, 
    HttpResponse
};
use crate::{
    AppData,
    schema::{
        Db,
        Int,
        ParamIndexer,
        clusters::{
            Cluster,
            Row as ClusterRow,
            table as clusters_table
        },
        definitions::{
            table as definitions_table
        },
        translations::{
            table as translations_table
        }
    },
    error_msg
};
use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct Info {
    pub text: String
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Reply {
    Ok {
        clusters: Vec<Cluster>
    },
    Error {
        error: String
    }
}

pub fn search_clusters(db: &mut Db, info: &Info) -> Result<Vec<Cluster>, String> {
    let translation_sub_query = vec![
        "select", "distinct", translations_table::DEFINITION_ID,
        "from", translations_table::TABLE_NAME,
        "where", translations_table::DEFINITION, "like", &format!("'%{}%'", info.text)
    ].join(" ");
    let definitions_sub_query = vec![
        "select", "distinct", definitions_table::CLUSTER_ID,
        "from", definitions_table::TABLE_NAME,
        "where", definitions_table::DEFINITION, "like", &format!("'%{}%'", info.text),
        "or", definitions_table::WORD, "like", &format!("'%{}%'", info.text),
        "or", definitions_table::ID, "in", &format!("({})", translation_sub_query)
    ].join(" ");
    let query = vec![
        "select", "distinct", clusters_table::ID,
        "from", clusters_table::TABLE_NAME,
        "where", definitions_table::ID, "in", &format!("({})", definitions_sub_query)
    ].join(" ");
    // println!("{}", cluster_sub_query);
    let clusters = match error_msg!(db.query(query.as_str(), &[])) {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error)
    }?.iter().map(|row| -> Result<Cluster, String> {
        let mut indexer = ParamIndexer::new();
        let definition_query = vec![
            "select", definitions_table::ID,
            "from", definitions_table::TABLE_NAME,
            "where", definitions_table::CLUSTER_ID, "=", &indexer.next()
        ].join(" ");
        let cluster = ClusterRow { id: error_msg!(row.try_get(clusters_table::ID))? };
        let cluster_id = cluster.id;
        Ok(Cluster {
            cluster,
            definitions: match error_msg!(db.query(definition_query.as_str(), &[ &cluster_id ])) {
                Ok(rows) => rows.iter().map(|row| -> Result<Int, String> {
                    Ok(error_msg!(row.try_get(definitions_table::ID))?)
                }).collect::<Result<Vec<Int>, String>>(),
                Err(error) => Err(error)
            }?
        })
    }).collect::<Result<Vec<Cluster>, String>>()?;
    Ok(clusters)
}

pub fn search(data: Data<AppData>, info: Query<Info>) -> HttpResponse {
    match error_msg!(data.db.try_lock()) {
        Ok(mut db) => match error_msg!(search_clusters(&mut db, &info.into_inner())) {
            Ok(clusters) => {
                return HttpResponse::Ok().json(Reply::Ok{ clusters });
            },
            Err(error) => {
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
