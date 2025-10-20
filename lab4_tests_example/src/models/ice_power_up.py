from src.models.power_up_decorator import PowerUpDecorator

class IcePowerUp(PowerUpDecorator):
    def attack(self, attacker, target):
        damage, result = self.weapon.attack(attacker, target)
        return damage+2, result + "con congelante valentía "