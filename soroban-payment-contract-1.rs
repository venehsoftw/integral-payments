#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, Vec, Map, String, token
};

#[derive(Clone)]
#[contracttype]
pub struct PaymentDetails {
    pub amount: i128,
    pub sender: Address,
    pub recipient: Address,
    pub token_address: Address,
    pub business_name: String,
    pub customer_name: String,
    pub order_id: String,
}

#[derive(Clone)]
#[contracttype]
pub struct PaymentRecord {
    pub payment_id: u64,
    pub details: PaymentDetails,
    pub timestamp: u64,
    pub status: Symbol,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    PaymentCounter,
    Payment(u64),
    BusinessConfig(Address),
    AuthorizedAddresses,
}

#[derive(Clone)]
#[contracttype]
pub struct BusinessConfig {
    pub fee_rate: i128, // Fee as basis points (100 = 1%)
    pub min_amount: i128,
    pub max_amount: i128,
    pub is_active: bool,
}

#[contract]
pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {
    /// Initialize the contract with authorized addresses
    pub fn initialize(
        env: Env,
        admin: Address,
        authorized_addresses: Vec<Address>,
    ) -> Result<(), &'static str> {
        // Ensure the admin is authenticated
        admin.require_auth();
        
        // Set authorized addresses for payment processing
        env.storage().instance().set(&DataKey::AuthorizedAddresses, &authorized_addresses);
        
        // Initialize payment counter
        env.storage().instance().set(&DataKey::PaymentCounter, &0u64);
        
        Ok(())
    }

    /// Configure business settings
    pub fn configure_business(
        env: Env,
        business_address: Address,
        fee_rate: i128,
        min_amount: i128,
        max_amount: i128,
    ) -> Result<(), &'static str> {
        business_address.require_auth();
        
        let config = BusinessConfig {
            fee_rate,
            min_amount,
            max_amount,
            is_active: true,
        };
        
        env.storage().instance().set(&DataKey::BusinessConfig(business_address.clone()), &config);
        
        Ok(())
    }

    /// Process XLM payment
    pub fn process_xlm_payment(
        env: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
        business_name: String,
        customer_name: String,
        order_id: String,
    ) -> Result<u64, &'static str> {
        // Authenticate sender
        sender.require_auth();
        
        // Validate authorized addresses
        let authorized_addresses: Vec<Address> = env.storage().instance()
            .get(&DataKey::AuthorizedAddresses)
            .ok_or("Authorized addresses not set")?;
        
        if !authorized_addresses.contains(&recipient) {
            return Err("Recipient not authorized");
        }
        
        // Validate business configuration
        let business_config: BusinessConfig = env.storage().instance()
            .get(&DataKey::BusinessConfig(recipient.clone()))
            .ok_or("Business not configured")?;
        
        if !business_config.is_active {
            return Err("Business not active");
        }
        
        if amount < business_config.min_amount || amount > business_config.max_amount {
            return Err("Amount out of range");
        }
        
        // Calculate fee
        let fee = (amount * business_config.fee_rate) / 10000;
        let net_amount = amount - fee;
        
        // Transfer XLM (native asset)
        // Note: In Soroban, native XLM transfers are handled differently
        // This is a simplified representation
        
        // Create payment record
        let payment_counter: u64 = env.storage().instance()
            .get(&DataKey::PaymentCounter)
            .unwrap_or(0);
        
        let payment_id = payment_counter + 1;
        
        let payment_details = PaymentDetails {
            amount,
            sender: sender.clone(),
            recipient: recipient.clone(),
            token_address: Address::from_contract_id(&env, &env.current_contract_address()),
            business_name,
            customer_name,
            order_id,
        };
        
        let payment_record = PaymentRecord {
            payment_id,
            details: payment_details,
            timestamp: env.ledger().timestamp(),
            status: symbol_short!("COMPLETE"),
        };
        
        // Store payment record
        env.storage().instance().set(&DataKey::Payment(payment_id), &payment_record);
        env.storage().instance().set(&DataKey::PaymentCounter, &payment_id);
        
        // Emit event
        env.events().publish(
            (symbol_short!("payment"), symbol_short!("xlm")),
            (payment_id, sender, recipient, amount)
        );
        
        Ok(payment_id)
    }

    /// Process token payment (for assets like USDC on Stellar)
    pub fn process_token_payment(
        env: Env,
        sender: Address,
        recipient: Address,
        token_address: Address,
        amount: i128,
        business_name: String,
        customer_name: String,
        order_id: String,
    ) -> Result<u64, &'static str> {
        // Authenticate sender
        sender.require_auth();
        
        // Validate authorized addresses
        let authorized_addresses: Vec<Address> = env.storage().instance()
            .get(&DataKey::AuthorizedAddresses)
            .ok_or("Authorized addresses not set")?;
        
        if !authorized_addresses.contains(&recipient) {
            return Err("Recipient not authorized");
        }
        
        // Validate business configuration
        let business_config: BusinessConfig = env.storage().instance()
            .get(&DataKey::BusinessConfig(recipient.clone()))
            .ok_or("Business not configured")?;
        
        if !business_config.is_active {
            return Err("Business not active");
        }
        
        if amount < business_config.min_amount || amount > business_config.max_amount {
            return Err("Amount out of range");
        }
        
        // Get token client
        let token_client = token::Client::new(&env, &token_address);
        
        // Calculate fee
        let fee = (amount * business_config.fee_rate) / 10000;
        let net_amount = amount - fee;
        
        // Transfer tokens
        token_client.transfer(&sender, &recipient, &net_amount);
        
        // Transfer fee if applicable
        if fee > 0 {
            // Transfer fee to contract or fee collector
            token_client.transfer(&sender, &env.current_contract_address(), &fee);
        }
        
        // Create payment record
        let payment_counter: u64 = env.storage().instance()
            .get(&DataKey::PaymentCounter)
            .unwrap_or(0);
        
        let payment_id = payment_counter + 1;
        
        let payment_details = PaymentDetails {
            amount,
            sender: sender.clone(),
            recipient: recipient.clone(),
            token_address: token_address.clone(),
            business_name,
            customer_name,
            order_id,
        };
        
        let payment_record = PaymentRecord {
            payment_id,
            details: payment_details,
            timestamp: env.ledger().timestamp(),
            status: symbol_short!("COMPLETE"),
        };
        
        // Store payment record
        env.storage().instance().set(&DataKey::Payment(payment_id), &payment_record);
        env.storage().instance().set(&DataKey::PaymentCounter, &payment_id);
        
        // Emit event
        env.events().publish(
            (symbol_short!("payment"), symbol_short!("token")),
            (payment_id, sender, recipient, amount)
        );
        
        Ok(payment_id)
    }

    /// Get payment details
    pub fn get_payment(env: Env, payment_id: u64) -> Option<PaymentRecord> {
        env.storage().instance().get(&DataKey::Payment(payment_id))
    }

    /// Get business configuration
    pub fn get_business_config(env: Env, business_address: Address) -> Option<BusinessConfig> {
        env.storage().instance().get(&DataKey::BusinessConfig(business_address))
    }

    /// Get authorized addresses
    pub fn get_authorized_addresses(env: Env) -> Option<Vec<Address>> {
        env.storage().instance().get(&DataKey::AuthorizedAddresses)
    }

    /// Update business status
    pub fn update_business_status(
        env: Env,
        business_address: Address,
        is_active: bool,
    ) -> Result<(), &'static str> {
        business_address.require_auth();
        
        let mut config: BusinessConfig = env.storage().instance()
            .get(&DataKey::BusinessConfig(business_address.clone()))
            .ok_or("Business not configured")?;
        
        config.is_active = is_active;
        
        env.storage().instance().set(&DataKey::BusinessConfig(business_address), &config);
        
        Ok(())
    }

    /// Get payment counter
    pub fn get_payment_counter(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::PaymentCounter).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_initialize_contract() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PaymentContract);
        let client = PaymentContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let authorized_addresses = vec![&env, Address::generate(&env), Address::generate(&env)];
        
        client.initialize(&admin, &authorized_addresses);
        
        let retrieved_addresses = client.get_authorized_addresses();
        assert_eq!(retrieved_addresses.unwrap().len(), 2);
    }
}