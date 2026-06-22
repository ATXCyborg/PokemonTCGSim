from enum import Enum

class CardType(Enum):
    POKEMON = "Pokemon"
    TRAINER = "Trainer"
    ENERGY  = "Energy"

class CardSubType(Enum):
    BASIC_POKEMON     = "Basic Pokemon"
    STAGE1_POKEMON    = "Stage 1 Pokemon"
    STAGE2_POKEMON    = "Stage 2 Pokemon"
    EVOLVED_POKEMON   = "Evolved Pokemon"
    EVOLUTION_POKEMON = "Evolution Pokemon"
    EX_POKEMON        = "ex Pokemon"
    TERA_POKEMON      = "Tera Pokemon"
    RULEBOX_POKEMON   = "Rulebox Pokemon"
    ITEM_TRAINER      = "Item Trainer"
    TOOL_TRAINER      = "Tool Trainer"
    SUPPORTER_TRAINER = "Supporter Trainer"
    STADIUM_TRAINER   = "Stadium Trainer"
    BASIC_ENERGY      = "Basic Energy"
    SPECIAL_ENERGY    = "Special Energy"
    ACE_SPEC          = "Ace Spec"

class EnergyType(Enum):
    GRASS     = "Grass"
    FIRE      = "Fire"
    LIGHTNING = "Lightning"
    PSYCHIC   = "Psychic"
    FIGHTING  = "Fighting"
    DARKNESS  = "Darkness"
    METAL     = "Metal"
    FAIRY     = "Fairy"
    DRAGON    = "Dragon"
    COLORLESS = "Colorless"
    RAINBOW   = "Rainbow"
