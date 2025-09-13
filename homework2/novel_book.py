from book import Book

class NovelBook(Book):

    def calculate_popularity(self):
        base = 50
        extra = self.pages / 10
        return base + extra