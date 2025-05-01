import { useState } from 'react'
import './App.css'
import { get_test } from './components/test_plan'
import { Step, StepResult } from './components/step'

function App() {

    const [steps, set_steps] = useState<Step[]>([])
    const [steps_results, set_steps_results] = useState<{[key: number]: StepResult | null}>({})
    const [steps_running, set_steps_running] = useState<{[key: number]: boolean}>({})
    
    function handle_click() {
        const test = get_test()

        const steps = test.get_steps()

        set_steps(steps)
        set_steps_results({}) // reset results when loading new steps
        set_steps_running({}) // reset running states
    }
    
    async function handle_step_click(step: Step, idx: number) {
        console.log(`\n========== running step "${step.get_name()}"`)
        
        // mark step as running
        set_steps_running(prev => (
            { ...prev, [idx]: true }
        ))

        const res = await step.run([])

        // mark step as NOT running
        set_steps_running(prev => (
            { ...prev, [idx]: false }
        ))
        
        set_steps_results(prev => (
            { ...prev, [idx]: res }
        ))
    }
    
    function get_step_status_text(step: Step, idx: number): string {
        if (steps_running[idx]) {
            return `Running ${step.get_name()}...`
        }
        
        if (!steps_results[idx]) {
            return "" // no results yet
        }
        
        if (!steps_results[idx].ok) {
            return "failed"
        }

        return `successful in ${steps_results[idx].duration_ms} ms`

    }

    return (
        <>
            <div className="card">
                <button onClick={handle_click}>
                    run test
                </button>
            </div>
            <div id='steps' className='flex flex-col gap-2'>
                {steps.map((step, idx) => (
                    <div key={idx} className='flex gap-3 items-center'>
                        <button onClick={() => handle_step_click(step, idx)} disabled={steps_running[idx]}>
                            {step.get_name()}
                        </button>
                        <span className='w-40 text-left'>
                            {get_step_status_text(step, idx)}
                        </span>
                    </div>
                ))}
            </div>
        </>
    )
}

export default App
