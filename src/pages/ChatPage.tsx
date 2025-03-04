import React, { useState, useRef, useEffect } from 'react';
import { Send, Bot, User, ArrowLeft, RefreshCw } from 'lucide-react';
import elizaService from '../services/elizaService';

const ChatPage: React.FC = () => {
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

  const handleReset = () => {
    elizaService.reset();
    setChatHistory(elizaService.getHistory());
  };

  // Scroll to bottom when chat history updates
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [chatHistory, isTyping]);

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Chat with AgentX</h1>
        <button 
          onClick={handleReset}
          className="bg-gray-700 hover:bg-gray-600 text-white px-4 py-2 rounded-lg flex items-center transition-colors"
          title="Reset conversation"
        >
          <RefreshCw className="h-4 w-4 mr-2" />
          Reset Chat
        </button>
      </div>
      
      <div className="bg-gray-800 rounded-lg shadow-lg overflow-hidden flex flex-col h-[calc(100vh-200px)]">
        {/* Chat header */}
        <div className="bg-blue-600 text-white p-4 flex justify-between items-center">
          <div className="flex items-center">
            <Bot className="h-5 w-5 mr-2" />
            <h3 className="font-semibold">AgentX Yield Assistant</h3>
          </div>
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
              className="flex-grow p-3 border border-gray-600 bg-gray-700 text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
              placeholder="Type your message..."
              rows={2}
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              onKeyDown={handleKeyPress}
              disabled={isTyping}
            />
            <button
              className={`ml-2 ${isTyping ? 'bg-gray-600' : 'bg-blue-600 hover:bg-blue-700'} text-white p-3 rounded-full transition-colors`}
              onClick={handleSendMessage}
              disabled={isTyping}
            >
              <Send className="h-5 w-5" />
            </button>
          </div>
        </div>
      </div>
      
      <div className="mt-6 bg-gray-800 rounded-lg p-6">
        <h2 className="text-xl font-semibold text-white mb-4">How to Use the AgentX Assistant</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-gray-700 p-4 rounded-lg">
            <h3 className="text-lg font-medium text-white mb-2">Ask About Yields</h3>
            <p className="text-gray-300 text-sm">
              Try asking questions like "What are the highest APY opportunities?" or "Which protocols have the lowest risk?"
            </p>
          </div>
          <div className="bg-gray-700 p-4 rounded-lg">
            <h3 className="text-lg font-medium text-white mb-2">Strategy Recommendations</h3>
            <p className="text-gray-300 text-sm">
              Ask for strategy advice like "What's a good conservative strategy?" or "How should I allocate my portfolio?"
            </p>
          </div>
          <div className="bg-gray-700 p-4 rounded-lg">
            <h3 className="text-lg font-medium text-white mb-2">Protocol Information</h3>
            <p className="text-gray-300 text-sm">
              Get information about specific protocols by asking "Tell me about Maiar Exchange" or "What is Hatom Protocol?"
            </p>
          </div>
          <div className="bg-gray-700 p-4 rounded-lg">
            <h3 className="text-lg font-medium text-white mb-2">Risk Management</h3>
            <p className="text-gray-300 text-sm">
              Learn about risk management with questions like "How can I reduce my risk?" or "What's the difference between risk levels?"
            </p>
          </div>
        </div>
        
        <div className="mt-6 bg-blue-900 bg-opacity-30 p-4 rounded-lg border border-blue-800">
          <h3 className="text-lg font-medium text-blue-300 mb-2">Sample Questions to Try</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-gray-800 p-3 rounded-lg">
              <p className="text-gray-300 text-sm">"I want to invest 50 EGLD, what do you recommend?"</p>
            </div>
            <div className="bg-gray-800 p-3 rounded-lg">
              <p className="text-gray-300 text-sm">"What's the difference between Hatom and Maiar Exchange?"</p>
            </div>
            <div className="bg-gray-800 p-3 rounded-lg">
              <p className="text-gray-300 text-sm">"I prefer low risk investments. What are my options?"</p>
            </div>
            <div className="bg-gray-800 p-3 rounded-lg">
              <p className="text-gray-300 text-sm">"Can you suggest a balanced portfolio for long-term growth?"</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ChatPage;