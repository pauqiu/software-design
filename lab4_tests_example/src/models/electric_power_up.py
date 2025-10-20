from src.models.power_up_decorator import PowerUpDecorator

class ElectricPowerUp(PowerUpDecorator):
    def attack(self, attacker, target):
        damage, result = self.weapon.attack(attacker, target)
        return damage+5, result + "con eléctrica confianza "