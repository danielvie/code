// src/App.jsx
import { useState } from 'react';
import './App.css';
import { helper_get_task3, helper_post_task1, helper_post_task2, type FileUploadResponse, type TaskDataResponse } from './helpers/tasks';
import { type Task1Response, type Task2Response, type Task3Response } from './helpers/tasks';
import FileUploader from './components/FileUploader';

function App() {
    const [message, set_message] = useState<string | null>(".");

    function handle_post_task1() {
        helper_post_task1((data: Task1Response) => {
            set_message(data.status)
            console.log('response task1:', data)
        });
    }

    function handle_post_task2() {
        helper_post_task2((data: Task2Response) => {
            set_message(data.status)
            console.log('response task2:', data)
        });
    }

    function handle_post_task3() {
        helper_get_task3((data: Task3Response) => {
            set_message(`task3: ${JSON.stringify(data.jobs)}`)
            console.log('response task4:', data)
        });
    }
    
    function callback_file(data: FileUploadResponse) {
        set_message(data.message)
    }

    return (
        <div className="App w-125">
            <h1>testing FastAPI</h1>

            <FileUploader className='mt-10' callback={callback_file}/>

            <div className='mt-4 flex flex-col gap-2'>
                <button onClick={handle_post_task1}>Execute Task 1 (POST)</button>
                <button onClick={handle_post_task2}>Execute Task 2 (POST)</button>
                <button onClick={handle_post_task3}>Execute Task 3 (GET)</button>
            </div>
            <div className='mt-7 flex flex-col gap-1'>
                <div className='p-4 border-2 border-[hsl(0,0%,50%)] border-dashed rounded-md' onClick={() => set_message('.')}>{message}</div>
            </div>

        </div>
    );
}

export default App;