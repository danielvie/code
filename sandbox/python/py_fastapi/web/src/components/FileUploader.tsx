// src/FileUploader.tsx
import React, { useState, useCallback } from 'react';
import type { FileUploadResponse } from '../helpers/tasks';

const API_BASE_URL = 'http://localhost:5000';

interface FileUploadProps extends React.HTMLAttributes<HTMLDivElement> {
    
}

const FileUploader: React.FC<FileUploadProps> = ({className}) => {
    const [file, set_file] = useState<File | null>(null);
    const [response, set_response] = useState<FileUploadResponse | null>(null);
    const [is_loading, set_is_loading] = useState(false);
    const [error, set_error] = useState<string | null>(null);
    const [drag_active, set_drag_active] = useState(false);

    // --- Drag and Drop Handlers ---
    const handle_drag = (e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
        if (e.type === "dragenter" || e.type === "dragover") {
            set_drag_active(true);
        } else if (e.type === "dragleave") {
            set_drag_active(false);
        }
    };

    const handle_drop = (e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
        set_drag_active(false);
        if (e.dataTransfer.files && e.dataTransfer.files[0]) {
            set_file(e.dataTransfer.files[0]);
            set_response(null); // Clear previous results
            set_error(null);
        }
    };

    const handle_file_change = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files && e.target.files[0]) {
            set_file(e.target.files[0]);
            set_response(null);
            set_error(null);
        }
    };
    
    // --- File Upload Logic ---
    const handle_upload = useCallback(async () => {
        if (!file) {
            set_error("Please select or drag a file first.");
            return;
        }

        set_is_loading(true);
        set_error(null);
        set_response(null);

        // 1. Create FormData object: This is the standard way to send files in a POST request
        const formData = new FormData();
        // The key 'file' must match the parameter name in your FastAPI endpoint!
        formData.append('file', file); 

        try {
            const endpoint = `${API_BASE_URL}/api/uploadfile`; 
            
            const res = await fetch(endpoint, {
                method: 'POST',
                body: formData,
            });

            const data = await res.json();

            if (!res.ok) {
                 throw new Error(data.detail || 'File upload failed.');
            }
            
            set_response(data as FileUploadResponse);
        } catch (e) {
            set_error(e instanceof Error ? e.message : 'An unknown error occurred.');
        } finally {
            set_is_loading(false);
        }
    }, [file]);

    return (
        // <div style={{ padding: '20px', border: '2px dashed #007bff', borderRadius: '5px', backgroundColor: dragActive ? '#e0f7ff' : '#f8f8f8' }}>
        <div className={`${className} p-5 border-2 border-dashed border-[hsl(0,0%,50%) rounded-xl  ${drag_active 
            ? 'bg-[hsl(200,50%,17%)]'
            :' bg-[hsl(0,0%,17%)]'}`}>
            <h3>File Upload (Task: Copy to Folder)</h3>
            <div
                onDragEnter={handle_drag} 
                onDragLeave={handle_drag} 
                onDragOver={handle_drag} 
                onDrop={handle_drop}
                onClick={() => document.getElementById('file-input')?.click()}
                // style={{ cursor: 'pointer', padding: '30px', textAlign: 'center' }}
                className='cursor-pointer p-6'
            >
                <input 
                    type="file" 
                    id="file-input" 
                    onChange={handle_file_change} 
                    style={{ display: 'none' }} 
                    disabled={is_loading}
                />
                <p>{file ? `File selected: ${file.name}` : "Drag and drop a file here, or click to select."}</p>
            </div>
            
            <button 
                onClick={handle_upload} 
                disabled={!file || is_loading}
                className='mt-6 px-4 py-6'
            >
                {is_loading ? 'Uploading...' : 'Send File to Server'}
            </button>

            {error && <p style={{ color: 'red', marginTop: '10px' }}>Error: {error}</p>}
            
            {response && (
                <div className='mt-4 p-4 rounded-xl bg-[hsl(0,0%,25%)]'>
                    <strong>Success!</strong>
                    <p>{response.message}</p>
                    <p>Filename: "<code>{response.filename}</code>"</p>
                </div>
            )}
        </div>
    );
};

export default FileUploader;