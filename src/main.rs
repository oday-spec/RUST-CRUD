mod state;
mod crud;
mod Entities;
use crate::Entities::{Post};
use crate::crud::{add_post, show_posts, show_post_by_id, update_post, delete_post_by_id};

use prettytable::{Cell, Row, Table};

#[tokio::main]
async fn main() {

 let state = state::State::new("postgres://authuser:authpass@localhost:5432/authlib").await;

 sqlx::migrate!("./migrations")
    .run(&state.clone().db)
    .await
    .expect("Can't migrate to DB");


    let post = Post {
      id : 1,
      title: "update post succesfully".to_string(),
    };




//  add_post(&state.clone().db, &post).await;

// let posts =  show_posts(&state.clone().db).await;
// println!("all posts{:?}", posts);

// update_post(&state.clone().db, &post).await;

let posts = show_post_by_id(&state.clone().db, 4).await;
// println!("show post by id: {:?}", post);

// delete_post_by_id(&state.clone().db, 1).await;


// let posts  =  show_posts(&state.clone().db).await;

let mut table = Table::new();
table.add_row(Row::new(vec![
  Cell::new("PostID"),
  Cell::new("Title"),
]));

for post  in posts {
  table.add_row(Row::new(vec![
    Cell::new(post.id.to_string().as_str()),
    Cell::new(post.title.to_string().as_str()),
  ]));
}
table.printstd();


}
