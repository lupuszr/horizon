
import React, { useState, useRef, useEffect } from "react";
import { open } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { UnlistenFn } from "@tauri-apps/api/event";

type Props = {
  setSelectedFiles: React.Dispatch<React.SetStateAction<Set<string>>>
}

const DragDropArea: React.FC<Props> = ({ setSelectedFiles }) => {
  // const [dragOver, setDragOver] = useState(false);
  // const [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set([]));

  useEffect(() => {

   // *  if (event.payload.type === 'over') {
   // *    console.log('User hovering', event.payload.position);
   // *  } else if (event.payload.type === 'drop') {
   // *    console.log('User dropped', event.payload.paths);
   // *  } else {
   // *    console.log('File drop cancelled');
   // *  }
   // * });
    
    let unlisten: UnlistenFn; //: () => Promise<void>;

    const setupListener = async () => {
      console.log("setup")
      unlisten = await getCurrentWebview().onDragDropEvent((event) => {
      // unlisten = await listen<string>("error", (event) => {
        // console.log(event.payload);
    // const droppedFiles = Array.from(event.dataTransfer.files);
        if (event.payload.type == 'drop') {
          const filePaths = event.payload.paths || [];//droppedFiles.map((file) => file.path); // `file.path` works in Tauri
          console.log("Dropped files/folders:", filePaths);
          setSelectedFiles((prev) => new Set([...prev, ...filePaths]));
        }
        // setErrorMessage(event.payload); // Update state with the error message
      });
    };

    setupListener();

    // Cleanup the listener on component unmount
    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  const handleBrowseClick = async () => {
    try {
      const paths = await open({
        multiple: true,
        directory: true, // Change to `true` if you want directory support in the dialog
      });
      if (paths) {
        const files = Array.isArray(paths) ? paths : [paths];
        console.log("Selected files/folders:", files);
        setSelectedFiles(new Set(files));
      }
    } catch (error) {
      console.error("Error selecting files:", error);
    }
  };


  return (
    <div
      id="drag-drop-area"
      className={`relative mb-8 bg-white shadow-lg rounded-2xl overflow-hidden group cursor-pointer hover:shadow-xl transition-all duration-300 opacity-90`}
    >
      <div className="relative h-[300px] bg-gradient-to-br from-blue-50 via-white to-indigo-50 p-12 text-center">
        <img
          className="absolute inset-0 w-full h-full object-cover opacity-10 translate-y-8"
          src="https://storage.googleapis.com/uxpilot-auth.appspot.com/bc5ccc36df-9045459f05097bc460d0.png"
          alt="cute cartoon fox sitting with laptop, digital art, pastel colors, minimal"
        />
        <div className="relative z-10">
          <div className="transform group-hover:scale-105 transition-transform duration-300">
            <div className="bg-white/80 backdrop-blur-sm rounded-2xl w-24 h-24 mx-auto mb-6 flex items-center justify-center border-2 border-dashed border-blue-200 group-hover:border-blue-400 transition-colors duration-300">
              <i className="fa-regular fa-cloud-arrow-up text-4xl text-blue-400 group-hover:text-blue-600 transition-colors duration-300"></i>
            </div>
            <h3 className="text-3xl font-bold mb-4 text-gray-800 tracking-tight">
              Drop files and folders here
            </h3>
            <p className="text-lg text-gray-600">
              or{" "}
              <span
                className="text-blue-600 hover:underline cursor-pointer"
                onClick={handleBrowseClick}
              >
                browse
              </span>{" "}
              to upload
            </p>
          </div>
        </div>
      </div>

      {/* Display selected files */}
    </div>
  );
};

export default DragDropArea;

