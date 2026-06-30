use crate::card::Card;
// include actions

// Number of cards in each player's deck. The unified `GameState::cards`
// array is `2*DECK_SIZE` long: player 0 owns slots `0...DECK_SIZE` and
// player 1 owns slots `DECK_SIZE...2*DECK_SIZE`.
pub const DECK_SIZE: usize = 60;
pub const TOTAL_CARDS: usize = DECK_SIZE * 2;


