from book import Book

class HistoryBook (Book):

    def calculate_popularity(self):
        base = 40
        extra = self.pages / 8
        return base + extra