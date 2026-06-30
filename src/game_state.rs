use crate::card::Card;
use crate::game_state_enumerations::{CardLocation, CardVisibleState, GamePhase};

// Number of cards in each player's deck. The unified `GameState::cards` array
// is `2*DECK_SIZE` long: player 0 owns slots `0...DECK_SIZE` and player 1 owns
// slots `DECK_SIZE...2*DECK_SIZE`.

pub const DECK_SIZE: usize = 60;
pub const TOTAL_CARDS: usize = DECK_SIZE * 2;

pub struct GameState {
    pub p1: Player,
    pub p2: Player,
    // All cards in the game, in one flat array shared by both players. Player 0
    // owns slots `0...`
}
