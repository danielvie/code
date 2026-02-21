import { describe, it, expect } from 'vitest';

describe('Vite Proxy Connection', () => {

  // Test the client dev proxy (needs dev server running)
  const VITE_URL = 'http://127.0.0.1:3000';

  it('should successfully fetch the catalog through the Vite Proxy', async () => {
    try {
      // Attempt to fetch via the Vite Proxy mapping
      const response = await fetch(`${VITE_URL}/catalog`, {
        method: 'GET',
        headers: {
          'Accept': 'application/ld+json'
        },
        signal: AbortSignal.timeout(2000) // fast fail
      });

      expect(response.status).toBe(200);
      expect(response.ok).toBe(true);

    } catch (err: any) {
      if (err.name === 'AbortError' || err.code === 'ECONNREFUSED' || err instanceof TypeError) {
        console.warn('⚠️ Vite proxy not running on 3000 or disconnected. Skipping proxy test throw.');
        // We will fail the test but print a nice message so the user knows they need 't c' running.
        throw new Error(`Proxy not reachable at ${VITE_URL}. Did you start 'task c'?`);
      }
      throw err;
    }
  });

});
