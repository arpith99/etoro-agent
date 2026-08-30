// Page logic for the generated candlestick chart.
//
// Reads window.CHART_DATA, which the Rust side writes immediately above this
// script. Kept as a real .js file rather than a string inside format!(), so
// braces do not have to be doubled and an editor can actually check it.

(() => {
  const DATA = window.CHART_DATA;
  const el = (id) => document.getElementById(id);

  // The library parses colours itself and rejects CSS keywords such as
  // currentColor -- it throws out of createChart, which aborts this whole
  // script and leaves a page with a heading and nothing else. Resolve against
  // the page so the chart still follows light and dark mode.
  const pageColor = getComputedStyle(document.body).color;

  const chart = LightweightCharts.createChart(el('chart'), {
    // Without this the chart is created 0x0 and stays invisible: the library
    // does not read the container's size unless asked to.
    autoSize: true,
    layout: { background: { color: 'transparent' }, textColor: pageColor },
    grid: {
      vertLines: { color: 'rgba(128,128,128,0.16)' },
      horzLines: { color: 'rgba(128,128,128,0.16)' },
    },
    crosshair: { mode: LightweightCharts.CrosshairMode.Normal },
    rightPriceScale: { borderVisible: false, scaleMargins: { top: 0.08, bottom: 0.26 } },
    timeScale: { borderVisible: false, rightOffset: 4 },
  });

  const candles = chart.addCandlestickSeries({
    upColor: '#26a69a', downColor: '#ef5350',
    borderUpColor: '#26a69a', borderDownColor: '#ef5350',
    wickUpColor: '#26a69a', wickDownColor: '#ef5350',
  });

  // Volume shares the pane but has its own scale, pinned to the lower quarter.
  let volume = null;
  if (DATA.volume && DATA.volume.length) {
    volume = chart.addHistogramSeries({
      priceScaleId: 'volume',
      priceFormat: { type: 'volume' },
    });
    chart.priceScale('volume').applyOptions({ scaleMargins: { top: 0.78, bottom: 0 } });
  }

  // ---- series selection (as reported vs total return) ----------------------

  let active = 'reported';
  const bars = () => DATA[active];
  // A split changes the share count, so volume steps at the split date on an
  // as-traded series exactly as price does. Switching one without the other
  // would leave the lower pane quietly disagreeing with the upper one.
  const volumeBars = () =>
    (active === 'adjusted' && DATA.volumeAdjusted) || DATA.volume;

  function showSeries(name) {
    active = name;
    candles.setData(bars());
    if (volume) volume.setData(volumeBars());
    refreshMas();
  }

  // ---- moving averages -----------------------------------------------------

  const MA_LENGTHS = [20, 50, 200];
  const MA_COLORS = { 20: '#2962ff', 50: '#ff9800', 200: '#9c27b0' };
  const maSeries = new Map();

  function sma(data, length) {
    const out = [];
    let sum = 0;
    for (let i = 0; i < data.length; i++) {
      sum += data[i].close;
      if (i >= length) sum -= data[i - length].close;
      if (i >= length - 1) out.push({ time: data[i].time, value: sum / length });
    }
    return out;
  }

  function toggleMa(length, on) {
    if (on && !maSeries.has(length)) {
      const line = chart.addLineSeries({
        color: MA_COLORS[length], lineWidth: 1, priceLineVisible: false,
        lastValueVisible: false, crosshairMarkerVisible: false,
      });
      line.setData(sma(bars(), length));
      maSeries.set(length, line);
    } else if (!on && maSeries.has(length)) {
      chart.removeSeries(maSeries.get(length));
      maSeries.delete(length);
    }
  }

  function refreshMas() {
    for (const [length, line] of maSeries) line.setData(sma(bars(), length));
  }

  // ---- legend --------------------------------------------------------------

  const legend = el('legend');
  const fmt = (n) => (n === undefined || n === null ? '-' : n.toFixed(2));

  function showLegend(bar) {
    if (!bar) { legend.innerHTML = '&nbsp;'; return; }
    const direction = bar.close >= bar.open ? 'up' : 'down';
    const change = bar.open ? ((bar.close / bar.open - 1) * 100).toFixed(2) : '0.00';
    legend.innerHTML =
      `<strong>${bar.time}</strong>  ` +
      `O ${fmt(bar.open)}  H ${fmt(bar.high)}  L ${fmt(bar.low)}  ` +
      `<span class="${direction}">C ${fmt(bar.close)} (${change > 0 ? '+' : ''}${change}%)</span>`;
  }

  chart.subscribeCrosshairMove((param) => {
    if (!param.time) { showLegend(bars()[bars().length - 1]); return; }
    showLegend(param.seriesData.get(candles));
  });

  // ---- controls ------------------------------------------------------------

  function group(label, buttons) {
    const wrap = document.createElement('div');
    wrap.className = 'group';
    if (label) {
      const tag = document.createElement('span');
      tag.className = 'label';
      tag.textContent = label;
      wrap.append(tag);
    }
    for (const [text, pressed, onClick, exclusive] of buttons) {
      const button = document.createElement('button');
      button.textContent = text;
      button.setAttribute('aria-pressed', String(pressed));
      button.onclick = () => {
        const next = exclusive ? true : button.getAttribute('aria-pressed') !== 'true';
        if (exclusive) {
          for (const sibling of wrap.querySelectorAll('button'))
            sibling.setAttribute('aria-pressed', String(sibling === button));
        } else {
          button.setAttribute('aria-pressed', String(next));
        }
        onClick(next);
      };
      wrap.append(button);
    }
    return wrap;
  }

  const bar = el('controls');

  if (DATA.adjusted) {
    bar.append(group('series', [
      ['As reported', true, () => showSeries('reported'), true],
      ['Total return', false, () => showSeries('adjusted'), true],
    ]));
  }

  bar.append(group('range', [
    ['1M', false, () => setRange(22), true],
    ['3M', false, () => setRange(66), true],
    ['6M', false, () => setRange(130), true],
    ['1Y', false, () => setRange(252), true],
    ['5Y', false, () => setRange(1260), true],
    ['All', true, () => chart.timeScale().fitContent(), true],
  ]));

  bar.append(group('MA', MA_LENGTHS.map(
    (length) => [String(length), false, (on) => toggleMa(length, on), false],
  )));

  bar.append(group('scale', [
    ['log', false, (on) => chart.priceScale('right').applyOptions({
      mode: on ? LightweightCharts.PriceScaleMode.Logarithmic
               : LightweightCharts.PriceScaleMode.Normal,
    }), false],
  ]));

  function setRange(sessions) {
    const total = bars().length;
    chart.timeScale().setVisibleLogicalRange({
      from: Math.max(0, total - sessions),
      to: total - 1,
    });
  }

  // ---- go ------------------------------------------------------------------

  showSeries('reported');
  chart.timeScale().fitContent();
  showLegend(bars()[bars().length - 1]);
})();
