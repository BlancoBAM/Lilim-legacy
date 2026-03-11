# 📱 Connecting Your iPhone to Lilim

Talk to Lilim from your iPhone — ask questions, schedule tasks, and manage your Lilith Linux system remotely.

## How It Works

Your iPhone sends messages to Lilim through ZeroClaw's secure Gateway API. Messages are:
1. **Encrypted in transit** (HTTPS via tunnel)
2. **Authenticated** with a pairing code (like Bluetooth pairing)
3. **Processed locally** on your Lilith Linux machine (nothing leaves your system except the tunnel)

## Setup Steps

### 1. Enable Remote Access (one-time)

On your Lilith Linux machine, edit `/etc/lilith/zeroclaw.toml`:

```toml
[tunnel]
provider = "cloudflare"  # or "tailscale" if you use Tailscale
```

Then restart Lilim:
```bash
sudo systemctl restart lilith-ai
```

The tunnel URL will be printed in the logs:
```bash
journalctl -u lilith-ai -f | grep "tunnel"
# Example output: https://lilim-abc123.trycloudflare.com
```

### 2. Get Your Pairing Code

Open Lilim on your desktop (Ctrl+Shift+L) and click the **📱 Pair Device** button in the settings. A 6-digit code will appear.

### 3. Create iPhone Shortcut

1. Open the **Shortcuts** app on your iPhone
2. Create a new shortcut
3. Add these actions:

   **a. Ask for Input** → Text → "What do you want to ask Lilim?"

   **b. Get Contents of URL**:
   - URL: `https://YOUR-TUNNEL-URL/pair`
   - Method: POST
   - Headers: `X-Pairing-Code: YOUR-6-DIGIT-CODE`
   - *(Only needed the first time — after pairing, the token is saved)*

   **c. Get Contents of URL**:
   - URL: `https://YOUR-TUNNEL-URL/webhook`
   - Method: POST
   - Headers: `Authorization: Bearer YOUR-SAVED-TOKEN`
   - Body: JSON → `{"message": "Shortcut Input"}`

   **d. Show Result**

4. Add the shortcut to your Home Screen for quick access

### 4. Alternative: Pinned Web App

If you prefer a chat-like interface on your phone:

1. Open Safari on your iPhone
2. Navigate to `https://YOUR-TUNNEL-URL`
3. Tap Share → **Add to Home Screen**
4. The web app provides a mobile-friendly chat interface

## Security Notes

- **Pairing tokens expire** — you can revoke access anytime from Lilim's settings
- **Gateway binds to localhost** — only accessible via tunnel, never directly exposed
- **All requests are logged** — check `journalctl -u lilith-ai` for activity
- **IP allowlisting** — optionally restrict to known IPs in `zeroclaw.toml`

## Troubleshooting

| Issue | Fix |
|-------|-----|
| "Connection refused" | Is `lilith-ai` service running? `systemctl status lilith-ai` |
| "Unauthorized" | Re-pair your device — token may have expired |
| Slow responses | Check system load: `htop`. LLM inference takes time on CPU |
| Tunnel not working | Ensure cloudflared/tailscale is installed |
