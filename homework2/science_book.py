from book import Book

class ScienceBook(Book):

    def calculate_popularity(self):
        base = 70
        extra = self.pages / 5
        return base + extra