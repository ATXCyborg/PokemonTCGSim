# PokemonTCGSim

---

## Project Description
Simulation engine to help test out possible decks.
Currently a work in progress with incremental increases in complexity as we go.

Framework adapted from [fab-ai by Nathan Groover](https://github.com/ngroover/fab-ai) and adapted to fit the Pokemon TCG.

---

## Roadmap
- [ ] Simple pokemon game working for simplified (no abilities, just basic attacks and basic energy, no evolutions, no trainer cards)
- [ ] Pokemon game with evolutions
- [ ] Pokemon game with effects of attacks
- [ ] Pokemon game with abilities
- [ ] Pokemon game with special energy
- [ ] Pokemon game with item cards
- [ ] Pokemon game with supporter cards
- [ ] Pokemon game with tool cards
- [ ] Pokemon game with stadium cards

---

## File Map

| File | Responsibility |
|------|----------------|
| `cards.py` | `Card` dataclass; enums `CardType`, `Typing`|
| `game_state.py` | `BoardState`, `Player`, `GameState` - pure data, no rules logic |
| `actions.py` | `ActionType` enum, `Action` dataclass, legal-action generators: `legal_attack_actions`, ...|
| `observations.py` | `CARD_VOCAB`, `CARD_IDX`, `PLAYER_OBS_SIZE`; `encode_player()`, `encode_opponent_public()`, `build_observation()` |
| `spaces.py` | Lightweight gymnasium-compatible `Discrete`, `box`, `Dict` spaces (no gymnasium dependency) |
| `ptcg_env.py` | `PTCGEnv` - main gym-style environment; `Phase` enum; `_make_***()` factories |
| `agents.py` | Agents: `RandomAgent`, `HumanAgent`, Each implements `select_action(obs, legal, player, opponent, context=None)` - `context` is the dict from `PTCGEnv.build_action_context()` and carries phase-specific info |
| `seed_decks.py` | Deterministic deck seeds for reproducible tests |
| `deck_db.py` | Card lookup / Deck database helpers |
| `run_env.py` | CLI entry point to run games or RL training loops |
| `test_seed57.py` | Regression tests (seeded at 57) |
| `web_viewer.py` | Browser-based game viewer |

---

## Architecture

```
PTCGEnv.step(action)
  ├─ Phase.ATTACH_ENERGY 
  ├─ Phase.ACTIVATE_ABILITY
  ├─ Phase.PLAY_TRAINER
  ├─ Phase.ATTACK
  └─ Phase.PASS
```

`PTCGEnv.legal_actions()` returns the valid `Action` list for the current phase. \\
`PTCGEnv._get_obs()` returns `{"p0": obs_dit, "p1": obs_dict}` via `build_observations()`.

---

More to come as work progresses.
