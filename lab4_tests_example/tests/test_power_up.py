import unittest
from unittest.mock import MagicMock
from src.app.combat_system import CombatSystem
from src.models.character import Character
from src.models.sword import Sword
from src.models.bow import Bow
from src.models.fire_power_up import FirePowerUp
from src.models.electric_power_up import ElectricPowerUp

class TestPowerUp(unittest.TestCase):

    def test_power_up_modifies_damage(self):
        """Test que el daño base es diferente al del power up"""
        mock_calculator = MagicMock()
        mock_calculator.check_critical_hit.return_value = False
        
        combat = CombatSystem(mock_calculator)
        cloud = Character("Cloud")
        sephiroth = Character("Sephiroth")
        sword = Sword()
        electric_sword = ElectricPowerUp(sword)
        
        base_damage, base_result = sword.attack(cloud, sephiroth)
        electric_damage, elec_result = electric_sword.attack(cloud, sephiroth)

        self.assertGreater(electric_damage, base_damage)
        

    def test_power_up_adds_extra_damage(self):
        """Test que el power up añade daño extra"""
        mock_calculator = MagicMock()
        mock_calculator.check_critical_hit.return_value = False
        
        combat = CombatSystem(mock_calculator)
        link = Character("Link")
        moblin = Character("Moblin")
        bow = Bow()
        fire_bow = FirePowerUp(bow)
        
        combat.perform_attack(link, fire_bow, moblin)
         
        self.assertEqual(moblin.health, 78)

