import sqlite3
import os
import shutil

# Constants
DATABASE_PATH = "../blog/blog_database.db"
FILES_FOLDER = "../blog/files"
BACKGROUND_IMAGE_FILE = "background_image.txt"

def connect_database():
    """Connect to the SQLite database."""
    try:
        conn = sqlite3.connect(DATABASE_PATH)
        return conn
    except sqlite3.Error as e:
        print(f"Database error: {e}")
        return None

def check_slug_unique(conn, slug):
    """Check if the file name (slug) is unique."""
    cursor = conn.cursor()
    cursor.execute("SELECT slug FROM files WHERE slug = ?", (slug,))
    return cursor.fetchone() is None

def add_tags(conn, tags):
    """Ensure tags exist in the tags table and return their IDs."""
    cursor = conn.cursor()
    tag_ids = []
    for tag in tags:
        cursor.execute("INSERT OR IGNORE INTO tags (name) VALUES (?)", (tag,))
        cursor.execute("SELECT id FROM tags WHERE name = ?", (tag,))
        tag_id = cursor.fetchone()[0]
        tag_ids.append(tag_id)
    return tag_ids

def link_tags_to_file(conn, slug, tag_ids):
    """Link tags to the file in the file_tags table."""
    cursor = conn.cursor()
    for tag_id in tag_ids:
        cursor.execute("INSERT INTO file_tags (file_slug, tag_id) VALUES (?, ?)", (slug, tag_id))

def insert_blog_metadata(conn, slug, file_name, title, description, author, background_image):
    """Insert the blog metadata into the files table."""
    cursor = conn.cursor()
    cursor.execute(
        """
        INSERT INTO files (slug, path, title, description, author, background_image)
        VALUES (?, ?, ?, ?, ?, ?)
        """,
        (slug, file_name, title, description, author, background_image),
    )

def read_background_image():
    """Read the background image content from the file."""
    if not os.path.exists(BACKGROUND_IMAGE_FILE):
        print(f"Warning: {BACKGROUND_IMAGE_FILE} not found. Setting background_image to None.")
        return None
    with open(BACKGROUND_IMAGE_FILE, "r") as file:
        return file.read().strip()

def main():
    # Connect to the database
    conn = connect_database()
    if not conn:
        return

    # Get user input
    print("Enter blog details:")
    title = input("Title: ")
    author = input("Author (default: Unknown): ") or "Unknown"
    slug = input("File name (slug): ")
    description = input("Short description: ")
    tags = input("Tags (comma-separated): ").split(",")

    # Check if slug is unique
    if not check_slug_unique(conn, slug):
        print(f"Error: The file name '{slug}' is already taken. Choose a different name.")
        conn.close()
        return

    # Read background image content
    background_image = read_background_image()

    # Prepare file paths
    markdown_file = "edit.md"
    file_name = f"{slug}.md"  # File name to store in the database
    destination_file = os.path.join(FILES_FOLDER, file_name)

    # Check if the source file exists
    if not os.path.exists(markdown_file):
        print(f"Error: Source file '{markdown_file}' does not exist.")
        conn.close()
        return

    # Copy the file to the destination
    shutil.copy(markdown_file, destination_file)
    print(f"Copied '{markdown_file}' to '{destination_file}'.")

    # Add tags to the database
    tags = [tag.strip() for tag in tags if tag.strip()]  # Clean up tags
    tag_ids = add_tags(conn, tags)

    # Insert blog metadata into the files table
    insert_blog_metadata(conn, slug, file_name, title, description, author, background_image)

    # Link tags to the file in the file_tags table
    link_tags_to_file(conn, slug, tag_ids)

    # Commit changes and close the database connection
    conn.commit()
    conn.close()
    print("Blog published successfully!")

if __name__ == "__main__":
    main()
