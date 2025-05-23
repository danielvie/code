import WasmSum from "./wasm_sum.js";

const module = await WasmSum()
const sum = module._sum

const a = 5
const b = 8
console.log(`sum of ${a} and ${b}: ${sum(a, b)}`)
