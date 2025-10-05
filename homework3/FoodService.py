import threading
#from HamburgerFactory import HamburgerFactory
#from PizzaFactory import PizzaFactory

class FoodService:
    def __init__(self) -> None:
        self.orders = []

    def add_order(self, order):
        self.orders.append(order)

    def process_orders(self):
        # this is where threads execute
        pass