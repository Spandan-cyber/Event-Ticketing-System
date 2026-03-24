#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, log};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    TicketPrice,
    TotalSupply,
    TicketsSold,
    TokenAddress,
}

#[contract]
pub struct EventTicketing;

#[contractimpl]
impl EventTicketing {
    // Initialize the event details
    pub fn initialize(env: Env, admin: Address, token: Address, price: i128, supply: u32) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenAddress, &token);
        env.storage().instance().set(&DataKey::TicketPrice, &price);
        env.storage().instance().set(&DataKey::TotalSupply, &supply);
        env.storage().instance().set(&DataKey::TicketsSold, &0u32);
    }

    // Purchase a ticket
    pub fn buy_ticket(env: Env, buyer: Address) {
        buyer.require_auth();

        let price: i128 = env.storage().instance().get(&DataKey::TicketPrice).unwrap();
        let supply: u32 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        let mut sold: u32 = env.storage().instance().get(&DataKey::TicketsSold).unwrap();
        let token_addr: Address = env.storage().instance().get(&DataKey::TokenAddress).unwrap();

        if sold >= supply {
            panic!("Sold out!");
        }

        // Create a client for the token (e.g., XLM or USDC) and transfer funds to the contract
        let token_client = soroban_sdk::token::Client::new(&env, &token_addr);
        token_client.transfer(&buyer, &env.current_contract_address(), &price);

        // Update sold count
        sold += 1;
        env.storage().instance().set(&DataKey::TicketsSold, &sold);

        log!(&env, "Ticket purchased by: {}", buyer);
    }

    // Check remaining tickets
    pub fn get_remaining(env: Env) -> u32 {
        let supply: u32 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        let sold: u32 = env.storage().instance().get(&DataKey::TicketsSold).unwrap_or(0);
        supply - sold
    }
}