from library import Library

class Report:
    def __init__(self, lib):
        self.lib = lib

    def generate_report(self):
        total = self.lib.count_books()
        antiguos = self.lib.count_old_books()
        disponibles = self.lib.count_available_books()

        for b in self.lib.books:
            total_popularity = 0
            b.print_data()
            total_popularity += (b.calculate_popularity())

        print("\nREPORTE BIBLIOTECA:")
        print(f"Total libros: {total}")
        print(f"Disponibles: {disponibles}")
        print(f"Antiguos: {antiguos}")
        print(f"Promedio de popularidad: {total_popularity / total if total > 0 else 0}")