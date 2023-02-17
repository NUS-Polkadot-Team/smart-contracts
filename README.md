# Bountifi Contracts

## Interface

`createBounty`: Create a new bounty from the user.
    - content: ipfshash of the data. It can be either: "Someone draw me an nft with a particular description" or an artist saying "I can draw an nft for you".
    - amount: is optional and if given, it means "Someone draw me an nft and I'll pay you <>"

`bidForBounty`: Bid for a bounty.
    - id: Id is the bounty id, bounty id just goes in increments.
    - content: is again ipfshash, ipfshash is just a string, the actual format is a pointer to valid JSON. Blank if it's someone just bidding extra amount.
    - amount: has to be more than previous amount. Bids with amount only make sense on asks where the artist is the one who is asking. Bids without amount is where user asks and artists bid. If there's a bid with amount, amount should be provided as stake, and if a newer bid displaces the current bid, stake is returned back.

`settle`: Settle is called by the asker to finish the bid cycle and award it to whoever is the current top bidder. In the case where it's a bunch of artists competing for a bid, the asker must call `force_set_current_bid` to his choice before calling settle so that his funds go to the right bidder. A `PSP34` NFT is minted on settlement.

`force_set_current_bid` can only be called by the asker and only where artists are competing.

## Deployment details

- Contract address: `YU7uQgPaeTvdmBDP97FxPn2kYbaWBtUY5LPJETWX9HA1Mzf`
- Network: Shibuya Testnet
