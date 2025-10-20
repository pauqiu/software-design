from src.models.power_up_decorator import PowerUpDecorator

class FirePowerUp(PowerUpDecorator):
    def attack(self, attacker, target):
        damage, result = self.weapon.attack(attacker, target)
        return damage+10, result + "con una llameante convicción "