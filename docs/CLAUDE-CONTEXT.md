# CLAUDE CONTEXT — Resonance Echoes

## Naming
- **App:** Resonance Echoes (or "Echoes")
- **Repo:** `resonance-echoes`
- **Identifier:** `com.audhd.resonance-echoes`
- **Crate:** `resonance_echoes_lib`

## The Senses
👁️ Seen · 👂 Heard · ✋ Felt · 💭 Thought · 🫀 Felt Inside · 🌙 Dreamt · 🙏 Grateful For · ✨ Other

Every sense has starter subcategories. Vessels can create custom ones.

## The ComfortBar
Not a MiniPlayer. No audio controls. A gentle footer with:
- Minimized: greeting + quick-add button
- Expanded: stats, quick actions
- Context-aware: changes based on current screen

## Key Patterns
- Navigation: `goto()` — never `window.location.href`
- z-index: ComfortBar 110, backdrop 49
- SQLite: echoes table, paginated queries
- Theme: CSS variables on `.app-shell`
- State: Svelte 5 runes

## Differences from Compass
- No audio engine (rodio, cpal, oboe)
- No visualizer, EQ, fragments
- No playlists, library scanning
- No Timer, Sattva, Focus Session
- ComfortBar replaces MiniPlayer
- Echo replaces Track
- Sense replaces Album/Artist