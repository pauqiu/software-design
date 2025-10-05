import random
import time

class Hamburger:
    def __init__(self, bun=True, patty=True, lettuce=False, tomato=False, cheese=False):
        self.bun = bun
        self.patty = patty
        self.lettuce = lettuce
        self.tomato = tomato
        self.cheese = cheese

    def __str__(self):
        ingredients = []
        if self.bun: ingredients.append("Bun")
        if self.patty: ingredients.append("Patty")
        if self.lettuce: ingredients.append("Lettuce")
        if self.tomato: ingredients.append("Tomato")
        if self.cheese: ingredients.append("Cheese")
        return "Hamburger with " + ", ".join(ingredients)

class HamburgerFactory():
    def __init__(self):
        pass

    def create_order(self):
        lettuce = random.choice([True, False])
        tomato = random.choice([True, False])
        cheese = random.choice([True, False])
        burger = Hamburger(lettuce=lettuce, tomato=tomato, cheese=cheese)
        return burger

    def make_order(self, burger):
        time.sleep(1)
        print(f"Made: {burger}")