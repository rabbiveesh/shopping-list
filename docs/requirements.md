# Shopping List: requirements

## The idea

A shopping list in three stages, so planning, shopping and catching up on what was missed each have their own screen.

1. **Pick** (before shopping): a catalogue of foods grouped by aisle. Tapping one puts it on the list; tapping again takes it off. A box at the top adds something new, straight onto the list. A small plus at the end of each aisle adds something to that aisle of the catalogue, not yet on the list. A long press on an item shows a trash can that deletes it, with Undo for a few seconds.
2. **Shop**: the list, grouped by aisle. Tapping an item (or swiping it right) ticks it off: it moves to "In the cart", crossed out, and tapping it there puts it back. A purple button (or swiping left) marks something that couldn't be found, which moves it to the fill-in list.
3. **Fill-in**: what couldn't be found, for another shop. "Got it" clears an item; "To list" puts it back on the main list.

"Finish shop" takes what's in the cart off the list.

## What it does today (v1)

- The three stages above, with counts on the Shop and Fill-in tabs.
- The user's own list, by aisle of their usual shop (`web/src/data/starter.ts`).
- Works on the phone with no account. With the Firebase project set up, "Sync" signs in with Google and keeps the list in the cloud, bringing along what was done on the phone.
- Two phones changing the same item at once (ticked off on one, can't find on the other) ask which is right, like Did It Last.
- Works offline and installs on a phone (PWA).

## Decisions

- **Same stack as Did It Last**, 2026-11-10: Rust core of pure functions, Vue + Vuetify PWA, an append-only change log folded by reducers, Firebase with Google sign-in.
- **Starter items aren't stored**, 2026-11-10: the log only records what the user does, and the core folds it over the starter catalogue. Changing the catalogue needs no data migration.
- **Local first**, 2026-11-10: the list lives on the phone until the user signs in, so the app is usable before the cloud project exists.
- **Finishing a shop keeps what wasn't got on the list**, and keeps the fill-in list as it is (best guess, 2026-11-10; open below).

## Open questions

- After a shop, should things not got stay on the list (today: yes)?
- Should two people share one list (today: each Google account has its own)?
- Quantities ("2 x milk"), or just names?
- Renaming catalogue items in the app (adding to an aisle and deleting exist).
- Swipes have no drag animation yet (Vuetify's `v-touch` fires on release).
- The log grows with every tap; it may need compacting once it holds thousands of changes.
