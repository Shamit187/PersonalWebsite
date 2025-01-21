use axum::{
    body::Body,
    http::StatusCode,
    extract::State,
    extract::Path,
    response::Html,
    response::Response,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs;

mod database;
use database::{Database, Blog};

mod parser;

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
        .route("/get-list/{start}/{end}", get(get_list))
        // .route("/favicon.ico", get(favicon))
        .with_state(Arc::new(app_state));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for the root route
async fn root() -> Response {
    // Path to the HTML file
    let file_path = "pages/index.html";

    // Read the file
    match fs::read_to_string(file_path) {
        Ok(content) => Response::builder()
            .header("Content-Type", "text/html; charset=utf-8")
            .body(content.into())
            .unwrap(),
        Err(_) => Response::builder()
            .status(500)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body("Failed to load the index.html file.".into())
            .unwrap(),
    }
}

// Handler for a specific blog post
async fn blog_post(Path(slug): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    let db = state.db.lock().await;

    // Increment click count and fetch the blog
    db.increment_click_count(&slug).expect("Failed to increment click count");
    let blog = db.fetch_blog_by_slug(&slug).expect("Failed to fetch blog");

    match blog {
        Some(blog) => {
            // Fetch the content from the file
            let file_path = format!("files/{}", blog.path);
            let file_content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| "Sorry, the blog content is not available".to_string());
            // let background_image_url = "https://picsum.photos/1920/1080"; // Placeholder image
            let background_image_url = if blog.background_image.is_some() {
                blog.background_image.unwrap()
            } else {
                "https://picsum.photos/1920/1080".to_string()
            };

            let parsed_content = parser::markdown_to_html(
                &file_content,
                &background_image_url,
                &blog.title,
                &blog.author,
                &blog.created_at,
            );

            // print parsed content into a file

            // Serve the parsed content as-is
            Response::builder()
                .header("Content-Type", "text/html; charset=utf-8")
                .body(parsed_content.into())
                .unwrap()
        }
        None => Response::builder()
            .header("Content-Type", "text/html; charset=utf-8")
            .body("<h1>Blog not found</h1>".to_string().into())
            .unwrap(),
    }
}

// Handler for the /get-list/{start}/{end} API
async fn get_list(
    Path((start, end)): Path<(i64, i64)>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    println!("start: {}, end: {}", start, end);

    let db = state.db.lock().await;

    // Fetch blogs in the range
    let blogs = db
        .fetch_blogs_in_range(start, end)
        .expect("Failed to fetch blogs");

    // Build the HTML string
    let html_content = blogs
        .into_iter()
        .map(|blog| {
            let tags_html = blog
                .tags
                .into_iter()
                .map(|tag| format!(r#"<div class="blog-tag">{}</div>"#, tag.name))
                .collect::<Vec<_>>()
                .join("");

            format!(
                r#"
                <div class="blog" onclick="location.href='/blog/{}';">
                    <div class="blog-title">{}</div>
                    <div class="blog-date">Created In: {}</div>
                    <div class="blog-description">{}</div>
                    <div class="blog-tag-list">{}</div>
                </div>
                "#,
                blog.slug, blog.title, blog.created_at, blog.description, tags_html
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    Html(html_content)
}

// async fn favicon() -> Result<Response<Body>, (StatusCode, &'static str)> {
//     let file_path = "pages/favicon.ico"; // Path to the favicon

//     match fs::read(file_path) {
//         Ok(contents) => Ok(Response::builder()
//             .header("Content-Type", "image/x-icon")
//             .body(Body::from(contents))
//             .unwrap()),
//         Err(_) => Err((StatusCode::NOT_FOUND, "Favicon not found")),
//     }
// }