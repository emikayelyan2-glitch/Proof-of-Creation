pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("C9Sio3xr6Qii65iAYU25VPsmASQ7LsamJKnrFD3Fmp73");

#[program]
pub mod proof_of_creation_chain {
    use super::*;

    pub fn create_proof(
        ctx: Context<CreateProof>,
        file_hash: [u8; 32],
        metadata_uri: String,
    ) -> Result<()> {
        crate::instructions::handle_create_proof(
            ctx,
            file_hash,
            metadata_uri,
        )
    }
}
