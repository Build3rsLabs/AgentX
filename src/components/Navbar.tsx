import React, { useState } from 'react';
import { Link, NavLink } from 'react-router-dom';
import { Wallet, Menu, X, ChevronDown, BarChart3, Layers, DollarSign, TrendingUp, MessageSquare } from 'lucide-react';
import { useAuth } from '../context/AuthContext';
import ConnectWalletModal from './ConnectWalletModal';

const Navbar: React.FC = () => {
  const { isAuthenticated, walletAddress, logout } = useAuth();
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [isWalletOpen, setIsWalletOpen] = useState(false);
  const [isConnectModalOpen, setIsConnectModalOpen] = useState(false);

  const handleLogout = async () => {
    try {
      await logout();
      setIsWalletOpen(false);
    } catch (error) {
      console.error('Logout error:', error);
    }
  };

  const formatWalletAddress = (address: string | null) => {
    if (!address) return '';
    return `erd1${address.substring(2, 8)}...${address.substring(address.length - 4)}`;
  };

  const navLinkClass = ({ isActive }: { isActive: boolean }) => 
    `px-3 py-2 rounded-md text-sm font-medium ${
      isActive 
        ? 'bg-gray-800 text-white' 
        : 'text-gray-300 hover:bg-gray-700 hover:text-white'
    }`;

  return (
    <>
      <nav className="bg-gray-900 border-b border-gray-800 py-4 px-6 sticky top-0 z-50">
        <div className="container mx-auto flex justify-between items-center">
          <div className="flex items-center">
            <Link to="/" className="flex items-center space-x-2">
              <div className="bg-blue-500 p-2 rounded-full">
                <Wallet className="h-5 w-5" />
              </div>
              <span className="text-xl font-bold">AgentX</span>
            </Link>
            
            {/* Desktop Navigation Links */}
            <div className="hidden md:flex ml-10 space-x-4">
              <NavLink to="/" end className={navLinkClass}>
                <div className="flex items-center">
                  <BarChart3 className="h-4 w-4 mr-1" />
                  <span>Dashboard</span>
                </div>
              </NavLink>
              <NavLink to="/chat" className={navLinkClass}>
                <div className="flex items-center">
                  <MessageSquare className="h-4 w-4 mr-1" />
                  <span>Chat</span>
                </div>
              </NavLink>
              <NavLink to="/earn" className={navLinkClass}>
                <div className="flex items-center">
                  <Wallet className="h-4 w-4 mr-1" />
                  <span>Earn</span>
                </div>
              </NavLink>
              <NavLink to="/protocols" className={navLinkClass}>
                <div className="flex items-center">
                  <Layers className="h-4 w-4 mr-1" />
                  <span>Protocols</span>
                </div>
              </NavLink>
              <NavLink to="/pools" className={navLinkClass}>
                <div className="flex items-center">
                  <DollarSign className="h-4 w-4 mr-1" />
                  <span>Pools</span>
                </div>
              </NavLink>
              <NavLink to="/assets" className={navLinkClass}>
                <div className="flex items-center">
                  <TrendingUp className="h-4 w-4 mr-1" />
                  <span>Assets</span>
                </div>
              </NavLink>
            </div>
          </div>

          {/* Desktop Wallet Button */}
          <div className="hidden md:flex items-center space-x-8">
            {isAuthenticated ? (
              <div className="relative">
                <button 
                  className="flex items-center space-x-1 text-gray-300 hover:text-white transition-colors"
                  onClick={() => setIsWalletOpen(!isWalletOpen)}
                >
                  <span>{formatWalletAddress(walletAddress)}</span>
                  <ChevronDown className="h-4 w-4" />
                </button>
                
                {isWalletOpen && (
                  <div className="absolute right-0 mt-2 w-64 bg-gray-800 rounded-lg shadow-lg p-4 z-50">
                    <div className="flex flex-col space-y-3">
                      <div className="flex justify-between items-center">
                        <span className="text-sm text-gray-400">Connected Wallet</span>
                        <button 
                          className="text-xs text-gray-400 hover:text-white"
                          onClick={() => setIsWalletOpen(false)}
                        >
                          <X className="h-4 w-4" />
                        </button>
                      </div>
                      <div className="bg-gray-700 p-3 rounded-lg">
                        <p className="text-sm font-mono">{walletAddress ? `erd1${walletAddress.substring(2)}` : ''}</p>
                        <div className="flex justify-between mt-2">
                          <span className="text-xs text-gray-400">Balance</span>
                          <span className="text-xs">12.45 EGLD</span>
                        </div>
                      </div>
                      <button 
                        className="bg-red-600 hover:bg-red-700 text-white py-2 rounded-lg text-sm transition-colors"
                        onClick={handleLogout}
                      >
                        Disconnect
                      </button>
                    </div>
                  </div>
                )}
              </div>
            ) : (
              <button 
                className="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg transition-colors"
                onClick={() => setIsConnectModalOpen(true)}
              >
                Connect Wallet
              </button>
            )}
          </div>

          {/* Mobile Menu Button */}
          <button 
            className="md:hidden text-white"
            onClick={() => setIsMenuOpen(!isMenuOpen)}
          >
            <Menu className="h-6 w-6" />
          </button>
        </div>

        {/* Mobile Navigation */}
        {isMenuOpen && (
          <div className="md:hidden bg-gray-800 mt-2 py-2 px-4 rounded-lg">
            <div className="flex flex-col space-y-2">
              <NavLink to="/" end className={navLinkClass}>
                <div className="flex items-center">
                  <BarChart3 className="h-4 w-4 mr-1" />
                  <span>Dashboard</span>
                </div>
              </NavLink>
              <NavLink to="/chat" className={navLinkClass}>
                <div className="flex items-center">
                  <MessageSquare className="h-4 w-4 mr-1" />
                  <span>Chat</span>
                </div>
              </NavLink>
              <NavLink to="/earn" className={navLinkClass}>
                <div className="flex items-center">
                  <Wallet className="h-4 w-4 mr-1" />
                  <span>Earn</span>
                </div>
              </NavLink>
              <NavLink to="/protocols" className={navLinkClass}>
                <div className="flex items-center">
                  <Layers className="h-4 w-4 mr-1" />
                  <span>Protocols</span>
                </div>
              </NavLink>
              <NavLink to="/pools" className={navLinkClass}>
                <div className="flex items-center">
                  <DollarSign className="h-4 w-4 mr-1" />
                  <span>Pools</span>
                </div>
              </NavLink>
              <NavLink to="/assets" className={navLinkClass}>
                <div className="flex items-center">
                  <TrendingUp className="h-4 w-4 mr-1" />
                  <span>Assets</span>
                </div>
              </NavLink>
              
              <div className="pt-2 mt-2 border-t border-gray-700">
                {isAuthenticated ? (
                  <>
                    <div className="flex justify-between items-center py-2">
                      <span className="text-gray-300">{formatWalletAddress(walletAddress)}</span>
                      <span className="text-gray-400 text-sm">12.45 EGLD</span>
                    </div>
                    <button 
                      className="w-full bg-red-600 hover:bg-red-700 px-4 py-2 rounded-lg transition-colors"
                      onClick={handleLogout}
                    >
                      Disconnect
                    </button>
                  </>
                ) : (
                  <button 
                    className="w-full bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg transition-colors"
                    onClick={() => {
                      setIsConnectModalOpen(true);
                      setIsMenuOpen(false);
                    }}
                  >
                    Connect Wallet
                  </button>
                )}
              </div>
            </div>
          </div>
        )}
      </nav>
      
      <ConnectWalletModal 
        isOpen={isConnectModalOpen}
        onClose={() => setIsConnectModalOpen(false)}
      />
    </>
  );
};

export default Navbar;