# main file just for testing purposes

from library import Library
from report import Report

def main():
    lib = Library()
    lib.add_book("Crime and Punishment", "Fyodor Dostoevsky", "Novel", 650, 1866)
    lib.add_book("No longer Human", "Dazai Osamu", "Novel", 177, 1958)

    reprt = Report(lib)
    reprt.generate_report()

if __name__ == "__main__":
    main()