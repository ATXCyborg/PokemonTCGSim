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

// Symmetric set of per-player zone constructors; some are not yet referenced
// yet as the engine is still being built out.
#[allow(dead_code)]
impl CardLocation {
    // Per-player zone constructors. `pid` is the owning player; each returns
    // that player's variant of the zone.
    pub const fn hand(pid: PlayerIndex) -> Self {
        if pid.get() == 0 {
            Self::P1Hand
        } else {
            Self::P2Hand
        }
    }
    pub const fn deck(pid: PlayerIndex) -> Self {
        if pid.get() == 0 {
            Self::P1Deck
        } else {
            Self::P2Deck
        }
    }
    pub const fn bench(pid: PlayerIndex) -> Self {
        if pid.get() == 0 {
            Self::P1Bench
        } else {
            Self::P2Bench
        }
    }
    pub const fn active(pid: PlayerIndex) -> Self {
        if pid.get() == 0 {
            Self::P1Active
        } else {
            Self::P2Active
        }
    }
    pub const fn discard(pid: PlayerIndex) -> Self {
        if pid.get() == 0 {
            Self::P1Discard
        } else {
            Self::P2Discard
        }
    }
    pub const fn prizes(pid: PlayerIndex) -> Self {
        if pid.get() == 0 {
            Self::P1Prizes
        } else {
            Self::P2Prizes
        }
    }
    pub const fn lost_zone(pid: PlayerIndex) -> Self {
        if pid.get() == 0 {
            Self::P1LostZone
        } else {
            Self::P2LostZone
        }
    }
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
    Draw, // Draw at the beginnning of turn (check for deckout lose condition, on-draw effects)
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
