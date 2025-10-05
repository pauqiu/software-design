from FoodService import FoodService
from HamburgerFactory import HamburgerFactory
from PizzaFactory import PizzaFactory
from OrderWrapper import Order

def main():
    # General food service class
    service = FoodService()

    # Hamburger factory class
    burger_factory = HamburgerFactory()

    for i in range(3):
        order = burger_factory.create_order()
        service.add_order(Order(burger_factory, order))

    # Pizza factory class
    pizza_factory = PizzaFactory()
    for i in range(6):
        order = pizza_factory.create_order()
        service.add_order(Order(burger_factory, order))

    service.process_orders()
    