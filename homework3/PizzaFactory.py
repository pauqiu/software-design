import random
import time

class Pizza:
    def __init__(self, pizza_type):
        self.pizza_type = pizza_type
        self.baked = False

class PizzaFactory():
    PIZZA_TYPES = ['Margherita', 'Pepperoni', 'Veggie', 'Hawaiian', 'BBQ Chicken']

    def __init__(self):
        pass

    def create_order(self):
        pizza_type = random.choice(self.PIZZA_TYPES)
        pizza = Pizza(pizza_type)
        return pizza

    def make_order(self, pizza):
        print(f"Baking {pizza.pizza_type} pizza...")
        time.sleep(1)
        pizza.baked = True
        print(f"{pizza.pizza_type} pizza is ready!")