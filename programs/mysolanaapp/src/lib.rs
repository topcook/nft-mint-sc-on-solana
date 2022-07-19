use anchor_lang::prelude::*;
pub mod mint;

use mint::*;

declare_id!("8EYBEPrSifDH5widDTynxRqWqD6guqXFXSsDK4LxTYCm");

#[program]
pub mod mysolanaapp {
    use super::*;

    pub fn mint(
        ctx: Context<MintNft>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String
    ) -> Result<()> {
        mint::mint(ctx, metadata_title, metadata_symbol, metadata_uri)
    }
}
