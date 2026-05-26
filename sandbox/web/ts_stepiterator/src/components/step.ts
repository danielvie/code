import { TimeMarker } from "./timemarker";

export interface StepData {
    func: (data: any) => StepResult| Promise<StepResult>,
    name: string;
    description: string;
}

export interface StepResult {
    ok: boolean,
    data?: any,
    duration_ms?: number,
}

/**
 * @class Step
 * Represents a single step in a process or workflow.
 */
export class Step {
    private data: StepData

    constructor(data: StepData) {
        this.data = data
    }
    
    // .. getters
    get_name(): string {
        return this.data.name
    }

    get_descr(): string {
        return this.data.description
    }

    async run(last_result: any): Promise<StepResult> {
        const start = new Date()
        let result = await this.data.func(last_result)
        const duration_ms = (new Date()).getTime() - start.getTime()
        
        return {...result, duration_ms}
    }
}

/**
 * @class StepCollection
 * Represents a collection of Step objects.
 */
export class StepCollection {
    private m_steps: Step[];
    private m_last_result: any;
    private m_time_marker: TimeMarker;
    
    constructor() {
        this.m_steps = []
        this.m_time_marker = new TimeMarker()
    }
    
    add_step(step: Step): void {
        const name_exist = this.m_steps.some(el => el.get_name() === step.get_name())
        if (name_exist) {
            throw Error(`name "${step.get_name()}" already exists!`)
        }
        
        this.m_steps.push(step)
    }
    
    get_steps(): Step[] {
        return this.m_steps
    }
    
    get_time_markers() {
        return this.m_time_marker
    }

    async run_all(): Promise<any> {
        const len = this.m_steps.length
        let count = 0
        for (const step of this.m_steps) {
            count++
            console.log(`\n========== running step (${count}/${len})`)
            
            const name = step.get_name()
            console.log(`${name}: ${step.get_descr()}`)
            
            // funning func
            this.m_time_marker.start(name)
            const result = await step.run(this.m_last_result)
            this.m_time_marker.stop(name)
            this.m_last_result = result
        }
    }

}