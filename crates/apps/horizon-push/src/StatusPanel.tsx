import React from 'react';

type StatusPanelProps = {
  connectionStatus: string;
  lastSync: string;
  downloadSpeed: string;
  uploadSpeed: string;
};

const StatusPanel: React.FC<StatusPanelProps> = ({ connectionStatus, lastSync, downloadSpeed, uploadSpeed }) => {
  return (
    <div
      id="status-panel"
      className="fixed bottom-0 left-0 right-0 bg-white border-t border-gray-200 shadow-lg"
    >
      <div className="container mx-auto px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-6">
            {/* Connection Status */}
            <div className="flex items-center text-green-600">
              <i className="fa-solid fa-circle text-xs mr-2"></i>
              <span className="text-sm font-medium">{connectionStatus}</span>
            </div>

            {/* Signal Strength */}
            <div className="flex items-center text-gray-600">
              <i className="fa-solid fa-signal mr-2"></i>
              <span className="text-sm">Strong Connection</span>
            </div>

            {/* Last Sync */}
            <div className="flex items-center text-gray-600">
              <i className="fa-solid fa-clock mr-2"></i>
              <span className="text-sm">Last sync: {lastSync}</span>
            </div>
          </div>

          <div className="flex items-center space-x-4">
            {/* Download Speed */}
            <div className="text-sm text-gray-600">
              <i className="fa-solid fa-download text-blue-500 mr-1"></i>
              <span>{downloadSpeed} MB/s</span>
            </div>

            {/* Upload Speed */}
            <div className="text-sm text-gray-600">
              <i className="fa-solid fa-upload text-green-500 mr-1"></i>
              <span>{uploadSpeed} MB/s</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default StatusPanel;
