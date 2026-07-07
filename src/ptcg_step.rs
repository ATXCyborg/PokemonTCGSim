use crate::action::Action;
use crate::action_enumerations::ActionType;
use crate::card_enumerations::{CardSubType, CardType};
use crate::game_state::{
    CardIdx, CardState, DECK_SIZE, GameState, Player, PlayerIndex, TOTAL_CARDS,
};
use crate::game_state_enumerations::{CardLocation, CardVisibleState, GamePhase};
//Include card effects when they are implemented
use rand::RngExt;

pub fn step(gs: &mut GameState, act: Action) {
    // Once the game has been won or drawn it is frozen: no further actions are
    // processed.
    if gs.is_over() {
        return;
    }
    match gs.phase {
        Phase::Setup => handle_setup(gs, act),
        Phase::Draw => handle_draw_for_turn(gs, act),
        Phase::PlayerTurn => handle_player_actions(gs, act),
        Phase::Attack => handle_attack(gs, act), //Processes attack, then calls handle_end_of_turn
        Phase::Pass => handle_end_of_turn(gs, act),
        Phase::Checkup => handle_checkup(gs, act), //Phase between player turns to handle upkeep/status conditions/etc.
        _ => {}
    }
}

// Short display name for a player in log messages.
fn player_name(pid: PlayerIndex) -> &'static str {
    if pid == PlayerIndex::P1 { "P1" } else { "P2" }
}

// The card names in 'player's' hand, joined for a log message.
fn hand_card_names(player: &Player, cards: &[CardState; TOTAL_CARDS]) -> String {
    player
        .hand_iter(cards)
        .map(|(_, cs)| format!("{:?}", cs.card))
        .collect::<Vec<_>>()
        .join(", ")
}

// Log that 'pid' drew their opening hand. The drawn cards are hidden
// information: the omniscient gamestate log and the drawing player's own log
// name the cards, while the opponent's log only records how many were drawn.
fn log_opening_hand(gs: &mut GameState, pid: PlayerIndex) {
    if !gs.logging_enabled() {
        return;
    }
    let player = if pid == PlayerIndex::P1 {
        &gs.p1
    } else {
        &gs.p2
    };
    let named = format!(
        "{} draws [{}]",
        player_name(pid),
        hand_card_names(player, &gs.cards)
    );
    let hidden = format!("{} draws {} cards", player_name(pid), player.hand_size);
    let (p1_msg, p2_msg) = if pid == PlayerIndex::P1 {
        (named.clone(), hidden)
    } else {
        (hidden, named.clone())
    };
    gs.log_views(named, p1_msg, p2_msg);
}

fn handle_setup(gs: &mut Gamestate, pid: PlayerIndex) {}

// Start the player's turn. Draw a card from the deck, check for on-draw effects,
// check for 'deck_out' conditions for ending game, add card to hand if card to draw.
fn handle_draw_for_turn(gs: &mut Gamestate, pid: PlayerIndex) {
   // Count this turn. Reaching 'MAX_TURNS' with no win conditions met ends the game
   // in a draw, which 'check_game_end' detects from teh bumped counter.
   gs.turn_count += 1;
   let turn_player = if gs.turn_player == PlayerIndex::P1 { &mut gs.p1 } else { &mut gs.p2 };
   // Check for deck-out condition
   // If not decked-out, draw card
}
fn handle_player_actions(gs: &mut Gamestate, pid: PlayerIndex) {}
fn handle_attack(gs: &mut Gamestate, pid: PlayerIndex) {}
fn handle_end_of_turn(gs: &mut Gamestate, pid: PlayerIndex) {}
fn handle_checkup(gs: &mut Gamestate, pid: PlayerIndex) {}

fn draw_cards(player: &mut Player, cards: &mut [CardState; TOTAL_CARDS], num: usize) {
    let mut drawn = 0;
    while drawn < num {
        // Re-read the top each iteration: drawing a card updates 'top_deck_idx',
        // and it becomes 'None' once the (formly last) card is drawn.
        let Some(current_idx) = player.top_deck_idx.map(|x| x.get()) else {
            break; // Deck is empty
        };
        move_from_top_of_deck_to_hand(player, cards, current_idx);
        drawn += 1;
    }
}

fn move_from_top_of_deck_to_hand(player: &mut Player, cards: &mut [CardState; TOTAL_CARDS], card_idx: usize) {
    // Pull the card off teh deck (updates top/bottom pointers and deck_size),
    // then prepend it to the hand and mark it as known to its owner.
    let pid = player.pid;
    detach_from_current_zone(player, cards, card_idx);
    cards[card_idx].location = CardLocation::hand(pid);
    cards[card_idx].visible = hand_visibility(pid);
    attach_to_front_of_zone(
        cards,
        &mut player.hand_idx,
        None,
        Some(&mut player.hand_size),
        card_idx,
    );
}

// Remove the card at global index 'idx' from teh bookkeeping of its current
// location so it can be placed elsewhere. The card's 'location' already encodes
// its owner, so the same 'player' (its owner) handles both 'P1*' and 'P2*'
// variants. Linked-list zones (hand, deck, discard, etc.) are relinked via
// 'detached_fron_linked_list'.
fn detach_from_current_zone(player: &mut Player, cards: &mut [CardState; TOTAL_CARDS], idx: usize) {
    let location = cards[idx].location;
    match location {
        
    }
}
