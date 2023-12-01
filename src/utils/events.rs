use near_sdk_contract_tools_macros::event;

#[event(
    crate = "crate",
    macros = "crate",
    standard = "infernet-sdk",
    version = "1.0.0"
)]
#[derive(Debug, Clone)]
pub enum InfernetEvent {
    /// Token mint event. Emitted when tokens are created and total_supply is
    /// increased.
    FtMint(Vec<FtMintData>),

    /// Token transfer event. Emitted when tokens are transferred between two
    /// accounts. No change to total_supply.
    FtTransfer(Vec<FtTransferData>),

    /// Token burn event. Emitted when tokens are burned (removed from supply).
    /// Decrease in total_supply.
    FtBurn(Vec<FtBurnData>),
}
use near_sdk::{json_types::U128, serde::Serialize, AccountId};


// TODO 
/*
   /// @notice Emitted when a new subscription is created
    /// @param id subscription ID
    event SubscriptionCreated(uint32 indexed id);

    /// @notice Emitted when a subscription is cancelled
    /// @param id subscription ID
    event SubscriptionCancelled(uint32 indexed id);

    /// @notice Emitted when a subscription is fulfilled
    /// @param id subscription ID
    /// @param node address of fulfilling node
    event SubscriptionFulfilled(uint32 indexed id, address indexed node);
*/