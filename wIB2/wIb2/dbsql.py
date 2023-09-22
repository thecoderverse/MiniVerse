import sqlite3

connection = sqlite3.connect("mydatabase.db")
cursor = connection.cursor()

class db: 
    
    @staticmethod
    def createsql():
        cursor.execute('''CREATE TABLE IF NOT EXISTS books (
                          id INTEGER PRIMARY KEY AUTOINCREMENT,
                          bookname TEXT,
                          path TEXT)''')
        connection.commit()
   
    @staticmethod
    def insertsql(book_name, filepath):
        cursor.execute("INSERT INTO books (bookname, path) VALUES (?, ?)", (book_name, filepath)) # Burada path'i belirli bir değer olarak vermelisiniz
        connection.commit()
    
    @staticmethod
    def deletebook(book):
        cursor.execute("DELETE FROM books WHERE bookname=?", (book,))
        connection.commit()

    @staticmethod
    def listbooks():
        cursor.execute("SELECT id, bookname, path FROM books")
        books = cursor.fetchall()
        return books
 
    @staticmethod
    def getonebook(name): 
        cursor.execute("SELECT bookname, path FROM books WHERE bookname=?", (name,))
        book = cursor.fetchone()
       
        return book
    