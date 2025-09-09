from book import Book

class Library:
    def __init__(self): 
        self.books = []

    def add_book(self, title, author, genre, pages, year) :
        recent_book = Book(title, author, genre, pages, year)
        self.libros.append(recent_book)
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
        for b in self.books:
            if b.is_old():
                olds += 1

        return olds
            
    def count_available_books(self):
        for b in self.books:
            if b.available:
                availables += 1

        return availables