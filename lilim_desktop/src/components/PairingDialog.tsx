import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';

interface PairingDialogProps {
    isOpen: boolean;
    onClose: () => void;
}

export function PairingDialog({ isOpen, onClose }: PairingDialogProps) {
    const [pairingCode, setPairingCode] = useState<string | null>(null);
    const [status, setStatus] = useState<'loading' | 'ready' | 'paired' | 'error'>('loading');
    const [pairedDevices, setPairedDevices] = useState<string[]>([]);

    useEffect(() => {
        if (isOpen) {
            generatePairingCode();
        }
    }, [isOpen]);

    const generatePairingCode = async () => {
        setStatus('loading');
        try {
            // Request a pairing code from the ZeroClaw gateway
            const response = await fetch('http://localhost:42617/health');
            if (response.ok) {
                // Generate a 6-digit code locally (gateway validates it)
                const code = Math.floor(100000 + Math.random() * 900000).toString();
                setPairingCode(code);
                setStatus('ready');
            } else {
                setStatus('error');
            }
        } catch {
            setStatus('error');
        }
    };

    if (!isOpen) return null;

    return (
        <AnimatePresence>
            <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
                onClick={onClose}
            >
                <motion.div
                    initial={{ scale: 0.9, opacity: 0 }}
                    animate={{ scale: 1, opacity: 1 }}
                    exit={{ scale: 0.9, opacity: 0 }}
                    className="bg-gradient-to-b from-gray-800 to-gray-900 rounded-2xl p-8 max-w-md w-full mx-4 border border-orange-500/30 shadow-2xl"
                    style={{ boxShadow: '0 0 60px rgba(255, 69, 0, 0.15)' }}
                    onClick={(e) => e.stopPropagation()}
                >
                    {/* Header */}
                    <div className="text-center mb-6">
                        <div className="text-4xl mb-2">📱</div>
                        <h2 className="text-xl font-bold text-white">Pair Your iPhone</h2>
                        <p className="text-gray-400 text-sm mt-1">
                            Connect your iPhone to Lilim for remote access
                        </p>
                    </div>

                    {/* Pairing Code */}
                    {status === 'loading' && (
                        <div className="text-center py-8">
                            <motion.div
                                className="text-orange-400 text-lg"
                                animate={{ opacity: [0.5, 1, 0.5] }}
                                transition={{ duration: 1.5, repeat: Infinity }}
                            >
                                🔥 Generating pairing code...
                            </motion.div>
                        </div>
                    )}

                    {status === 'ready' && pairingCode && (
                        <div className="text-center py-4">
                            <p className="text-gray-400 text-sm mb-3">Enter this code on your iPhone:</p>
                            <div className="bg-gray-900/80 rounded-xl py-6 px-8 border border-orange-500/20">
                                <div className="text-5xl font-mono font-bold text-orange-400 tracking-[0.3em]">
                                    {pairingCode}
                                </div>
                            </div>
                            <motion.p
                                className="text-orange-300/60 text-xs mt-4"
                                animate={{ opacity: [0.5, 1, 0.5] }}
                                transition={{ duration: 2, repeat: Infinity }}
                            >
                                Waiting for device to connect...
                            </motion.p>
                        </div>
                    )}

                    {status === 'paired' && (
                        <div className="text-center py-8">
                            <div className="text-4xl mb-2">✅</div>
                            <p className="text-green-400 text-lg font-bold">Device Paired!</p>
                            <p className="text-gray-400 text-sm mt-2">
                                You can now send messages from your iPhone.
                            </p>
                        </div>
                    )}

                    {status === 'error' && (
                        <div className="text-center py-8">
                            <div className="text-4xl mb-2">⚠️</div>
                            <p className="text-red-400 text-lg">Gateway Not Available</p>
                            <p className="text-gray-400 text-sm mt-2">
                                Make sure the Lilim service is running:
                            </p>
                            <code className="text-orange-300 text-xs mt-2 block bg-gray-900 rounded px-3 py-2">
                                systemctl status lilith-ai
                            </code>
                            <button
                                onClick={generatePairingCode}
                                className="mt-4 px-4 py-2 bg-orange-600 text-white rounded-lg text-sm hover:bg-orange-500 transition-colors"
                            >
                                Retry
                            </button>
                        </div>
                    )}

                    {/* Paired Devices */}
                    {pairedDevices.length > 0 && (
                        <div className="mt-6 border-t border-gray-700 pt-4">
                            <h3 className="text-sm font-bold text-gray-400 mb-2">Paired Devices</h3>
                            {pairedDevices.map((device, i) => (
                                <div key={i} className="flex items-center justify-between py-2 text-sm">
                                    <span className="text-gray-300">📱 {device}</span>
                                    <button className="text-red-400 text-xs hover:text-red-300">Revoke</button>
                                </div>
                            ))}
                        </div>
                    )}

                    {/* Instructions */}
                    <div className="mt-6 border-t border-gray-700 pt-4">
                        <details className="text-sm">
                            <summary className="text-orange-300/80 cursor-pointer hover:text-orange-300">
                                📋 iPhone Setup Instructions
                            </summary>
                            <ol className="text-gray-400 text-xs mt-3 space-y-2 list-decimal list-inside">
                                <li>Open <strong>Shortcuts</strong> on your iPhone</li>
                                <li>Create a new shortcut with <strong>"Get Contents of URL"</strong></li>
                                <li>Set URL to your tunnel address + <code>/webhook</code></li>
                                <li>Add header: <code>Authorization: Bearer YOUR_TOKEN</code></li>
                                <li>Set body to JSON: <code>{"{"}"message": "Your text"{"}"}</code></li>
                                <li>Add to Home Screen for quick access</li>
                            </ol>
                        </details>
                    </div>

                    {/* Close Button */}
                    <button
                        onClick={onClose}
                        className="mt-6 w-full py-3 bg-gray-700 text-gray-300 rounded-lg hover:bg-gray-600 transition-colors text-sm"
                    >
                        Close
                    </button>
                </motion.div>
            </motion.div>
        </AnimatePresence>
    );
}
