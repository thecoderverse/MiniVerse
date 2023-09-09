import typer 
from __init__ import __app_name__, __version__
import book 
app=typer.Typer()

@app.command()
def build(): 
    #command'lar yer alsın
    pass
    
    
@app.command()
def goodbye():
    typer.echo(f"{__app_name__} v{__version__}")
    
        
    while True:
        print("Aradığınız kelimeyi yazınız: ")
        
        user_input = input()
        if user_input.lower() == 'q':
            break
        else:
            my_book = book.Book() 
            my_book.getmybook(name=user_input)
            print("Arama tamamlandı. Çıkmak için 'q' tuşuna basın.")


if __name__=="__main__":
    
    app()