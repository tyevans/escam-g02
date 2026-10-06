//! # Embedded Static Assets for Snappy Modern SPA
//!
//! Provides the pre-compressed, embedded single-page application loaded directly
//! from camera memory without external CDN requests.
//!
//! Features dual theming:
//! - Surrealist Dada Mode (Max Ernst 'L'Oeil Céleste', decalcomania & brass)
//! - FNAF Security Station Mode (Five Nights at Freddy's CRT surveillance monitor)

pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
  <title>ESCAM G02 • Surveillance Station</title>
  <style>
    :root {
      --bg-dark: #0c110f; --bg-forest: #131a17; --patina-verdigris: #4fa394; --patina-mint: #7eceba;
      --patina-dark: #24443b; --ochre-rich: #c68b39; --ochre-bright: #e5a84b; --sienna-burnt: #8c4327;
      --brass-border: #7d5e33; --card-bg: rgba(19, 25, 22, 0.90); --text-col: #e2dbcb;
      --decal-strata: radial-gradient(circle at 18% 22%, rgba(140, 67, 39, 0.18) 0%, transparent 45%),
                      radial-gradient(circle at 82% 16%, rgba(79, 163, 148, 0.22) 0%, transparent 40%),
                      radial-gradient(ellipse at 50% 85%, rgba(34, 26, 21, 0.6) 0%, transparent 60%);
      --main-font: 'Cinzel', 'Palatino Linotype', 'Book Antiqua', Georgia, serif;
    }
    body.theme-fnaf {
      --bg-dark: #040804; --bg-forest: #070e08; --patina-verdigris: #228b22; --patina-mint: #39ff14;
      --patina-dark: #0f2b13; --ochre-rich: #2bb82b; --ochre-bright: #39ff14; --sienna-burnt: #164016;
      --brass-border: #1f4d24; --card-bg: rgba(6, 14, 7, 0.95); --text-col: #39ff14;
      --decal-strata: radial-gradient(ellipse at 50% 50%, rgba(15, 38, 18, 0.3) 0%, transparent 80%);
      --main-font: 'VT323', 'Courier New', Consolas, monospace;
    }
    * { box-sizing: border-box; margin: 0; padding: 0; user-select: none; }
    body {
      background: var(--decal-strata), linear-gradient(175deg, #090e0c 0%, #121815 45%, #181d19 100%);
      color: var(--text-col); font-family: var(--main-font);
      min-height: 100vh; display: flex; flex-direction: column; align-items: center; padding: 12px;
      transition: background 0.2s, color 0.2s;
    }
    body.theme-fnaf {
      background: var(--decal-strata), radial-gradient(circle at center, #0a140a 0%, #030603 100%);
      text-shadow: 0 0 5px rgba(57, 255, 20, 0.5);
    }
    header {
      width: 100%; max-width: 920px; display: flex; justify-content: space-between; align-items: center;
      padding: 10px 16px; background: var(--card-bg); border-radius: 8px; border: 1px solid var(--brass-border);
      box-shadow: 0 4px 18px rgba(0, 0, 0, 0.6); margin-bottom: 8px;
    }
    .logo {
      font-size: 1rem; font-weight: 700; letter-spacing: 1.2px; color: var(--ochre-bright);
      display: flex; align-items: center; gap: 8px;
    }
    .logo span { font-size: 0.72rem; color: var(--patina-mint); font-family: monospace; }
    .header-actions { display: flex; align-items: center; gap: 8px; }
    .badge {
      font-family: 'Courier New', monospace; font-size: 0.7rem; letter-spacing: 1.2px; padding: 4px 8px;
      border-radius: 4px; background: rgba(79, 163, 148, 0.12); color: var(--patina-mint);
      border: 1px dashed var(--patina-verdigris); text-transform: uppercase;
    }
    body.theme-fnaf .badge {
      background: rgba(57, 255, 20, 0.1); border-color: #39ff14; box-shadow: 0 0 8px rgba(57, 255, 20, 0.3);
    }
    .theme-toggle-btn {
      padding: 4px 10px; font-size: 0.72rem; letter-spacing: 1px; border-radius: 4px; cursor: pointer;
      border: 1px solid var(--brass-border); background: rgba(255,255,255,0.06); color: var(--text-col);
      font-family: inherit; font-weight: 600;
    }
    .fnaf-subbar {
      display: none; width: 100%; max-width: 920px; justify-content: space-between; align-items: center;
      padding: 6px 14px; background: rgba(4, 10, 5, 0.95); border: 1px solid #1f4d24;
      font-family: 'Courier New', monospace; font-size: 0.78rem; font-weight: 700; color: #39ff14;
      margin-bottom: 8px; border-radius: 6px; box-shadow: 0 0 10px rgba(57, 255, 20, 0.15);
    }
    body.theme-fnaf .fnaf-subbar { display: flex; }
    .rec-dot { color: #ff3b30; text-shadow: 0 0 8px #ff3b30; animation: blink 1s steps(1) infinite; }
    @keyframes blink { 0%, 49% { opacity: 1; } 50%, 100% { opacity: 0.1; } }
    main { width: 100%; max-width: 920px; display: grid; grid-template-columns: 1fr; gap: 12px; }
    .viewport-card {
      position: relative; background: #000; border-radius: 8px; overflow: hidden; aspect-ratio: 16 / 9;
      border: 2px solid var(--brass-border); box-shadow: 0 12px 36px rgba(0, 0, 0, 0.85);
    }
    body.theme-fnaf .viewport-card {
      border: 8px solid #232a22; outline: 2px dashed #3a4738; outline-offset: -5px;
      box-shadow: 0 0 0 3px #111610, 0 12px 36px rgba(0, 0, 0, 0.95), 0 0 24px rgba(57, 255, 20, 0.2);
    }
    video { width: 100%; height: 100%; object-fit: contain; }
    .crt-overlay { position: absolute; inset: 0; pointer-events: none; opacity: 0; transition: opacity 0.2s; }
    body.theme-fnaf .crt-overlay {
      opacity: 1;
      background: linear-gradient(rgba(18, 16, 16, 0) 50%, rgba(0, 0, 0, 0.45) 50%),
                  radial-gradient(ellipse at center, transparent 60%, rgba(0, 0, 0, 0.85) 100%);
      background-size: 100% 3px, 100% 100%;
      animation: crtFlicker 0.15s infinite;
    }
    @keyframes crtFlicker { 0% { opacity: 0.92; } 50% { opacity: 0.99; } 100% { opacity: 0.94; } }
    .hud-overlay {
      position: absolute; top: 12px; left: 14px; right: 14px; display: flex; justify-content: space-between;
      pointer-events: none; font-family: 'Courier New', monospace; font-size: 0.8rem; letter-spacing: 0.5px;
      color: var(--ochre-bright); text-shadow: 0 2px 6px rgba(0,0,0,0.95), 0 0 8px rgba(57,255,20,0.4);
    }
    .controls-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
    @media (max-width: 600px) { .controls-grid { grid-template-columns: 1fr; } }
    .card {
      background: var(--card-bg); border: 1px solid var(--brass-border); border-radius: 8px;
      padding: 16px; box-shadow: 0 6px 20px rgba(0, 0, 0, 0.55);
    }
    .card h3 {
      font-size: 0.82rem; margin-bottom: 12px; color: var(--ochre-rich); text-transform: uppercase;
      letter-spacing: 1.5px; border-bottom: 1px solid rgba(198, 139, 57, 0.2); padding-bottom: 6px;
    }
    body.theme-fnaf .card h3 { border-bottom-color: rgba(57, 255, 20, 0.2); }
    .joystick-container {
      position: relative; width: 180px; height: 180px; margin: 0 auto; border-radius: 50%;
      background: radial-gradient(circle, #15221c 0%, #0d1411 65%, #1f2d25 100%);
      border: 2px solid var(--patina-dark); box-shadow: inset 0 0 24px rgba(0,0,0,0.9), 0 0 16px rgba(79,163,148,0.2);
      display: flex; align-items: center; justify-content: center; touch-action: none;
    }
    .joystick-container::before {
      content: ''; position: absolute; width: 110px; height: 110px; border-radius: 50%;
      border: 1px dashed rgba(198, 139, 57, 0.25); pointer-events: none;
    }
    body.theme-fnaf .joystick-container {
      background: radial-gradient(circle, #08170c 0%, #030a05 75%),
                  linear-gradient(0deg, transparent 49%, rgba(57,255,20,0.18) 50%, transparent 51%),
                  linear-gradient(90deg, transparent 49%, rgba(57,255,20,0.18) 50%, transparent 51%);
      border-color: #1f4d24; box-shadow: inset 0 0 24px rgba(0,0,0,0.95), 0 0 16px rgba(57,255,20,0.25);
    }
    body.theme-fnaf .joystick-container::before { border-color: rgba(57, 255, 20, 0.35); }
    .joystick-handle {
      width: 64px; height: 64px; border-radius: 50%;
      background: radial-gradient(circle at 35% 35%, #fff6e0 0%, #d49b43 18%, #8c4327 42%, #2c594d 72%, #0e1613 100%);
      border: 2px solid var(--ochre-rich); box-shadow: 0 6px 18px rgba(0,0,0,0.85), 0 0 16px rgba(212,155,67,0.5);
      position: absolute; cursor: grab; transition: transform 0.05s ease-out;
    }
    body.theme-fnaf .joystick-handle {
      background: radial-gradient(circle at 35% 35%, #66ff66 0%, #208020 40%, #0a260a 80%);
      border: 2px solid #39ff14; box-shadow: 0 4px 14px rgba(0,0,0,0.9), 0 0 18px rgba(57, 255, 20, 0.6);
    }
    .joystick-handle:active { cursor: grabbing; }
    .btn-group { display: flex; gap: 8px; flex-wrap: wrap; }
    button {
      flex: 1; padding: 10px 14px; border-radius: 6px; border: 1px solid var(--brass-border);
      background: linear-gradient(180deg, #2b3933 0%, #19221e 100%); color: var(--text-col);
      font-family: inherit; font-weight: 600; font-size: 0.82rem; letter-spacing: 0.5px;
      cursor: pointer; box-shadow: inset 0 1px 0 rgba(255,255,255,0.1), 0 2px 5px rgba(0,0,0,0.6);
      transition: all 0.15s ease;
    }
    button:hover { background: linear-gradient(180deg, #374b43 0%, #202d27 100%); color: #fff; }
    button.active {
      background: linear-gradient(180deg, var(--ochre-rich) 0%, var(--sienna-burnt) 100%);
      color: #fff9ed; border-color: var(--ochre-bright); box-shadow: 0 0 14px rgba(198, 139, 57, 0.5);
    }
    body.theme-fnaf button {
      background: linear-gradient(180deg, #2b362c 0%, #151d16 100%); color: #4af626;
      border: 2px outset #3d4f3e; box-shadow: 0 3px 0 #0d140e, 0 4px 8px rgba(0,0,0,0.7);
      text-transform: uppercase; font-weight: 700;
    }
    body.theme-fnaf button:active { border-style: inset; transform: translateY(2px); box-shadow: 0 1px 0 #0d140e; }
    body.theme-fnaf button.active {
      background: linear-gradient(180deg, #1d4d23 0%, #0d2811 100%); color: #afffaf;
      border: 2px inset #39ff14; box-shadow: 0 0 12px rgba(57, 255, 20, 0.45);
    }
    .fnaf-only { display: none; }
    body.theme-fnaf .norm-only { display: none; }
    body.theme-fnaf .fnaf-only { display: inline; }
  </style>
</head>
<body>
  <header>
    <div class="logo">
      <span class="norm-only">⚡ ESCAM G02 <span>• Pure Rust Firmware</span></span>
      <span class="fnaf-only">FREDDY FAZBEAR'S PIZZA • SURVEILLANCE SYS</span>
    </div>
    <div class="header-actions">
      <button class="theme-toggle-btn" id="themeToggle" onclick="toggleTheme()">
        <span class="norm-only">THEME: STUDIO</span><span class="fnaf-only">THEME: FAZBEAR</span>
      </button>
      <div class="badge" id="connStatus">WEBRTC ONLINE</div>
    </div>
  </header>
  <div class="fnaf-subbar">
    <div><span class="rec-dot">● REC</span> CAM 01 - SHOW STAGE</div>
    <div id="fnafClock">12:00 AM • NIGHT 1</div>
    <div>POWER LEFT: <span id="powerMeter">99%</span> [||| ]</div>
  </div>
  <main>
    <div class="viewport-card">
      <video id="liveVideo" autoplay playsinline muted style="display:none;"></video>
      <img id="liveImg" src="/api/v1/stream" alt="Live Stream" style="width:100%;height:100%;object-fit:cover;display:block;border-radius:4px;">
      <div class="crt-overlay"></div>
      <div class="hud-overlay">
        <span id="hudFps">25.0 FPS • 720p H.264</span>
        <span class="fnaf-only rec-dot">● REC</span>
        <span id="hudLatency">&lt; 65ms Latency</span>
      </div>
    </div>
    <div class="controls-grid">
      <div class="card">
        <h3>
          <span class="norm-only">PTZ S-Curve Virtual Joystick</span>
          <span class="fnaf-only">SURVEILLANCE RADAR TRACKPAD</span>
        </h3>
        <div class="joystick-container" id="joystickZone">
          <div class="joystick-handle" id="joystickHandle"></div>
        </div>
      </div>
      <div class="card">
        <h3>
          <span class="norm-only">Optical & Sensor Mode</span>
          <span class="fnaf-only">OPTICAL MODE CONTROL</span>
        </h3>
        <div class="btn-group">
          <button id="btnDay" class="active" onclick="setIrCut('Day')">
            <span class="norm-only">Day (IR-Cut ON)</span><span class="fnaf-only">DAYTIME CAM</span>
          </button>
          <button id="btnNight" onclick="setIrCut('Night')">
            <span class="norm-only">Night / Astro (Hα)</span><span class="fnaf-only">NIGHT VISION</span>
          </button>
        </div>
        <h3 style="margin-top: 14px;">
          <span class="norm-only">Quick Presets</span>
          <span class="fnaf-only">FACILITY PRESET ACTIONS</span>
        </h3>
        <div class="btn-group">
          <button onclick="sendPtz('Home')">
            <span class="norm-only">Home</span><span class="fnaf-only">CAM 1A (HOME)</span>
          </button>
          <button onclick="sendPtz('Stop')">
            <span class="norm-only">Halt</span><span class="fnaf-only">LOCK DOWN (HALT)</span>
          </button>
          <button onclick="triggerSnapshot()">
            <span class="norm-only">Snapshot</span><span class="fnaf-only">SNAP EVIDENCE</span>
          </button>
        </div>
      </div>
    </div>
  </main>
  <script src="/jmuxer.min.js"></script>
  <script>
    const liveVideo = document.getElementById('liveVideo');
    const liveImg = document.getElementById('liveImg');
    let jmuxer = null;
    let frameCount = 0;
    let lastFpsTime = performance.now();

    if (window.JMuxer) {
      jmuxer = new JMuxer({
        node: 'liveVideo',
        mode: 'video',
        flv: false,
        fps: 20,
        clearBuffer: true,
        debug: false
      });
      liveVideo.style.display = 'block';
      if (liveImg) {
        liveImg.src = '';
        liveImg.style.display = 'none';
      }
      liveVideo.play().catch(() => {});
    }

    const ws = new WebSocket(`ws://${location.host}/api/v1/ws`);
    ws.binaryType = 'arraybuffer';

    ws.onmessage = (event) => {
      if (event.data instanceof ArrayBuffer) {
        if (jmuxer) {
          jmuxer.feed({ video: new Uint8Array(event.data) });
          frameCount++;
          const now = performance.now();
          if (now - lastFpsTime >= 1000) {
            const fps = (frameCount * 1000 / (now - lastFpsTime)).toFixed(1);
            const hudFps = document.getElementById('hudFps');
            if (hudFps) hudFps.innerText = `${fps} FPS • 720p H.264`;
            frameCount = 0;
            lastFpsTime = now;
          }
        }
      } else {
        try {
          const msg = JSON.parse(event.data);
          console.log("Telemetry:", msg);
        } catch {}
      }
    };

    function toggleTheme() {
      const isFnaf = document.body.classList.toggle('theme-fnaf');
      localStorage.setItem('theme', isFnaf ? 'fnaf' : 'default');
    }
    if (localStorage.getItem('theme') === 'fnaf') {
      document.body.classList.add('theme-fnaf');
    }

    setInterval(() => {
      const d = new Date();
      const h = d.getHours() % 12 || 12;
      const m = String(d.getMinutes()).padStart(2, '0');
      const clock = document.getElementById('fnafClock');
      if (clock) clock.innerText = `${h}:${m} AM • NIGHT 1`;
    }, 1000);

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

pub const LIVE_FRAME_JPEG: &[u8] = include_bytes!("frame.jpg");
pub const JMUXER_JS: &str = include_str!("jmuxer.min.js");
