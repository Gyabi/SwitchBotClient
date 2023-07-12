import React, { FC } from 'react';

interface MessageDialogProps {
  isOpen: boolean;
  onClose: () => void;
  errorMessage: string;
}

const MessageDialog: FC<MessageDialogProps> = ({ isOpen, onClose, errorMessage }) => {
  return (
    <div
      className={`fixed inset-0 flex items-center justify-center z-50 ${
        isOpen ? '' : 'hidden'
      }`}
    >
      <div className="fixed inset-0 bg-gray-900 opacity-50"></div>
      <div className="bg-white p-8 rounded-lg z-10">
        <h2 className="text-xl font-bold mb-4">Message Dialog</h2>
        <p className="text-red-500 mb-4">{errorMessage}</p>
        <div className="flex justify-end">
          <button
            className="bg-red-500 text-white px-4 py-2 rounded"
            onClick={onClose}
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};

export default MessageDialog;
