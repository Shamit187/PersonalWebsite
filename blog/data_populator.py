import sqlite3
from datetime import date

# Connect to the SQLite database
conn = sqlite3.connect("blog_database.db")
cursor = conn.cursor()

# Insert fake topics
topics = [
    ("Tech",),
    ("Travel",),
    ("Lifestyle",),
    ("Food",),
]
cursor.executemany("INSERT INTO topic (topic_name) VALUES (?)", topics)

# Insert fake files (blog posts)
files = [
    ("tech-news", "Latest Tech News", date.today().isoformat(), "Stay updated with the latest in technology.", "https://picsum.photos/1920/1080"),
    ("travel-adventure", "Travel Adventure", date.today().isoformat(), "Explore the world with these amazing travel stories.", "https://picsum.photos/1920/1080"),
    ("lifestyle-tips", "Lifestyle Tips", date.today().isoformat(), "Improve your lifestyle with these tips.", "https://picsum.photos/1920/1080"),
    ("food-recipes", "Delicious Food Recipes", date.today().isoformat(), "Discover mouth-watering recipes.", "https://picsum.photos/1920/1080"),
]
cursor.executemany(
    "INSERT INTO file (file_name, title, date, description, image_url) VALUES (?, ?, ?, ?, ?)",
    files
)

# Commit to save the inserted topics and files
conn.commit()

# Retrieve the generated IDs for topics and files
cursor.execute("SELECT topic_id, topic_name FROM topic")
topic_map = {row[1]: row[0] for row in cursor.fetchall()}

cursor.execute("SELECT file_id, file_name FROM file")
file_map = {row[1]: row[0] for row in cursor.fetchall()}

# Insert linking rows in the topic_file table with a defined order_num
topic_file_entries = [
    (topic_map["Tech"], file_map["tech-news"], 1),
    (topic_map["Travel"], file_map["travel-adventure"], 1),
    (topic_map["Lifestyle"], file_map["lifestyle-tips"], 1),
    (topic_map["Food"], file_map["food-recipes"], 1),
    # Optionally, link a blog to multiple topics
    (topic_map["Tech"], file_map["lifestyle-tips"], 2),
]

cursor.executemany(
    "INSERT INTO topic_file (topic_id, file_id, order_num) VALUES (?, ?, ?)",
    topic_file_entries
)

# Commit changes and close the connection
conn.commit()
conn.close()

print("Fake data inserted successfully!")
