use warp::Filter;

#[tokio::main]
async fn main() {
    // Define route: GET /
    let html = warp::path::end().map(|| {
        warp::reply::html(
            r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>Basic WebSocket Client</title>
  <meta name="viewport" content="width=device-width,initial-scale=1" />
  <style>
    body { font-family: system-ui, -apple-system, "Segoe UI", Roboto, Arial; margin: 18px; background:#f7f8fb; color:#111; }
    .card { background: white; border-radius: 10px; padding: 14px; box-shadow: 0 6px 18px rgba(20,20,40,0.06); max-width:900px; margin:auto; }
    .row { display:flex; gap:8px; align-items:center; margin-bottom:10px; }
    input[type=text], select, textarea { padding:8px; border-radius:6px; border:1px solid #d6d9e6; font-size:14px; }
    button { padding:8px 12px; border-radius:8px; border:0; cursor:pointer; background:#2563eb; color:#fff; font-weight:600; }
    button.secondary { background:#e6e9f7; color:#111; }
    #log { height: 360px; overflow:auto; border:1px solid #eef0f6; padding:8px; border-radius:8px; background:#fbfcff; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, "Roboto Mono", monospace; font-size:13px; }
    .msg { padding:6px 8px; border-radius:6px; margin-bottom:6px; }
    .incoming { background:#eef9ff; border-left:4px solid #60a5fa; }
    .outgoing { background:#f1fdf7; border-left:4px solid #34d399; }
    .system { background:#fff7ed; border-left:4px solid #f59e0b; }
    .meta { font-size:12px; color:#6b7280; margin-bottom:6px; }
    .grow { flex:1; }
    label { font-size:13px; color:#374151; }
  </style>
</head>
<body>
  <div class="card">
    <h2>WebSocket Client</h2>

    <div class="row">
      <label for="url">Server URL</label>
      <input id="url" type="text" value="wss://echo.websocket.events" class="grow" />
      <button id="connectBtn">Connect</button>
      <button id="disconnectBtn" class="secondary" disabled>Disconnect</button>
    </div>

    <div class="row">
      <label for="protocol">Protocol (optional)</label>
      <input id="protocol" type="text" placeholder="e.g. my-protocol" />
      <label style="margin-left:8px;">Message type</label>
      <select id="msgType">
        <option value="text">Text</option>
        <option value="json">JSON</option>
      </select>
    </div>

    <div class="row">
      <textarea id="message" rows="3" class="grow" placeholder="Type a message (or JSON) and press Send"></textarea>
      <button id="sendBtn" disabled>Send</button>
    </div>

    <div class="row">
      <label>Status:</label>
      <div id="status" class="meta">Disconnected</div>
    </div>

    <div id="log" aria-live="polite"></div>
  </div>

  <script>
    // Elements
    const urlInput = document.getElementById('url');
    const protocolInput = document.getElementById('protocol');
    const connectBtn = document.getElementById('connectBtn');
    const disconnectBtn = document.getElementById('disconnectBtn');
    const sendBtn = document.getElementById('sendBtn');
    const msgInput = document.getElementById('message');
    const statusEl = document.getElementById('status');
    const logEl = document.getElementById('log');
    const msgType = document.getElementById('msgType');

    let ws = null;

    function addLog(text, kind = 'system') {
      const el = document.createElement('div');
      el.className = 'msg ' + (kind === 'incoming' ? 'incoming' : kind === 'outgoing' ? 'outgoing' : 'system');
      const time = new Date().toLocaleTimeString();
      el.innerHTML = `<div class="meta">${time} — ${kind.toUpperCase()}</div><div>${escapeHtml(text)}</div>`;
      logEl.appendChild(el);
      logEl.scrollTop = logEl.scrollHeight;
    }

    function escapeHtml(s) {
      if (typeof s !== 'string') s = String(s);
      return s.replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;').replaceAll('"','&quot;');
    }

    function setStatus(text, color) {
      statusEl.textContent = text;
      statusEl.style.color = color || '';
    }

    connectBtn.addEventListener('click', () => {
      if (ws) {
        addLog('Already connected (or connecting).', 'system');
        return;
      }
      const url = urlInput.value.trim();
      if (!url) { addLog('Please enter a WebSocket URL (ws:// or wss://).', 'system'); return; }
      let protocols = protocolInput.value.trim();
      if (!protocols) protocols = undefined;
      try {
        // Create WebSocket
        ws = protocols ? new WebSocket(url, protocols) : new WebSocket(url);

        setStatus('Connecting...', '#d97706');
        addLog(`Connecting to ${url}${protocols ? ' (protocol: ' + protocols + ')' : ''}`, 'system');

        // Events
        ws.addEventListener('open', (ev) => {
          setStatus('Connected', '#10b981');
          addLog('Connection opened', 'system');
          sendBtn.disabled = false;
          disconnectBtn.disabled = false;
          connectBtn.disabled = true;
        });

        ws.addEventListener('message', (event) => {
          // Distinguish text vs binary
          if (typeof event.data === 'string') {
            addLog(event.data, 'incoming');
          } else {
            // ArrayBuffer / Blob — convert to hex preview if ArrayBuffer
            if (event.data instanceof Blob) {
              const reader = new FileReader();
              reader.onload = () => {
                addLog('[binary blob] ' + bufferPreview(reader.result), 'incoming');
              };
              reader.readAsArrayBuffer(event.data);
            } else {
              addLog('[binary] ' + bufferPreview(event.data), 'incoming');
            }
          }
        });

        ws.addEventListener('close', (ev) => {
          addLog(`Closed (code: ${ev.code}, reason: ${ev.reason || 'none'})`, 'system');
          setStatus('Disconnected', '#ef4444');
          ws = null;
          sendBtn.disabled = true;
          disconnectBtn.disabled = true;
          connectBtn.disabled = false;
        });

        ws.addEventListener('error', (err) => {
          addLog('WebSocket error (see console)', 'system');
          console.error('WebSocket error', err);
          setStatus('Error', '#b91c1c');
        });

      } catch (err) {
        addLog('Failed to create WebSocket: ' + err.message, 'system');
      }
    });

    disconnectBtn.addEventListener('click', () => {
      if (ws) {
        ws.close(1000, 'Client disconnect'); // normal close
      } else {
        addLog('Not connected', 'system');
      }
    });

    sendBtn.addEventListener('click', () => {
      if (!ws || ws.readyState !== WebSocket.OPEN) { addLog('Socket is not open', 'system'); return; }
      let value = msgInput.value;
      if (msgType.value === 'json') {
        try {
          // Attempt to parse then serialize to ensure valid JSON
          const parsed = JSON.parse(value);
          value = JSON.stringify(parsed);
        } catch (e) {
          addLog('Invalid JSON — fix it before sending.', 'system');
          return;
        }
      }
      try {
        ws.send(value);
        addLog(value, 'outgoing');
      } catch (e) {
        addLog('Send failed: ' + e.message, 'system');
      }
    });

    // Send when pressing Ctrl/Cmd+Enter inside textarea
    msgInput.addEventListener('keydown', (ev) => {
      if ((ev.ctrlKey || ev.metaKey) && ev.key === 'Enter') {
        sendBtn.click();
      }
    });

    // Helper: preview ArrayBuffer as short hex
    function bufferPreview(buf) {
      const view = new Uint8Array(buf);
      const len = Math.min(16, view.length);
      const hex = Array.from(view.slice(0, len)).map(b => ('0' + b.toString(16)).slice(-2)).join(' ');
      return `${hex}${view.length > len ? ' ...' : ''} (len=${view.length})`;
    }

    // Quick connection example on load (optional)
    // (comment out if you don't want auto-connect)
    /*
    window.addEventListener('load', () => {
      connectBtn.click();
    });
    */
  </script>
</body>
</html>"#,
        )
    });

    // Start server on localhost:3030
    println!("Server running at http://127.0.0.1:8080/");
    warp::serve(html).run(([127, 0, 0, 1], 8080)).await;
}
