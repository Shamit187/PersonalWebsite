use axum::{
    body::Body, extract::Path, extract::State, http::StatusCode, response::Html,
    response::Response, routing::get, Router,
};
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Instant;

mod database;
use database::Database;

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
        .route("/favicon.ico", get(favicon))
        .with_state(Arc::new(app_state));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Helper function to create a bad response
fn bad_response() -> Response {
    Response::builder()
        .status(500)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body("Internal Server Error".into())
        .unwrap()
}

// Handler for the root route
async fn root() -> Response {
    println!("/ API called");
    let html_head = match fs::read_to_string("pages/index.head") {
        Ok(content) => content,
        Err(_) => return bad_response(),
    };

    let html_body = match fs::read_to_string("pages/index.body") {
        Ok(content) => content,
        Err(_) => return bad_response(),
    };

    let css = match fs::read_to_string("pages/index.css") {
        Ok(content) => content,
        Err(_) => return bad_response(),
    };

    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            {}
            <style>{}</style>
        </head>
        <body>
            {}
        </body>
        </html>
        "#,
        html_head, css, html_body
    );

    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(html_content.into())
        .unwrap()
}

// Handler for a specific blog post
async fn blog_post(Path(slug): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    let start_time = Instant::now();
    let db = state.db.lock().await;

    if let Err(_) = db.increment_click_count(&slug) {
        return bad_response();
    }
    let blog = match db.fetch_blog_by_slug(&slug) {
        Ok(blog) => blog,
        Err(_) => return bad_response(),
    };

    let response = match blog {
        Some(blog) => {
            // Fetch the content from the file
            let file_path = format!("files/{}", blog.path);
            let file_content = match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(_) => return bad_response(),
            };

            let background_image_url = blog.background_image.unwrap_or_else(|| {
                "https://picsum.photos/1920/1080".to_string()
            });

            let parsed_content = parser::markdown_to_html(
                &file_content,
                &background_image_url,
                &blog.title,
                &blog.author,
                &blog.created_at,
            );

            if let Some(parsed_content) = parsed_content {
                Response::builder()
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(parsed_content.into())
                    .unwrap()
            } else {
                bad_response()
            }
        }
        None => Response::builder()
            .header("Content-Type", "text/html; charset=utf-8")
            .body("<h1>Blog not found</h1>".to_string().into())
            .unwrap(),
    };
    let duration = start_time.elapsed();
    println!("/blog/{slug} API called: Time elapsed in blog_post: {}ms", duration.as_secs_f64() * 1000.0);
    response
}

// Handler for the /get-list/{start}/{end} API
async fn get_list(
    Path((start, end)): Path<(i64, i64)>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    println!("/get-list/{start}/{end} API called");

    let db = state.db.lock().await;

    // Fetch blogs in the range
    let blogs = db
        .fetch_blogs_in_range(start, end)
        .expect("Failed to fetch blogs");

    let total_rows = db
        .count_total_rows()
        .expect("Failed to fetch total rows") - 1;

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
    let previous_start = if start - 9 < 0 { 0 } else { start - 9 };
    let previous_end = previous_start + 9;
    let next_end = if end + 9 > total_rows {total_rows } else {end + 9};
    let next_start = next_end - 9;
    let previous_disabled = if start == 0 { "disabled" } else { "" };
    let next_disabled = if end == total_rows { "disabled" } else { "" };

    Html(format!(
        r##"
        <div class="blog-list flex-grow flex flex-col gap-4" id="blogList">
            {}
            <div class="foot">
                <button class="foot-button" id="previousButton" 
                    hx-get="/get-list/{}/{}" 
                    hx-target="#blogList" 
                    hx-trigger="click" 
                    hx-swap="outerHTML"
                    {}
                >
                    Previous
                </button>
                <div class="foot-info" id="footInfo">Showing: {} - {}</div>
                <button class="foot-button" id="nextButton" 
                    hx-get="/get-list/{}/{}" 
                    hx-target="#blogList" 
                    hx-trigger="click"
                    hx-swap="outerHTML"
                    {}
                >
                    Next
                </button>
            </div>
        </div>
        "##,
        html_content, previous_start, previous_end, previous_disabled, start, end, next_start, next_end, next_disabled
    ))
}

async fn favicon() -> Result<Response<Body>, (StatusCode, &'static str)> {
    println!("/favicon.ico API called");
    let file_path = "pages/favicon.ico"; // Path to the favicon

    match fs::read(file_path) {
        Ok(contents) => Ok(Response::builder()
            .header("Content-Type", "image/x-icon")
            .body(Body::from(contents))
            .unwrap()),
        Err(_) => Err((StatusCode::NOT_FOUND, "Favicon not found")),
    }
}
