import React from "react";

type ReadyToShareProps = {
  paths: string[];
  onRemove: (path: string) => void;
  shareAll: (paths: Array<string>) => Promise<void[]>;
}

const ReadyToShare: React.FC<ReadyToShareProps> = ({ paths, onRemove, shareAll }) => {
  const getIconForPath = (path: string) => {
    const extension = (path.split(".").pop() || "").toLowerCase();
    if (path.includes(".") && extension) {
      switch (extension) {
        case "jpg":
        case "jpeg":
        case "png":
        case "gif":
          return "fa-regular fa-file-image text-purple-500";
        case "pdf":
          return "fa-regular fa-file-pdf text-red-500";
        case "mp4":
        case "mov":
        case "avi":
          return "fa-regular fa-file-video text-blue-500";
        case "zip":
        case "rar":
          return "fa-regular fa-file-archive text-yellow-500";
        default:
          return "fa-regular fa-file text-gray-500";
      }
    } else {
      return "fa-regular fa-folder text-orange-500";
    }
  };

  return (
    <div id="ready-to-share" className="mb-8 bg-white shadow-lg rounded-2xl overflow-hidden">
      {/* Header Section */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <i className="fa-solid fa-share-from-square text-green-500 text-xl"></i>
            <h2 className="text-xl font-semibold text-gray-800">Ready to Share</h2>
          </div>
          <button
              onClick={async () => await shareAll(paths)}
              className="px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors duration-200 flex items-center">
            <i className="fa-solid fa-share-nodes mr-2"></i>
            Share All
          </button>
        </div>
      </div>

      <div className="p-6 grid grid-cols-3 gap-6">
        {paths.map((path, index) => (
          <div
            key={index}
            className="bg-white border border-gray-200 rounded-xl p-4 hover:shadow-md transition-all duration-300"
          >
            <div className="flex items-center justify-between mb-4">
              <i className={`${getIconForPath(path)} text-2xl`}></i>
              <button
                onClick={() => onRemove(path)} // Call the function to remove the specific file
                className="text-red-500 hover:text-red-600"
              >
                <i className="fa-solid fa-trash-can"></i> {/* Remove icon */}
              </button>
            </div>
            <h4 className="text-sm font-medium text-gray-900 mb-1 truncate">{path.split('/')[path.split('/').length - 1]}</h4>
          </div>
        ))}
      </div>

      {/* Footer Section */}
      <div className="px-6 py-4 bg-gray-50 border-t border-gray-200">
        <div className="flex items-center justify-between">
          <div className="text-sm text-gray-600">{paths.length} items ready to share</div>
          <div className="text-sm text-gray-600">Click to view details</div>
        </div>
      </div>
    </div>
  );
};

export default ReadyToShare;

