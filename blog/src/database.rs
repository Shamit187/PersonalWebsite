use rusqlite::{Connection, Result};

#[derive(Debug, Clone)]
pub struct Blog {
    pub file_name: String,   // serves as the slug
    pub file_id: i32,
    pub title: String,
    pub date: String,        // you might want to change this to a proper date type
    pub description: String,
    pub image_url: String,
}

#[derive(Debug, Clone)]
pub struct Topic {
    pub topic_id: i32,
    pub topic_name: String,
}

pub struct Database {
    pub connection: Connection,
}

impl Database {
    /// Opens a connection to the given database file.
    pub fn new(db_path: &str) -> Result<Self> {
        let connection = Connection::open(db_path)?;
        Ok(Self { connection })
    }

    /// Fetch a single blog by its slug (which is the file_name in the file table)
    pub fn fetch_blog_by_id(&self, file_id: &str) -> Result<Option<Blog>> {
        let mut stmt =match self.connection.prepare(
            "SELECT file_name, file_id, title, date, description, image_url
             FROM file
             WHERE file_id = ?1;"
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Error preparing statement for blog {}: {}", file_id, e);
                return Err(e);
            }
        };
        let blog =match stmt.query_row([file_id], |row| {
            Ok(Blog {
                file_name: row.get(0)?,
                file_id: row.get(1)?,
                title: row.get(2)?,
                date: row.get(3)?,
                description: row.get(4)?,
                image_url: row.get(5)?,
            })
        }) {
            Ok(blog) => Some(blog),
            Err(e) => {
                if e == rusqlite::Error::QueryReturnedNoRows {
                    println!("No blog found for id {}", file_id);
                    None
                } else {
                    println!("Error fetching blog {}: {}", file_id, e);
                    return Err(e);
                }
            }
        };
        Ok(blog)
    }

    /// Fetch all topics from the topic table.
    pub fn fetch_topics(&self) -> Result<Vec<Topic>> {
        let mut stmt = self.connection.prepare(
            "SELECT topic_id, topic_name FROM topic;"
        )?;
        let topic_iter = match stmt.query_map([], |row| {
            Ok(Topic {
                topic_id: row.get(0)?,
                topic_name: row.get(1)?,
            })
        }) {
            Ok(iter) => iter,
            Err(e) => {
                println!("Error executing query_map for topics: {}", e);
                return Err(e);
            }
        };
        let mut topics = Vec::new();
        for topic in topic_iter {
            topics.push(topic?);
        }
        Ok(topics)
    }

    /// Fetch all blogs for a given topic, ordered by the "order_num" field in the topic_file table.
    pub fn fetch_blogs_by_topic(&self, topic_id: i32) -> Result<Vec<Blog>> {
        println!("Fetching blogs for topic {}", topic_id);
    
        // Prepare the SQL statement with error logging.
        let mut stmt = match self.connection.prepare(
            "SELECT f.file_name, f.file_id, f.title, f.date, f.description, f.image_url
             FROM file f
             INNER JOIN topic_file tf ON f.file_id = tf.file_id
             WHERE tf.topic_id = ?1
             ORDER BY tf.order_num ASC;"
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Error preparing statement for topic {}: {}", topic_id, e);
                return Err(e);
            }
        };
    
        // Execute the query and log any errors.
        let blog_iter = match stmt.query_map([topic_id], |row| {
            Ok(Blog {
                file_name: row.get(0)?,
                file_id: row.get(1)?,
                title: row.get(2)?,
                date: row.get(3)?,
                description: row.get(4)?,
                image_url: row.get(5)?,
            })
        }) {
            Ok(iter) => iter,
            Err(e) => {
                println!("Error executing query_map for topic {}: {}", topic_id, e);
                return Err(e);
            }
        };
    
        // Iterate through the results, logging any errors for individual rows.
        let mut blogs = Vec::new();
        for blog_result in blog_iter {
            match blog_result {
                Ok(blog) => {
                    println!("Fetched blog: {:?}", blog);
                    blogs.push(blog);
                }
                Err(e) => {
                    println!("Error processing a blog row for topic {}: {}", topic_id, e);
                    return Err(e);
                }
            }
        }
    
        println!("Total blogs fetched for topic {}: {}", topic_id, blogs.len());
        Ok(blogs)
    }
    
}
