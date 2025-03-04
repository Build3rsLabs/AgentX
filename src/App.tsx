import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Navbar from './components/Navbar';
import Footer from './components/Footer';
import ProtocolInterface from './pages/ProtocolInterface';
import Dashboard from './pages/Dashboard';
import Protocols from './pages/Protocols';
import Pools from './pages/Pools';
import Assets from './pages/Assets';
import ChatPage from './pages/ChatPage';
import AgentChat from './components/AgentChat';
import { MessageSquare } from 'lucide-react';
import { AuthProvider } from './context/AuthContext';
import { ProtocolProvider } from './context/ProtocolContext';

function App() {
  const [isChatOpen, setIsChatOpen] = React.useState(false);

  return (
    <AuthProvider>
      <ProtocolProvider>
        <Router>
          <div className="min-h-screen bg-gray-900 text-white flex flex-col">
            <Navbar />
            <main className="flex-grow">
              <Routes>
                <Route path="/" element={<Dashboard />} />
                <Route path="/chat" element={<ChatPage />} />
                <Route path="/earn" element={<ProtocolInterface />} />
                <Route path="/earn/:protocolId" element={<ProtocolInterface />} />
                <Route path="/protocols" element={<Protocols />} />
                <Route path="/pools" element={<Pools />} />
                <Route path="/assets" element={<Assets />} />
              </Routes>
            </main>
            <Footer />
            
            {/* Agent Chat */}
            <AgentChat isOpen={isChatOpen} onClose={() => setIsChatOpen(false)} />
            
            {!isChatOpen && (
              <button
                className="fixed bottom-6 right-6 bg-blue-600 text-white p-4 rounded-full shadow-lg hover:bg-blue-700 transition-colors z-50"
                onClick={() => setIsChatOpen(true)}
              >
                <MessageSquare className="h-6 w-6" />
              </button>
            )}
          </div>
        </Router>
      </ProtocolProvider>
    </AuthProvider>
  );
}

export default App;