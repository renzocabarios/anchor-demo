use anchor_lang::prelude::*;

declare_id!("GTziRJjeQC8AGc7zYq7oLYkVYgzT2h5gVkLvNmkAuyiR");

#[program]
pub mod anchor_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
