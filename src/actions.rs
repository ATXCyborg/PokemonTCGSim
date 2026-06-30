use crate::action_enumerations::ActionType;
use crate::game_state::CardIdx;

#[derive(Clone, Copy, Debug)]
pub struct Action {
    pub typ: ActionType,
    pub card: Option<Cardidx>,
}

impl Action {
    pub fn card_index(&self) -> usize {
        self.card.expect("action carries no card").get()
    }
}
