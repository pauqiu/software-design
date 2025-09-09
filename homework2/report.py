from library import Library

class Report:
    def __init__(self):
        pass

    def generate_report(self):
        total = len(Library.count_books())
        antiguos = Library.count_old_books()
        disponibles = Library.count_available_books()

        for b in Library.books:
            b.print_data()
            total_popularity += (b.calculate_popularity())

        print("\nREPORTE BIBLIOTECA:")
        print(f"Total libros: {total}")
        print(f"Disponibles: {disponibles}")
        print(f"Antiguos: {antiguos}")
        print(f"Promedio de popularidad: {total_popularity / total if total > 0 else 0}")