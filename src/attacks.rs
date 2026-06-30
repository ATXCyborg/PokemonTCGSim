use crate::card_enumerations::EnergyType; //Will eventually include other enumerations

#[derive(Clone, Copy, Debug)]
pub struct Attack {
    pub name: String,
    pub attack_cost: Vec<EnergyType>,
    pub base_damage: u16,
    // Will eventually include other fields such as damage_modification,
    // damage allocation targets, attack effects, etc.
}
