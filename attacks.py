"""
Attacks for use by pokemon cards.

Attacks are used by pokemon and can be sourced from:
   - The attacking Pokemon's Card
   - Attached TM tool card
   - Copied from another Pokemon Card

Usage
-----
Define an attack on a card::

   Attack(
      Name: str (used for logging which attack is used)
      Attack Cost: list[EnergyType]
      Base Damage: int
      Damage Modification: callable function
      Damage Allocation: callable function
      Attack Effect: callable function
   )
"""

from dataclasses import dataclass
from typing import Callable, List

from enumerations import EnergyType

@dataclass
class Attack:
    name: str
    attack_cost: List[EnergyType]
    base_damage: int
    damage_modification: Callable[[int], int]
    damage_allocation: Callable[[int], int]
    attack_effect: Callable[[int], int]

