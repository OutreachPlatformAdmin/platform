use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result, Type};

#[derive(Type, Serialize, Deserialize)]
#[sqlx(type_name = "media_type", rename_all = "lowercase")]
pub enum MediaType {
    Audio,
    Video,
    Web,
    Book,
    ScientificArticle,
}

#[derive(Type, Serialize, Deserialize)]
#[sqlx(type_name = "image_type", rename_all = "lowercase")]
pub enum ImageType {
    PDF,
    PNG,
    TIFF,
    JPEG,
    GIF,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Source {
    id: i32,
    name: Option<String>,
    url: Option<String>,
    author: Option<String>,
    author_url: Option<String>,
    media_type: Option<MediaType>,
    image_url: Option<String>,
    image_type: Option<ImageType>,
    ai_generated: Option<bool>,
}

/*
 /sources
- returns all sources
 */
pub async fn get_all_sources_handler(State(db_pool): State<PgPool>) -> Response {
    let sources = get_all_sources(&db_pool).await;
    match sources {
        Ok(sources) => (StatusCode::OK, Json(sources)).into_response(),
        // for errors Axum expects the axum::response::Response type
        // example output: error returned from database: relation "platform.tipics" does not exist
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

pub async fn get_all_sources(db_pool: &PgPool) -> Result<Vec<Source>> {
    let sources = sqlx::query_as::<_, Source>(
        "SELECT id,
        name,
        url,
        author,
        author_url,
        media_type,
        image_url,
        image_type,
        ai_generated
    FROM platform.sources",
    )
    .fetch_all(db_pool)
    .await?;
    Ok(sources)
}
