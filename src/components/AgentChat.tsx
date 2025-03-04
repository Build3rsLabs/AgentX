import React, { useState, useRef, useEffect } from 'react';
import { Send, Bot, User, X } from 'lucide-react';
import elizaService from '../services/elizaService';

interface AgentChatProps {
  isOpen: boolean;
  onClose: () => void;
}

const AgentChat: React.FC<AgentChatProps> = ({ isOpen, onClose }) => {
  const [message, setMessage] = useState('');
  const [chatHistory, setChatHistory] = useState(elizaService.getHistory());
  const [isTyping, setIsTyping] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const handleSendMessage = () => {
    if (message.trim() === '') return;
    
    // Add user message to history immediately
    const userMessage = message;
    setMessage('');
    
    // Show typing indicator
    setIsTyping(true);
    
    // Simulate a slight delay for more natural conversation
    setTimeout(() => {
      // Send message to Eliza service
      elizaService.sendMessage(userMessage);
      
      // Update chat history
      setChatHistory(elizaService.getHistory());
      
      // Hide typing indicator
      setIsTyping(false);
    }, 500 + Math.random() * 1000); // Random delay between 500-1500ms
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  // Scroll to bottom when chat history updates
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [chatHistory, isTyping]);

  if (!isOpen) return null;

  return (
    <div className="fixed bottom-0 right-0 w-full md:w-96 h-[500px] bg-gray-800 rounded-t-xl shadow-lg flex flex-col z-50">
      {/* Chat header */}
      <div className="bg-blue-600 text-white p-4 rounded-t-xl flex justify-between items-center">
        <div className="flex items-center">
          <Bot className="h-5 w-5 mr-2" />
          <h3 className="font-semibold">AgentX Yield Assistant</h3>
        </div>
        <button 
          onClick={onClose}
          className="text-white hover:text-gray-200 transition-colors"
        >
          <X className="h-5 w-5" />
        </button>
      </div>
      
      {/* Chat messages */}
      <div className="flex-grow p-4 overflow-y-auto bg-gray-800">
        {chatHistory.map((entry, index) => (
          <div 
            key={index} 
            className={`mb-4 flex ${entry.type === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div 
              className={`max-w-[80%] p-3 rounded-lg ${
                entry.type === 'user' 
                  ? 'bg-blue-600 text-white rounded-tr-none' 
                  : 'bg-gray-700 text-gray-100 rounded-tl-none'
              }`}
            >
              <div className="flex items-center mb-1">
                {entry.type === 'agent' ? (
                  <Bot className="h-4 w-4 mr-1" />
                ) : (
                  <User className="h-4 w-4 mr-1" />
                )}
                <span className="text-xs font-semibold">
                  {entry.type === 'agent' ? 'AgentX' : 'You'}
                </span>
              </div>
              <p className="text-sm whitespace-pre-wrap">{entry.message}</p>
            </div>
          </div>
        ))}
        
        {/* Typing indicator */}
        {isTyping && (
          <div className="mb-4 flex justify-start">
            <div className="max-w-[80%] p-3 rounded-lg bg-gray-700 text-gray-100 rounded-tl-none">
              <div className="flex items-center mb-1">
                <Bot className="h-4 w-4 mr-1" />
                <span className="text-xs font-semibold">AgentX</span>
              </div>
              <div className="flex space-x-2">
                <div className="w-2 h-2 rounded-full bg-blue-400 animate-bounce" style={{ animationDelay: '0ms' }}></div>
                <div className="w-2 h-2 rounded-full bg-blue-400 animate-bounce" style={{ animationDelay: '300ms' }}></div>
                <div className="w-2 h-2 rounded-full bg-blue-400 animate-bounce" style={{ animationDelay: '600ms' }}></div>
              </div>
            </div>
          </div>
        )}
        
        <div ref={messagesEndRef} />
      </div>
      
      {/* Chat input */}
      <div className="border-t border-gray-700 p-4">
        <div className="flex items-center">
          <textarea
            className="flex-grow p-2 border border-gray-600 bg-gray-700 text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
            placeholder="Type your message..."
            rows={2}
            value={message}
            onChange={(e) => setMessage(e.target.value)}
            onKeyDown={handleKeyPress}
            disabled={isTyping}
          />
          <button
            className={`ml-2 ${isTyping ? 'bg-gray-600' : 'bg-blue-600 hover:bg-blue-700'} text-white p-2 rounded-full transition-colors`}
            onClick={handleSendMessage}
            disabled={isTyping}
          >
            <Send className="h-5 w-5" />
          </button>
        </div>
      </div>
    </div>
  );
};

export default AgentChat;