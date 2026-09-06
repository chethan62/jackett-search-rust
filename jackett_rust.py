# VERSION: 1.2
# AUTHORS: Chethan (Rust Engine)

import subprocess
import os
import sys

class jackett_rust:
    name = 'Jackett (Rust)'
    url = 'http://127.0.0.1:9117'
    BINARY_PATH = os.path.join(os.path.dirname(__file__), "jackett-search")

    # Jackett API key is read from the JACKETT_API_KEY environment variable.
    # Do NOT hardcode it here — the key is a secret. Set it e.g. in your shell
    # profile or the qBittorrent service environment:
    #   export JACKETT_API_KEY="your-jackett-api-key"
    API_KEY = os.environ.get("JACKETT_API_KEY", "").strip()

    # qBittorrent categories supported by this plugin
    supported_categories = {'all': 'all', 'movies': 'movies', 'tv': 'tv', 'music': 'music', 'games': 'games', 'software': 'software', 'books': 'books'}

    def search(self, what, cat='all'):
        if not self.API_KEY:
            print("Error: JACKETT_API_KEY environment variable is not set. "
                  "Set it to your Jackett API key (see README).", file=sys.stderr)
            return
        try:
            # Passing category as the 3rd argument to Rust
            process = subprocess.run(
                [self.BINARY_PATH, self.API_KEY, what, cat],
                capture_output=True,
                text=True
            )
            print(process.stdout)
        except Exception as e:
            print(f"Error: {e}", file=sys.stderr)

    def download_torrent(self, info):
        print(info + " " + info)
