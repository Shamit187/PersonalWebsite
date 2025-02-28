import os
import random
import sqlite3
from datetime import datetime, timedelta

# Database and files directory
DATABASE = "database.db"  # Replace with your actual database name
FILES_DIR = "files"
NUM_ENTRIES = 100  # Number of entries to generate

# Ensure the files directory exists
os.makedirs(FILES_DIR, exist_ok=True)

# Function to generate random date within a range
def random_date(days_back):
    return datetime.now() - timedelta(days=random.randint(0, days_back))

# Connect to the SQLite database
conn = sqlite3.connect(DATABASE)
cursor = conn.cursor()

# Populate the tags table
tags = ["Technology", "Health", "Education", "Lifestyle"]
cursor.executemany("""
    INSERT OR IGNORE INTO tags (name)
    VALUES (?)
""", [(tag,) for tag in tags])

print(f"Inserted {len(tags)} tags into the tags table.")

# Generate random entries for the files table
print(f"Generating {NUM_ENTRIES} entries...")

for i in range(1, NUM_ENTRIES + 1):
    # Generate random values
    slug = f"blog_{i}"
    path = f"{slug}.md"
    title = f"Test Blog {i}"
    description = f"This is a description for Test Blog {i}"
    click_count = random.randint(0, 100)
    author = f"Author_{random.randint(1, 10)}"
    created_at = random_date(365).strftime("%Y-%m-%d %H:%M:%S")
    updated_at = random_date(30).strftime("%Y-%m-%d %H:%M:%S")
    background_image = f"https://picsum.photos/seed/{i}/1920/1080"
    tags_count = random.randint(1, 4)

    # Generate a random markdown file
    md_content = f"# {title}\n\n{description}\n\nCreated by {author} on {created_at}."
    with open(os.path.join(FILES_DIR, path), "w") as md_file:
        md_file.write(md_content)

    # Insert into files table
    cursor.execute("""
        INSERT INTO files (slug, path, title, description, click_count, created_at, updated_at, author, background_image)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    """, (slug, path, title, description, click_count, created_at, updated_at, author, background_image))

    # Assign random tags
    for _ in range(tags_count):
        tag_id = random.randint(1, len(tags))  # Adjusted to match the number of tags in the table
        cursor.execute("""
            INSERT OR IGNORE INTO file_tags (file_slug, tag_id)
            VALUES (?, ?)
        """, (slug, tag_id))

# Commit changes and close the database connection
conn.commit()
conn.close()

print(f"Generated {NUM_ENTRIES} blog entries and associated markdown files.")
