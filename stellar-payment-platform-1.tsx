import React, { useState } from 'react';
import { Wallet, CreditCard, X, ExternalLink } from 'lucide-react';

const PaymentPlatform = () => {
  const [showXLMModal, setShowXLMModal] = useState(false);
  const [showUSDCModal, setShowUSDCModal] = useState(false);
  const [selectedNetwork, setSelectedNetwork] = useState(null);
  
  // Sample JSON data structure
  const paymentData = {
    amount: 150.00,
    currency: "USD",
    stellarAddresses: [
      "GCKFBEIYTKP5RDBZ7QVRHKK5GFTYUXD5WFJE3DFXDGF3HDVYGRRHIKMR",
      "GADTMGF3XDZXGQJZF7VQXHWSYKQRQ3VBKXMDGRQXFKWYLXZM4QKJHKFM",
      "GAKBPBDMKQRQXFKWYLXZM4QKJHKFMGADTMGF3XDZXGQJZF7VQXHWSYKQ"
    ],
    usdcAddresses: [
      "0x742d35Cc6634C0532925a3b8D400a6ff5E3b0e4b",
      "0x8ba1f109551bD432803012645Hac136c22C3BA6",
      "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984"
    ],
    customerName: "John Doe",
    businessName: "TechStore Solutions",
    description: "Premium Software License",
    orderId: "ORD-2025-001"
  };

  const handleXLMPayment = () => {
    setShowXLMModal(true);
  };

  const handleUSDCPayment = () => {
    setShowUSDCModal(true);
  };

  const handleWalletConnect = (address, network) => {
    // This would integrate with actual wallet kit
    console.log(`Connecting to ${network} wallet for address: ${address}`);
    
    if (network === 'stellar') {
      // Integration with Stellar Wallet Kit
      // window.StellarWalletKit.connect(address);
      alert(`Connecting to Stellar Wallet Kit for address: ${address.slice(0, 8)}...`);
    } else {
      // Integration with Web3/Ethereum wallet
      // window.ethereum.request({ method: 'eth_requestAccounts' });
      alert(`Connecting to Web3 Wallet for address: ${address.slice(0, 8)}...`);
    }
  };

  const AddressButton = ({ address, network, label }) => (
    <div className="border rounded-lg p-4 bg-gray-50 hover:bg-gray-100 transition-colors">
      <div className="flex items-center justify-between mb-2">
        <span className="text-sm font-medium text-gray-700">{label}</span>
        <Wallet className="w-4 h-4 text-gray-500" />
      </div>
      <div className="text-xs text-gray-500 mb-3 font-mono break-all">
        {address.slice(0, 12)}...{address.slice(-8)}
      </div>
      <button
        onClick={() => handleWalletConnect(address, network)}
        className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 transition-colors flex items-center justify-center gap-2"
      >
        <ExternalLink className="w-4 h-4" />
        Connect Wallet
      </button>
    </div>
  );

  const Modal = ({ isOpen, onClose, title, children }) => {
    if (!isOpen) return null;

    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
        <div className="bg-white rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto">
          <div className="flex items-center justify-between p-6 border-b">
            <h2 className="text-xl font-semibold">{title}</h2>
            <button
              onClick={onClose}
              className="text-gray-500 hover:text-gray-700"
            >
              <X className="w-6 h-6" />
            </button>
          </div>
          <div className="p-6">
            {children}
          </div>
        </div>
      </div>
    );
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 p-4">
      <div className="max-w-2xl mx-auto">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">
            Multi-Chain Payment Platform
          </h1>
          <p className="text-gray-600">
            Secure payments with Stellar XLM and USDC
          </p>
        </div>

        {/* Payment Card */}
        <div className="bg-white rounded-xl shadow-lg p-6 mb-6">
          <div className="border-b pb-4 mb-4">
            <h2 className="text-xl font-semibold text-gray-900">
              {paymentData.businessName}
            </h2>
            <p className="text-gray-600">{paymentData.description}</p>
          </div>

          <div className="grid grid-cols-2 gap-4 mb-6">
            <div>
              <label className="text-sm font-medium text-gray-700">Customer</label>
              <p className="text-gray-900">{paymentData.customerName}</p>
            </div>
            <div>
              <label className="text-sm font-medium text-gray-700">Order ID</label>
              <p className="text-gray-900">{paymentData.orderId}</p>
            </div>
          </div>

          <div className="text-center mb-6">
            <div className="text-3xl font-bold text-gray-900 mb-2">
              ${paymentData.amount.toFixed(2)} {paymentData.currency}
            </div>
            <p className="text-gray-600">Amount to pay</p>
          </div>

          {/* Payment Buttons */}
          <div className="grid grid-cols-2 gap-4">
            <button
              onClick={handleXLMPayment}
              className="bg-gradient-to-r from-purple-600 to-blue-600 text-white py-4 px-6 rounded-lg hover:from-purple-700 hover:to-blue-700 transition-all duration-300 flex items-center justify-center gap-3 shadow-lg hover:shadow-xl"
            >
              <Wallet className="w-5 h-5" />
              Pay with XLM
            </button>
            <button
              onClick={handleUSDCPayment}
              className="bg-gradient-to-r from-green-600 to-teal-600 text-white py-4 px-6 rounded-lg hover:from-green-700 hover:to-teal-700 transition-all duration-300 flex items-center justify-center gap-3 shadow-lg hover:shadow-xl"
            >
              <CreditCard className="w-5 h-5" />
              Pay with USDC
            </button>
          </div>
        </div>

        {/* XLM Payment Modal */}
        <Modal
          isOpen={showXLMModal}
          onClose={() => setShowXLMModal(false)}
          title="Pay with XLM - Stellar Network"
        >
          <div className="mb-6">
            <div className="bg-purple-50 border border-purple-200 rounded-lg p-4 mb-4">
              <h3 className="font-semibold text-purple-900 mb-2">Payment Details</h3>
              <p className="text-purple-800">Amount: ${paymentData.amount} USD</p>
              <p className="text-purple-800">Network: Stellar</p>
            </div>
          </div>
          
          <h3 className="text-lg font-semibold mb-4">Select Stellar Address</h3>
          <div className="grid gap-4">
            {paymentData.stellarAddresses.map((address, index) => (
              <AddressButton
                key={index}
                address={address}
                network="stellar"
                label={`Stellar Address ${index + 1}`}
              />
            ))}
          </div>
        </Modal>

        {/* USDC Payment Modal */}
        <Modal
          isOpen={showUSDCModal}
          onClose={() => setShowUSDCModal(false)}
          title="Pay with USDC - Ethereum L2"
        >
          <div className="mb-6">
            <div className="bg-green-50 border border-green-200 rounded-lg p-4 mb-4">
              <h3 className="font-semibold text-green-900 mb-2">Payment Details</h3>
              <p className="text-green-800">Amount: ${paymentData.amount} USDC</p>
              <p className="text-green-800">Network: Ethereum L2 (Avalanche)</p>
            </div>
          </div>
          
          <h3 className="text-lg font-semibold mb-4">Select Ethereum Address</h3>
          <div className="grid gap-4">
            {paymentData.usdcAddresses.map((address, index) => (
              <AddressButton
                key={index}
                address={address}
                network="ethereum"
                label={`Ethereum Address ${index + 1}`}
              />
            ))}
          </div>
        </Modal>

        {/* Network Status */}
        <div className="bg-white rounded-lg shadow p-4">
          <h3 className="font-semibold mb-2">Network Status</h3>
          <div className="flex items-center gap-4 text-sm">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 bg-green-500 rounded-full"></div>
              <span>Stellar Network</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 bg-green-500 rounded-full"></div>
              <span>Avalanche Network</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default PaymentPlatform;