#!/usr/bin/env python

import subprocess
import typer 
from __init__ import __app_name__, __version__
from bookcontroller import Book
from dbsql import db

app=typer.Typer()
  
    
@app.command()
def book():
    #while begining sql created
    db.createsql() 
    
    typer.echo(f"{__app_name__} v{__version__}")
    

@app.command()
def getbook(name:str):
    print("burası kitap getirme bloğu")
    book_name = name
    book_info = Book.Getbook(book_name)
    
    print(book_info)
    if book_info:
            file_path = r"{0}\{1}.pdf".format(book_info[1], book_info[0])
            print(f"fullypath: r{book_info[1]}\\{book_info[0]}")
            #dosyayı açar
            subprocess.Popen(["start",file_path, ], shell=True)
    else:
        print("Kitap bulunamadı.")
     
    
@app.command()
def insert(name: str, path:str):
    book = Book()  
    book_name = name
    book_path=path
    book.Insertbook(book_name,book_path)
    print("Kitap ekledi")
  
@app.command()
def list(): 
    Book.ListAllBooks() 
@app.command()
def remove(name:str): 
    remove_book=name
    Book.Removebook(remove_book)  
    print("başarı ile kaldırıdıldı") 
    Book.ListAllBooks()    
if __name__=="__main__":
    app()







