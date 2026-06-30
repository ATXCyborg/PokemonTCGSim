#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ActionType {
    PlayCard,        // As many times as you like during turn
    ActivateAbility, // As many times as ability allows during turns
    Attack,          // Terminal Action
    Pass,            // Terminal Actions
}

// Implement other action specific stuff here later (draw, various effects of actions, etc.)
