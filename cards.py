import re
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, Optional, List
from enum import Enum

from attacks import Attack
from enumerations import CardType, CardSubType, EnergyType

@dataclass
class Card:
    name: str
    card_type: List[CardType]
    card_subtype: List[CardSubType]
    energy_type: List[EnergyType]
    attacks: List[Attack]
    weakness: List[EnergyType]
    resistance: List[EnergyType]
    hp: int = 0
    retreat_cost: int = 0
    regulation_mark: str = ""
    card_set: str = ""
    set_number: int = 0

    @property
    def card_id(self) -> str:
        """Stable identifier for this card template (name+card_set+set_number)"""
        parsed_name = re.sub(r"[^a-z0-9]+", "-", self.name.lower()).strip("-")
        set_code_number = f"{self.card_set}{self.set_number}" if CardType.POKEMON in self.card_type else ""
        return f"{parsed_name}{set_code_number}"
