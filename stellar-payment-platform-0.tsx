import React, { useState } from 'react';
import { CreditCard, Wallet, Copy, Check, ExternalLink } from 'lucide-react';

const PaymentPlatform = () => {
  const [paymentData, setPaymentData] = useState({
    amount: '',
    xlmAddresses: ['', '', ''],
    usdcAddresses: ['', '', ''],
    denomination: '',
    businessName: '',
    description: ''
  });
  
  const [showXLMModal, setShowXLMModal] = useState(false);
  const [showUSDCModal, setShowUSDCModal] = useState(false);
  const [copiedAddress, setCopiedAddress] = useState('');
  const [jsonInput, setJsonInput] = useState('');

  // Load payment data from JSON
  const loadFromJSON = () => {
    try {
      const data = JSON.parse(jsonInput);
      setPaymentData({
        amount: data.amount || '',
        xlmAddresses: data.xlmAddresses || ['', '', ''],
        usdcAddresses: data.usdcAddresses || ['', '', ''],
        denomination: data.denomination || '',
        businessName: data.businessName || '',
        description: data.description || ''
      });
    } catch (error) {
      alert('Invalid JSON format');
    }
  };

  const copyToClipboard = (text, type) => {
    navigator.clipboard.writeText(text);
    setCopiedAddress(type);
    setTimeout(() => setCopiedAddress(''), 2000);
  };

  const connectToStellarWallet = (address, index) => {
    // Integration with Stellar Wallet Kit
    console.log(`Connecting to Stellar Wallet for address: ${address}`);
    // This would integrate with @stellar/wallet-kit
    alert(`Connecting to Stellar Wallet for payment of ${paymentData.amount} XLM\nAddress: ${address}`);
  };

  const connectToUSDCWallet = (address, index) => {
    // Integration with wallet connection (MetaMask, WalletConnect, etc.)
    console.log(`Connecting to USDC Wallet for address: ${address}`);
    // This would integrate with Web3 wallet connections
    alert(`Connecting to USDC Wallet for payment of ${paymentData.amount} USDC\nAddress: ${address}`);
  };

  const WalletModal = ({ isOpen, onClose, title, addresses, onConnect, currency }) => {
    if (!isOpen) return null;

    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div className="bg-white rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-2xl font-bold text-gray-800">{title}</h2>
            <button
              onClick={onClose}
              className="text-gray-500 hover:text-gray-700 text-2xl"
            >
              ×
            </button>
          </div>
          
          <div className="mb-6 p-4 bg-blue-50 rounded-lg">
            <p className="text-lg font-semibold text-blue-800">
              Payment Amount: {paymentData.amount} {currency}
            </p>
            <p className="text-sm text-blue-600">
              Business: {paymentData.businessName}
            </p>
            {paymentData.description && (
              <p className="text-sm text-blue-600">
                Description: {paymentData.description}
              </p>
            )}
          </div>

          <div className="space-y-4">
            {addresses.map((address, index) => (
              <div key={index} className="border rounded-lg p-4">
                <div className="flex items-center justify-between mb-3">
                  <h3 className="font-semibold text-gray-700">
                    Wallet Option {index + 1}
                  </h3>
                  <button
                    onClick={() => copyToClipboard(address, `${currency}-${index}`)}
                    className="flex items-center text-blue-600 hover:text-blue-800"
                  >
                    {copiedAddress === `${currency}-${index}` ? (
                      <>
                        <Check className="w-4 h-4 mr-1" />
                        Copied!
                      </>
                    ) : (
                      <>
                        <Copy className="w-4 h-4 mr-1" />
                        Copy
                      </>
                    )}
                  </button>
                </div>
                
                <div className="bg-gray-50 p-3 rounded mb-3">
                  <code className="text-sm break-all">{address}</code>
                </div>
                
                <button
                  onClick={() => onConnect(address, index)}
                  className="w-full bg-gradient-to-r from-blue-500 to-purple-600 text-white py-3 px-4 rounded-lg hover:from-blue-600 hover:to-purple-700 transition-all duration-200 flex items-center justify-center"
                >
                  <Wallet className="w-5 h-5 mr-2" />
                  Connect to {currency === 'XLM' ? 'Stellar' : 'USDC'} Wallet
                </button>
              </div>
            ))}
          </div>
        </div>
      </div>
    );
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-100 via-blue-50 to-cyan-100 p-4">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-gray-800 mb-2">
            Multi-Chain Payment Platform
          </h1>
          <p className="text-gray-600">
            Stellar XLM & USDC Payment Gateway
          </p>
        </div>

        {/* JSON Input Section */}
        <div className="bg-white rounded-lg shadow-lg p-6 mb-6">
          <h2 className="text-xl font-semibold text-gray-800 mb-4">
            Load Payment Data from JSON
          </h2>
          <textarea
            value={jsonInput}
            onChange={(e) => setJsonInput(e.target.value)}
            placeholder='{"amount": "100", "xlmAddresses": ["GABC...", "GDEF...", "GHIJ..."], "usdcAddresses": ["0x123...", "0x456...", "0x789..."], "denomination": "USD", "businessName": "Example Store", "description": "Product purchase"}'
            className="w-full h-32 p-3 border border-gray-300 rounded-lg resize-none font-mono text-sm"
          />
          <button
            onClick={loadFromJSON}
            className="mt-3 bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 transition-colors"
          >
            Load Payment Data
          </button>
        </div>

        {/* Payment Details Display */}
        {paymentData.amount && (
          <div className="bg-white rounded-lg shadow-lg p-6 mb-6">
            <h2 className="text-xl font-semibold text-gray-800 mb-4">
              Payment Details
            </h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <p className="text-sm text-gray-600">Amount</p>
                <p className="text-2xl font-bold text-green-600">
                  {paymentData.amount} {paymentData.denomination}
                </p>
              </div>
              <div>
                <p className="text-sm text-gray-600">Business</p>
                <p className="text-lg font-semibold text-gray-800">
                  {paymentData.businessName}
                </p>
              </div>
              {paymentData.description && (
                <div className="md:col-span-2">
                  <p className="text-sm text-gray-600">Description</p>
                  <p className="text-gray-800">{paymentData.description}</p>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Payment Buttons */}
        <div className="bg-white rounded-lg shadow-lg p-6">
          <h2 className="text-xl font-semibold text-gray-800 mb-6">
            Choose Payment Method
          </h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* XLM Payment Button */}
            <button
              onClick={() => setShowXLMModal(true)}
              disabled={!paymentData.amount}
              className="group relative overflow-hidden bg-gradient-to-r from-yellow-400 to-orange-500 text-white p-6 rounded-lg hover:from-yellow-500 hover:to-orange-600 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-105"
            >
              <div className="flex items-center justify-center mb-3">
                <div className="w-16 h-16 bg-white bg-opacity-20 rounded-full flex items-center justify-center">
                  <CreditCard className="w-8 h-8" />
                </div>
              </div>
              <h3 className="text-xl font-bold mb-2">Pay with XLM</h3>
              <p className="text-sm opacity-90">
                Pay using Stellar Network (XLM)
              </p>
              <div className="absolute inset-0 bg-white opacity-0 group-hover:opacity-10 transition-opacity"></div>
            </button>

            {/* USDC Payment Button */}
            <button
              onClick={() => setShowUSDCModal(true)}
              disabled={!paymentData.amount}
              className="group relative overflow-hidden bg-gradient-to-r from-blue-500 to-purple-600 text-white p-6 rounded-lg hover:from-blue-600 hover:to-purple-700 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-105"
            >
              <div className="flex items-center justify-center mb-3">
                <div className="w-16 h-16 bg-white bg-opacity-20 rounded-full flex items-center justify-center">
                  <Wallet className="w-8 h-8" />
                </div>
              </div>
              <h3 className="text-xl font-bold mb-2">Pay with USDC</h3>
              <p className="text-sm opacity-90">
                Pay using USDC (Ethereum/Avalanche)
              </p>
              <div className="absolute inset-0 bg-white opacity-0 group-hover:opacity-10 transition-opacity"></div>
            </button>
          </div>
        </div>

        {/* Wallet Connection Modals */}
        <WalletModal
          isOpen={showXLMModal}
          onClose={() => setShowXLMModal(false)}
          title="Connect Stellar Wallet"
          addresses={paymentData.xlmAddresses}
          onConnect={connectToStellarWallet}
          currency="XLM"
        />

        <WalletModal
          isOpen={showUSDCModal}
          onClose={() => setShowUSDCModal(false)}
          title="Connect USDC Wallet"
          addresses={paymentData.usdcAddresses}
          onConnect={connectToUSDCWallet}
          currency="USDC"
        />

        {/* Smart Contract Integration Info */}
        <div className="mt-8 bg-white rounded-lg shadow-lg p-6">
          <h2 className="text-xl font-semibold text-gray-800 mb-4">
            Smart Contract Integration
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="p-4 bg-yellow-50 rounded-lg">
              <h3 className="font-semibold text-yellow-800 mb-2">
                Stellar Soroban Contract
              </h3>
              <p className="text-sm text-yellow-700">
                Handles XLM payments and smart contract execution on Stellar network
              </p>
            </div>
            <div className="p-4 bg-blue-50 rounded-lg">
              <h3 className="font-semibold text-blue-800 mb-2">
                Ethereum/Avalanche Contract
              </h3>
              <p className="text-sm text-blue-700">
                Manages USDC payments on L2 Ethereum Avalanche network
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default PaymentPlatform;