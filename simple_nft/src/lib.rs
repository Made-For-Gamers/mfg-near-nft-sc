use near_sdk::PanicOnDefault;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{collections::UnorderedMap, near_bindgen, AccountId, Promise, env};
use serde::{Deserialize, Serialize};
 
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct metadata {
    pub title: String,
    pub description: String,
    pub media: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SimpleNFT {
    nfts: UnorderedMap<String, AccountId>,
    metadata: UnorderedMap<String, metadata>,
    owner_id: AccountId,
}
 
#[near_bindgen]
impl SimpleNFT {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            nfts: UnorderedMap::new(b"nfts".to_vec()),
            metadata: UnorderedMap::new(b"metadata".to_vec()),
            owner_id,
        }
    }

    pub fn mint_nft(&mut self, token_id: String, receiver_id: AccountId, metadata: metadata) {
        assert_eq!(&self.owner_id, &env::predecessor_account_id(), "Only the owner can mint tokens");
        self.nfts.insert(&token_id, &receiver_id);
        self.metadata.insert(&token_id, &metadata);
    }

    pub fn transfer_nft(&mut self, token_id: String, receiver_id: AccountId) {
        assert_eq!(&self.owner_id, &env::predecessor_account_id(), "Only the owner can transfer tokens");
        let owner = self.nfts.get(&token_id).expect("NFT not found");
        assert_eq!(
            owner,
            near_sdk::env::predecessor_account_id(),
            "Caller is not the owner of the NFT"
        );
        Promise::new(receiver_id.clone()).transfer(near_sdk::env::attached_deposit());
        self.nfts.insert(&token_id, &receiver_id);
    }

    pub fn get_nft(&self, token_id: String) -> Option<(AccountId, metadata)> {
        let owner = self.nfts.get(&token_id)?;
        let metadata = self.metadata.get(&token_id)?;
        Some((owner, metadata))
    }
}
