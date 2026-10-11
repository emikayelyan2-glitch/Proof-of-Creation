use anchor_lang::prelude::*;

#[account]
pub struct ProofAccount {
    pub creator: Pubkey,
    pub file_hash: [u8; 32],
    pub timestamp: i64,
    pub metadata_uri: String,
    pub bump: u8,
}

impl ProofAccount {
    pub const SPACE: usize =
        8 +      // Anchor discriminator
        32 +     // creator
        32 +     // file_hash
        8 +      // timestamp
        4 + 200 + // String length + max metadata
        1;       // bump
}
