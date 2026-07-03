# Half-Earth Socialism (quarter_earth)

Rust workspace for the game "Half-Earth Socialism".

## Structure

- `engine/` (`hes-engine`) — model/simulation logic driving the game.
- `game/` (`hes-game`) — the game itself, visual layer over the engine. Builds for both web (wasm) and native.
- `editor/` (`hes-editor`) — editor for game/engine data (projects, parameters), used to build `.world` files. Native only, not shipped with the web build.
- `design/`, `images/`, `util/` — supporting assets/tooling.

Two build targets (web vs native) that should stay in sync except where noted in the readme.

See `references/project-brief.md` for the acTe assembly-workshop use case
(model-only mode + encyclopedia, target Oct 2026) currently motivating work
on this repo. The workshop-mode work itself is spec-driven in
`references/simplification/` — `README.md` there is the entry point, and
`implementation-plan.md` carries the work breakdown plus an execution log
with current status.

## Commands

Uses [`just`](https://github.com/casey/just):

```
just run         # Run the game
just surfaces    # Generate biome surface textures and regional climates
just sharing     # Generate sharing images
```

Setup:

```bash
git submodule init
git submodule update
cd game/assets/js && npm install -d
```

## Notes

- Debug via `DEBUG`/`DEBUG_VIEW` env vars, see `game/src/debug.rs`.
- Game state exports are base64+DEFLATE session dumps, loadable via `DEBUG_STATE=/path cargo run`.
- Hector climate model is vendored: WASM build for web, `hector-rs` wrapper for native.
