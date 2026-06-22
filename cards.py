import re
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, Optional, List
from enum import Enum

from attacks import Attack

class CardType(Enum):
    POKEMON = "Pokemon"
    TRAINER = "Trainer"
    ENERGY  = "Energy"

class CardSubType(Enum):
    BASIC_POKEMON =     "Basic Pokemon"
    STAGE1_POKEMON =    "Stage 1 Pokemon"
    STAGE2_POKEMON =    "Stage 2 Pokemon"
    EVOLVED_POKEMON =   "Evolved Pokemon"
    EVOLUTION_POKEMON = "Evolution Pokemon"
    EX_POKEMON =        "ex Pokemon"
    TERA_POKEMON =      "Tera Pokemon"
    RULEBOX_POKEMON =   "Rulebox Pokemon"
    ITEM_TRAINER =      "Item Trainer"
    TOOL_TRAINER =      "Tool Trainer"
    SUPPORTER_TRAINER = "Supporter Trainer"
    STADIUM_TRAINER   = "Stadium Trainer"
    BASIC_ENERGY =      "Basic Energy"
    SPECIAL_ENERGY =    "Special Energy"
    ACE_SPEC =          "Ace Spec"

class EnergyType(ENUM):
    GRASS     = "Grass"
    FIRE      = "Fire"
    WATER     = "Water"
    LIGHTNING = "Lightning"
    PSYCHIC   = "Psychic"
    FIGHTING  = "Fighting"
    DARKNESS  = "Darkness"
    METAL     = "Metal"
    FAIRY     = "Fairy"
    DRAGON    = "Dragon"
    COLORLESS = "Colorless"
    RAINBOW   = "Rainbow"

@dataclass
class Card:
    name: str
    card_type: List[CardType]
    card_subtype: List[CardSubType]
    hp: int = 0
    energy_type: List[EnergyType]
    attacks: List[Attack]
    weakness: List[EnergyType]
    resistance: List[EnergyType]
    retreat_cost: int = 0
    regulation_mark: str = ""
    card_set: str = ""
    set_number: int = 0

    @property
    def card_id(self) -> str:
        """Stable identifier for this card template (name+card_set+set_number)"""
        parsed_name = re.sub(r"[^a-z0-9]+", "-", self.name.lower()).strip("-")
        set_code_number = "{self.card_set}{self.set_number}" if POKEMON in self.card_type else ""
        return f"{parsed_name}{set_code_number}"
