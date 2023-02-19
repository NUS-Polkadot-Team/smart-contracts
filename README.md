# BountiFi Contracts

Built with ink! smart contracts.

## Getting Started

To get started, first build with `cargo`

```bash
cargo +nightly contract build --release --optimization-passes=0
```

## Interface

`createBounty`: Create a new bounty from the user

- `content`: IPFS hash of the data, which contains a message, which can be either a commissioner "Someone draw me an nft with a particular description" or an artist saying "I can draw an nft for you".
- `amount`: is optional and if given, it refers to the amount of tokens given as a bounty reward

`bidForBounty`: Bid for a bounty

- `id`: refers to bounty id, which increments normally.
- `content`: IPFS hash of data, which is just a string, the actual format is a pointer to valid JSON. Blank if it's someone just bidding extra amount.
- `amount`: has to be more than previous amount. Bids with amount only make sense on asks where the artist is the one who is asking. Bids without amount is where user asks and artists bid. If there's a bid with amount, amount should be provided as stake, and if a newer bid displaces the current bid, stake is returned back.

`settle`: Settle is called by the asker to finish the bid cycle and award it to whoever is the current top bidder. In the case where it's a bunch of artists competing for a bid, the asker must call `force_set_current_bid` to his choice before calling settle so that his funds go to the right bidder. A `PSP34` NFT is minted on settlement.

`force_set_current_bid` can only be called by the asker and only where artists are competing.

## Deployment details

- Contract address: `YU7uQgPaeTvdmBDP97FxPn2kYbaWBtUY5LPJETWX9HA1Mzf`
- Network: Shibuya Testnet
