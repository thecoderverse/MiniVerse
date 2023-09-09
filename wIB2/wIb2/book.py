import os
from multiprocessing import Pool, cpu_count


class Book:
    def __init__(self):
        self.name = None  
        self.rr_files = []
    def getmybook(self, name):    
        self.name = name
        aranan_dosya_adi = name
        arama_dizini = os.path.expanduser("~")
        
        pool = Pool(processes=cpu_count()) 

        for root, dirs, files in os.walk(arama_dizini):
            for file in files:
                
                    pool.apply_async(self.search_file, args=(file, aranan_dosya_adi, root))
        pool.close()
        pool.join()
                  
                
    def search_file(self, file, names, root):
        if names in file:
            dosya_yolu = os.path.join(root, file)
            self.rr_files.append(dosya_yolu)
            print("Dosya Bulundu:", dosya_yolu)
        return self.rr_files
         

  
            