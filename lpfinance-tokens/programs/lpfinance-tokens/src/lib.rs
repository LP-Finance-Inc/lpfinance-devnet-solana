use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount }
};
use std::mem::size_of;
use std::str::FromStr;

declare_id!("Cdk1hTsGs375ua7xC66g1dPQwnnk1xesXKmxH8hHVLig");

// PROTOCOL
pub const CBS_PDA: &str = "9SYSA3RPEakev2i9GNDBLD5NGNELnVYKchWkWRrK1J6B";

const LP_TOKEN_DECIMALS: u8 = 9;
const PREFIX: &str = "lptokens";

// DAO governance token
const INITIAL_SUPPLY: u64 = 25000000; // 25,000,000 = 25M

const DAY_IN_SECONDS: i64 = 86400; 
// Reward Rate => 0.00809%
// so need to divide with 10000
const DAILY_REWARD_RATE: u64 = 10000809;
const DENOMINATOR: u64 =       10000000;

#[program]
pub mod lpfinance_tokens {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>
    ) -> Result<()> {
        msg!("INITIALIZE TOKEN PROGRAM");

        let state_account = &mut ctx.accounts.state_account;
        let config = &mut ctx.accounts.config;

        state_account.owner = ctx.accounts.authority.key();

        config.lpsol_mint = ctx.accounts.lpsol_mint.key();
        config.lpusd_mint = ctx.accounts.lpusd_mint.key();
        config.lpdao_mint = ctx.accounts.lpdao_mint.key();
        config.state_account = ctx.accounts.state_account.key();
        config.last_mint_timestamp = 0;

        // INITIAL SUPPLY
        let (mint_token_authority, mint_token_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if mint_token_authority != ctx.accounts.state_account.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // Mint
        let seeds = &[
            PREFIX.as_bytes(),
            &[mint_token_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.lpdao_mint.to_account_info(),
            to: ctx.accounts.user_daotoken.to_account_info(),
            authority: ctx.accounts.state_account.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, INITIAL_SUPPLY * 1000000000)?;

        Ok(())
    }
    // Let CBS to mint tokens
    pub fn mint_lptoken(
        ctx: Context<MintLpToken>,
        amount: u64
    ) -> Result<()> {
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        let cbs_pubkey = Pubkey::from_str(CBS_PDA).unwrap();
        if cbs_pubkey != ctx.accounts.signer.key()
        {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let (mint_token_authority, mint_token_authority_bump) = 
            Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
        
        if mint_token_authority != ctx.accounts.state_account.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // Mint
        let seeds = &[
            PREFIX.as_bytes(),
            &[mint_token_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.lptoken_mint.to_account_info(),
            to: ctx.accounts.user_lptoken.to_account_info(),
            authority: ctx.accounts.state_account.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    // Owner can mint token
    pub fn owner_mint_lptoken(
        ctx: Context<OwnerLpToken>,
        amount: u64
    ) -> Result<()> {
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        let (mint_token_authority, mint_token_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if mint_token_authority != ctx.accounts.state_account.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // Mint
        let seeds = &[
            PREFIX.as_bytes(),
            &[mint_token_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.lptoken_mint.to_account_info(),
            to: ctx.accounts.user_lptoken.to_account_info(),
            authority: ctx.accounts.state_account.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, amount)?;
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

    // DAIL Mint DAO Token
    pub fn mint_dao_lptoken (
        ctx: Context<MintDaoLpToken>
    ) -> Result<()> {
        let total_supply = ctx.accounts.lptoken_mint.supply;
        let config = &mut ctx.accounts.config;

        if total_supply == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        if ctx.accounts.state_account.second_owner != ctx.accounts.owner.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let clock = Clock::get()?; // Returns real-world time in second uint
        let dur_seconds = clock.unix_timestamp  - config.last_mint_timestamp ;
        if dur_seconds < DAY_IN_SECONDS {
            return Err(ErrorCode::TooOftenMint.into());
        }
        config.last_mint_timestamp = clock.unix_timestamp;

        let mint_amount = total_supply * (DAILY_REWARD_RATE - DENOMINATOR) / DENOMINATOR;


        let (mint_token_authority, mint_token_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if mint_token_authority != ctx.accounts.state_account.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // Mint
        let seeds = &[
            PREFIX.as_bytes(),
            &[mint_token_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.lptoken_mint.to_account_info(),
            to: ctx.accounts.user_lptoken.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, mint_amount)?;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    // Token program owner
    #[account(mut)]
    pub authority: Signer<'info>,
    // State Accounts
    #[account(init,
        seeds = [PREFIX.as_bytes()],
        bump,
        space = size_of::<TokenStateAccount>() + 8,
        payer = authority
    )]
    pub state_account: Box<Account<'info, TokenStateAccount>>,

    // Config Accounts
    #[account(init,
        space = size_of::<Config>() + 8,
        payer = authority
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(init,
        mint::decimals = LP_TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"lpsol_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub lpsol_mint: Box<Account<'info, Mint>>,  

    #[account(init,
        mint::decimals = LP_TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"lpusd_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub lpusd_mint: Box<Account<'info, Mint>>,

    // This is LPFI token (DAO)
    #[account(init,
        mint::decimals = LP_TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"lpdao_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub lpdao_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = lpdao_mint,
        associated_token::authority = authority
    )]
    pub user_daotoken: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

// Proxy mintable
#[derive(Accounts)]
pub struct MintLpToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    #[account(mut, has_one= state_account)]
    pub config: Box<Account<'info, Config>>,
    #[account(mut,
        constraint = user_lptoken.mint == lptoken_mint.key(),
    )]
    pub user_lptoken: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub lptoken_mint: Account<'info, Mint>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

// Program deployer mintable
#[derive(Accounts)]
pub struct OwnerLpToken<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one=owner)]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = lptoken_mint,
        associated_token::authority = owner
    )]
    pub user_lptoken: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub lptoken_mint: Account<'info, Mint>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct MintDaoLpToken<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner)]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = lptoken_mint,
        associated_token::authority = owner
    )]
    pub user_lptoken: Box<Account<'info, TokenAccount>>,
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>,
    #[account(mut, constraint=config.lpdao_mint == lptoken_mint.key())]
    pub lptoken_mint: Account<'info, Mint>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct UpdateConfigAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner)]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>
}

#[account]
#[derive(Default)]
pub struct TokenStateAccount {
    pub owner: Pubkey,
    pub second_owner: Pubkey
}

#[account]
#[derive(Default)]
pub struct Config {
    pub state_account: Pubkey,
    pub lpusd_mint: Pubkey,
    pub lpsol_mint: Pubkey,
    pub lpdao_mint: Pubkey,
    pub last_mint_timestamp: i64 // 8
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Invalid Owner")]
    InvalidOwner,
    #[msg("Too often mint")]
    TooOftenMint
}