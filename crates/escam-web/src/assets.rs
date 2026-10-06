//! # Embedded Static Assets for Snappy Modern SPA
//!
//! Provides the pre-compressed, embedded single-page application loaded directly
//! from camera memory without external CDN requests.

pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
  <title>ESCAM G02 • Pure Rust Camera</title>
  <style>
    :root {
      --bg: #0a0e17;
      --card-bg: rgba(18, 26, 43, 0.85);
      --accent: #00d2ff;
      --accent-glow: rgba(0, 210, 255, 0.35);
      --text: #e2e8f0;
      --text-muted: #94a3b8;
      --danger: #ef4444;
      --success: #10b981;
      --border: rgba(255, 255, 255, 0.08);
    }
    * { box-sizing: border-box; margin: 0; padding: 0; user-select: none; }
    body {
      background: var(--bg);
      color: var(--text);
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 12px;
    }
    header {
      width: 100%;
      max-width: 900px;
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 12px 16px;
      background: var(--card-bg);
      border-radius: 12px;
      border: 1px solid var(--border);
      backdrop-filter: blur(12px);
      margin-bottom: 12px;
    }
    .logo { font-size: 1.1rem; font-weight: 700; letter-spacing: 0.5px; color: var(--accent); }
    .badge {
      font-size: 0.75rem;
      padding: 4px 8px;
      border-radius: 6px;
      background: rgba(16, 185, 129, 0.15);
      color: var(--success);
      border: 1px solid rgba(16, 185, 129, 0.3);
    }
    main {
      width: 100%;
      max-width: 900px;
      display: grid;
      grid-template-columns: 1fr;
      gap: 12px;
    }
    .viewport-card {
      position: relative;
      background: #000;
      border-radius: 14px;
      overflow: hidden;
      aspect-ratio: 16 / 9;
      border: 1px solid var(--border);
      box-shadow: 0 10px 30px rgba(0,0,0,0.5);
    }
    video { width: 100%; height: 100%; object-fit: contain; }
    .hud-overlay {
      position: absolute;
      top: 12px;
      left: 12px;
      right: 12px;
      display: flex;
      justify-content: space-between;
      pointer-events: none;
      font-family: monospace;
      font-size: 0.8rem;
      color: rgba(255, 255, 255, 0.85);
      text-shadow: 0 2px 4px rgba(0,0,0,0.8);
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
      border: 1px solid var(--border);
      border-radius: 14px;
      padding: 16px;
      backdrop-filter: blur(12px);
    }
    .card h3 { font-size: 0.95rem; margin-bottom: 12px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.5px; }
    .joystick-container {
      position: relative;
      width: 180px;
      height: 180px;
      margin: 0 auto;
      border-radius: 50%;
      background: rgba(0, 0, 0, 0.4);
      border: 2px solid var(--border);
      display: flex;
      align-items: center;
      justify-content: center;
      touch-action: none;
    }
    .joystick-handle {
      width: 64px;
      height: 64px;
      border-radius: 50%;
      background: linear-gradient(135deg, #00d2ff, #0077ff);
      box-shadow: 0 4px 16px var(--accent-glow);
      position: absolute;
      cursor: grab;
      transition: transform 0.05s ease-out;
    }
    .btn-group { display: flex; gap: 8px; flex-wrap: wrap; }
    button {
      flex: 1;
      padding: 10px 14px;
      border-radius: 8px;
      border: 1px solid var(--border);
      background: rgba(255, 255, 255, 0.05);
      color: var(--text);
      font-weight: 600;
      font-size: 0.85rem;
      cursor: pointer;
      transition: all 0.15s ease;
    }
    button:hover { background: rgba(0, 210, 255, 0.15); border-color: var(--accent); }
    button.active { background: var(--accent); color: #000; box-shadow: 0 0 12px var(--accent-glow); }
  </style>
</head>
<body>
  <header>
    <div class="logo">⚡ ESCAM G02 • Rust Core</div>
    <div class="badge" id="connStatus">WEBRTC ONLINE</div>
  </header>
  <main>
    <div class="viewport-card">
      <video id="liveVideo" autoplay playsinline muted></video>
      <div class="hud-overlay">
        <span id="hudFps">25.0 FPS • 720p H.264</span>
        <span id="hudLatency">< 85ms</span>
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

    // Virtual Joystick Controller
    const zone = document.getElementById('joystickZone');
    const handle = document.getElementById('joystickHandle');
    let dragging = false;
    const maxRadius = 55;

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
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ type: 'ptz_joystick', x: normX, y: normY }));
      }
    }

    zone.addEventListener('pointerdown', (e) => { dragging = true; handleMove(e.clientX, e.clientY); });
    window.addEventListener('pointermove', (e) => { if (dragging) handleMove(e.clientX, e.clientY); });
    window.addEventListener('pointerup', () => {
      if (dragging) {
        dragging = false;
        handle.style.transform = 'translate(0px, 0px)';
        if (ws.readyState === WebSocket.OPEN) {
          ws.send(JSON.stringify({ type: 'ptz_joystick', x: 0, y: 0 }));
        }
      }
    });
  </script>
</body>
</html>"#;
