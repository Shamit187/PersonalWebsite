use axum::{
    extract::State,
    extract::Path,
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs;

mod database;
use database::{Database, Blog};

struct AppState {
    db: Arc<Mutex<Database>>,
}

#[tokio::main]
async fn main() {
    // Initialize the database
    let db = Database::new("blog_database.db").expect("Failed to initialize database");
    let app_state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    // Set up the Axum router
    let app = Router::new()
        .route("/", get(root))
        .route("/blog/{slug}", get(blog_post))
        .with_state(Arc::new(app_state));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for the root route
async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let db = state.db.lock().await;

    // Fetch all blogs
    let blogs = db.fetch_all_blogs().expect("Failed to fetch blogs");

    let blog_list = blogs
        .into_iter()
        .map(|blog| {
            format!(
                "<li><h2><a href='/blog/{}'>{}</a></h2><p>{}</p><p><em>Clicks: {}</em></p></li>",
                blog.slug, blog.title, blog.description, blog.created_at
            )
        })
        .collect::<Vec<_>>()
        .join("");

    Html(format!("<h1>Blog List</h1><ul>{}</ul>", blog_list))
}

// Handler for a specific blog post
async fn blog_post(Path(slug): Path<String>, State(state): State<Arc<AppState>>) -> Html<String> {
    let db = state.db.lock().await;

    // Increment click count and fetch the blog
    db.increment_click_count(&slug).expect("Failed to increment click count");
    let blog = db.fetch_blog_by_slug(&slug).expect("Failed to fetch blog");

    match blog {
        Some(blog) => {
            // Fetch the content from the file
            let file_path = format!("files/{}", blog.path);
            let file_content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| "Error reading blog content".to_string());

            Html(format!(
                "<h1>{}</h1><p>{}</p><p>{}</p><p><em>Created at: {}</em></p>",
                blog.title,
                blog.description,
                file_content,
                blog.created_at,
            ))
        }
        None => Html("<h1>Blog not found</h1>".to_string()),
    }
}