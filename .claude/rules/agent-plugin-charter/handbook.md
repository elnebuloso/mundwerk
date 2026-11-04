# Writing handbook pages

The handbook is what a reader of the product reads. It lives in `docs/handbook/` and is
written to be published on its own one day. It is a manual for using the product, not a record of
how it was built.

- **Handbook pages are written in `German`.** An English-in-source rule governs code, comments,
  commits and logs — not the product's documentation.
- **The handbook links only within itself.** Published, it is the only thing there is, and a link
  that leaves it leads nowhere. Point into the code by naming the path instead of linking it —
  whoever has the repository open finds it, whoever has only the pages loses nothing but a click.
- **Terms and names are quoted verbatim.** Code excerpts, paths and identifiers appear as written,
  never replaced by a description of them. Where the English term is the one people say out loud —
  and usually the one the code uses — it stays untranslated; inventing a local word costs the
  reader the shared vocabulary and the string they would have searched for. Translate what has a
  real equivalent, and name the identifier alongside when a page introduces a term. Labels in the
  interface are spelled exactly as the product shows them, casing included, even where the
  sentence's grammar would want another form.
- **A sentence stays only if the reader does or decides something differently for knowing it.**
  How the product's parts fit together, as the reader meets them, passes that test — a page may
  explain it before any step. Delete the rest, and a heading left with nothing under it goes too.
- **Write what the reader cannot see.** The product is in front of them: no description of the
  screen, no step for a click the screen already makes obvious. Numbered steps are for a task
  whose order matters or that spans several places. A page earns its text with what the interface
  does not show — limits, quotas, durations, validation rules, retention, something shown only
  once; the exact error text, its cause and the fix; what cannot be undone, said before the step
  that does it; what each option leads to where the reader has to choose; a path like `Settings >
  Team > Roles` where the target is buried. A feature that takes a paragraph to find is a flaw in
  the product: the page gives the shortest way there, and the flaw is reported to whoever builds
  the product, not explained to the reader.
- **Never in the handbook:** how the product is built — algorithms, storage, frameworks; why it
  was built that way, what it used to do, what is planned; reassurance without the guarantee
  behind it; settings and files the page's reader cannot reach — the page names who can, where the
  reader has to ask them. What only maintainers need belongs next to the code.
- **One reader per page.** Where the product has several roles, the page names its role at the
  top, and content for another role goes on that role's page. What a reader needs before a feature
  works for them — a plan, a version, a permission — stands at the top of the page, not at the step
  where it fails.
- **Troubleshooting sits with the feature.** No separate FAQ page: an error and its fix go under
  the feature that raises it. Headings use the words the reader searches for, error code included —
  `Rate limit exceeded (429)`, not `API throttling considerations`.
- **Examples run once the placeholders are filled in.** A code block is complete, with nothing
  left out but the values the reader must replace, and those are visibly placeholders. Every other
  value is a realistic one from the product's domain, never `foo` or `test123`.
- **A screenshot shows, the text instructs.** Screenshots are welcome where they help the reader
  find their way, but every instruction, path, label and value also stands in the text — a page
  must work for a reader who cannot see the image, and a screenshot outdated by a redesign must
  not leave it wrong. Every image has alt text that says what it shows.
