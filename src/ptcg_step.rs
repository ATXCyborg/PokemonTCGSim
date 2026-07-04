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
fn handle_draw_for_turn(gs: &mut Gamestate, pid: PlayerIndex) {}
fn handle_player_actions(gs: &mut Gamestate, pid: PlayerIndex) {}
fn handle_attack(gs: &mut Gamestate, pid: PlayerIndex) {}
fn handle_end_of_turn(gs: &mut Gamestate, pid: PlayerIndex) {}
fn handle_checkup(gs: &mut Gamestate, pid: PlayerIndex) {}
