import React from 'react';

type FileItem = {
  icon: string;
  color: string;
  title: string;
  subtitle: string;
};

type FileSectionProps = {
  id: string;
  title: string;
  icon: string;
  itemCount: number;
  items: FileItem[];
};

const FileSection: React.FC<FileSectionProps> = ({ id, title, icon, itemCount, items }) => (
  <div id={id} className="bg-white shadow-md rounded-lg overflow-hidden">
    <div className="px-6 py-4 border-b border-gray-200 flex justify-between items-center">
      <div className="flex items-center">
        <i className={`fa-solid ${icon} text-blue-500 mr-2`}></i>
        <h3 className="text-lg text-gray-800">{title}</h3>
      </div>
      <span className="text-sm text-gray-500">{itemCount} items</span>
    </div>
    <div className="h-96 overflow-y-auto scrollbar-thin scrollbar-thumb-gray-300 scrollbar-track-gray-100">
      {items.map((item, index) => (
        <div
          key={index}
          className="px-6 py-4 border-b border-gray-200 flex items-center"
        >
          <i className={`fa-solid ${item.icon} text-${item.color} mr-3 text-xl`}></i>
          <div className="flex-1">
            <h4 className="text-gray-800">{item.title}</h4>
            <p className="text-sm text-gray-500">{item.subtitle}</p>
          </div>
        </div>
      ))}
    </div>
  </div>
);

export default FileSection;
