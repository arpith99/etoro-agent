#!/usr/bin/env bash
# Loads a generated chart page in headless Chrome and fails on console errors.
#
# The Rust tests can check that the page contains the right strings; they
# cannot check that it runs. Two bugs shipped past them: a chart created 0x0
# and therefore invisible, and a CSS colour keyword the charting library
# rejects, which threw out of createChart and aborted the whole script. Both
# produced a page that looked structurally perfect and rendered nothing.
#
# Not part of the default gate, because it needs a browser installed.
#
#   cargo run -- chart AAPL --html /tmp/aapl.html
#   ./scripts/check_chart_page.sh /tmp/aapl.html

set -euo pipefail

page="${1:-}"
if [[ -z $page || ! -f $page ]]; then
    echo "usage: $0 <chart.html>" >&2
    exit 2
fi

browser=""
for candidate in google-chrome chromium chromium-browser microsoft-edge; do
    if command -v "$candidate" >/dev/null 2>&1; then
        browser=$candidate
        break
    fi
done
if [[ -z $browser ]]; then
    echo "SKIP: no Chrome-family browser found" >&2
    exit 0
fi

workdir=$(mktemp -d)
trap 'rm -rf "$workdir"' EXIT

# --virtual-time-budget lets the page's timers run before the DOM is dumped.
"$browser" --headless --disable-gpu --no-sandbox \
    --virtual-time-budget=6000 --enable-logging=stderr --v=0 \
    --dump-dom "file://$(realpath "$page")" \
    >"$workdir/dom.html" 2>"$workdir/log.txt" || true

status=0

if grep -q "CONSOLE" "$workdir/log.txt"; then
    echo "FAIL: the page logged to the console:" >&2
    grep "CONSOLE" "$workdir/log.txt" | sed 's/^/  /' >&2
    status=1
fi

# A page whose script threw still has its static markup, so the check has to be
# for things only the script creates.
buttons=$(grep -o "<button" "$workdir/dom.html" | wc -l)
canvases=$(grep -o "<canvas" "$workdir/dom.html" | wc -l)

if [[ $buttons -lt 1 ]]; then
    echo "FAIL: no controls were rendered (script did not finish)" >&2
    status=1
fi
if [[ $canvases -lt 1 ]]; then
    echo "FAIL: no canvas was rendered (chart was not drawn)" >&2
    status=1
fi

if [[ $status -eq 0 ]]; then
    echo "OK: $buttons controls, $canvases canvases, no console output"
fi
exit $status
