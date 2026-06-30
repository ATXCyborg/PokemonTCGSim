#[repr(u8)]
pub enum CardType {
    Pokemon,
    Trainer,
    Energy,
}

#[repr(u8)]
pub enum CardSubType {
    PokemonBasic,
    PokemonStage1,
    PokemonStage2,
    PokemonEvolved,
    PokemonEvolution,
    PokemonEx,
    PokemonTera,
    PokemonRulebox,
    TrainerItem,
    TrainerTool,
    TrainerSupporter,
    TrainerStadium,
    EnergyBasic,
    EnergySpecial,
    AceSpec,
}

#[repr(u8)]
pub enum EnergyType {
    Grass,
    Fire,
    Lightning,
    Psychic,
    Fighting,
    Darkness,
    Metal,
    Fairy,
    Dragon,
    Colorless,
    Rainbow,
}
