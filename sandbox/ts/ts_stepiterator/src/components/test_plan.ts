
import { Step, StepCollection } from "./step";

export function get_test(): StepCollection {
    const step1 = new Step({
        name: "Step 1",
        description: "This is the first step",
        func: async () => {
            await new Promise(resolve => setTimeout(resolve, 5500));
            console.log("Step 1 executed");
            return { ok: true, data: "my message here" }
        },
    });
    const step2 = new Step({
        name: "Step 2",
        description: "This is the second step",
        func: async () => {
            await new Promise(resolve => setTimeout(resolve, 200));
            console.log("Step 2 executed");
            return { ok: false }
        },
    });
    const step3 = new Step({
        name: "Step 3",
        description: "This is the third step",
        func: async () => {
            await new Promise(resolve => setTimeout(resolve, 800));
            console.log("Step 3 executed");
            return { ok: true }
        },
    });

    const collection = new StepCollection();
    collection.add_step(step1);
    collection.add_step(step2);
    collection.add_step(step3);

    // console.log("\nRunning in parallel:");
    // collection.run_all();
    return collection
}