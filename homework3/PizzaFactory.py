import random
import time

class Pizza:
    def __init__(self, pizza_type):
        self.pizza_type = pizza_type
        self.baked = False

    def bake(self):
        print(f"Baking {self.pizza_type} pizza...")
        time.sleep(1)
        self.baked = True
        print(f"{self.pizza_type} pizza is ready!")

class PizzaFactory:
    PIZZA_TYPES = ['Margherita', 'Pepperoni', 'Veggie', 'Hawaiian', 'BBQ Chicken']

    def __init__(self):
        self.orders = []

    def create_order(self):
        pizza_type = random.choice(self.PIZZA_TYPES)
        pizza = Pizza(pizza_type)
        self.orders.append(pizza)

    def make_pizza(self, pizza):
        pizza.bake()