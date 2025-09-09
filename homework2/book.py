from abc import ABC, abstractmethod

class Libro (ABC):
    def __init__(self, author, title, genre, pages, year, available=True):
        self.title = title
        self.author = author
        self.title = title
        self.genre = genre
        self.pages = pages
        self.year = year
        self.available = available

    @abstractmethod
    def calculate_popularity(self):
        pass

    def is_old(self):
        if self.year < 1980:
            return True
        else:
            return False
        
    def print_data(self):
        print(f"Título: {self.titulo}")
        print(f"Autor: {self.autor}")
        print(f"Género: {self.genero}")
        print(f"Páginas: {self.paginas}")
        print(f"Año: {self.anio_publicacion}")
        print(f"Disponible: {'Sí' if self.disponible else 'No'}")
        print(f"Popularidad: {self.calcular_popularidad()}")
        print(f"Es antiguo: {'Sí' if self.es_antiguo() else 'No'}")
        print("------------------------")
        