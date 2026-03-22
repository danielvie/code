import { add } from "@math/add";
import { subtract } from "@math/sub";
import { multiply } from "@math/mult";

console.log("Welcome to the Bun Monorepo Math Calculator! 🚀");

const a = 20;
const b = 4;

console.log(`${a} + ${b} = ${add(a, b)}`);
console.log(`${a} - ${b} = ${subtract(a, b)}`);
console.log(`${a} * ${b} = ${multiply(a, b)}`);
