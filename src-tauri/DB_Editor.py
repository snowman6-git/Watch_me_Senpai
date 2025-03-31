import sqlite3


db = sqlite3.connect(f"Data.db", check_same_thread=False)
db_command = db.cursor()

db_command.execute("")