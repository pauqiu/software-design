from src.models.weapon import Weapon

class Bow(Weapon):
    def attack(self, attacker, target):
        damage = 12
        return damage, f"{attacker.name} dispara flecha a {target.name} "