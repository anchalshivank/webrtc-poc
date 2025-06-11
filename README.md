# 🔄 P2P Hole Punching via WebTorrent (WebRTC Signaling with BitTorrent Tracker)

This is a **proof-of-concept** for establishing **direct peer-to-peer (P2P)** communication between two devices using **WebRTC**, with signaling handled via a **public BitTorrent WebSocket tracker**.

No custom signaling server required. No TURN server. No port forwarding. Just **pure P2P**!

## 🚀 How it Works

- Each peer serves the frontend app **locally** via a static HTTP server.
- Both peers connect to the **same signaling room** using a **public WebTorrent tracker**.
- After exchanging WebRTC offer/answer messages through the tracker, the peers connect directly.



## 🧪 Steps to Reproduce

### 1. Serve App Locally on Each Peer

On **each device** (independently):

```bash
cd webrtc-poc
python3 -m http.server 8081
````

Then open your browser to:

```
http://localhost:8081
```

⚠️ You are **not** accessing each other's servers. Both users run this locally.



### 2. WebRTC Signaling via Public Tracker

* Use this tracker URL (or choose another from the list below):

  ```html
  <option value="wss://tracker.files.fm:7073">wss://tracker.files.fm:7073</option>
  ```

* Both peers must enter the **same Room ID**

* One peer clicks **“Connect”** → the other clicks **“Join Room”**

* Initiate **WebRTC Offer/Answer exchange**

If NAT hole punching is successful, you'll see a **direct connection established** between the peers.



## ✅ Public WebTorrent Trackers

* `wss://tracker.files.fm:7073`
* `wss://tracker.openwebtorrent.com`
* `wss://tracker.webtorrent.dev` ([usage stats](https://tracker.webtorrent.dev))


## 🌐 Public STUN Servers (Optional)

In case you use WebRTC ICE servers:

* `stun:stun.cloudflare.com`
* `stun:stun.l.google.com:19302`



## 🛠️ Inspired by

* [https://github.com/webtorrent/bittorrent-tracker](https://github.com/webtorrent/bittorrent-tracker)
* [https://github.com/subins2000/p2pt](https://github.com/subins2000/p2pt)
* Hacker News thread on [Peerfetch - P2P HTTP over WebRTC](https://news.ycombinator.com/item?id=41136372)

## 🤔 Why This Matters

This POC shows that **WebRTC + public signaling infrastructure** can enable **truly decentralized P2P communication** — ideal for:

* Offline-first web apps
* Decentralized chat
* File sharing
* Games
* Experiments in Web3, Mesh networks, and more

This can easily be extended to languages like **Rust**, **Go**, or **Node.js** for CLI or backend use cases.


## 🔗 License

MIT

