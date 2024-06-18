use anchor_lang::prelude::*;

declare_id!("5RDmTmTJCd2WdsNXHQhydtinDXF18RrdKmDANjmEqKYN");

#[program]
pub mod mint_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
