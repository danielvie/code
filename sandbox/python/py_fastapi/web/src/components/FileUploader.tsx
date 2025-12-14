// src/FileUploader.tsx
import React, { useState, useCallback } from 'react';
import { useDropzone } from 'react-dropzone'; // <--- NEW IMPORT
import type { FileUploadResponse } from '../helpers/tasks';

const API_BASE_URL = 'http://localhost:5000';

interface FileUploadProps extends React.HTMLAttributes<HTMLDivElement> {

}

const FileUploader: React.FC<FileUploadProps> = ({ className }) => {

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
        // noClick: true, // We will use our separate click handler for the input
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

    return (

        // --- 2. Apply Dropzone Props to the Root Element ---
        <>
            <div className={ `${className} flex gap-2` }>
                <div
                    {...getRootProps()} // Handles drag/drop events
                    className={`
                p-5 
                grow
                border-2 
                border-dashed 
                border-[hsl(0,0%,50%)] 
                rounded-xl transition-colors 
                cursor-pointer
                ${isDragActive  // Use Dropzone's state for visual feedback
                            ? 'bg-[hsl(200,50%,17%)]'
                            : 'bg-[hsl(0,0%,17%)]'}
            `}
                >

                    {!file_to_upload && (<p>Drag your file here</p>)}
                    {file_to_upload && (<p>file: {file_to_upload.name}</p>)}

                    <input
                        {...getInputProps()}
                        id="file-input"
                        className='hidden'
                        disabled={is_loading}
                    />



                    {error && <p className='bg-red-400 mt-4'>Error: {error}</p>}

                    {response && (
                        <div className='mt-4 p-4 rounded-xl bg-[hsl(0,0%,25%)]'>
                            <strong>Success!</strong>
                            <p>{response.message}</p>
                            <p>Filename: "<code>{response.filename}</code>"</p>
                        </div>
                    )}
                </div>

                {file_to_upload && (
                    <div className='p-5 bg-[hsl(0,0%,17%)] border-2 border-[hsl(0,0%,50%)] rounded-xl'>
                        <button
                            onClick={handle_upload}
                            disabled={!file_to_upload || is_loading}
                            className=''
                        >
                            {is_loading ? 'Uploading...' : 'Send'}
                        </button>
                    </div>
                )}
            </div>
        </>


    );
};

export default FileUploader;