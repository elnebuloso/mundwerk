# Marking chat replies

Start every final chat reply to the user with `[<folder>]` on the first line, where `<folder>` is
the name of the project's root directory (e.g. `[my-app]`) — never in progress updates, files,
commits, tool calls or replies to another agent.
Why: a reply without the marker shows that this instruction is no longer in context.
