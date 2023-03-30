use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{collections::UnorderedMap, near_bindgen, AccountId, Promise};
use serde::{Deserialize, Serialize};
 
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTMetadata {
    pub title: String,
    pub description: String,
    pub uri: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleNFT {
    nfts: UnorderedMap<String, AccountId>,
    metadata: UnorderedMap<String, NFTMetadata>,
    owner_id: AccountId,
}

impl Default for SimpleNFT {
    fn default() -> Self {
        panic!("SimpleNFT should be initialized before usage")
    }
}

#[near_bindgen]
impl SimpleNFT {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            nfts: UnorderedMap::new(b"nfts".to_vec()),
            metadata: UnorderedMap::new(b"metadata".to_vec()),
            owner_id,
        }
    }

    pub fn mint_nft(
        &mut self,
        token_id: String,
        receiver_id: AccountId,
        nft_metadata: NFTMetadata,
    ) {
        assert_eq!(self.owner_id, near_sdk::env::predecessor_account_id());
        self.nfts.insert(&token_id, &receiver_id);
        self.metadata.insert(&token_id, &nft_metadata);
    }

    pub fn transfer_nft(&mut self, token_id: String, receiver_id: AccountId) {
        assert_eq!(self.owner_id, near_sdk::env::predecessor_account_id());
        let owner = self.nfts.get(&token_id).expect("NFT not found");
        assert_eq!(
            owner,
            near_sdk::env::predecessor_account_id(),
            "Caller is not the owner of the NFT"
        );
        Promise::new(receiver_id.clone()).transfer(near_sdk::env::attached_deposit());
        self.nfts.insert(&token_id, &receiver_id);
    }

    pub fn get_nft(&self, token_id: String) -> Option<(AccountId, NFTMetadata)> {
        let owner = self.nfts.get(&token_id)?;
        let metadata = self.metadata.get(&token_id)?;
        Some((owner, metadata))
    }
}
