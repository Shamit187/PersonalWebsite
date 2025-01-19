use axum::{
    extract::State,
    extract::Path,
    response::Html,
    routing::get,
    Router,
};
use rusqlite::{Connection, Result, OptionalExtension};
use std::sync::Arc;
use tokio::sync::Mutex;

struct AppState {
    db_pool: Arc<Mutex<Connection>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the SQLite database
    let conn = Connection::open("blog.db")?;
    
    // Ensure the `blogs` table exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blogs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            slug TEXT UNIQUE NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )?;

    // Create the shared database pool
    let db_pool = Arc::new(Mutex::new(conn));
    let shared_state = AppState { db_pool };

    // Set up the Axum router
    let app = Router::new()
        .route("/", get(root))
        .route("/blog/{slug}", get(blog_post))
        .with_state(Arc::new(shared_state));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// Handler for the root route
async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let db = state.db_pool.clone();

    // Query to fetch all blogs
    let rows = {
        let conn = db.lock().await;
        let mut stmt = conn
            .prepare("SELECT id, title, slug, content, created_at FROM blogs")
            .expect("Failed to prepare statement");
        let rows = stmt
            .query_map([], |row| {
                Ok(format!(
                    "<li><h2><a href='/blog/{}'>{}</a></h2><p>{}</p><p><em>Created at: {}</em></p></li>",
                    row.get::<_, String>(2)?, // slug in href
                    row.get::<_, String>(1)?, // title in h2
                    row.get::<_, String>(3)?, // content in p
                    row.get::<_, String>(4)?, // created_at in em
                ))
            })
            .expect("Failed to query rows");
        rows.collect::<Result<Vec<_>, _>>().expect("Failed to collect rows")
    };

    // Build the HTML response
    let html = format!(
        "<h1>Blog List</h1><ul>{}</ul>",
        rows.join("")
    );

    Html(html)
}

async fn blog_post(Path(slug): Path<String>, State(state): State<Arc<AppState>>) -> Html<String> {
    let db = state.db_pool.clone();

    // Query the database for the blog with the given slug
    let blog = {
        let conn = db.lock().await;
        let mut stmt = conn
            .prepare("SELECT title, content, created_at FROM blogs WHERE slug = ?1")
            .expect("Failed to prepare statement");
        stmt.query_row([slug.clone()], |row| {
            Ok((
                row.get::<_, String>(0)?, // title
                row.get::<_, String>(1)?, // content
                row.get::<_, String>(2)?, // created_at
            ))
        })
        .optional()
        .expect("Failed to execute query")
    };

    // Build the HTML response based on whether the blog was found
    match blog {
        Some((title, content, created_at)) => Html(format!(
            "<h1>{}</h1><p>{}</p><p><em>Created at: {}</em></p>",
            title, content, created_at
        )),
        None => Html("<h1>Blog not found</h1>".to_string()),
    }
}
