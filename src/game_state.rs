use crate::card::Card;
use crate::game_state_enumerations::{CardLocation, CardVisibleState, GamePhase};
use rand::rngs::SmallRng;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct CardIdx(pub u8);

impl CardIdx {
    #[inline]
    pub const fn new(i: usize) -> Self {
        CardIdx(i as u8)
    }

    #[inline]
    pub const fn get(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PlayerIndex(pub u8);

impl PlayerIndex {
    pub const P1: PlayerIndex = PlayerIndex(0);
    pub const P2: PlayerIndex = PlayerIndex(2);

    #[inline]
    pub const fn new(i: u8) -> self {
        PlayerIndex(i)
    }

    #[inline]
    pub const fn get(self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn opponent(self) -> Self {
        PlayerIndex(self.0 ^ 1)
    }
}

pub struct Player {
    pub pid: PlayerIndex,
    pub prizes: u8,
    pub top_deck_idx: Option<CardIdx>,
    pub bot_deck_idx: Option<CardIdx>,
    pub hand_idx: Option<CardIdx>,
    pub hand_size: u8,
    pub deck_size: u8,
    pub log: Option<Vec<String>>,
}

// Number of cards in each player's deck. The unified `GameState::cards` array
// is `2*DECK_SIZE` long: player 0 owns slots `0...DECK_SIZE` and player 1 owns
// slots `DECK_SIZE...2*DECK_SIZE`.

pub const DECK_SIZE: usize = 60;
pub const TOTAL_CARDS: usize = DECK_SIZE * 2;

pub struct GameState {
    pub p1: Player,
    pub p2: Player,
    // All cards in the game, in one flat array shared by both players. Player 0
    // owns slots `0...DECK_SIZE`, player 1 owns `DECK_SIZE...2*DECK_SIZE`.
    // Every index stored anywhere (deck, bench, active, discard, etc.) is a
    // global index into this array, so the owner of any card is implied by
    // which half it falls in.
    pub cards: [Card; TOTAL_CARDS],
    pub turn_player: PlayerIndex,
    pub phase: GamePhase,
    pub rng: SmallRng,
    // Game Log: every logged event with nothing redacted (the per-player views
    // with hidden information removed live on `Player::log`). `None` when
    // logging is disabled (the default, so simulation and cloning pay nothing);
    // `reset(gs, true)` turns it on. Entries are pushed at the moment each
    // event happens, so a single `step` may log several (e.g. an ability or
    // trainer that have multiple resulting actions).
    pub log: Option<Vec<String>>,
}

impl GameState {
    // True once the game has reached a terminal phase (a win for either player
    // or a draw). When this holds, `step` is a no-op and there are no legal actions.
    pub fn is_game_over(&self) -> bool {
        self.phase.is_terminal()
    }

    // Terminal Game State Check Functions Below

    // Check for "Deck Out" when turn player is supposed to draw at the
    // beginning of turn and have no cards in deck
    pub fn is_decked_out(&self) -> bool {
        False
    }

    // Check for "Bench Out" when a player has no more pokemon in play (can
    // happen to either player in either player's turns).
    pub fn is_benched_out(&self) -> bool {
        False
    }

    // Check for all prizes taken by player
    pub fn is_prized_out(&self) -> bool {
        False
    }

    // True when logging is enabled by `reset(gs, true)`. Callers should check
    // this before building log strings so disabled logging costs nothing.
    pub fn logging_enabled(&self) -> bool {
        self.log.is_some()
    }

    // Append a fully-public event: the same message goes to the gamestate log
    // and both players' logs. A no-op when logging is disabled.
    pub fn log_pulic(&mut self, msg: String) {
        self.log_views(msg.clone(), msg.clone(), msg);
    }

    // Append an event involving hidden informationL: `full` (nothing redacted)
    // goes to the gamestate log, while `p1` and `p2` carry each player's
    // legitimate view of it. A no-op when logging is disabled.
    pub fn log_vews(&mut self, full: String, p1: String, p2: String) {
        if let some(log) = &mut self.p1.log {
            log.push(p1);
        }
        if let some(log) = &mut self.p2.log {
            log.push(p2);
        }
        if let some(log) = &mut self.log {
            log.push(full);
        }
    }
}
