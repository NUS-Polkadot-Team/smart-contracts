#![cfg_attr(not(feature = "std"), no_std)]

mod types;

use ink_lang as ink;

#[ink::contract]
mod bountify {

    use crate::types::*;
    use ink_prelude::vec::Vec;
    use ink_storage::{traits::SpreadAllocate, Mapping};

    #[ink(event)]
    pub struct Bid {
        #[ink(topic)]
        id: u32,

        #[ink(topic)]
        bidder: AccountId,

        bid_amount: Option<u128>,

        bid_data: IpfsHash,
    }

    #[ink(event)]
    pub struct Settle {
        #[ink(topic)]
        id: u32,

        #[ink(topic)]
        asker: AccountId,

        #[ink(topic)]
        bidder: AccountId,

        amount: u128,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate, Default)]
    pub struct Bountify {
        active_bounties: Mapping<u32, Bounty>,
        bounty_counter: u32,
        bounty_lookup_table: Vec<u32>,
    }

    impl Bountify {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        #[ink(message, payable)]
        pub fn create_bounty(&mut self, content: IpfsHash, amount: Option<u128>) {
            let id = self.get_and_increment_bounty_counter();
            self.active_bounties.insert(
                id,
                &Bounty {
                    id,
                    ask: AskBid {
                        doer: self.env().caller(),
                        data: content,
                        amount,
                    },
                    current_bid: None,
                },
            );
            self.bounty_lookup_table.push(id);

            if let Some(amount) = amount {
                assert!(self.env().transferred_value() == amount);
            }
        }

        #[ink(message, payable)]
        pub fn bid_for_bounty(&mut self, id: u32, content: IpfsHash, amount: Option<u128>) {
            let mut bounty = self
                .active_bounties
                .get(id)
                .expect("No active bounty found with id. Perhaps it doesn't exist or is closed");

            if let Some(amount) = amount {
                // Bidding for an NFT sale
                assert!(bounty.ask.amount.is_none());
                assert!(
                    self.env().transferred_value() == amount,
                    "Stake the bid amount"
                );
                if let Some(previous_bid) = bounty.current_bid {
                    assert!(
                        amount > previous_bid.amount.unwrap(),
                        "Bid amount lower than previous bid"
                    );

                    // Transfer previous stake
                    self.env()
                        .transfer(previous_bid.doer, previous_bid.amount.unwrap())
                        .expect("Transfer failed");
                }

                bounty.current_bid = Some(AskBid {
                    doer: self.env().caller(),
                    data: content.clone(),
                    amount: Some(amount),
                });
                self.active_bounties.insert(id, &bounty);
            } else {
                // Bidding to meet the requirements of an NFT
                assert!(bounty.ask.amount.is_some());

                bounty.current_bid = Some(AskBid {
                    doer: self.env().caller(),
                    data: content.clone(),
                    amount: None,
                });
                self.active_bounties.insert(id, &bounty);
            }

            self.env().emit_event(Bid {
                id,
                bidder: self.env().caller(),
                bid_amount: amount,
                bid_data: content,
            });
        }

        #[ink(message, payable)]
        pub fn settle(&mut self, id: u32) {
            let bounty = self
                .active_bounties
                .get(id)
                .expect("No active bounty found with id. Perhaps it doesn't exist or is closed");

            assert!(
                bounty.ask.doer == self.env().caller(),
                "Only caller can settle the bounty"
            );

            if let Some(chosen_bid) = bounty.current_bid {
                let amount = bounty.ask.amount.unwrap_or(chosen_bid.amount.unwrap());
                let receiver = bounty
                    .ask
                    .amount
                    .map(|_| chosen_bid.doer)
                    .unwrap_or(bounty.ask.doer);

                self.env()
                    .transfer(receiver, amount)
                    .expect("Transfer failed");

                self.env().emit_event(Settle {
                    id,
                    asker: self.env().caller(),
                    bidder: chosen_bid.doer,
                    amount,
                });
            }

            self.active_bounties.remove(id);
            for i in 0..self.bounty_lookup_table.len() {
                if self.bounty_lookup_table[i] == id {
                    self.bounty_lookup_table
                        .pop()
                        .map(|x| self.bounty_lookup_table[i] = x);
                    break;
                }
            }
        }

        #[ink(message, payable)]
        pub fn force_set_current_bid(&mut self, id: u32, bidder: AccountId, bid_data: IpfsHash) {
            let mut bounty = self
                .active_bounties
                .get(id)
                .expect("No active bounty found with id. Perhaps it doesn't exist or is closed");

            assert!(
                bounty.ask.doer == self.env().caller(),
                "Only caller can settle the bounty"
            );
            assert!(bounty.ask.amount.is_some());

            bounty.current_bid = Some(AskBid {
                doer: bidder,
                data: bid_data,
                amount: None,
            });
            self.active_bounties.insert(id, &bounty);
        }

        #[ink(message)]
        pub fn get_bounties(&self) -> Vec<Bounty> {
            let mut bounties = Vec::with_capacity(self.bounty_lookup_table.len());
            for bounty_id in &self.bounty_lookup_table {
                bounties.push(self.active_bounties.get(bounty_id).unwrap());
            }

            return bounties;
        }

        #[ink(message)]
        pub fn get_bounties_for_user(&self, account_id: AccountId) -> Vec<Bounty> {
            self.bounty_lookup_table
                .iter()
                .map(|bounty_id| self.active_bounties.get(bounty_id).unwrap())
                .filter(|bounty| bounty.ask.doer == account_id)
                .collect()
        }

        fn get_and_increment_bounty_counter(&mut self) -> u32 {
            let id = self.bounty_counter;
            self.bounty_counter += 1;
            id
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn bounty_counter_works() {
            let mut bountify = Bountify::default();
            assert_eq!(bountify.get_and_increment_bounty_counter(), 0);
            assert_eq!(bountify.get_and_increment_bounty_counter(), 1);
            assert_eq!(bountify.get_and_increment_bounty_counter(), 2);
        }
    }
}
