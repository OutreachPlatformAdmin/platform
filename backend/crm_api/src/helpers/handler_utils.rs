use sqlx::{query, FromRow, PgPool, Result};
use serde::{Deserialize};

pub fn process_optional_param(param: Option<Vec<String>>) -> Vec<String> {
    let mut processed_param = vec![];
    if let Some(populated_param) = param {
        processed_param = populated_param
    }
    processed_param
}

#[derive(Deserialize, FromRow)]
pub struct CreateTopic {
    topic: String,
    is_verified: Option<bool>, 
    brief_description: Option<String>,
    full_description: Option<String>,
    bullet_points: Option<Vec<String>>,
    examples: Option<Vec<String>>,
    parallels: Option<Vec<String>>,
    ai_brief_description: Option<String>,
    ai_full_description: Option<String>,
    ai_bullet_points: Option<Vec<String>>,
    ai_parallels: Option<Vec<String>>,
    ai_examples: Option<Vec<String>>,
}

#[derive(Deserialize, FromRow)]
pub struct CreateTerm {
    term: String,
    is_verified: Option<bool>, 
    brief_description: Option<String>,
    full_description: Option<String>,
    bullet_points: Option<Vec<String>>,
    examples: Option<Vec<String>>,
    parallels: Option<Vec<String>>,
    ai_brief_description: Option<String>,
    ai_full_description: Option<String>,
    ai_bullet_points: Option<Vec<String>>,
    ai_parallels: Option<Vec<String>>,
    ai_examples: Option<Vec<String>>,
}

/*
Create an ENUM that has CreateTopic and CreateTerm as options 
 */
pub enum PostBodies {
    CreateTopic(CreateTopic),
    CreateTerm(CreateTerm),
}


pub async fn insert_topic_or_term(payload: PostBodies, db_pool: &PgPool) -> Result<()> {
    let mut table_name: String;
    let mut topic_or_term: String;
    let mut topic_or_term_value: String;

    let body = match payload {
        PostBodies::CreateTopic(create_topic) => {
            table_name = "topics".to_string();
            topic_or_term_value = create_topic.topic;
            topic_or_term = "topic".to_string();
            CreateTerm { 
                term: &topic_or_term_value, 
                is_verified: create_topic.is_verified, 
                brief_description: create_topic.brief_description, 
                full_description: create_topic.full_description, 
                bullet_points: create_topic.bullet_points, 
                examples: create_topic.examples, 
                parallels: create_topic.parallels, 
                ai_brief_description: create_topic.ai_brief_description, 
                ai_full_description: create_topic.ai_full_description, 
                ai_bullet_points: create_topic.ai_bullet_points, 
                ai_parallels: create_topic.ai_parallels, 
                ai_examples: create_topic.ai_examples 
            }
        },
        PostBodies::CreateTerm(create_term) => {
            table_name = "terms".to_string();
            topic_or_term_value = create_term.term;
            topic_or_term = "term".to_string();
            CreateTerm { 
                    value: create_term.term, 
                    is_verified: create_term.is_verified, 
                    brief_description: create_term.brief_description, 
                    full_description: create_term.full_description, 
                    bullet_points: create_term.bullet_points, 
                    examples: create_term.examples, 
                    parallels: create_term.parallels, 
                    ai_brief_description: create_term.ai_brief_description, 
                    ai_full_description: create_term.ai_full_description, 
                    ai_bullet_points: create_term.ai_bullet_points, 
                    ai_parallels: create_term.ai_parallels, 
                    ai_examples: create_term.ai_examples 
                }
        }
    };

    let bullet_points = process_optional_param(body.bullet_points);
    let examples = process_optional_param(body.examples);
    let parallels = process_optional_param(body.parallels);
    let ai_bullet_points = process_optional_param(body.ai_bullet_points);
    let ai_parallels = process_optional_param(body.ai_parallels);
    let ai_examples = process_optional_param(body.ai_examples);
    

    let query_string = format!("INSERT INTO platform.{} ({}, is_verified, brief_description, full_description, 
        bullet_points, examples, parallels, ai_brief_description, ai_full_description, ai_bullet_points, ai_parallels, 
        ai_examples) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)", table_name, topic_or_term);

    let insert_result = sqlx::query(&query_string)
    .bind(topic_or_term_value)
    .bind(body.is_verified)
    .bind(body.brief_description)
    .bind(body.full_description)
    .bind(bullet_points.as_slice())
    .bind(examples.as_slice())
    .bind(parallels.as_slice())
    .bind(body.ai_brief_description)
    .bind(body.ai_full_description)
    .bind(ai_bullet_points.as_slice())
    .bind(ai_parallels.as_slice())
    .bind(ai_examples.as_slice())
    .execute(db_pool).await;

    Ok(())
}
