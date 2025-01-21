use rusqlite::{params, Connection, OptionalExtension, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Blog {
    pub slug: String,
    pub path: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub _audio: Option<String>,
    pub click_count: i32,
    pub created_at: String,
    pub _updated_at: String,
    pub background_image: Option<String>,
    pub tags: Vec<Tag>, // Add tags field to Blog
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: u32,
    pub name: String,
}

pub struct Database {
    pub connection: Connection,
}

impl Database {
    // Initialize the database and create schema
    pub fn new(db_path: &str) -> Result<Self> {
        let connection = Connection::open(db_path)?;
        Ok(Self { connection })
    }

    // Fetch all blogs
    pub fn fetch_all_blogs(&self) -> Result<Vec<Blog>> {
        let mut stmt = self.connection.prepare(
            "SELECT f.slug, f.path, f.title, f.author, f.description, f.audio, 
                    f.click_count, f.created_at, f.updated_at, f.background_image, 
                    t.id AS tag_id, t.name AS tag_name
             FROM files f
             LEFT JOIN file_tags ft ON f.slug = ft.file_slug
             LEFT JOIN tags t ON ft.tag_id = t.id
             ORDER BY f.created_at DESC;",
        )?;

        let mut blogs_map: HashMap<String, Blog> = HashMap::new();

        // Iterate through rows and build the Blog and Tag data
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let slug: String = row.get("slug")?;
            let tag_id: Option<u32> = row.get("tag_id")?;
            let tag_name: Option<String> = row.get("tag_name")?;

            // Check if the blog is already in the map
            if !blogs_map.contains_key(&slug) {
                blogs_map.insert(
                    slug.clone(),
                    Blog {
                        slug: slug.clone(),
                        path: row.get("path")?,
                        title: row.get("title")?,
                        author: row.get("author")?,
                        description: row.get("description")?,
                        _audio: row.get("audio")?,
                        click_count: row.get("click_count")?,
                        created_at: row.get("created_at")?,
                        _updated_at: row.get("updated_at")?,
                        background_image: row.get("background_image")?,
                        tags: Vec::new(),
                    },
                );
            }

            // Add the tag to the blog if it's not null
            if let (Some(id), Some(name)) = (tag_id, tag_name) {
                blogs_map
                    .get_mut(&slug)
                    .unwrap()
                    .tags
                    .push(Tag { id, name });
            }
        }

        // Convert the map into a vector of blogs
        Ok(blogs_map.into_values().collect())
    }

    // Fetch a single blog by slug
    pub fn fetch_blog_by_slug(&self, slug: &str) -> Result<Option<Blog>> {
        // Fetch the blog data
        let mut stmt = self.connection.prepare(
        "SELECT slug, path, title, author, description, audio, click_count, created_at, updated_at, background_image
         FROM files WHERE slug = ?1;",
    )?;

        let blog_result = stmt
            .query_row([slug], |row| {
                Ok(Blog {
                    slug: row.get(0)?,
                    path: row.get(1)?,
                    title: row.get(2)?,
                    author: row.get(3)?,
                    description: row.get(4)?,
                    _audio: row.get(5)?,
                    click_count: row.get(6)?,
                    created_at: row.get(7)?,
                    _updated_at: row.get(8)?,
                    background_image: row.get(9)?,
                    tags: Vec::new(), // Placeholder for tags
                })
            })
            .optional()?;

        // If no blog is found, return None
        if let Some(mut blog) = blog_result {
            // Fetch the tags for the blog
            let mut tag_stmt = self.connection.prepare(
                "SELECT t.id, t.name
             FROM tags t
             INNER JOIN file_tags ft ON t.id = ft.tag_id
             WHERE ft.file_slug = ?1;",
            )?;

            let tags = tag_stmt
                .query_map([slug], |row| {
                    Ok(Tag {
                        id: row.get(0)?,
                        name: row.get(1)?,
                    })
                })?
                .collect::<Result<Vec<_>>>()?;

            // Add the tags to the blog
            blog.tags = tags;

            Ok(Some(blog))
        } else {
            Ok(None)
        }
    }

    // Increment click count for a blog
    pub fn increment_click_count(&self, slug: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE files SET click_count = click_count + 1 WHERE slug = ?1;",
            [slug],
        )?;
        Ok(())
    }

    pub fn fetch_blogs_in_range(&self, start: i64, end: i64) -> Result<Vec<Blog>> {
        let mut stmt = self.connection.prepare(
            "SELECT f.slug, f.path, f.title, f.author, f.description, f.audio, 
                    f.click_count, f.created_at, f.updated_at, f.background_image, 
                    t.id AS tag_id, t.name AS tag_name
             FROM files f
             LEFT JOIN file_tags ft ON f.slug = ft.file_slug
             LEFT JOIN tags t ON ft.tag_id = t.id
             GROUP BY f.slug
             ORDER BY f.created_at DESC
             LIMIT ?1, ?2;",
        )?;
        let mut blogs_map: HashMap<String, Blog> = HashMap::new();
        // Iterate through rows and build the Blog and Tag data
        let mut rows = stmt.query([start, end - start + 1])?;


        while let Some(row) = rows.next()? {
            let slug: String = row.get("slug")?;
            let tag_id: Option<u32> = row.get("tag_id")?;
            let tag_name: Option<String> = row.get("tag_name")?;

            if !blogs_map.contains_key(&slug) {
                blogs_map.insert(
                    slug.clone(),
                    Blog {
                        slug: slug.clone(),
                        path: row.get("path")?,
                        title: row.get("title")?,
                        author: row.get("author")?,
                        description: row.get("description")?,
                        _audio: row.get("audio")?,
                        click_count: row.get("click_count")?,
                        created_at: row.get("created_at")?,
                        _updated_at: row.get("updated_at")?,
                        background_image: row.get("background_image")?,
                        tags: Vec::new(),
                    },
                );
            }

            if let (Some(id), Some(name)) = (tag_id, tag_name) {
                blogs_map
                    .get_mut(&slug)
                    .unwrap()
                    .tags
                    .push(Tag { id, name });
            }
        }

        Ok(blogs_map.into_values().collect())
    }
}
