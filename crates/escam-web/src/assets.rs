//! # Embedded Static Assets for Snappy Modern SPA
//!
//! Provides the pre-compressed, embedded single-page application loaded directly
//! from camera memory without external CDN requests.
//!
//! Themed in a surrealist Dada aesthetic inspired by Max Ernst ('L'Oeil Céleste',
//! decalcomania strata, petrified biomorphic textures, and patinated Dada machinery).

pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
  <title>ESCAM G02 • Pure Rust Camera</title>
  <style>
    :root {
      --bg-dark: #0c110f;
      --bg-forest: #131a17;
      --patina-verdigris: #4fa394;
      --patina-mint: #7eceba;
      --patina-dark: #24443b;
      --ochre-rich: #c68b39;
      --ochre-bright: #e5a84b;
      --sienna-burnt: #8c4327;
      --umber-dark: #221a15;
      --parchment: #e2dbcb;
      --parchment-dim: #9e9584;
      --brass-border: #7d5e33;
      --card-bg: rgba(19, 25, 22, 0.90);
      --decal-strata: radial-gradient(circle at 18% 22%, rgba(140, 67, 39, 0.18) 0%, transparent 45%),
                      radial-gradient(circle at 82% 16%, rgba(79, 163, 148, 0.22) 0%, transparent 40%),
                      radial-gradient(ellipse at 50% 85%, rgba(34, 26, 21, 0.6) 0%, transparent 60%);
    }
    * { box-sizing: border-box; margin: 0; padding: 0; user-select: none; }
    body {
      background: var(--decal-strata), linear-gradient(175deg, #090e0c 0%, #121815 45%, #181d19 100%);
      color: var(--parchment);
      font-family: 'Cinzel', 'Palatino Linotype', 'Book Antiqua', Georgia, serif;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 12px;
    }
    header {
      width: 100%;
      max-width: 920px;
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 12px 18px;
      background: var(--card-bg);
      border-radius: 8px;
      border: 1px solid var(--brass-border);
      box-shadow: 0 4px 18px rgba(0, 0, 0, 0.6), inset 0 0 12px rgba(79, 163, 148, 0.08);
      margin-bottom: 12px;
    }
    .logo {
      font-size: 1.05rem;
      font-weight: 700;
      letter-spacing: 1.5px;
      color: var(--ochre-bright);
      text-shadow: 0 1px 3px rgba(0, 0, 0, 0.9);
      display: flex;
      align-items: center;
      gap: 8px;
    }
    .logo span { font-size: 0.72rem; color: var(--patina-mint); font-family: monospace; letter-spacing: 1px; }
    .badge {
      font-family: 'Courier New', monospace;
      font-size: 0.72rem;
      letter-spacing: 1.2px;
      padding: 4px 10px;
      border-radius: 4px;
      background: rgba(79, 163, 148, 0.12);
      color: var(--patina-mint);
      border: 1px dashed var(--patina-verdigris);
      text-transform: uppercase;
    }
    main {
      width: 100%;
      max-width: 920px;
      display: grid;
      grid-template-columns: 1fr;
      gap: 12px;
    }
    .viewport-card {
      position: relative;
      background: #000;
      border-radius: 10px;
      overflow: hidden;
      aspect-ratio: 16 / 9;
      border: 2px solid var(--brass-border);
      box-shadow: 0 12px 36px rgba(0, 0, 0, 0.8), 0 0 20px rgba(79, 163, 148, 0.12);
    }
    video { width: 100%; height: 100%; object-fit: contain; }
    .hud-overlay {
      position: absolute;
      top: 12px;
      left: 14px;
      right: 14px;
      display: flex;
      justify-content: space-between;
      pointer-events: none;
      font-family: 'Courier New', monospace;
      font-size: 0.8rem;
      letter-spacing: 0.5px;
      color: var(--ochre-bright);
      text-shadow: 0 2px 6px rgba(0, 0, 0, 0.95), 0 0 8px rgba(198, 139, 57, 0.4);
    }
    .controls-grid {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 12px;
    }
    @media (max-width: 600px) {
      .controls-grid { grid-template-columns: 1fr; }
    }
    .card {
      background: var(--card-bg);
      border: 1px solid var(--brass-border);
      border-radius: 8px;
      padding: 16px;
      box-shadow: 0 6px 20px rgba(0, 0, 0, 0.55), inset 0 0 14px rgba(34, 26, 21, 0.4);
    }
    .card h3 {
      font-size: 0.82rem;
      margin-bottom: 12px;
      color: var(--ochre-rich);
      text-transform: uppercase;
      letter-spacing: 1.5px;
      border-bottom: 1px solid rgba(198, 139, 57, 0.2);
      padding-bottom: 6px;
    }
    .joystick-container {
      position: relative;
      width: 180px;
      height: 180px;
      margin: 0 auto;
      border-radius: 50%;
      background: radial-gradient(circle, #15221c 0%, #0d1411 65%, #1f2d25 100%);
      border: 2px solid var(--patina-dark);
      box-shadow: inset 0 0 24px rgba(0, 0, 0, 0.9), 0 0 16px rgba(79, 163, 148, 0.2);
      display: flex;
      align-items: center;
      justify-content: center;
      touch-action: none;
    }
    .joystick-container::before {
      content: '';
      position: absolute;
      width: 110px;
      height: 110px;
      border-radius: 50%;
      border: 1px dashed rgba(198, 139, 57, 0.25);
      pointer-events: none;
    }
    .joystick-handle {
      width: 64px;
      height: 64px;
      border-radius: 50%;
      background: radial-gradient(circle at 35% 35%, #fff6e0 0%, #d49b43 18%, #8c4327 42%, #2c594d 72%, #0e1613 100%);
      border: 2px solid var(--ochre-rich);
      box-shadow: 0 6px 18px rgba(0, 0, 0, 0.85), 0 0 16px rgba(212, 155, 67, 0.5), inset 2px 2px 4px rgba(255, 255, 255, 0.4);
      position: absolute;
      cursor: grab;
      transition: transform 0.05s ease-out;
    }
    .joystick-handle:active { cursor: grabbing; box-shadow: 0 0 24px rgba(229, 168, 75, 0.8), inset 0 0 6px rgba(255, 255, 255, 0.6); }
    .btn-group { display: flex; gap: 8px; flex-wrap: wrap; }
    button {
      flex: 1;
      padding: 10px 14px;
      border-radius: 6px;
      border: 1px solid var(--brass-border);
      background: linear-gradient(180deg, #2b3933 0%, #19221e 100%);
      color: var(--parchment);
      font-family: inherit;
      font-weight: 600;
      font-size: 0.82rem;
      letter-spacing: 0.5px;
      cursor: pointer;
      box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.1), 0 2px 5px rgba(0, 0, 0, 0.6);
      transition: all 0.15s ease;
    }
    button:hover {
      background: linear-gradient(180deg, #374b43 0%, #202d27 100%);
      border-color: var(--patina-verdigris);
      color: #fff;
    }
    button.active {
      background: linear-gradient(180deg, var(--ochre-rich) 0%, var(--sienna-burnt) 100%);
      color: #fff9ed;
      border-color: var(--ochre-bright);
      box-shadow: 0 0 14px rgba(198, 139, 57, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.3);
    }
  </style>
</head>
<body>
  <header>
    <div class="logo">⚡ ESCAM G02 <span>• Pure Rust Firmware</span></div>
    <div class="badge" id="connStatus">WEBRTC ONLINE</div>
  </header>
  <main>
    <div class="viewport-card">
      <video id="liveVideo" autoplay playsinline muted></video>
      <div class="hud-overlay">
        <span id="hudFps">25.0 FPS • 720p H.264</span>
        <span id="hudLatency">&lt; 85ms Latency</span>
      </div>
    </div>
    <div class="controls-grid">
      <div class="card">
        <h3>PTZ S-Curve Virtual Joystick</h3>
        <div class="joystick-container" id="joystickZone">
          <div class="joystick-handle" id="joystickHandle"></div>
        </div>
      </div>
      <div class="card">
        <h3>Optical & Sensor Mode</h3>
        <div class="btn-group">
          <button id="btnDay" class="active" onclick="setIrCut('Day')">Day (IR-Cut ON)</button>
          <button id="btnNight" onclick="setIrCut('Night')">Night / Astro (Hα)</button>
        </div>
        <h3 style="margin-top: 16px;">Quick Presets</h3>
        <div class="btn-group">
          <button onclick="sendPtz('Home')">Home</button>
          <button onclick="sendPtz('Stop')">Halt</button>
          <button onclick="triggerSnapshot()">Snapshot</button>
        </div>
      </div>
    </div>
  </main>
  <script>
    const ws = new WebSocket(`ws://${location.host}/api/v1/ws`);
    ws.onmessage = (msg) => { console.log("Telemetry:", msg.data); };

    function setIrCut(mode) {
      fetch('/api/v1/ircut', { method: 'POST', body: JSON.stringify({ mode }), headers: { 'Content-Type': 'application/json' } });
      document.getElementById('btnDay').classList.toggle('active', mode === 'Day');
      document.getElementById('btnNight').classList.toggle('active', mode === 'Night');
    }

    function sendPtz(act) {
      fetch('/api/v1/ptz', { method: 'POST', body: JSON.stringify({ action: act }), headers: { 'Content-Type': 'application/json' } });
    }

    function triggerSnapshot() { window.open('/api/v1/snapshot', '_blank'); }

    // Virtual Joystick Controller with HTTP Fallback & Throttling
    const zone = document.getElementById('joystickZone');
    const handle = document.getElementById('joystickHandle');
    let dragging = false;
    let lastPtzSend = 0;
    const maxRadius = 55;

    function sendJoystickCoords(x, y) {
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ type: 'ptz_joystick', x, y }));
      } else {
        const now = Date.now();
        if (now - lastPtzSend > 75) { // ~13 Hz rate limit
          lastPtzSend = now;
          fetch('/api/v1/ptz', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ action: 'Joystick', x, y })
          }).catch(() => {});
        }
      }
    }

    function handleMove(clientX, clientY) {
      const rect = zone.getBoundingClientRect();
      const centerX = rect.left + rect.width / 2;
      const centerY = rect.top + rect.height / 2;
      let dx = clientX - centerX;
      let dy = clientY - centerY;
      const dist = Math.sqrt(dx * dx + dy * dy);
      if (dist > maxRadius) {
        dx = (dx / dist) * maxRadius;
        dy = (dy / dist) * maxRadius;
      }
      handle.style.transform = `translate(${dx}px, ${dy}px)`;
      const normX = dx / maxRadius;
      const normY = -dy / maxRadius;
      sendJoystickCoords(normX, normY);
    }

    zone.addEventListener('pointerdown', (e) => { dragging = true; handleMove(e.clientX, e.clientY); });
    window.addEventListener('pointermove', (e) => { if (dragging) handleMove(e.clientX, e.clientY); });
    function handleEnd() {
      if (dragging) {
        dragging = false;
        handle.style.transform = 'translate(0px, 0px)';
        fetch('/api/v1/ptz', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ action: 'Stop' })
        }).catch(() => {});
      }
    }
    window.addEventListener('pointerup', handleEnd);
    window.addEventListener('pointercancel', handleEnd);
  </script>
</body>
</html>"#;
