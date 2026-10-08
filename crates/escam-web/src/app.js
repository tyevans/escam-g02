// ESCAM G02 Astrophotography & Camera Control Dashboard
const liveVideo = document.getElementById('liveVideo');
const liveImg = document.getElementById('liveImg');
let jmuxer = null, frameCount = 0, lastFpsTime = performance.now();
let activeExposure = 0.04, activeGain = 1.0, activeFps = 25.0, activeRes = 'Hd720p', activeStack = 'Off';
let exposureProgressTimer = null, exposureStartTime = 0;

function fetchLastPhoto() {
  if (liveVideo && liveVideo.videoWidth > 0 && liveImg) {
    try {
      const c = document.createElement('canvas');
      c.width = liveVideo.videoWidth; c.height = liveVideo.videoHeight;
      c.getContext('2d').drawImage(liveVideo, 0, 0);
      liveImg.src = c.toDataURL('image/jpeg', 0.92);
      return;
    } catch (_) {}
  }
  if (liveImg) liveImg.src = `/api/v1/snapshot?t=${Date.now()}`;
}

function setViewportMode(isAstro) {
  if (!liveVideo || !liveImg) return;
  if (isAstro) {
    liveVideo.style.position = 'absolute'; liveVideo.style.opacity = '0';
    liveVideo.style.pointerEvents = 'none'; liveImg.style.display = 'block';
    fetchLastPhoto();
  } else {
    liveVideo.style.position = 'static'; liveVideo.style.opacity = '1';
    liveVideo.style.pointerEvents = 'auto'; liveVideo.style.display = 'block';
    liveImg.style.display = 'none';
  }
}

if (window.JMuxer) {
  jmuxer = new JMuxer({ node: 'liveVideo', mode: 'video', flv: false, fps: 30, flushingTime: 0, maxDelay: 50, clearBuffer: true, debug: false });
}

let ws = null;
function connectWebSocket() {
  const proto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  ws = new WebSocket(`${proto}//${window.location.host}/api/v1/ws`);
  ws.binaryType = 'arraybuffer';
  ws.onopen = () => {
    const s = document.getElementById('hudStatus');
    if (s) s.innerText = 'WS LIVE (VIDEO/PTZ)';
  };
  ws.onmessage = (event) => {
    if (typeof event.data === 'string') return;
    if (jmuxer && liveVideo) {
      jmuxer.feed({ video: new Uint8Array(event.data) });
      frameCount++;
      const now = performance.now();
      if (now - lastFpsTime >= 1000) {
        const curFps = (frameCount * 1000) / (now - lastFpsTime);
        const hudFps = document.getElementById('hudFps');
        if (hudFps && activeExposure < 1.0) hudFps.innerText = `${curFps.toFixed(1)} FPS`;
        frameCount = 0; lastFpsTime = now;
      }
    }
  };
  ws.onclose = () => {
    const s = document.getElementById('hudStatus');
    if (s) s.innerText = 'WS RECONNECTING...';
    setTimeout(connectWebSocket, 2000);
  };
  ws.onerror = () => ws.close();
}
connectWebSocket();

let fnafMode = false;
function toggleTheme() {
  fnafMode = !fnafMode;
  document.body.classList.toggle('theme-fnaf', fnafMode);
}
window.toggleTheme = toggleTheme;

function toggleFnafTheme() {
  toggleTheme();
}
window.toggleFnafTheme = toggleFnafTheme;

setInterval(() => {
  const d = new Date(), h = d.getHours() % 12 || 12, m = String(d.getMinutes()).padStart(2, '0');
  const clock = document.getElementById('fnafClock');
  if (clock) clock.innerText = `${h}:${m} AM • NIGHT 1`;
}, 1000);

const expLadder = [0.001, 0.01, 0.04, 1.0, 2.0, 5.0, 10.0, 30.0, 60.0];
function onExpSliderChange(valIndex) { setExposure(expLadder[parseInt(valIndex, 10)] || 0.04); }
function onCustomExpChange(val) { const num = parseFloat(val); if (!isNaN(num) && num > 0) setExposure(num); }
function setExposure(secs) {
  activeExposure = secs;
  let targetFps = secs > 0.04 ? (1.0 / secs) : 25.0;
  activeFps = targetFps;
  updateExpDisplays(secs, targetFps);
  fetch('/api/v1/camera', {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ exposure_secs: secs, target_fps: targetFps })
  }).then(r => r.json()).then(res => { if (res.controls) syncControlsToUI(res.controls); }).catch(() => {});
  triggerExposureAnimation(secs);
}

function updateExpDisplays(secs, fps) {
  const dispExp = document.getElementById('dispExp'), numExp = document.getElementById('numExp');
  const dispFps = document.getElementById('dispFps'), hudExp = document.getElementById('hudExp'), hudFps = document.getElementById('hudFps');
  const isAstro = secs >= 1.0;
  const txt = isAstro ? `${secs.toFixed(1)}s` : `1/${Math.round(1/secs)}s (${secs.toFixed(3)}s)`;
  if (dispExp) dispExp.innerText = txt;
  if (numExp) numExp.value = secs.toFixed(isAstro ? 1 : 3);
  if (dispFps) dispFps.innerText = isAstro ? `Astro (${secs.toFixed(1)}s)` : `${fps.toFixed(1)} FPS`;
  if (hudExp) hudExp.innerText = `Exp: ${isAstro ? secs.toFixed(1) + 's' : secs.toFixed(2) + 's'}`;
  if (hudFps) hudFps.innerText = isAstro ? 'LAST PHOTO' : `${fps.toFixed(1)} FPS`;
  updateExposureFilter();
}

let activeStretch = 'auto';
function updateExposureFilter() {
  const video = document.getElementById('liveVideo'), img = document.getElementById('liveImg');
  let b = 1.0, c = 1.0, s = 1.0;
  if (activeStretch === 'auto') { b = 1.35; c = 1.3; }
  else if (activeStretch === 'aggressive') { b = 1.8; c = 1.55; }
  else if (activeStretch === 'asinh') { b = 1.25; c = 1.2; s = 1.35; }
  const filterVal = activeStretch === 'none' ? 'none' : `brightness(${b.toFixed(2)}) contrast(${c.toFixed(2)}) saturate(${s.toFixed(2)})`;
  if (video) video.style.filter = filterVal;
  if (img) img.style.filter = filterVal;
}

let sampleCanvas = null, sampleCtx = null, accumBuffer = null, accumCount = 0, accumTimer = null;
function startAccumulation(secs) {
  if (accumTimer) clearInterval(accumTimer);
  accumCount = 0; if (secs < 1.0) return;
  const w = liveVideo.videoWidth || 640, h = liveVideo.videoHeight || 360;
  if (!sampleCanvas) { sampleCanvas = document.createElement('canvas'); sampleCtx = sampleCanvas.getContext('2d', { willReadFrequently: true }); }
  sampleCanvas.width = w; sampleCanvas.height = h;
  accumBuffer = new Float32Array(w * h * 4);
  accumTimer = setInterval(() => {
    if (!liveVideo || liveVideo.paused || liveVideo.videoWidth === 0) return;
    try {
      sampleCtx.drawImage(liveVideo, 0, 0, w, h);
      const d = sampleCtx.getImageData(0, 0, w, h).data;
      for (let i = 0; i < d.length; i += 4) { accumBuffer[i] += d[i]; accumBuffer[i+1] += d[i+1]; accumBuffer[i+2] += d[i+2]; }
      accumCount++;
    } catch (_) {}
  }, 50);
}

function finalizeAccumulatedPhoto(secs) {
  if (accumTimer) clearInterval(accumTimer);
  if (!accumBuffer || accumCount === 0) { fetchLastPhoto(); return; }
  const w = sampleCanvas.width, h = sampleCanvas.height;
  const outImgData = sampleCtx.createImageData(w, h), out = outImgData.data;
  const scale = (secs / (accumCount * 0.05)) * (activeGain > 1.0 ? Math.sqrt(activeGain) : 1.0);
  for (let i = 0; i < out.length; i += 4) {
    out[i] = Math.min(255, (accumBuffer[i] / accumCount) * scale);
    out[i+1] = Math.min(255, (accumBuffer[i+1] / accumCount) * scale);
    out[i+2] = Math.min(255, (accumBuffer[i+2] / accumCount) * scale);
    out[i+3] = 255;
  }
  sampleCtx.putImageData(outImgData, 0, 0);
  if (liveImg) liveImg.src = sampleCanvas.toDataURL('image/jpeg', 0.92);
}

let lastExposureCycle = 0;
function triggerExposureAnimation(secs) {
  const container = document.getElementById('exposureProgressContainer');
  const bar = document.getElementById('exposureProgressBar');
  if (exposureProgressTimer) clearInterval(exposureProgressTimer);
  if (secs < 1.0) {
    if (container) container.style.display = 'none';
    setViewportMode(false);
    return;
  }
  setViewportMode(true);
  if (container) container.style.display = 'block';
  exposureStartTime = performance.now();
  lastExposureCycle = 0;
  startAccumulation(secs);
  const totalMs = secs * 1000;
  exposureProgressTimer = setInterval(() => {
    const elapsed = performance.now() - exposureStartTime;
    const cycle = Math.floor(elapsed / totalMs);
    if (cycle > lastExposureCycle) {
      lastExposureCycle = cycle;
      finalizeAccumulatedPhoto(secs);
      startAccumulation(secs);
    }
    const pct = Math.min(100, ((elapsed % totalMs) / totalMs) * 100);
    if (bar) bar.style.width = `${pct}%`;
  }, 100);
}

const gainLadder = [1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0];
function onGainSliderChange(valIndex) { setGain(gainLadder[parseInt(valIndex, 10)] || 1.0); }
function setGain(val) {
  activeGain = val;
  const disp = document.getElementById('dispGain');
  if (disp) disp.innerText = `${val.toFixed(1)}x (${Math.round(20 * Math.log10(val))} dB • ISO ${Math.round(val * 100)})`;
  updateExposureFilter();
  fetch('/api/v1/camera', {
    method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ gain: val })
  }).then(r => r.json()).then(res => { if (res.controls) syncControlsToUI(res.controls); }).catch(() => {});
}

function onFpsSelect(val) {
  const fps = val === 'auto' ? (activeExposure > 0.04 ? (1.0 / activeExposure) : 25.0) : parseFloat(val);
  setExposure(1.0 / fps);
}
function onResSelect(val) {
  activeRes = val;
  const disp = document.getElementById('dispRes');
  if (disp) disp.innerText = val === 'Astro1280x960' ? '1280x960 (Astro)' : val === 'Sd360p' ? '640x360 (Binning)' : '1280x720 (HD)';
  fetch('/api/v1/camera', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ resolution: val }) }).catch(() => {});
}

const stretchLabels = { none: 'Linear Raw', auto: 'Auto MTF (PixInsight STF)', asinh: 'Arcsinh (Color Stars)', aggressive: 'Aggressive (Faint DSOs)' };
function applyAstroPreset(p) {
  if (p === 'planetary') { setIrCut('Day'); setGain(2.0); setExposure(0.01); onResSelect('Hd720p'); }
  else if (p === 'deepsky') { setIrCut('Night'); setGain(16.0); setExposure(10.0); onResSelect('Astro1280x960'); }
  else if (p === 'widefield') { setIrCut('Night'); setGain(8.0); setExposure(5.0); onResSelect('Hd720p'); }
  else if (p === 'daylight') { setIrCut('Day'); setGain(1.0); setExposure(0.04); onResSelect('Hd720p'); }
}

function setStackMode(mode) {
  activeStack = mode;
  document.getElementById('btnStackOff').classList.toggle('active', mode === 'Off');
  document.getElementById('btnStackAvg').classList.toggle('active', mode === 'Average');
  document.getElementById('btnStackAdd').classList.toggle('active', mode === 'Additive');
  fetch('/api/v1/camera', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ stack_mode: mode }) }).catch(() => {});
}
function resetStack() {
  fetch('/api/v1/camera/stack/reset', { method: 'POST' }).then(() => {
    const txt = document.getElementById('stackStatusText');
    if (txt) txt.innerText = 'Stacked: 0 frames';
  }).catch(() => {});
}
function setIrCut(mode) {
  fetch('/api/v1/ircut', { method: 'POST', body: JSON.stringify({ mode }), headers: { 'Content-Type': 'application/json' } });
  const isNight = mode === 'Night';
  document.getElementById('btnDay').classList.toggle('active', !isNight);
  document.getElementById('btnNight').classList.toggle('active', isNight);
  document.body.classList.toggle('night-vision', isNight);
}
let irLedState = false;
function toggleIrLed() {
  irLedState = !irLedState;
  fetch('/api/v1/irled', { method: 'POST', body: JSON.stringify({ enabled: irLedState }), headers: { 'Content-Type': 'application/json' } });
  const btn = document.getElementById('btnIrLed');
  if (btn) btn.classList.toggle('active', irLedState);
}

function sendPtz(act) { fetch('/api/v1/ptz', { method: 'POST', body: JSON.stringify({ action: act }), headers: { 'Content-Type': 'application/json' } }); }
function triggerSnapshot() { fetchLastPhoto(); window.open('/api/v1/snapshot', '_blank'); }
function downloadFits() { window.open(`/api/v1/astro/capture.fits?exposure=${activeExposure}&gain=${activeGain}&stretch=${activeStretch}`, '_blank'); }
function onStretchSelect(val) {
  activeStretch = val;
  const disp = document.getElementById('dispStretch');
  if (disp) disp.innerText = stretchLabels[val] || val;
  updateExposureFilter();
  fetch('/api/v1/camera', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ auto_stretch: val }) }).catch(() => {});
}

// Celestial Slew, Autoguide, Calibration, and Clip Recorder
function slewToTarget(val) {
  if (!val) return;
  fetch('/api/v1/astro/slew', {
    method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ target: val })
  }).then(r => r.json()).then(res => alert(res.success ? `Slewing to ${res.target}...` : `Error: ${res.error}`)).catch(() => {});
}

let isGuiding = false;
function toggleAutoguide() {
  isGuiding = !isGuiding;
  fetch(isGuiding ? '/api/v1/astro/guide/start' : '/api/v1/astro/guide/stop', { method: 'POST' }).then(() => {
    const btn = document.getElementById('btnGuide');
    if (btn) btn.innerText = isGuiding ? 'Guiding: ACTIVE' : 'Autoguide';
  }).catch(() => {});
}

function captureMasterDark() {
  fetch('/api/v1/astro/calibration/dark', { method: 'POST' }).then(r => r.json()).then(() => alert('Master Dark Calibrated!')).catch(() => {});
}
function triggerClipRecord() {
  fetch('/api/v1/recorder/trigger', { method: 'POST' }).then(r => r.json()).then(() => alert('Pre-roll Event Clip recorded!')).catch(() => {});
}
function calibrateHome() {
  fetch('/api/v1/ptz/home', { method: 'POST' }).then(() => alert('Mount Soft-Homed to Center!')).catch(() => {});
}

setInterval(() => {
  fetch('/api/v1/astro/focus').then(r => r.json()).then(f => {
    const el = document.getElementById('hudFocus');
    if (el) el.innerText = `Stars: ${f.star_count} | FWHM: ${f.median_fwhm.toFixed(1)}px`;
  }).catch(() => {});
}, 3000);

function syncControlsToUI(ctrl) {
  if (ctrl.exposure_secs) {
    activeExposure = ctrl.exposure_secs;
    updateExpDisplays(ctrl.exposure_secs, ctrl.target_fps || (1.0 / ctrl.exposure_secs));
    if (ctrl.exposure_secs >= 1.0) triggerExposureAnimation(ctrl.exposure_secs);
  }
  if (ctrl.gain) {
    activeGain = ctrl.gain;
    const disp = document.getElementById('dispGain');
    if (disp) disp.innerText = `${ctrl.gain.toFixed(1)}x (${Math.round(20 * Math.log10(ctrl.gain))} dB • ISO ${Math.round(ctrl.gain * 100)})`;
  }
}

fetch('/api/v1/status').then(r => r.json()).then(st => {
  if (st.ircut_mode) setIrCut(st.ircut_mode);
  if (st.controls) syncControlsToUI(st.controls);
}).catch(() => {});

// Virtual Joystick
const zone = document.getElementById('joystickZone'), handle = document.getElementById('joystickHandle');
let dragging = false, lastPtzSend = 0, maxRadius = 55;
function sendJoystickCoords(x, y) {
  const isStop = (x === 0 && y === 0);
  const now = Date.now();
  if (!isStop && now - lastPtzSend < 40) return;
  lastPtzSend = now;
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify({ type: 'ptz_joystick', x, y }));
    if (isStop) ws.send(JSON.stringify({ type: 'ptz_stop' }));
  } else {
    const act = isStop ? 'Stop' : 'Joystick';
    fetch('/api/v1/ptz', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ action: act, x, y }) }).catch(() => {});
  }
}
function handleMove(clientX, clientY) {
  const rect = zone.getBoundingClientRect();
  const dx = Math.max(-maxRadius, Math.min(maxRadius, clientX - (rect.left + rect.width / 2)));
  const dy = Math.max(-maxRadius, Math.min(maxRadius, clientY - (rect.top + rect.height / 2)));
  handle.style.transform = `translate(${dx}px, ${dy}px)`;
  sendJoystickCoords(dx / maxRadius, -dy / maxRadius);
}
zone.addEventListener('pointerdown', (e) => {
  dragging = true;
  if (zone.setPointerCapture) { try { zone.setPointerCapture(e.pointerId); } catch (_) {} }
  handleMove(e.clientX, e.clientY);
});
window.addEventListener('pointermove', (e) => { if (dragging) handleMove(e.clientX, e.clientY); });
function handleEnd() {
  if (dragging) {
    dragging = false;
    handle.style.transform = 'translate(0px, 0px)';
    sendJoystickCoords(0, 0);
    fetch('/api/v1/ptz', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ action: 'Stop' }) }).catch(() => {});
  }
}
window.addEventListener('pointerup', handleEnd);
window.addEventListener('pointercancel', handleEnd);

// Keyboard arrow and WASD steering
window.addEventListener('keydown', (e) => {
  if (['INPUT', 'SELECT', 'TEXTAREA'].includes(e.target.tagName)) return;
  let kx = 0, ky = 0;
  if (e.key === 'ArrowLeft' || e.key === 'a' || e.key === 'A') kx = -0.8;
  else if (e.key === 'ArrowRight' || e.key === 'd' || e.key === 'D') kx = 0.8;
  else if (e.key === 'ArrowUp' || e.key === 'w' || e.key === 'W') ky = 0.8;
  else if (e.key === 'ArrowDown' || e.key === 's' || e.key === 'S') ky = -0.8;
  if (kx !== 0 || ky !== 0) {
    e.preventDefault();
    handle.style.transform = `translate(${kx * maxRadius}px, ${-ky * maxRadius}px)`;
    sendJoystickCoords(kx, ky);
  }
});
window.addEventListener('keyup', (e) => {
  if (['INPUT', 'SELECT', 'TEXTAREA'].includes(e.target.tagName)) return;
  if (['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'a', 'd', 'w', 's', 'A', 'D', 'W', 'S'].includes(e.key)) {
    e.preventDefault();
    handle.style.transform = 'translate(0px, 0px)';
    sendJoystickCoords(0, 0);
    fetch('/api/v1/ptz', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ action: 'Stop' }) }).catch(() => {});
  }
});
