use crate::database::establish_connection;
use crate::handlers::post::postsResponse;
use diesel::prelude::*;
use std::io;

#[derive(Queryable)] // will generate all of the code needed to load a Post struct from a SQL query.
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// 获得所有已发布的帖子
pub fn get_all() -> Result<postsResponse, io::Error> {
    use crate::schema::posts::dsl::{posts, published};

    let connection = establish_connection();
    let all_posts = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    Ok(all_posts.into())
}
