use anchor_lang::prelude::*;

use crate::constants::{MAX_METADATA_URI, PROOF_SEED};
use crate::error::ErrorCode;
use crate::state::ProofAccount;

pub fn handle_create_proof(
    ctx: Context<CreateProof>,
    file_hash: [u8; 32],
    metadata_uri: String,
) -> Result<()> {
    require!(
        metadata_uri.as_bytes().len() <= MAX_METADATA_URI,
        ErrorCode::MetadataTooLong
    );

    let proof = &mut ctx.accounts.proof;

    proof.creator = ctx.accounts.creator.key();
    proof.file_hash = file_hash;
    proof.timestamp = Clock::get()?.unix_timestamp;
    proof.metadata_uri = metadata_uri;
    proof.bump = ctx.bumps.proof;

    msg!("Proof created: {}", proof.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(file_hash: [u8; 32])]
pub struct CreateProof<'info> {
    #[account(
        init,
        payer = creator,
        space = ProofAccount::SPACE,
        seeds = [
            PROOF_SEED,
            creator.key().as_ref(),
            file_hash.as_ref()
        ],
        bump
    )]
    pub proof: Account<'info, ProofAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}
