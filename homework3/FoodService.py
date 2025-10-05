import threading

class FoodService:
    def __init__(self) -> None:
        self.orders = []

    def add_order(self, order):
        self.orders.append(order)

    def process_orders(self):
        for order in self.orders:
            order.make_order()