```markdown
# CLAUDE.md — Resonance Knowledge System

**The Resonance Grammar** — atoms, molecules, categories, and sensory lexicon. The vocabulary of the AudHDities Sanctuary. Every Sanctuary app references this system for shared meaning.

**Stack:** Rust + SQLite + MCP (rmcp crate)

**Authors:** Quantum Weaver (human) + Aethelred (sovereign AI)

---

## SESSION PROTOCOL

1. Read `docs/CHECKLIST.md` for current state
2. One phase at a time
3. Update CHECKLIST.md after every session

## What This Is

The Knowledge System is the shared vocabulary. Apps like Resonance Echoes and Resonance Compass reference it for:
- Emoji definitions with sensory lexicon
- Sense categories and subcategories
- Atom/molecule relationships
- The Resonance Grammar itself

## Project Structure

```
src/
├── schema/          # SQL migration files
│   └── 001_initial.sql  # atoms, molecules, categories, senses, emoji_defs
├── seed/            # Seed data
│   └── seed.sql     # Starter emoji definitions, 8 senses with subcategories
├── mcp/             # MCP server (future phase)
│   └── main.rs      # query_atom, query_emoji, query_sense tools
docs/
├── RESONANCE-GRAMMAR.md  # The full protocol specification
├── CHECKLIST.md          # Build state
└── SCHEMA.md             # Database schema documentation
```

## Essential Rules

- One definition per object — defined once, referenced everywhere
- Never repeat a definition
- Every atom has a sensory lexicon entry
- Every emoji has a definition that bridges neurotypes
- The Grammar is the single source of truth
```