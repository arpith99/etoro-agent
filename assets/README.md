# Vendored assets

## `lightweight-charts.standalone.production.js`

TradingView Lightweight Charts v4.2.3, Apache License 2.0. The upstream licence
header is preserved at the top of the file; the full text is at
<https://www.apache.org/licenses/LICENSE-2.0>.

Vendored rather than loaded from a CDN so that generated charts open offline,
render identically a year from now, and do not report a fetch to a third party
every time somebody looks at their own portfolio.

Sourced from
<https://unpkg.com/lightweight-charts@4.2.3/dist/lightweight-charts.standalone.production.js>.
Replacing it is a deliberate act: check the release notes, because the chart
API is not stable across major versions.
