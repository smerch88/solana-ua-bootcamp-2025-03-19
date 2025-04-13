use anchor_lang::prelude::*;

declare_id!("BhD2HqdX5nvtuZSajFZUDcf3u6rNVN6BK2g6P1uoA3AS");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
