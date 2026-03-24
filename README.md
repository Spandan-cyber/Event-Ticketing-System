# Stellar Event Ticketing System (Soroban)

A decentralized smart contract built on **Stellar Soroban** to manage event ticket sales securely and transparently.

## Project Description
The **Event Ticketing System** is a Rust-based smart contract designed to handle the primary sale of event tickets. By leveraging Stellar's low-cost and high-speed network, it ensures that ticket issuance is immutable and verifiable, preventing double-selling and manual bookkeeping errors.

## What it Does
This contract acts as an automated "Box Office." 
1. **Event Setup:** An administrator initializes the contract with a specific ticket price and a maximum supply.
2. **Secure Payments:** It integrates with Stellar assets (like XLM or USDC) to handle payments directly within the contract.
3. **Inventory Management:** It tracks exactly how many tickets have been sold and automatically stops sales once the event is sold out.

## Features
* **Permissioned Initialization:** Only the contract creator can set event parameters.
* **Atomic Transactions:** Ticket purchase and payment happen in a single, unbreakable step.
* **Real-time Inventory:** Public functions to check ticket availability instantly.
* **Auth Verification:** Uses Soroban's `require_auth` to ensure only the actual wallet owner can purchase tickets.

## Deployed Smart Contract Link
**Network:** Testnet  
**Contract ID:** `CD... (Replace with your deployed ID)`  
**Stellar Expert Link:** [https://stellar.expert/explorer/testnet/contract/XXX](https://stellar.expert/explorer/testnet/contract/XXX)
