from history_book import HistoryBook
from novel_book import NovelBook
from science_book import ScienceBook

class Library:
    def __init__(self): 
        self.books = []

    def add_book(self, title, author, genre, pages, year) :

        if genre.lower() == "novel":
            recent_book = NovelBook(title, author, genre, pages, year)
        elif genre.lower() == "science":
            recent_book = ScienceBook(title, author, genre, pages, year)
        elif genre.lower() == "history":
            recent_book = HistoryBook(title, author, genre, pages, year)

        self.books.append(recent_book)
        print("Book registered!")

    def ask_information(self):
        title = input("Title: ")
        author = input("Author: ")
        genre = input("Genre (novel/science/history): ").lower()
        pages = int(input("Amount of pages: "))
        year = int(input("Publish year: "))

        self.add_book(title, author, genre, pages, year)

    def count_books(self):
        return len(self.books)
    
    def count_old_books(self):
        olds = 0
        for b in self.books:
            if b.is_old():
                olds += 1

        return olds
            
    def count_available_books(self):
        availables = 0
        for b in self.books:
            if b.available:
                availables += 1

        return availables