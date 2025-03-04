import React from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { useProtocol } from '../context/ProtocolContext';

const ProtocolSelector: React.FC = () => {
  const navigate = useNavigate();
  const { protocolId } = useParams();
  const { protocols, isLoading } = useProtocol();
  
  const handleProtocolChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedId = e.target.value;
    if (selectedId) {
      navigate(`/earn/${selectedId}`);
    }
  };

  return (
    <div className="bg-gray-800 rounded-lg p-4 mb-6">
      <div className="flex flex-col md:flex-row md:items-center justify-between">
        <div className="mb-4 md:mb-0">
          <h2 className="text-xl font-semibold text-white">Select Protocol</h2>
          <p className="text-gray-400 text-sm">Choose a protocol to interact with</p>
        </div>
        
        <div className="w-full md:w-64">
          <select
            className="w-full bg-gray-700 border border-gray-600 text-white rounded-lg p-3 focus:outline-none focus:ring-2 focus:ring-blue-500"
            value={protocolId || ''}
            onChange={handleProtocolChange}
            disabled={isLoading || protocols.length === 0}
          >
            {isLoading ? (
              <option value="">Loading protocols...</option>
            ) : protocols.length === 0 ? (
              <option value="">No protocols available</option>
            ) : (
              <>
                <option value="" disabled>Select a protocol</option>
                {protocols.map(protocol => (
                  <option key={protocol.id} value={protocol.id}>
                    {protocol.name}
                  </option>
                ))}
              </>
            )}
          </select>
        </div>
      </div>
    </div>
  );
};

export default ProtocolSelector;