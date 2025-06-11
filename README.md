### 📘 WebRTC Hole Punching via BitTorrent Tracker (WebTorrent)

This is a simple P2P proof-of-concept to demonstrate **NAT hole punching** using a **BitTorrent WebSocket tracker** and **WebTorrent** APIs.

Each device hosts the frontend locally and uses a public tracker for signaling and peer discovery.

---

#### 🧪 Steps to Run

1. **Start a Local Server on Each Device**

   On **both computers**:

   ```bash
   cd webrtc-poc
   python3 -m http.server 8081
   ```

   Then open:
   `http://localhost:8081` in your browser.

   ⚠️ No need to access the other’s IP or LAN. Each person runs this locally.

2. **Connect via WebTorrent P2P**

   * Use this tracker (public WebSocket):

     ```
     wss://tracker.files.fm:7073
     ```
   * Both peers must use the **same Room ID**.
   * Steps in the app:

     1. **First peer** clicks **"Connect"**
     2. **Second peer** clicks **"Join Room"**
     3. One of them **creates an offer**, the other **responds**
   * If successful, a **direct P2P WebRTC connection** is established.
     This means hole punching worked and you can now send messages/data without any central server.

