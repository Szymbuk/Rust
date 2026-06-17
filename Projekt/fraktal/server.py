import http.server
import socketserver
import sys

PORT = 8080

class SecurityHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Magiczne nagłówki wielowątkowości
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        # Brutalne wyłączenie cache'u na czas testów
        self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
        super().end_headers()

try:
    with socketserver.TCPServer(("", PORT), SecurityHandler) as httpd:
        print(f"🚀 Wielowątkowy serwer działa na: http://localhost:{PORT}")
        httpd.serve_forever()
except KeyboardInterrupt:
    pass
except Exception:
    pass # Ignorujemy nieistotne błędy połączenia (Broken Pipe)