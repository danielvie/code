import { rm } from 'node:fs/promises';
await rm('artifacts', { recursive: true, force: true });
console.log('Removed generated reports. extension/ and videos/ were not touched.');
