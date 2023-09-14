
from dbsql import db

class Book: 
    
    def Insertbook(book, filepath):
        
        try:
            db.insertsql(book, filepath)
            print("Kitap başarıyla eklendi.")
        except Exception as e:
            print(f"Hata oluştu: {e}")
    def Removebook(bb):
        try:
            db.deletebook(bb)
            print("kitap kaldırıldı")
        except Exception as e:
            print(f"Hata oluştu: {e}")     
    def ListAllBooks():
        try:
            print(db.listbooks())
        except Exception as e:
            print(f"Hata oluştu: {e}") 
    def Getbook(book): 
         
        try:
            result = db.getonebook(book)
            return result 
        except Exception as e:
            print(f"Hata oluştu: {e}")
            return None  