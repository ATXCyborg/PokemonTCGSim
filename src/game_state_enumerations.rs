#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CardLocation {
    P1Hand,
    P2Hand,
    P1Deck,
    P2Deck,
    P1Bench,
    P2Bench,
    P1Active,
    P2Active,
    P1Discard,
    P2Discard,
    P1Prizes,
    P2Prizes,
    Stadium,
    P1LostZone,
    P2LostZone,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CardVisibleState {
    Hidden,
    P1Known,
    P2Known,
    BothKnown,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum GamePhase {
    Setup,      // Possibly add more steps here for setup if needed
    PlayerTurn, // Turn to play items, tools, supporters, attach energy, activate ability, etc.
    Attack, // Phase entered if attack action taken. Pass action bypasses attack and goes directly to Checkup.
    Checkup, // Perform checkup between player turns
    //Terminal Phases below
    Player1Win,
    Player2Win,
    Draw, // Both Players have the same number of win conditions
}

impl GamePhase {
    // True for terminal phases. No more actions once game reaches terminal
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            GamePhase::Player1Win | GamePhase::Player2Win | GamePhase::Draw
        )
    }
}
