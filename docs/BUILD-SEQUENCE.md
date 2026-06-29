# BUILD SEQUENCE — Resonance Knowledge

Each phase on its own branch from main. Zero errors on `cargo build`.

---

## Phase K-0: Schema & Seed
**Branch:** `resonance-knowledge/schema`

- SQLite schema: atoms, molecules, categories, senses, subcategories, emoji_definitions
- Seed data: 8 senses, 19 subcategories, 12 emoji definitions with sensory lexicon
- Cargo.toml with rusqlite, serde, serde_json
- **Test:** Database creates. Seed data inserts. `cargo build` clean.
- **Status:** ✅ Complete

## Phase K-1: Query CLI
**Branch:** `resonance-knowledge/cli`

- `resonance-knowledge atom <term>` — query by term
- `resonance-knowledge emoji <char>` — query by emoji
- `resonance-knowledge sense <id>` — query sense with subcategories
- `resonance-knowledge list-atoms` — all atoms
- `resonance-knowledge list-emojis` — all emojis
- `resonance-knowledge list-senses` — all senses
- All output is valid JSON
- **Test:** Each command returns correct data. `cargo build` clean.
- **Status:** ⬜ Ready to build

## Phase K-2: MCP Server
**Branch:** `resonance-knowledge/mcp`

- rmcp server with HTTP transport
- Tools: query_atom, query_emoji, query_sense
- Claude Code integration
- **Test:** Claude queries the Grammar. `cargo build` clean.

## Phase K-3: Echoes Integration
**Branch:** `resonance-knowledge/echoes`

- Echoes reads senses from Knowledge system instead of hardcoded data
- Echoes reads emoji definitions from Knowledge system
- Single source of truth for all apps
- **Test:** Echoes builds with Knowledge as dependency.

## Phase K-4: Compass Integration
**Branch:** `resonance-knowledge/compass`

- Compass v2 reads mood emojis from Knowledge system
- Validation against shared vocabulary
- **Test:** Compass builds with Knowledge as dependency.