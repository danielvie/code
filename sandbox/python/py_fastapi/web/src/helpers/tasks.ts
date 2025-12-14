
// .. constants
export const API_BASE_URL = 'http://localhost:5000';

// .. interfaces
export interface Job {
    id: string;
    status: string;
}

export interface TaskDataResponse {
    status_filter: string;
    jobs: Job[];
}

export interface Task1Request {
    data_id: number;
    config: string;
}

export interface Task1Response {
    status: string;
    data_id: number;
}

export interface Task2Request {
  user_token: string;
  target_value: number; // Use number in TS for float
  parameters: Record<string, any>; // Represents a dictionary
}

export interface Task2Response {
    status: string;
    target: number;
}

export interface Task3Response {
    status_filter: string,
    jobs: Job[],
}

export interface FileUploadResponse {
    filename: string;
    content_type: string;
    message: string;
}


// .. functions

export function helper_post_task1(callback: CallableFunction) {
    const endpoint = `${API_BASE_URL}/api/task1`; 

    const payload: Task1Request = {
        data_id: Math.floor(Math.random() * 1000) + 1,
        config: "high-priority"
    };

    fetch(endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
    })
        .then(response => response.json() as Promise<Task1Response>) // Type the POST response
        .then(data => {
            callback(data)
        })
        .catch(e => console.error("POST Error:", e));
}

export function helper_post_task2(callback: CallableFunction) {
    const endpoint = `${API_BASE_URL}/api/task2`; 

    const payload: Task2Request = {
        user_token: "USR-my_token",
        target_value: 0,
        parameters: {
            "param1": {}
        },
    };

    fetch(endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
    })
        .then(response => response.json() as Promise<Task2Response>)
        .then(data => {
            callback(data)
        })
        .catch(e => console.error("POST Error:", e));
}

export function helper_get_task3(callback: CallableFunction) {
    const endpoint = `${API_BASE_URL}/api/task4`; 

    fetch(endpoint, {
        method: 'GET',
    })
        .then(response => response.json() as Promise<Task3Response>)
        .then(data => {
            callback(data)
        })
        .catch(e => console.error("GET Error:", e));
}
