use rusqlite::{params, Connection, Result, OptionalExtension};

#[derive(Debug, Clone)]
pub struct Blog {
    pub slug: String,
    pub path: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub audio: Option<String>,
    pub click_count: i32,
    pub created_at: String,
    pub updated_at: String,
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
            "SELECT slug, title, author, description, created_at, click_count FROM files;"
        )?;
        let blogs = stmt.query_map([], |row| {
            Ok(Blog {
                slug: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                description: row.get(3)?,
                audio: None,              
                created_at: row.get(4)?,
                path: String::new(),      
                click_count: row.get(5)?,           
                updated_at: String::new() 
            })
        })?;
    
        blogs.collect()
    }

    // Fetch a single blog by slug
    pub fn fetch_blog_by_slug(&self, slug: &str) -> Result<Option<Blog>> {
        let mut stmt = self.connection.prepare(
            "SELECT slug, path, title, author, description, audio, click_count, created_at, updated_at
             FROM files WHERE slug = ?1;"
        )?;
        stmt.query_row([slug], |row| {
            Ok(Blog {
                slug: row.get(0)?,
                path: row.get(1)?,
                title: row.get(2)?,
                author: row.get(3)?,
                description: row.get(4)?,
                audio: row.get(5)?,
                click_count: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .optional()
    }

    // Fetch tags for a blog
    pub fn fetch_tags_for_blog(&self, slug: &str) -> Result<Vec<Tag>> {
        let mut stmt = self.connection.prepare(
            "SELECT t.id, t.name
             FROM tags t
             INNER JOIN file_tags ft ON t.id = ft.tag_id
             WHERE ft.file_slug = ?1;",
        )?;
        let tags = stmt.query_map([slug], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        tags.collect()
    }

    // Increment click count for a blog
    pub fn increment_click_count(&self, slug: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE files SET click_count = click_count + 1 WHERE slug = ?1;",
            [slug],
        )?;
        Ok(())
    }
}
