import subprocess
import os

class jackett_rust:
    name = 'Jackett (Rust Engine)'
    url = 'http://127.0.0.1:9117'

    # Path to your compiled Rust binary
    # On Linux, this is usually ~/.local/share/qBittorrent/nova3/engines/jackett-search
    BINARY_PATH = os.path.join(os.path.dirname(__file__), "jackett-search")
    API_KEY = "YOUR_JACKETT_API_KEY_HERE"

    def search(self, what, cat='all'):
        try:
            # Call the Rust binary and capture stdout
            process = subprocess.run(
                [self.BINARY_PATH, self.API_KEY, what],
                capture_output=True,
                text=True
            )
            # Print the Rust output directly to qBittorrent's stdout
            print(process.stdout)
        except Exception as e:
            # Redirect errors to stderr so they don't break the UI
            import sys
            print(f"Rust engine error: {e}", file=sys.stderr)

    def download_torrent(self, info):
        print(info + " " + info)
