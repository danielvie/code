import WasmCalculatorFactory from './build/wasm_calculator';

interface WasmModule {
    cwrap: (name: string, returnType: string, argTypes: string[]) => (...args: any[]) => any;
}

// declare const WasmCalculator: () => Promise<WasmModule>;

interface CalculatorWrapper {
    sum(a: number, b: number): number;
    subtract(a: number, b: number): number;
    multiply(a: number, b: number): number;
    greet(name: string): string;
}

async function createCalculator(): Promise<CalculatorWrapper> {
    const Module = await WasmCalculatorFactory();
    return {
        sum: Module.cwrap('sum', 'number', ['number', 'number']),
        subtract: Module.cwrap('subtract', 'number', ['number', 'number']),
        multiply: Module.cwrap('multiply', 'number', ['number', 'number']),
        greet: Module.cwrap('greet', 'string', ['string']),
    };
}

async function main() {
    const calc = await createCalculator();
    const a: number = 5;
    const b: number = 3;
    const name: string = "User";
    console.log(`Sum of ${a} and ${b} is: ${calc.sum(a, b)}`);
    console.log(`Difference of ${a} and ${b} is: ${calc.subtract(a, b)}`);
    console.log(`Product of ${a} and ${b} is: ${calc.multiply(a, b)}`);
    console.log(`Greeting: ${calc.greet(name)}`);
}

main();