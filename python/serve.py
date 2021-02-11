import http.server
import socketserver

PORT = 8080

Handler = http.server.SimpleHTTPRequestHandler

Handler.extensions_map[".wasm"] = "applications/wasm"

httpd = socketserver.TCPSERVER(("", PORT), Handler)

print("serving at port", PORT)
httpd.server_forever()