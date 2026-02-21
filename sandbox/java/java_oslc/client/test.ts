async function testConnection() {
  console.log('Testing connection to OSLC Server...');
  try {
    const response = await fetch('http://127.0.0.1:8080/catalog', {
      method: 'GET',
      headers: {
        'Accept': 'application/ld+json'
      }
    });

    console.log(`Status: ${response.status} ${response.statusText}`);
    
    if (response.ok) {
      const data = await response.json();
      console.log('✅ Connection Sucessful! Catalog Data:');
      console.log(JSON.stringify(data, null, 2));
    } else {
      console.error('❌ Connection Failed with status:', response.status);
    }
  } catch (error) {
    console.error('❌ Fetch failed:', error);
  }
}

testConnection();
