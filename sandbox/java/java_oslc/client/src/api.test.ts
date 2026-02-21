import { describe, it, expect, beforeAll } from 'vitest';

describe('OSLC Backend Connection', () => {

  const SERVER_URL = 'http://127.0.0.1:8080';

  it('should successfully fetch the OSLC Service Provider Catalog', async () => {
    // Attempt to fetch the catalog
    const response = await fetch(`${SERVER_URL}/catalog`, {
      method: 'GET',
      headers: {
        'Accept': 'application/ld+json' // Using exactly the format Lyo expects
      }
    });

    // We expect the connection itself to be successful (not connection refused)
    // and the HTTP response code should be 200 OK.
    expect(response.status).toBe(200);
    expect(response.ok).toBe(true);
    
    // Validate the JSON structure
    const data = await response.json();
    
    // We expect basic OSLC Catalog structure 
    expect(data['@graph']).toBeDefined();
    expect(Array.isArray(data['@graph'])).toBe(true);
    
    // Check if the graph contains "oslc:ServiceProviderCatalog" 
    const hasCatalog = data['@graph'].some((node: any) => node['@type'] === 'oslc:ServiceProviderCatalog');
    expect(hasCatalog).toBe(true);
  });

});
