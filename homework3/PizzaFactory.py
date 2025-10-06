import random
import time
import threading

print_lock = threading.Lock()

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
        with print_lock:
            print(f"[{threading.current_thread().name}] baking {pizza.pizza_type} pizza...")
        time.sleep(1)
        pizza.baked = True
        with print_lock:
            print(f"[{threading.current_thread().name}] {pizza.pizza_type} pizza is ready!")