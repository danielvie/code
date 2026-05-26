import { serve } from "bun";
import { join } from "path";

const PORT = 3000;
const PUBLIC_DIR = join(import.meta.dir, "../public");

console.log(`--- SemanticLabs Dashboard ---`);
console.log(`Starting server on http://localhost:${PORT}`);

serve({
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    let path = url.pathname;

    if (path === "/") path = "/index.html";

    const file = Bun.file(join(PUBLIC_DIR, path));
    if (await file.exists()) {
      return new Response(file);
    }

    return new Response("Not Found", { status: 404 });
  },
});
