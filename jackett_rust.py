# VERSION: 1.1
# AUTHORS: Chethan (Rust Engine)

import subprocess
import os
import sys

class jackett_rust:
    name = 'Jackett (Rust)'
    url = 'http://127.0.0.1:9117'
    BINARY_PATH = os.path.join(os.path.dirname(__file__), "jackett-search")
    API_KEY = "5jpajouo5494gl16hifjljkq68de8ku4"

    # qBittorrent categories supported by this plugin
    supported_categories = {'all': 'all', 'movies': 'movies', 'tv': 'tv', 'music': 'music', 'games': 'games', 'software': 'software', 'books': 'books'}

    def search(self, what, cat='all'):
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
