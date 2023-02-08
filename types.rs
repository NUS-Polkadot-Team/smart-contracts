use ink_env::AccountId;
use ink_prelude::string::String;
use ink_primitives::Key;
use ink_storage::traits::{PackedAllocate, PackedLayout, SpreadAllocate, SpreadLayout};

pub type IpfsHash = String;

/// Struct that represents an ask or a bid
#[derive(Debug, SpreadLayout, PackedLayout, SpreadAllocate, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct AskBid {
    pub doer: AccountId,
    pub data: IpfsHash,
    pub amount: Option<u128>,
}

impl PackedAllocate for AskBid {
    fn allocate_packed(&mut self, at: &Key) {
        PackedAllocate::allocate_packed(&mut self.doer, at);
        PackedAllocate::allocate_packed(&mut self.data, at);
        PackedAllocate::allocate_packed(&mut self.amount, at);
    }
}

#[derive(Debug, SpreadLayout, PackedLayout, SpreadAllocate, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct Bounty {
    pub id: u32,
    pub ask: AskBid,
    pub current_bid: Option<AskBid>,
}

impl PackedAllocate for Bounty {
    fn allocate_packed(&mut self, at: &Key) {
        PackedAllocate::allocate_packed(&mut self.id, at);
        PackedAllocate::allocate_packed(&mut self.ask, at);
        PackedAllocate::allocate_packed(&mut self.current_bid, at);
    }
}
