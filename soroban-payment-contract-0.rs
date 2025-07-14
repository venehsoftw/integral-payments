#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contractmeta,
    Address, Env, String, Vec, Map, log,
    token::{Client as TokenClient, StellarAssetClient},
    auth::{Context, CustomAccountInterface},
    panic_with_error
};

// Contract metadata
contractmeta!(
    key = "Description",
    val = "Multi-Chain Payment Platform - Stellar XLM Payment Contract"
);

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    PaymentRequest(u64),
    BusinessConfig(String),
    PaymentHistory(Address),
    AuthorizedAddresses,
    ContractOwner,
    FeeConfig,
}

#[derive(Clone)]
#[contracttype]
pub struct PaymentRequest {
    pub id: u64,
    pub amount: i128,
    pub business_name: String,
    pub description: String,
    pub denomination: String,
    pub authorized_addresses: Vec<Address>,
    pub requester: Address,
    pub timestamp: u64,
    pub status: PaymentStatus,
    pub fee_percentage: u32, // Basis points (100 = 1%)
}

#[derive(Clone)]
#[contracttype]
pub enum PaymentStatus {
    Pending,
    Authorized,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone)]
#[contracttype]
pub struct BusinessConfig {
    pub name: String,
    pub owner: Address,
    pub fee_recipient: Address,
    pub default_fee_percentage: u32,
    pub is_active: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct PaymentHistory {
    pub total_payments: u64,
    pub total_amount: i128,
    pub last_payment_id: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum Error {
    NotAuthorized = 1,
    PaymentNotFound = 2,
    InvalidAmount = 3,
    InvalidFeePercentage = 4,
    PaymentAlreadyCompleted = 5,
    BusinessNotActive = 6,
    InsufficientBalance = 7,
    InvalidAddress = 8,
    PaymentExpired = 9,
    ContractNotInitialized = 10,
}

#[contract]
pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {
    /// Initialize the contract with owner and default configurations
    pub fn initialize(
        env: Env,
        owner: Address,
        default_fee_percentage: u32,
    ) -> Result<(), Error> {
        if default_fee_percentage > 10000 {
            panic_with_error!(&env, Error::InvalidFeePercentage);
        }

        owner.require_auth();

        // Set contract owner
        env.storage().instance().set(&DataKey::ContractOwner, &owner);
        
        // Set default fee configuration
        env.storage().instance().set(&DataKey::FeeConfig, &default_fee_percentage);

        log!(&env, "Contract initialized with owner: {}", owner);
        Ok(())
    }

    /// Register a new business for payment processing
    pub fn register_business(
        env: Env,
        business_name: String,
        business_owner: Address,
        fee_recipient: Address,
        fee_percentage: u32,
    ) -> Result<(), Error> {
        business_owner.require_auth();

        if fee_percentage > 10000 {
            panic_with_error!(&env, Error::InvalidFeePercentage);
        }

        let business_config = BusinessConfig {
            name: business_name.clone(),
            owner: business_owner,
            fee_recipient,
            default_fee_percentage: fee_percentage,
            is_active: true,
        };

        env.storage().persistent().set(&DataKey::BusinessConfig(business_name), &business_config);
        Ok(())
    }

    /// Create a new payment request
    pub fn create_payment_request(
        env: Env,
        amount: i128,
        business_name: String,
        description: String,
        denomination: String,
        authorized_addresses: Vec<Address>,
        requester: Address,
        custom_fee_percentage: Option<u32>,
    ) -> Result<u64, Error> {
        requester.require_auth();

        if amount <= 0 {
            panic_with_error!(&env, Error::InvalidAmount);
        }

        if authorized_addresses.len() == 0 {
            panic_with_error!(&env, Error::InvalidAddress);
        }

        // Verify business exists and is active
        let business_config: BusinessConfig = env.storage()
            .persistent()
            .get(&DataKey::BusinessConfig(business_name.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, Error::BusinessNotActive));

        if !business_config.is_active {
            panic_with_error!(&env, Error::BusinessNotActive);
        }

        // Determine fee percentage
        let fee_percentage = custom_fee_percentage.unwrap_or(business_config.default_fee_percentage);
        
        if fee_percentage > 10000 {
            panic_with_error!(&env, Error::InvalidFeePercentage);
        }

        // Generate unique payment ID
        let payment_id = env.ledger().timestamp();

        let payment_request = PaymentRequest {
            id: payment_id,
            amount,
            business_name,
            description,
            denomination,
            authorized_addresses,
            requester,
            timestamp: env.ledger().timestamp(),
            status: PaymentStatus::Pending,
            fee_percentage,
        };

        env.storage().persistent().set(&DataKey::PaymentRequest(payment_id), &payment_request);

        log!(&env, "Payment request created with ID: {}", payment_id);
        Ok(payment_id)
    }

    /// Execute payment from one of the authorized addresses
    pub fn execute_payment(
        env: Env,
        payment_id: u64,
        payer: Address,
        token_address: Address,
    ) -> Result<(), Error> {
        payer.require_auth();

        let mut payment_request: PaymentRequest = env.storage()
            .persistent()
            .get(&DataKey::PaymentRequest(payment_id))
            .unwrap_or_else(|| panic_with_error!(&env, Error::PaymentNotFound));

        // Verify payment is still pending
        match payment_request.status {
            PaymentStatus::Pending => {},
            PaymentStatus::Completed => panic_with_error!(&env, Error::PaymentAlreadyCompleted),
            _ => panic_with_error!(&env, Error::PaymentNotFound),
        }

        // Verify payer is authorized
        if !payment_request.authorized_addresses.contains(&payer) {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        // Get business configuration
        let business_config: BusinessConfig = env.storage()
            .persistent()
            .get(&DataKey::BusinessConfig(payment_request.business_name.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, Error::BusinessNotActive));

        // Calculate fee and net amount
        let fee_amount = (payment_request.amount * payment_request.fee_percentage as i128) / 10000;
        let net_amount = payment_request.amount - fee_amount;

        // Initialize token client
        let token_client = TokenClient::new(&env, &token_address);

        // Check payer balance
        let payer_balance = token_client.balance(&payer);
        if payer_balance < payment_request.amount {
            panic_with_error!(&env, Error::InsufficientBalance);
        }

        // Execute transfers
        if net_amount > 0 {
            token_client.transfer(&payer, &payment_request.requester, &net_amount);
        }

        if fee_amount > 0 {
            token_client.transfer(&payer, &business_config.fee_recipient, &fee_amount);
        }

        // Update payment status
        payment_request.status = PaymentStatus::Completed;
        env.storage().persistent().set(&DataKey::PaymentRequest(payment_id), &payment_request);

        // Update payment history
        Self::update_payment_history(&env, &payer, payment_id, payment_request.amount);

        log!(&env, "Payment {} executed successfully", payment_id);
        Ok(())
    }

    /// Execute XLM payment (native Stellar asset)
    pub fn execute_xlm_payment(
        env: Env,
        payment_id: u64,
        payer: Address,
    ) -> Result<(), Error> {
        payer.require_auth();

        let mut payment_request: PaymentRequest = env.storage()
            .persistent()
            .get(&DataKey::PaymentRequest(payment_id))
            .unwrap_or_else(|| panic_with_error!(&env, Error::PaymentNotFound));

        // Verify payment is still pending
        match payment_request.status {
            PaymentStatus::Pending => {},
            PaymentStatus::Completed => panic_with_error!(&env, Error::PaymentAlreadyCompleted),
            _ => panic_with_error!(&env, Error::PaymentNotFound),
        }

        // Verify payer is authorized
        if !payment_request.authorized_addresses.contains(&payer) {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        // Get business configuration
        let business_config: BusinessConfig = env.storage()
            .persistent()
            .get(&DataKey::BusinessConfig(payment_request.business_name.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, Error::BusinessNotActive));

        // Calculate fee and net amount
        let fee_amount = (payment_request.amount * payment_request.fee_percentage as i128) / 10000;
        let net_amount = payment_request.amount - fee_amount;

        // Use Stellar Asset Client for XLM
        let stellar_asset = StellarAssetClient::new(&env);

        // Execute transfers
        if net_amount > 0 {
            stellar_asset.transfer(&payer, &payment_request.requester, &net_amount);
        }

        if fee_amount > 0 {
            stellar_asset.transfer(&payer, &business_config.fee_recipient, &fee_amount);
        }

        // Update payment status
        payment_request.status = PaymentStatus::Completed;
        env.storage().persistent().set(&DataKey::PaymentRequest(payment_id), &payment_request);

        // Update payment history
        Self::update_payment_history(&env, &payer, payment_id, payment_request.amount);

        log!(&env, "XLM Payment {} executed successfully", payment_id);
        Ok(())
    }

    /// Get payment request details
    pub fn get_payment_request(env: Env, payment_id: u64) -> Result<PaymentRequest, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::PaymentRequest(payment_id))
            .ok_or(Error::PaymentNotFound)
    }

    /// Get business configuration
    pub fn get_business_config(env: Env, business_name: String) -> Result<BusinessConfig, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::BusinessConfig(business_name))
            .ok_or(Error::BusinessNotActive)
    }

    /// Get payment history for an address
    pub fn get_payment_history(env: Env, address: Address) -> PaymentHistory {
        env.storage()
            .persistent()
            .get(&DataKey::PaymentHistory(address))
            .unwrap_or(PaymentHistory {
                total_payments: 0,
                total_amount: 0,
                last_payment_id: 0,
            })
    }

    /// Cancel a payment request (only by requester or contract owner)
    pub fn cancel_payment_request(env: Env, payment_id: u64, caller: Address) -> Result<(), Error> {
        caller.require_auth();

        let mut payment_request: PaymentRequest = env.storage()
            .persistent()
            .get(&DataKey::PaymentRequest(payment_id))
            .unwrap_or_else(|| panic_with_error!(&env, Error::PaymentNotFound));

        // Verify caller is authorized to cancel
        let contract_owner: Address = env.storage()
            .instance()
            .get(&DataKey::ContractOwner)
            .unwrap_or_else(|| panic_with_error!(&env, Error::ContractNotInitialized));

        if caller != payment_request.requester && caller != contract_owner {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        // Verify payment can be cancelled
        match payment_request.status {
            PaymentStatus::Pending => {},
            PaymentStatus::Completed => panic_with_error!(&env, Error::PaymentAlreadyCompleted),
            _ => panic_with_error!(&env, Error::PaymentNotFound),
        }

        // Update payment status
        payment_request.status = PaymentStatus::Cancelled;
        env.storage().persistent().set(&DataKey::PaymentRequest(payment_id), &payment_request);

        log!(&env, "Payment request {} cancelled", payment_id);
        Ok(())
    }

    /// Update business status (activate/deactivate)
    pub fn update_business_status(
        env: Env,
        business_name: String,
        is_active: bool,
        caller: Address,
    ) -> Result<(), Error> {
        caller.require_auth();

        let mut business_config: BusinessConfig = env.storage()
            .persistent()
            .get(&DataKey::BusinessConfig(business_name.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, Error::BusinessNotActive));

        // Verify caller is business owner or contract owner
        let contract_owner: Address = env.storage()
            .instance()
            .get(&DataKey::ContractOwner)
            .unwrap_or_else(|| panic_with_error!(&env, Error::ContractNotInitialized));

        if caller != business_config.owner && caller != contract_owner {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        business_config.is_active = is_active;
        env.storage().persistent().set(&DataKey::BusinessConfig(business_name), &business_config);

        Ok(())
    }

    // Private helper function to update payment history
    fn update_payment_history(env: &Env, payer: &Address, payment_id: u64, amount: i128) {
        let mut history = env.storage()
            .persistent()
            .get(&DataKey::PaymentHistory(payer.clone()))
            .unwrap_or(PaymentHistory {
                total_payments: 0,
                total_amount: 0,
                last_payment_id: 0,
            });

        history.total_payments += 1;
        history.total_amount += amount;
        history.last_payment_id = payment_id;

        env.storage().persistent().set(&DataKey::PaymentHistory(payer.clone()), &history);
    }
}