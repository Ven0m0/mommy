---
name: add-mood
description: Add a new affirmation mood to assets/affirmations.json — use when the user wants to add, create, or define a new mood/tone for mommy's affirmations
---

# Add a mood

Adds a new mood to the embedded affirmation set and verifies it loads correctly.

## Steps

1. Read `assets/affirmations.json` and find the `moods` object. Each mood is
   keyed by name and has `positive` and `negative` arrays of template
   strings. Templates may use the placeholders `{roles}`, `{pronouns}`,
   `{little}`, `{emotes}` (resolved by `src/utils.rs`).

2. Add the new mood following the existing shape exactly:

   ```json
   "new_mood": {
     "positive": ["great job, {little}~ {emotes}"],
     "negative": ["it's okay, {little}, {pronouns} still {roles}"]
   }
   ```

   Keep both `positive` and `negative` non-empty — `src/affirmations.rs`
   falls back to a different mood if either array is empty for the
   requested one (see `test_mood_fallback_on_invalid`).

3. Validate the JSON parses and the mood loads:

   ```bash
   cargo test affirmations::
   ```

4. Smoke-test it manually:

   ```bash
   cargo build
   SHELL_MOMMYS_MOODS=new_mood ./target/*/debug/mommy echo test
   ```

5. If the user wants a permanent test for the new mood, add one alongside
   the existing mood tests in `src/affirmations.rs` (e.g.
   `test_mood_ominous`, `test_mood_thirsty` are good templates to copy).

Do not add a mood without both `positive` and `negative` arrays, and do not
introduce new placeholder syntax beyond `{roles}`/`{pronouns}`/`{little}`/`{emotes}`
— `src/utils.rs`'s template engine only recognizes those four.
