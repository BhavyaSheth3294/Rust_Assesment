use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use std::error::Error;

// Define the structure for post data based on the API response
#[derive(Debug, Deserialize)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Fetch data from API
    let posts = fetch_data_from_api()?;

    // Create SQLite connection
    let conn = Connection::open("posts.db")?;

    // Create the posts table
    create_table(&conn)?;

    // Insert data into the database
    for post in &posts {
        insert_post(&conn, post)?;
    }

    // Retrieve data from the database and print it
    let stored_posts = get_all_posts(&conn)?;
    println!("Stored posts:");
    for post in stored_posts {
        println!("{:?}", post);
    }

    Ok(())
}

// Function to fetch data from JSONPlaceholder API
fn fetch_data_from_api() -> Result<Vec<Post>, Box<dyn Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts";
    let response = reqwest::blocking::get(url)?;
    let posts: Vec<Post> = response.json()?;
    Ok(posts)
}

// Function to create the posts table
fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts (
                  id INTEGER PRIMARY KEY,
                  userId INTEGER NOT NULL,
                  title TEXT NOT NULL,
                  body TEXT NOT NULL
              )",
        [],
    )?;
    Ok(())
}

// Function to insert a post into the database
fn insert_post(conn: &Connection, post: &Post) -> Result<()> {
    conn.execute(
        "INSERT INTO posts (id, userId, title, body) VALUES (?1, ?2, ?3, ?4)",
        params![post.id, post.userId, post.title, post.body],
    )?;
    Ok(())
}

// Function to retrieve all posts from the database
fn get_all_posts(conn: &Connection) -> Result<Vec<Post>> {
    let mut stmt = conn.prepare("SELECT id, userId, title, body FROM posts")?;
    let posts_iter = stmt.query_map([], |row| {
        Ok(Post {
            id: row.get(0)?,
            userId: row.get(1)?,
            title: row.get(2)?,
            body: row.get(3)?,
        })
    })?;

    let mut posts = Vec::new();
    for post in posts_iter {
        posts.push(post?);
    }
    Ok(posts)
}
