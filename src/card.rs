use crate::card_enumerations::{CardSubType, CardType, EnergyType};

pub struct Card {
    pub name: String,
    pub card_type: Vec<CardType>,
    pub card_subtype: Vec<CardSubType>,
    pub energy_type: Vec<EnergyType>,
    //pub attacks: Vec<Attack>
    //pub ability: Ability
    pub attached_tools: Vec<Card>,
    pub attached_energy: Vec<Card>,
    pub weakness: Vec<EnergyType>,
    pub resistance: Vec<EnergyType>,
    pub hp: u16,
    pub retreat_cost: u8,
    pub regulation_mark: String,
    pub card_set: String,
    pub set_number: u16,
}
