#!/bin/sh
# Advisory, once per session: tell the human when the `cospan` binary is not on
# PATH. The plugin ships the MCP-server config and this checkout; installing the
# binary is a separate `cargo install`, which nothing enforces. Without this, the
# user just sees an MCP server that fails to start, with nothing naming the fix.
#
# ADVISORY like day's equivalent: it emits a `systemMessage` (a notice to the
# human, never a decision) and exits 0 unconditionally — a missing dependency
# must not fail a session. Emits NOTHING when `cospan` is already present.

command -v cospan >/dev/null 2>&1 && exit 0

# The plugin root is a full cospan checkout, so it can be installed from itself —
# no crates.io publish, no git auth for the private repo.
root=${CLAUDE_PLUGIN_ROOT:-.}
banner='cospan plugin: the cospan binary is not on PATH'
body='cospan mcp serves the comment layer to agents; the plugin points at a cospan that must be installed first:'

# Built with literal \n sequences (printf %s does not process escapes in the
# argument), so they reach stdout as the two characters JSON wants and the
# payload stays one valid line. No double quote appears in the text, deliberately.
printf '{"systemMessage":"%s"}\n' "\
⚠  $banner\\n\\n\
  $body\\n\\n\
    cargo install --path $root\\n\\n\
  Then run /reload-plugins, or start a new session."
exit 0
