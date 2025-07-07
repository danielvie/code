import  fs  from 'fs';

/**
 * @interface WasmExports
 * Defines the expected shape of the exports from our WebAssembly module.
 * This provides type safety and autocompletion for the WASM functions.
 */
interface WasmExports {
  /**
   * The 'add' function exported from our 'add.c' file.
   * @param a - The first number.
   * @param b - The second number.
   * @returns The sum of a and b.
   */
  add: (a: number, b: number) => number;
  sub: (a: number, b: number) => number;
}

async function loadWasm(): Promise<WasmExports> {
  // load the wasm file
  const wasmBuffer = fs.readFileSync("./build/add.wasm");

  // instantiate the wasm module
  const wasmModule = await WebAssembly.instantiate(
    new Uint8Array(wasmBuffer),
    { env: {} } // no imports needed
  );

  return wasmModule.instance.exports as unknown as WasmExports;
}

const wasm = await loadWasm()
console.log(wasm.add(35,38))
console.log(wasm.sub(35,38))