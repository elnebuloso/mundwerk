# Commits before 1.0

While the project's version is below 1.0, leaving it is the user's call, never the side effect of
a commit — and release tooling that derives the version from Conventional Commits turns a
breaking-change marker into `1.0.0`.

- **No breaking-change markers.** Neither a `BREAKING CHANGE:` footer nor `!` after the type. A
  change that breaks something goes in as `feat` or `fix` and says what breaks in the body, in
  plain words — never with `BREAKING CHANGE` at the start of a line, where it counts as a marker
  too.
