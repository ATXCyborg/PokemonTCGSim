use crate::enumerations::{CardSubType, CardType, EnergyType};

pub struct CardData {
    pub name: String,
    pub card_type: Vec<CardType>,
    pub card_subtype: Vec<CardSubType>,
    pub energy_type: Vec<EnergyType>,
    //pub attacks: Vec<Attack>
    //pub ability: Ability
    pub weakness: Vec<EnergyType>,
    pub resistance: Vec<EnergyType>,
    pub hp: u16,
    pub retreat_cost: u8,
    pub regulation_mark: String,
    pub card_set: String,
    pub set_number: u16,
}
