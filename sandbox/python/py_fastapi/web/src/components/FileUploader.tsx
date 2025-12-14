// src/FileUploader.tsx
import React, { useState, useCallback } from 'react';
import { useDropzone } from 'react-dropzone'; // <--- NEW IMPORT
import type { FileUploadResponse } from '../helpers/tasks';

const API_BASE_URL = 'http://localhost:5000';

interface FileUploadProps extends React.HTMLAttributes<HTMLDivElement> {
    
}

const FileUploader: React.FC<FileUploadProps> = ({className}) => {
    // Renamed state variables for clarity (optional but recommended)
    const [file_to_upload, set_file_to_upload] = useState<File | null>(null);
    const [response, set_response] = useState<FileUploadResponse | null>(null);
    const [is_loading, set_is_loading] = useState(false);
    const [error, set_error] = useState<string | null>(null);

    // useDropzone Hook Setup
    const onDrop = useCallback((acceptedFiles: File[]) => {
        // Dropzone handles all drag/drop events and gives us the files
        if (acceptedFiles.length > 0) {
            set_file_to_upload(acceptedFiles[0]);
            set_response(null); // Clear previous results
            set_error(null);
        }
    }, []);

    const { getRootProps, getInputProps, isDragActive } = useDropzone({
        onDrop,
        maxFiles: 1, // Restrict to one file at a time
        noClick: true, // We will use our separate click handler for the input
    });
    
    const handle_upload = useCallback(async () => {
        if (!file_to_upload) {
            set_error("Please select or drag a file first.");
            return;
        }

        set_is_loading(true);
        set_error(null);
        set_response(null);

        const form_data = new FormData();
        form_data.append('file', file_to_upload); 

        const endpoint = `${API_BASE_URL}/api/task4`; 

        fetch(endpoint, {
            method: 'POST',
            body: form_data,
        })
        .then(response => response.json())
        .then(data => {
            console.log('response data: ', data)
            set_is_loading(false); 
        })
        .catch(e => console.error("FILE_UPLOAD Error:", e));

    }, [file_to_upload]); // Dependency on fileToUpload remains crucial

    const lay_file_name = file_to_upload ? file_to_upload.name : "Drag and drop a file here, or click to select.";

    return (
        // --- 2. Apply Dropzone Props to the Root Element ---
        <div 
            {...getRootProps()} // Handles drag/drop events
            className={`
                ${className} 
                p-5 
                border-2 
                border-dashed 
                border-[hsl(0,0%,50%)] 
                rounded-xl transition-colors 
                ${isDragActive  // Use Dropzone's state for visual feedback
                    ? 'bg-[hsl(200,50%,17%)]'
                    : ' bg-[hsl(0,0%,17%)]'}
            `}
        >
            <h3>File Upload (Task: Copy to Folder)</h3>
            
            {/* 3. Input is hidden but connected via getInputProps */}
            <input 
                {...getInputProps()}
                id="file-input" 
                className='hidden'
                disabled={is_loading}
            />
            
            <div
                // The root div now handles drag events, this inner div handles the click
                onClick={() => document.getElementById('file-input')?.click()}
                className='cursor-pointer p-6'
            >
                <p>{lay_file_name}</p>
            </div>
            
            <button 
                onClick={handle_upload} 
                disabled={!file_to_upload || is_loading}
                className='mt-6 px-4 py-6'
            >
                {is_loading ? 'Uploading...' : 'Send File to Server'}
            </button>

            {error && <p className='bg-red-400 mt-4'>Error: {error}</p>}
            
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