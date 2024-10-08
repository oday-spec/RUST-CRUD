use sqlx::postgres::{PgPool, PgPoolOptions};
#[derive(Debug, Clone)]
pub struct State {
   pub  db: PgPool,
}

impl State {
   pub  async fn new (db: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db)
        .await
        {
           Ok(pool) => pool,
           Err(e) => panic!("Can't Stablish connection with DB: {}", e)
        };
        State {
            db: db_pool,
        }
      }  
}
