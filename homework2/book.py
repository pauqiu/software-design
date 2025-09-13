from abc import ABC, abstractmethod

class Book (ABC):
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
        
    def is_available(self):
        return self.available    