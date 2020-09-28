use crate::schema::posts;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable)] // will generate all of the code needed to load a Post struct from a SQL query.
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PostList(pub Vec<Post>);

impl PostList {
    // 获得post列表
    pub fn list(connection: &PgConnection) -> Self {
        use crate::schema::posts::dsl::*;

        let result = posts
            .limit(10)
            .load::<Post>(connection)
            .expect("Error loading posts");

        PostList(result)
    }
}

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

// // 获得所有已发布的帖子
// pub fn get_all(_req: HttpRequest, pool: web::Data<MysqlPool>) -> Result<Vec<Post>, io::Error> {
//     use crate::schema::posts::dsl::{posts, published};

//     let connection = mysql_pool_handler(pool).unwrap();
//     let all_posts = posts
//         .filter(published.eq(true))
//         .limit(5)
//         .load::<Post>(&connection)
//         .expect("Error loading posts");

//     Ok(all_posts.into())
// }

// // 提交一个post
// pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) {
//     let new_post = NewPost { title, body };

//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(conn)
//         .expect("Error saving new post");
// }
