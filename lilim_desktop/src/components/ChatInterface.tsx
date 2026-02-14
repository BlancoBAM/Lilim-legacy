import { useState, useRef, useEffect } from 'react';
import { motion } from 'motion/react';
import { Send, Maximize2, Minimize2 } from 'lucide-react';
import { FlameBackground } from './FlameBackground';
import { EmberOverlay } from './EmberOverlay';
import bannerImage from 'figma:asset/c80b4d356e3c7b98f2baabf558ea7bacc2421ec9.png';
import centerLogo from 'figma:asset/03a17ee9fd4fe33c3ca16baf528b1598cfae5797.png';
import topLeftLogo from 'figma:asset/51350c1f0fe5a2742ba35cd8899037600d9d9f62.png';

interface Message {
  id: string;
  text: string;
  sender: 'user' | 'ai';
  timestamp: Date;
}

export function ChatInterface() {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: '1',
      text: 'Greetings, seeker. I am Lilim, your guide through the flames of knowledge. What wisdom do you seek today?',
      sender: 'ai',
      timestamp: new Date(),
    },
  ]);
  const [input, setInput] = useState('');
  const [isMaximized, setIsMaximized] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim()) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      text: input,
      sender: 'user',
      timestamp: new Date(),
    };

    setMessages([...messages, userMessage]);
    setInput('');

    try {
      // Import the API module
      const { sendQuery, getSessionId } = await import('../api/lilim');
      
      const response = await sendQuery({
        text: userMessage.text,
        session_id: getSessionId(),
        tools_enabled: true,
        yolo_mode: false,
      });

      const aiMessage: Message = {
        id: (Date.now() + 1).toString(),
        text: response.response,
        sender: 'ai',
        timestamp: new Date(),
      };
      
      setMessages((prev) => [...prev, aiMessage]);
    } catch (error) {
      console.error('Lilim Error:', error);
      const errorMessage: Message = {
        id: (Date.now() + 1).toString(),
        text: error instanceof Error 
          ? `*The flames flicker... ${error.message}*`
          : '*The flames flicker... An unknown error occurred.*',
        sender: 'ai',
        timestamp: new Date(),
      };
      setMessages((prev) => [...prev, errorMessage]);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className={`relative bg-gradient-to-b from-orange-950/60 via-red-950/50 to-gray-900/80 rounded-lg shadow-2xl border border-orange-500/30 overflow-hidden h-full flex flex-col ${
        isMaximized ? 'fixed inset-4 z-50' : 'relative'
      }`}
      style={{
        boxShadow: '0 0 40px rgba(255, 69, 0, 0.2), inset 0 0 40px rgba(255, 69, 0, 0.05)',
      }}
    >
      {/* Flame Background - Bottom Layer */}
      <FlameBackground />
      
      {/* Ember Overlay - Top Layer */}
      <EmberOverlay />

      {/* Header */}
      <div className="relative bg-gradient-to-r from-orange-600 via-red-600 to-orange-600 p-6 border-b border-orange-500/50 z-10">
        <div className="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTAwJSIgaGVpZ2h0PSIxMDAlIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxkZWZzPjxwYXR0ZXJuIGlkPSJmbGFtZSIgcGF0dGVyblVuaXRzPSJ1c2VyU3BhY2VPblVzZSIgd2lkdGg9IjEwMCIgaGVpZ2h0PSIxMDAiPjxwYXRoIGQ9Ik0gMCw1MCBRIDI1LDIwIDUwLDUwIFQgMTAwLDUwIiBzdHJva2U9InJnYmEoMjU1LDEyMCwwLDAuMykiIGZpbGw9Im5vbmUiIHN0cm9rZS13aWR0aD0iMiIvPjwvcGF0dGVybj48L2RlZnM+PHJlY3Qgd2lkdGg9IjEwMCUiIGhlaWdodD0iMTAwJSIgZmlsbD0idXJsKCNmbGFtZSkiLz48L3N2Zz4=')] opacity-30" />
        <div className="relative flex items-center justify-between">
          <div className="flex items-center gap-4">
            <motion.img
              src={topLeftLogo}
              alt="Lilim Logo"
              className="w-16 h-16 object-contain"
              animate={{
                filter: [
                  'drop-shadow(0 0 8px rgba(255, 69, 0, 0.8))',
                  'drop-shadow(0 0 16px rgba(255, 69, 0, 1))',
                  'drop-shadow(0 0 8px rgba(255, 69, 0, 0.8))',
                ],
              }}
              transition={{
                duration: 2,
                repeat: Infinity,
                ease: 'easeInOut',
              }}
            />
            <img
              src={bannerImage}
              alt="Lilith"
              className="h-16 object-contain"
            />
          </div>
          <button
            onClick={() => setIsMaximized(!isMaximized)}
            className="p-2 hover:bg-white/10 rounded transition-colors"
          >
            {isMaximized ? (
              <Minimize2 className="w-5 h-5 text-white" />
            ) : (
              <Maximize2 className="w-5 h-5 text-white" />
            )}
          </button>
        </div>
        
        {/* Divider Line */}
        <div className="absolute bottom-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-orange-400 to-transparent" />
      </div>

      {/* Chat Messages */}
      <div className="relative flex-1 overflow-y-auto p-6 space-y-4 custom-scrollbar z-10">
        {/* Center Background Logo */}
        <div className="absolute inset-0 flex items-center justify-center pointer-events-none z-0">
          <img
            src={centerLogo}
            alt="Lilim Background"
            className="w-80 h-80 object-contain opacity-15"
            style={{
              filter: 'drop-shadow(0 0 20px rgba(255, 69, 0, 0.3))',
            }}
          />
        </div>
        
        {messages.map((message, index) => (
          <motion.div
            key={message.id}
            initial={{ opacity: 0, x: message.sender === 'user' ? 20 : -20 }}
            animate={{ opacity: 1, x: 0 }}
            className={`flex ${message.sender === 'user' ? 'justify-end' : 'justify-start'} relative z-10`}
          >
            <div
              className={`max-w-[80%] px-4 py-3 rounded-lg relative overflow-hidden ${
                message.sender === 'user'
                  ? index % 2 === 0
                    ? 'bg-gradient-to-br from-orange-600 via-orange-500 to-yellow-600 text-white'
                    : 'bg-gradient-to-br from-red-600 via-orange-600 to-red-500 text-white'
                  : index % 2 === 0
                  ? 'bg-gradient-to-t from-gray-800/90 via-gray-700/80 to-gray-600/70 text-gray-100 border border-orange-500/20'
                  : 'bg-gradient-to-t from-gray-900/90 via-gray-800/80 to-gray-700/70 text-gray-100 border border-red-500/20'
              }`}
              style={
                message.sender === 'ai'
                  ? {
                      boxShadow: '0 0 20px rgba(255, 69, 0, 0.1)',
                    }
                  : {
                      boxShadow: '0 0 25px rgba(255, 120, 0, 0.4), inset 0 -2px 15px rgba(255, 200, 0, 0.2)',
                    }
              }
            >
              {/* Smoke effect for AI messages */}
              {message.sender === 'ai' && (
                <div 
                  className="absolute inset-0 opacity-20"
                  style={{
                    background: 'linear-gradient(to top, rgba(100, 100, 100, 0) 0%, rgba(150, 150, 150, 0.3) 100%)',
                  }}
                />
              )}
              <p className="relative z-10">{message.text}</p>
            </div>
          </motion.div>
        ))}
        <div ref={messagesEndRef} />
      </div>

      {/* Input Area */}
      <div className="relative p-4 bg-gray-900/80 border-t border-orange-500/30 z-10">
        <div className="flex gap-3">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="Enter your query..."
            className="flex-1 bg-gray-800/80 text-white px-4 py-3 rounded-lg border border-orange-500/30 focus:border-orange-500 focus:outline-none focus:ring-2 focus:ring-orange-500/20 transition-all placeholder-gray-500"
            style={{
              boxShadow: 'inset 0 0 20px rgba(0, 0, 0, 0.3)',
            }}
          />
          <motion.button
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            onClick={handleSend}
            className="px-6 py-3 bg-gradient-to-r from-orange-600 to-red-600 text-white rounded-lg hover:from-orange-500 hover:to-red-500 transition-all flex items-center gap-2"
            style={{
              boxShadow: '0 0 20px rgba(255, 69, 0, 0.4)',
            }}
          >
            <Send className="w-5 h-5" />
          </motion.button>
        </div>
      </div>
    </motion.div>
  );
}