use anchor_lang::prelude::*;

declare_id!("2WN8Y73WEBMztVNqQ6GjWFs7iynYLak7ERtYHtwYQ89y");

#[program]
pub mod practice1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        //increment logic

        ctx.accounts.initialized_account.data = 0;
        msg!(
            "Initialized with value = {}",
            ctx.accounts.initialized_account.data
        );
        Ok(())
    }
    pub fn increment_data(ctx: Context<Increment>) -> Result<()> {
        let account = &mut ctx.accounts.initialized_account;
        account.data = account.data.checked_add(1).unwrap();
        msg!("Incremented value = {}", account.data);
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let account = &mut ctx.accounts.initialized_account;
        account.data = account.data.checked_sub(1).unwrap();
        msg!("Decremented value = {}", account.data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = accountState::INIT_SPACE,
        seeds = [b"initialize_account"],
        bump
    )]
    pub initialized_account: Account<'info, accountState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]

pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"initialize_account"],
        bump
    )]
    pub initialized_account: Account<'info, accountState>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]

pub struct Decrement<'info> {
    #[account(
        mut,
        seeds = [b"initialize_account"],
        bump
    )]
    pub initialized_account: Account<'info, accountState>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct accountState {
    data: u32,
}
