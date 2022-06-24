use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer };

mod states;
pub use states::*;

use test_tokens::cpi::accounts::MintToken;
use test_tokens::{self};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const PREFIX: &str = "apricot0";

// Reward distribution duration
const DAY_IN_SECONDS: i64 = 86400;

const DENOMINATOR: u64 = 10000000;

#[program]
pub mod apricot {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        let state_account = &mut ctx.accounts.state_account;
        let config = &mut ctx.accounts.config;

        state_account.owner = ctx.accounts.authority.key();


        config.wsol_mint = ctx.accounts.wsol_mint.key();
        config.msol_mint = ctx.accounts.msol_mint.key();
        config.srm_mint = ctx.accounts.srm_mint.key();
        config.scnsol_mint = ctx.accounts.scnsol_mint.key();
        config.stsol_mint = ctx.accounts.stsol_mint.key();
        config.ray_mint = ctx.accounts.ray_mint.key();

        config.wsol_rate = DENOMINATOR;
        config.msol_rate = DENOMINATOR;
        config.srm_rate = DENOMINATOR;
        config.scnsol_rate = DENOMINATOR;
        config.stsol_rate = DENOMINATOR;
        config.ray_rate = DENOMINATOR;

        config.state_account = ctx.accounts.state_account.key();

        Ok(())
    }

    // Init user account
    pub fn init_user_account(
        ctx: Context<InitUserAccount>
    ) -> Result<()> {
        // Make as 1 string for pubkey
        let user_account = &mut ctx.accounts.user_account;
        user_account.owner = ctx.accounts.user.key();

        user_account.wsol_amount = 0;
        user_account.msol_amount = 0;
        user_account.srm_amount = 0;
        user_account.scnsol_amount = 0;
        user_account.stsol_amount = 0;
        user_account.ray_amount = 0;

        Ok(())
    }

    pub fn deposit_token(
        ctx: Context<DepositToken>,
        amount: u64
    ) -> Result<()> {

        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        let user_account = &mut ctx.accounts.user_account;
        let config = &mut ctx.accounts.config;

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token.to_account_info(),
            to: ctx.accounts.pool_token.to_account_info(),
            authority: ctx.accounts.authority.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        if config.wsol_mint == ctx.accounts.token_mint.key() {
            let sum = user_account.wsol_amount + amount * DENOMINATOR / config.wsol_rate;
            
            user_account.wsol_amount = sum;
            config.wsol_amount = config.wsol_amount + amount;
        } else if config.msol_mint == ctx.accounts.token_mint.key() {
            let sum = user_account.msol_amount + amount * DENOMINATOR / config.msol_rate;
            
            user_account.msol_amount = sum;
            config.msol_amount = config.msol_amount + amount;
        } else if config.srm_mint == ctx.accounts.token_mint.key() {
            let sum = user_account.srm_amount + amount * DENOMINATOR / config.srm_rate;
            
            user_account.srm_amount = sum;
            config.srm_amount = config.srm_amount + amount;
        } else if config.scnsol_mint == ctx.accounts.token_mint.key() {
            let sum = user_account.scnsol_amount + amount * DENOMINATOR / config.scnsol_rate;
            
            user_account.scnsol_amount = sum;
            config.scnsol_amount = config.scnsol_amount + amount;
        } else if config.stsol_mint == ctx.accounts.token_mint.key() {
            let sum = user_account.stsol_amount + amount * DENOMINATOR / config.stsol_rate;
            
            user_account.stsol_amount = sum;
            config.stsol_amount = config.stsol_amount + amount;
        } else if config.ray_mint == ctx.accounts.token_mint.key() {
            let sum = user_account.ray_amount + amount * DENOMINATOR / config.ray_rate;
            
            user_account.ray_amount = sum;
            config.ray_amount = config.ray_amount + amount;
        }

        Ok(())
    }

    pub fn withdraw_token(
        ctx: Context<WithdrawToken>,
        amount: u64
    ) -> Result<()> {
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        
        let user_account = &mut ctx.accounts.user_account;
        let config = &mut ctx.accounts.config;

        if config.wsol_mint == ctx.accounts.token_mint.key() {
            let withdrawable_amount = (user_account.wsol_amount as u128 * config.wsol_rate  as u128/ DENOMINATOR as u128) as u64;
        
            if amount > withdrawable_amount {
                return Err(ErrorCode::ExceedAmount.into());
            }

            let remain_amount = ((withdrawable_amount - amount)  as u128 * DENOMINATOR  as u128 / config.wsol_rate  as u128) as u64;
            config.wsol_amount = config.wsol_amount - amount;
            user_account.wsol_amount = remain_amount;
        } else if config.msol_mint == ctx.accounts.token_mint.key() {
            
            let withdrawable_amount = (user_account.msol_amount  as u128 * config.msol_rate  as u128 / DENOMINATOR as u128) as u64;
        
            if amount > withdrawable_amount {
                return Err(ErrorCode::ExceedAmount.into());
            }

            let remain_amount = ((withdrawable_amount - amount) as u128 * DENOMINATOR as u128 / config.msol_rate as u128) as u64;
            config.msol_amount = config.msol_amount - amount;
            user_account.msol_amount = remain_amount;
        } else if config.srm_mint == ctx.accounts.token_mint.key() {
            
            let withdrawable_amount = (user_account.srm_amount as u128 * config.srm_rate as u128 / DENOMINATOR as u128) as u64;
        
            if amount > withdrawable_amount {
                return Err(ErrorCode::ExceedAmount.into());
            }

            let remain_amount = ((withdrawable_amount - amount) as u128 * DENOMINATOR as u128 / config.srm_rate as u128) as u64;
            config.srm_amount = config.srm_amount - amount;
            user_account.srm_amount = remain_amount;
        } else if config.scnsol_mint == ctx.accounts.token_mint.key() {
            
            let withdrawable_amount = (user_account.scnsol_amount as u128 * config.scnsol_rate as u128 / DENOMINATOR as u128) as u64;
        
            if amount > withdrawable_amount {
                return Err(ErrorCode::ExceedAmount.into());
            }

            let remain_amount = ((withdrawable_amount - amount) as u128 * DENOMINATOR as u128 / config.scnsol_rate as u128) as u64;
            config.scnsol_amount = config.scnsol_amount - amount;
            user_account.scnsol_amount = remain_amount;
        } else if config.stsol_mint == ctx.accounts.token_mint.key() {
            
            let withdrawable_amount = (user_account.stsol_amount as u128 * config.stsol_rate as u128 / DENOMINATOR as u128 ) as u64;
        
            if amount > withdrawable_amount {
                return Err(ErrorCode::ExceedAmount.into());
            }

            let remain_amount = ((withdrawable_amount - amount) as u128 * DENOMINATOR as u128 / config.stsol_rate as u128) as u64;
            config.stsol_amount = config.stsol_amount - amount;
            user_account.stsol_amount = remain_amount;
        } else if config.ray_mint == ctx.accounts.token_mint.key() {
            
            let withdrawable_amount = (user_account.ray_amount as u128 * config.ray_rate as u128 / DENOMINATOR as u128) as u64;
        
            if amount > withdrawable_amount {
                return Err(ErrorCode::ExceedAmount.into());
            }

            let remain_amount = ((withdrawable_amount - amount) as u128 * DENOMINATOR as u128 / config.ray_rate as u128) as u64;
            config.ray_amount = config.ray_amount - amount;
            user_account.ray_amount = remain_amount;
        } else {
            return Err(ErrorCode::InvalidToken.into())
        }
        
        let (token_authority, token_authority_bump) = 
            Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
        
        if token_authority != ctx.accounts.state_account.to_account_info().key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[token_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.state_account.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn daily_reward(
        ctx: Context<DailyReward>,
        rate: u64
    ) -> Result<()> {

        if ctx.accounts.state_account.second_owner != ctx.accounts.second_owner.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        if rate < DENOMINATOR {
            return Err(ErrorCode::InvalidAmount.into());
        }

        let config = &mut ctx.accounts.config;

        let clock = Clock::get()?; // Returns real-world time in second uint
        let dur_seconds = clock.unix_timestamp  - config.last_mint_timestamp ;
        if dur_seconds < DAY_IN_SECONDS {
            return Err(ErrorCode::TooOftenMint.into());
        }

        let mut mint_amount = 0;
        if config.wsol_mint == ctx.accounts.token_mint.key() {
            let reward_amount = config.wsol_amount * (rate - DENOMINATOR) / DENOMINATOR;

            let rate_will = config.wsol_rate * rate  / DENOMINATOR;

            config.wsol_amount = config.wsol_amount + reward_amount;
            config.wsol_rate = rate_will;
            mint_amount = reward_amount;
        } else if config.msol_mint == ctx.accounts.token_mint.key() {
            let reward_amount = config.msol_amount * (rate - DENOMINATOR) / DENOMINATOR;

            let rate_will = config.msol_rate * rate  / DENOMINATOR;

            config.msol_amount = config.msol_amount + reward_amount;
            config.msol_rate = rate_will;
            mint_amount = reward_amount;
        } else if config.srm_mint == ctx.accounts.token_mint.key() {
            let reward_amount = config.srm_amount * (rate - DENOMINATOR) / DENOMINATOR;

            let rate_will = config.srm_rate * rate  / DENOMINATOR;

            config.srm_amount = config.srm_amount + reward_amount;
            config.srm_rate = rate_will;
            mint_amount = reward_amount;
        } else if config.scnsol_mint == ctx.accounts.token_mint.key() {
            let reward_amount = config.scnsol_amount * (rate - DENOMINATOR) / DENOMINATOR;

            let rate_will = config.scnsol_rate * rate  / DENOMINATOR;

            config.scnsol_amount = config.scnsol_amount + reward_amount;
            config.scnsol_rate = rate_will;
            mint_amount = reward_amount;
        } else if config.stsol_mint == ctx.accounts.token_mint.key() {
            let reward_amount = config.stsol_amount * (rate - DENOMINATOR) / DENOMINATOR;

            let rate_will = config.stsol_rate * rate  / DENOMINATOR;

            config.stsol_amount = config.stsol_amount + reward_amount;
            config.stsol_rate = rate_will;
            mint_amount = reward_amount;
        } else if config.ray_mint == ctx.accounts.token_mint.key() {
            let reward_amount = config.ray_amount * (rate - DENOMINATOR) / DENOMINATOR;

            let rate_will = config.ray_rate * rate  / DENOMINATOR;

            config.ray_amount = config.ray_amount + reward_amount;
            config.ray_rate = rate_will;
            mint_amount = reward_amount;
        } else {
            return Err(ErrorCode::InvalidToken.into())
        }
        
        // MINT TOkENS
        let cpi_program = ctx.accounts.lending_program.to_account_info();
        let cpi_accounts = MintToken {
            owner: ctx.accounts.state_account.to_account_info(),
            state_account: ctx.accounts.token_state.to_account_info(),
            user_token: ctx.accounts.pool_token.to_account_info(),
            token_mint: ctx.accounts.token_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        test_tokens::cpi::mint_token(cpi_ctx, mint_amount)?;
        // END MINT
        
        config.last_mint_timestamp = clock.unix_timestamp;

        Ok(())
    }

    pub fn update_owner(
        ctx: Context<UpdateConfigAccount>,
        new_owner: Pubkey
    ) -> Result<()> {
        let state_account = &mut ctx.accounts.state_account;
        if state_account.owner != ctx.accounts.owner.key() || ctx.accounts.owner.key() == new_owner {
            return Err(ErrorCode::InvalidOwner.into());
        }

        state_account.owner = new_owner;

        Ok(())
    }

    pub fn update_second_owner(
        ctx: Context<UpdateConfigAccount>,
        new_owner: Pubkey
    ) -> Result<()> {
        let state_account = &mut ctx.accounts.state_account;
        if state_account.owner != ctx.accounts.owner.key() || state_account.second_owner == new_owner {
            return Err(ErrorCode::InvalidOwner.into());
        }

        state_account.second_owner = new_owner;

        Ok(())
    }

    pub fn update_rate(
        ctx: Context<UpdateConfigAccount>
    ) -> Result<()> {
        let state_account = &mut ctx.accounts.state_account;
        let config =  &mut ctx.accounts.config;
        
        if state_account.owner != ctx.accounts.owner.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        config.wsol_rate = DENOMINATOR;
        config.msol_rate = DENOMINATOR;
        config.srm_rate = DENOMINATOR;
        config.scnsol_rate = DENOMINATOR;
        config.stsol_rate = DENOMINATOR;
        config.ray_rate = DENOMINATOR;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Invalid Owner")]
    InvalidOwner,
    #[msg("Too often mint")]
    TooOftenMint,
    #[msg("Exceed Amount")]
    ExceedAmount,
    #[msg("Invalid Token")]
    InvalidToken
}