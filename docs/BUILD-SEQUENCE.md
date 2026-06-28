# BUILD SEQUENCE — Resonance Knowledge

Each phase on its own branch from main. Human test before merge. Zero errors on `npm run check` and `cargo build`.

---

## Phase 0: Shell
**Branch:** `resonance-knowledge/shell`

- App layout with ComfortBar (greeting + quick-add button, expandable)
- Collapsible sidebar (proven pattern from Compass: 20vw, hamburger button below status bar)
- COSMIC theme system (dark/light/amoled, 6 presets)
- Mobile-safe areas (safe-area-inset-top, safe-area-inset-bottom)
- Window identifier: `com.audhd.resonance-knowledge`
- **Test:** App opens on Android. Sidebar works. ComfortBar visible. Theme applies. No crashes.

## Phase 1: The Knowledge
**Branch:** `resonance-knowledge/knowledge`

- New knowledge form: name, sense (8 options), subcategory (per sense + custom), emoji (EmojiGrid), optional note, intensity (1-5), timestamp
- SQLite knowledge table (already migrated)
- Rust command: `add_knowledge`, `get_knowledge` (paginated, 200 at a time)
- knowledgeStore with reactive state
- Home screen: timeline of recent knowledge, newest first
- KnowledgeCard component: emoji, name, sense badge, subcategory, relative timestamp, intensity dots
- **Test:** Create knowledge. Appears in timeline. Survives restart.

## Phase 2: Browse & Filter
**Branch:** `resonance-knowledge/browse`

- Search bar with 150ms debounce
- Filter by sense (tap sense badge)
- Filter by emoji (tap emoji to filter)
- Sort: newest first (default), oldest first, by sense, by intensity
- Display cap 200 + "Load more"
- **Test:** Filter by sense. Search works. Load more works.

## Phase 3: Gentle Insights
**Branch:** `resonance-knowledge/insights`

- Top Emojis — most-used, sized by frequency
- By Sense — simple distribution
- Streak — consecutive days with at least one knowledge
- Time of Day — "You often log in the evening"
- Recent Mood — last 7 days as emoji row
- No charts. No complex graphs. Just gentle patterns.
- **Test:** Create varied Knowledge. Insights populate.

## Phase 4: Onboarding
**Branch:** `resonance-knowledge/onboarding`

- First-launch welcome with 🧭 sigil
- "What should we call you?" — vessel name input
- Brief explanation of Knowledge
- Theme selection (Dark/Warm/Ocean)
- **Test:** Fresh install → onboarding → home with greeting.

## Phase 5: Data Sovereignty
**Branch:** `resonance-knowledge/sovereignty`

- Export all knowledge as JSON (Tauri dialog for save location)
- Purge all data (double confirmation)
- Export & Purge
- Settings page with theme, app info
- **Test:** Export produces valid JSON. Purge clears everything.

## Phase 6: Mobile Ship
**Branch:** `resonance-knowledge/ship`

- Final Android testing
- Sign APK
- App icons (sacred geometry sigil)
- Store listing prep
- **Test:** Full walkthrough on Android device.