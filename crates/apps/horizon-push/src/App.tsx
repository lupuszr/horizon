import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./index.css";
import 'tailwindcss/tailwind.css';
import DragDropArea from "./DragDropArea";
import ReadyToShare from "./ReadyToShare";
  import { listen } from '@tauri-apps/api/event';
import StatusPanel from "./StatusPanel";
import FileSection from "./FileSection";
import processEvent, { IrohEvent, IrohEventPayload, IrohKey } from "./irohEventProcessor";



const NavItem = ({ href, icon, text }) => (
  <a
    href={href}
    className="px-6 py-2 flex items-center text-gray-700 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-all duration-200"
  >
    <i className={`fa-solid ${icon} mr-2`}></i>
    <span>{text}</span>
  </a>
);


const AdSection = () => (
  <div id="ad-section" className="bg-black shadow-lg rounded-xl overflow-hidden">
    <div className="relative h-[400px]">
      <img
        className="absolute inset-0 w-full h-full object-cover"
        src="https://storage.googleapis.com/uxpilot-auth.appspot.com/56c6481536-210715cc7f9d9c7f81d2.png"
        alt="tesla model 3 on a modern minimalist background, dramatic lighting, luxury car photography"
      />
      <div className="absolute inset-0 bg-gradient-to-r from-black/80 to-transparent">
        <div className="p-12 h-full flex flex-col justify-between">
          <div>
            <i className="fa-brands fa-tesla text-white text-4xl mb-4"></i>
            <h2 className="text-4xl font-bold text-white mb-4">Model 3</h2>
            <p className="text-xl text-gray-300 mb-6">Experience the future of driving</p>
            <div className="flex space-x-8 mb-8">
              <FeatureStat value="3.1s" label="0-60 mph" />
              <FeatureStat value="358mi" label="Range (EPA est.)" />
              <FeatureStat value="AWD" label="Dual Motor" />
            </div>
          </div>
          <div className="flex space-x-4">
            <button className="px-8 py-3 bg-white text-black rounded-md hover:bg-gray-100 transition duration-300">
              Custom Order
            </button>
            <button className="px-8 py-3 bg-gray-800 text-white rounded-md hover:bg-gray-700 transition duration-300">
              Learn More
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
);

const FeatureStat = ({ value, label }) => (
  <div className="text-white">
    <div className="text-3xl font-bold">{value}</div>
    <div className="text-sm text-gray-400">{label}</div>
  </div>
);



function App() {
  const processedPayloads = useRef<Map<IrohKey, number>>(new Map());
  
  useEffect(() => { 
    listen<IrohEventPayload>('iroh-event', (event) => {
      processEvent(processedPayloads.current, event);
    });
  }, []);
  const [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set([]));


  async function startPush(path: string) {
    console.log(path)
    const operationId = await invoke('create_operation', { path, verbose: 1 });
    console.log("Operation created:", operationId);

    const result = await invoke('push_send', { operationId });
    console.log("Push result:", result);

    // const status = await invoke('get_operation_status', { operationId });
    // console.log("Operation status:", status);
  }
  
return (
    <div className="h-full text-base-content">
      <div id="app" className="min-h-screen bg-gray-100 flex flex-col">
        <header id="header" className="bg-white border-b border-gray-200">
          <div className="container mx-auto px-6">
            <div className="flex items-center justify-between h-16">
              <div className="flex items-center space-x-2">
                <i className="fa-solid fa-cloud text-blue-600 text-2xl"></i>
                <h1 className="text-2xl font-bold bg-gradient-to-r from-blue-600 to-indigo-600 bg-clip-text text-transparent tracking-tight">
                  HorizonPush
                </h1>
              </div>
              <nav className="flex">
                <NavItem href="#" icon="fa-share-nodes" text="Files" />
                <NavItem href="#" icon="fa-users" text="Shared" />
                <NavItem href="#" icon="fa-gear" text="Settings" />
              </nav>
            </div>
          </div>
        </header>
        <main id="main-content" className="flex-1 p-8">
          <DragDropArea setSelectedFiles={(files) => setSelectedFiles(files)} />
          <ReadyToShare 
            paths={[...selectedFiles]}
            onRemove={(path) => setSelectedFiles(new Set([...selectedFiles].filter(file => path != file)))}
            shareAll={async (_p) => {
              const sf = [...selectedFiles];
              setSelectedFiles(new Set([]));
              
              return await Promise.all(sf.map(sf => startPush(sf)))}
            } />
          <div className="grid grid-cols-2 gap-6 mb-8">
            <FileSection
              id="syncing-files"
              title="Syncing"
              icon="fa-sync"
              itemCount={6}
              items={[
                { icon: "fa-folder", color: "yellow-500", title: "Project Documents", subtitle: "3 items" },
                { icon: "fa-folder", color: "yellow-500", title: "Images", subtitle: "12 items" },
                { icon: "fa-file-word", color: "blue-500", title: "report-2025.docx", subtitle: "3.2 MB" },
                { icon: "fa-file-powerpoint", color: "orange-500", title: "presentation.pptx", subtitle: "5.8 MB" },
              ]}
            />
            <FileSection
              id="shared-files"
              title="Shared"
              icon="fa-share-nodes"
              itemCount={8}
              items={[
                { icon: "fa-folder", color: "yellow-500", title: "Team Resources", subtitle: "8 items" },
                { icon: "fa-file-pdf", color: "red-500", title: "Project_Brief.pdf", subtitle: "2.5 MB" },
                { icon: "fa-folder", color: "yellow-500", title: "Marketing Assets", subtitle: "15 items" },
                { icon: "fa-file-excel", color: "green-600", title: "Financial_Report.xlsx", subtitle: "4.2 MB" },
              ]}
            />
          </div>
          <AdSection />
          <StatusPanel connectionStatus={""} lastSync={""} downloadSpeed={""} uploadSpeed={""} />
        </main>
      </div>
    </div>
  );
}

export default App;
