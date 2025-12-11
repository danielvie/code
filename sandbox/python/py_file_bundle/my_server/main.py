import sys
import datetime
from http.server import BaseHTTPRequestHandler, HTTPServer

# Importing the necessary components from Jinja2
from jinja2 import Environment, PackageLoader

# --- Resource Handling ---

# Configure Jinja2 to use the PackageLoader.
# This tells Jinja2:
# 1. The templates are packaged inside the Python module named 'my_server'.
# 2. They are located in the sub-directory named 'templates'.
# This avoids using filesystem paths and relies on package structure,
# which is robust for frozen executables (like those created by PyInstaller).
jinja_env = Environment(loader=PackageLoader("my_server", "templates"), autoescape=True)

# --- Server Logic ---


class SimpleServer(BaseHTTPRequestHandler):
    """
    A simple HTTP request handler that serves an HTML page rendered
    from an embedded template.
    """

    def do_GET(self):
        if self.path == "/":
            self.send_response(200)
            self.send_header("Content-type", "text/html")
            self.end_headers()

            # Load the template using PackageLoader's reference
            template = jinja_env.get_template("index.html")

            # Data to pass to the template
            context = {
                "title": "Python Embedded Server",
                "name": "Embedded File Example",
                "timestamp": datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
            }

            output = template.render(context)
            self.wfile.write(output.encode("utf-8"))
        else:
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b"404 Not Found")


def run(server_class=HTTPServer, handler_class=SimpleServer, port=5000):
    """Starts the HTTP server."""
    server_address = ("", port)
    httpd = server_class(server_address, handler_class)
    print(f"Starting httpd on port {port}...")

    # We include a try/except block for graceful shutdown
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nServer stopping...")
        httpd.server_close()
    finally:
        sys.exit(0)


if __name__ == "__main__":
    # This block is primarily for testing when running the file directly,
    # but the run function is what server_run.py will call.
    run()
