# Commits that cut releases

Release tooling derives the next version from the commits on the default branch, so a commit's
type decides what gets released.

- **Pick the type for what the user gets.** `feat` and `fix` only for behaviour that ships;
  everything else — `chore`, `docs`, `ci`, `test`, `refactor` — for what doesn't. A fix to
  something not released yet belongs to its feature and is no `fix` of its own.
- **An empty `feat` commit, when asked for, is made.** The user records a finished state —
  planning included — as a release this way; don't object that nothing ships.
