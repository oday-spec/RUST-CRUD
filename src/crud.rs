use crate::Entities::Post;
use log::error;
use sqlx::postgres::PgPool;

//------------------------------------------------------------------
use thiserror::Error;
#[derive(Error, Debug, Clone)]
pub enum ErrorDB {
    #[error("Internal error")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
}
impl std::convert::From<sqlx::Error> for ErrorDB {
    fn from (err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ErrorDB::NotFound("Not Found".into()),
            _ => ErrorDB::Internal(err.to_string()),
        }
    }
}
//-----------------------------------------------------------------------
//CREATE
pub async  fn add_post( db: &PgPool, post: &Post) -> Result<(), anyhow::Error>  {
    const QUERY: &str = " INSERT INTO posts (id, title) VALUES ($1, $2)";
    match sqlx::query(QUERY) 
        .bind(post.id)
        .bind(post.title.clone())
        .execute(db)
        .await {
           Err(err) => {
              error!("Can't insert into posts table: {}", &err);
              Err(err.into())
           }
           Ok(_) => Ok(()),
        }
        }
//READ
pub async fn show_posts(db: &PgPool) -> Result<Vec<Post>, anyhow::Error> {
   const QUERY: &str = "SELECT * FROM posts";
   match sqlx::query_as::<_, Post>(QUERY).fetch_all(db).await{
      Err(err) => {
         error!("show posts: {}", &err);
         Err(err.into())
      }
      Ok(res) => Ok(res),
   }
}
// READ BY ID
pub async fn show_post_by_id(db: &PgPool, id : i32) -> Result<Post, ErrorDB> {
   const QUERY: &str = "SELECT * FROM posts WHERE id = $1";

   match sqlx::query_as::<_, Post>(QUERY)
   .bind(id)
   .fetch_optional(db)
   .await
    {
      Err(err)=> {
         error!("Can't show post by id {}", &err);
         Err(err.into())
      }
      Ok( None) => Err(ErrorDB::NotFound("POST NOT FOUND".to_string())),
      Ok(Some(res)) => Ok(res),
    
   }
}
//UPDATE
pub async fn update_post(db: &PgPool, post: &Post) -> Result<(), ErrorDB> {
   const QUERY: &str = "UPDATE posts SET title = $1 WHERE id = $2";

   match sqlx::query(QUERY)
   .bind(post.title.clone())
   .bind(post.id)
   .execute(db)
   .await {
      Err(err) => {
         error!("update post {}", &err);
         Err(err.into())
      }
      Ok(_) => Ok(()),
   }
}
//DELETE
pub async fn delete_post_by_id(db: &PgPool, id : i32) -> Result<(), ErrorDB> {
   const QUERY: &str = "DELETE FROM posts WHERE id = $1";

  let _ = sqlx::query(QUERY)
   .bind(id)
   .execute(db)
   .await  ;
  Ok(()) 
}
