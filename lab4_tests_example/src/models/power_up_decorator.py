from abc import ABC, abstractmethod
from src.models.weapon import Weapon

class PowerUpDecorator(Weapon):
    def __init__(self, weapon):
        self.weapon = weapon

    def attack(self, attacker, target):
        return self.weapon.attack(attacker, target)