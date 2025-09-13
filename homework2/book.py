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
        
    def print_data(self):
        print(f"Title: {self.title}")
        print(f"Author: {self.author}")
        print(f"Genre: {self.genre}")
        print(f"Pages: {self.pages}")
        print(f"Publish year: {self.year}")
        print(f"Available: {'Yes' if self.available else 'No'}")
        print(f"Popularity: {self.calculate_popularity()}")
        print(f"Old: {'Yes' if self.is_old() else 'No'}")
        print("------------------------")
        