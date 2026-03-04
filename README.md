# Jackett Search Engine for qBittorrent (Rust Core) 🦀🚀

A high-performance search plugin for qBittorrent that uses a compiled **Rust** binary to handle API requests and XML parsing. This replaces the traditional interpreted Python logic with a native, async middleware for maximum efficiency.

## 🌟 Features
- **Blazing Fast:** Native execution speeds for processing large Torznab XML result sets.
- **Async Concurrency:** Built on the `Tokio` runtime for non-blocking I/O.
- **Resilient Networking:** Implemented 45-second timeouts and custom User-Agent to handle slow trackers.
- **Linux & Windows Support:** Optimized for Void Linux, CachyOS, and Windows 10/11.

---

## 🛠️ Installation (Linux)

### 1. Build the Binary
```bash
cargo build --release
```

### 2. Deploy to qBittorrent
```bash
mkdir -p ~/.local/share/qBittorrent/nova3/engines/
cp target/release/jackett-search ~/.local/share/qBittorrent/nova3/engines/
cp jackett_rust.py ~/.local/share/qBittorrent/nova3/engines/
chmod +x ~/.local/share/qBittorrent/nova3/engines/jackett-search
```

### 3. Configure
Edit `~/.local/share/qBittorrent/nova3/engines/jackett_rust.py` and replace `YOUR_API_KEY_HERE` with your actual Jackett API key.

---

## 🪟 Installation (Windows)

### 1. Build the Binary
```powershell
cargo build --release
```

### 2. Deploy to qBittorrent
1. Open `%localappdata%\qBittorrent\nova3\engines\` in File Explorer.
2. Copy `target/release/jackett-search.exe` and `jackett_rust.py` into that folder.
3. In `jackett_rust.py`, ensure the binary path is:
   `BINARY_PATH = os.path.join(os.path.dirname(__file__), "jackett-search.exe")`

---

## 📝 License
MIT
