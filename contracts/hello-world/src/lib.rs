#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, symbol_short};

// Structure to represent an NFT
#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub id: u64,
    pub owner: Address,
    pub name: String,
    pub uri: String,
}

// Storage keys
#[contracttype]
pub enum DataKey {
    NFT(u64),
    Counter,
}

#[contract]
pub struct NFTGalleryContract;

#[contractimpl]
impl NFTGalleryContract {

    // Mint a new NFT
    pub fn mint(env: Env, owner: Address, name: String, uri: String) -> u64 {
        owner.require_auth();

        let mut counter: u64 = env.storage().instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        let nft = NFT {
            id: counter,
            owner: owner.clone(),
            name,
            uri,
        };

        env.storage().instance().set(&DataKey::NFT(counter), &nft);
        env.storage().instance().set(&DataKey::Counter, &counter);

        counter
    }

    // Get NFT details
    pub fn get_nft(env: Env, id: u64) -> NFT {
        env.storage().instance()
            .get(&DataKey::NFT(id))
            .unwrap()
    }

    // Transfer NFT ownership
    pub fn transfer(env: Env, from: Address, to: Address, id: u64) {
        from.require_auth();

        let mut nft: NFT = env.storage().instance()
            .get(&DataKey::NFT(id))
            .unwrap();

        if nft.owner != from {
            panic!("Not the owner");
        }

        nft.owner = to;

        env.storage().instance().set(&DataKey::NFT(id), &nft);
    }
}