# Release notes

semantic-release fills a GitHub release with commit subjects only. A user reading the releases
page should learn what a release brings without decoding `feat:` lines.

- **After every release, write its notes.** Once the CI run is green and the release exists,
  put a description above the generated changelog with `gh release edit <tag> --notes-file`.
  Keep the generated part below a `---` line.
- **Written in German, for users, not developers:** a `## Mundwerk X.Y.Z – <what's new>` heading,
  what changed and how to use it, what the user has to do (first start, permissions), known
  limits, and the download files per system. Match the notes of the earlier releases.
- **Only what is shipped and checked.** Say plainly what is untested (e.g. a platform nobody has
  run yet). Never touch the `v0.0.0` placeholder release.
