use anchor_lang::prelude::*;

declare_id!("DujYufgNvyey7ttDcf6a9uVQbG7nTtVWVKgLn1rZ1MZf");

#[program]
pub mod anchor_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
