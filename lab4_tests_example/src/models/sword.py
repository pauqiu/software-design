from src.models.weapon import Weapon

class Sword(Weapon):
    def attack(self, attacker, target):
        damage = 15
        return damage, f"{attacker.name} ataca con espada a {target.name} "