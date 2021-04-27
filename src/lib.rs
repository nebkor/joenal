use std::collections::BTreeSet;

use anyhow::Result as AResult;
use lazy_static::lazy_static;
use mime::TEXT_PLAIN_UTF_8;
use sqlx::{query, query_as, query_scalar, sqlite::SqlitePool};
use uuid::Uuid;

mod db;
mod models;
mod util;

pub use db::*;
pub use models::*;
pub use util::*;

pub type StarDate = chrono::DateTime<chrono::Utc>;

#[derive(Debug, PartialEq)]
pub struct RawJot {
    pub content: String,
    pub creation_date: StarDate,
    pub tags: Vec<String>,
}

pub async fn insert_jot(pool: &SqlitePool, jot: &RawJot) -> AResult<()> {
    lazy_static! {
        static ref UTF_8_MIME: String = TEXT_PLAIN_UTF_8.to_string();
    }

    let mut jot_id = mk_jot_id(&jot);
    let dev_id = get_device_id();

    let mut dup_id = None;

    // do everything in a single transaction
    let mut tx = pool.begin().await?;

    let jot_count: u32 = query_scalar(
        r#"
SELECT COUNT(*) FROM jots WHERE jot_id = ?1
"#,
    )
    .bind(&jot_id)
    .fetch_one(&mut tx)
    .await?;

    if jot_count > 0 {
        dup_id = Some(jot_id);
        jot_id = Uuid::new_v4();
    };

    let new_jot = models::Jot::new(
        jot_id,
        Some(jot.creation_date),
        jot.content.as_bytes().to_vec(),
        UTF_8_MIME.clone(),
        dev_id,
        dup_id,
    );

    let mut new_tags: Vec<models::Tag> = Vec::with_capacity(jot.tags.len());
    //let mut new_scores: Vec<(Uuid, i32)> = Vec::with_capacity(jot.tags.len());
    let mut mappings: Vec<models::Mapping> = Vec::with_capacity(jot.tags.len());

    for tag in jot.tags.iter() {
        let id = mk_tag_id(tag);
        let old_score: i32 = query_scalar("select score from tags where tag_id = ?1")
            .bind(&id)
            .fetch_one(&mut tx)
            .await?;

        let new_score = old_score + 1;

        let new_tag = models::Tag::new(tag.clone(), id, dev_id, Some(jot.creation_date), new_score);

        if old_score == 0 {
            new_tags.push(new_tag);
        } else {
            //new_scores.push((id.clone(), new_score))
            let _ = query(r#"UPDATE tags SET score = ?1 WHERE tag_id = ?2"#)
                .bind(new_score)
                .bind(&id)
                .execute(&mut tx)
                .await?;
        }

        // now the mapping
        let mapping_id = mk_mapping_id(&jot_id, &id);

        let mapping = models::Mapping::new(mapping_id, id, jot_id, Some(jot.creation_date));
        mappings.push(mapping);
    }

    let _ = new_jot.as_insert().execute(&mut tx).await?;

    Ok(())
}

pub async fn get_jots(conn: &SqlitePool) -> Vec<Jot> {
    match query_as(
        r#"
SELECT * FROM jots ORDER BY jot_creation_date DESC
"#,
    )
    .fetch_all(conn)
    .await
    {
        Ok(jots) => jots,
        _ => panic!(),
    }
}

pub fn parse_tags(tagline: &str) -> Vec<String> {
    let tags: BTreeSet<String> = tagline
        .split(',')
        .map(|t| t.trim().to_owned())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_lowercase())
        .collect();

    tags.into_iter().collect()
}
