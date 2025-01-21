import http.server
import subprocess

PORT = 8001
CARGO_COMMAND = ["target/release/blog_writer", "Demo Title", "unknown"]
HTML_OUTPUT = "preview.html"

class MarkdownPreviewHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        # Run the cargo command before serving the file
        subprocess.run(CARGO_COMMAND, check=True)
        # Serve the compiled HTML
        self.path = HTML_OUTPUT
        return super().do_GET()

if __name__ == "__main__":
    print(f"Serving at http://localhost:{PORT}")
    http.server.HTTPServer(("", PORT), MarkdownPreviewHandler).serve_forever()