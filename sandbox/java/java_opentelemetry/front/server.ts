import { join, extname } from "path";
import { stat, readFile } from "fs/promises";

const PORT = process.env.PORT || 3000;
const STATIC_DIR = join(import.meta.dir, "static");

// simple MIME map
const mimeTypes: Record<string, string> = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "application/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".gif": "image/gif",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
};

const server = Bun.serve({
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    let filePath = join(STATIC_DIR, url.pathname);

    try {
      const fileStat = await stat(filePath);

      // if directory, serve index.html
      if (fileStat.isDirectory()) {
        filePath = join(filePath, "index.html");
      }

      const data = await readFile(filePath);
      const ext = extname(filePath).toLowerCase();
      const type = mimeTypes[ext] || "application/octet-stream";

      return new Response(data, {
        headers: { "Content-Type": type },
      });
    } catch {
      return new Response("404 Not Found", { status: 404 });
    }
  },
});

console.log(`🚀 Server running at http://localhost:${server.port}`);
