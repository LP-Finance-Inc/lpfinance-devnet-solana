
//! Accounts state.

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount }
};
use test_tokens::program::TestTokens;

pub const PREFIX: &str = "solend-pool";

// This state account would be used as tx signer
#[account]
#[derive(Default)]
pub struct StateAccount {
    pub owner: Pubkey,
    pub second_owner: Pubkey
}

impl StateAccount {
    pub const LEN: usize = 32 * 2 + 8;
}

// Store main infos of apricot
#[account]
#[derive(Default)]
pub struct Config {
    pub state_account: Pubkey,      // 32
    /// available tokens
    pub wsol_mint: Pubkey,          // 32
    pub msol_mint: Pubkey,          // 32
    pub srm_mint: Pubkey,           // 32
    pub scnsol_mint: Pubkey,        // 32
    pub stsol_mint: Pubkey,         // 32
    pub ray_mint: Pubkey,           // 32
    /// total amount
    pub wsol_amount: u64,           // 8
    pub msol_amount: u64,           // 8
    pub srm_amount: u64,            // 8
    pub scnsol_amount: u64,         // 8
    pub stsol_amount: u64,          // 8
    pub ray_amount: u64,            // 8
    //
    pub wsol_rate: u64,             // 8
    pub msol_rate: u64,             // 8
    pub srm_rate: u64,              // 8
    pub scnsol_rate: u64,           // 8
    pub stsol_rate: u64,            // 8
    pub ray_rate: u64,              // 8
    // 
    pub last_mint_timestamp: i64    // 8
}

impl Config {
    pub const LEN: usize = 32 * 7 + 8 * 13 + 8;
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Token program owner
    #[account(mut)]
    pub authority: Signer<'info>,
    // Apricot Signer
    #[account(init,
        seeds = [PREFIX.as_bytes()],
        bump,
        space = StateAccount::LEN,
        payer = authority
    )]
    pub state_account: Box<Account<'info, StateAccount>>,
 
    // Infos Account
    #[account(init,
        space = Config::LEN,
        payer = authority
    )]
    pub config: Box<Account<'info, Config>>,
    
    pub wsol_mint: Box<Account<'info, Mint>>,
    pub msol_mint: Box<Account<'info, Mint>>,
    pub srm_mint: Box<Account<'info, Mint>>,
    pub scnsol_mint: Box<Account<'info, Mint>>,
    pub stsol_mint: Box<Account<'info, Mint>>,
    pub ray_mint: Box<Account<'info, Mint>>,

    // wSOL POOL
    #[account(
        init,
        token::mint = wsol_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_wsol".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_wsol: Box<Account<'info, TokenAccount>>,
    // RAY POOL
    #[account(
        init,
        token::mint = ray_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_ray".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_ray: Box<Account<'info, TokenAccount>>,
    // mSOL POOL
    #[account(
        init,
        token::mint = msol_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_msol".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_msol: Box<Account<'info, TokenAccount>>,
    // srm POOL
    #[account(
        init,
        token::mint = srm_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_srm".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_srm: Box<Account<'info, TokenAccount>>,
    // scnsol POOL
    #[account(
        init,
        token::mint = scnsol_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_scnsol".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_scnsol: Box<Account<'info, TokenAccount>>,
    // stsol POOL
    #[account(
        init,
        token::mint = stsol_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_stsol".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_stsol: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub owner: Pubkey,
    pub wsol_amount: u64,
    pub msol_amount: u64,
    pub srm_amount: u64,
    pub scnsol_amount: u64,
    pub stsol_amount: u64,
    pub ray_amount: u64,
    pub temp: u64
}

impl UserAccount {
    pub const LEN: usize = 32 + 8 * 7 + 8;
    
    pub fn get_key_amount (
        &self, 
        dest_mint: Pubkey,
        config: &mut Account<Config>
    ) -> Result<u64> {
        let mut _amount = 0;
        if dest_mint.key() == config.ray_mint {
            _amount = self.ray_amount;
        } else if dest_mint.key() == config.wsol_mint {
            _amount = self.wsol_amount;
        } else if dest_mint.key() == config.msol_mint {
            _amount = self.msol_amount;
        } else if dest_mint.key() == config.srm_mint {
            _amount = self.srm_amount;

        } else if dest_mint.key() == config.scnsol_mint {
            _amount = self.scnsol_amount;

        } else if dest_mint.key() == config.stsol_mint {
            _amount = self.stsol_amount;
        }

        Ok(_amount)
    }
}


#[derive(Accounts)]
pub struct InitUserAccount<'info> {
    // State account for each user/wallet
    #[account(
        init,
        seeds = [PREFIX.as_bytes(), user.key().as_ref()],
        bump,
        space = UserAccount::LEN,
        payer = user_authority
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    // Contract Authority accounts
    #[account(mut)]
    pub user_authority: Signer<'info>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DepositToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,
        constraint = user_token.mint == token_mint.key(),
        constraint = user_token.owner == authority.key()
    )]
    pub user_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub pool_token: Account<'info, TokenAccount>,
    // State Accounts
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,
    #[account(mut)]
    pub user_account: Box<Account<'info, UserAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,
        constraint = user_token.mint == token_mint.key(),
        constraint = user_token.owner == authority.key()
    )]
    pub user_token: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub pool_token: Box<Account<'info, TokenAccount>>,
    // State Accounts
    #[account(mut)]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>,
    #[account(mut)]
    pub user_account: Box<Account<'info, UserAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}


#[derive(Accounts)]
pub struct DailyReward<'info> {
    #[account(mut)]
    pub second_owner: Signer<'info>,
    
    #[account(mut,
        constraint = pool_token.owner == config.key(),
        constraint = pool_token.mint == token_mint.key()
    )]
    pub pool_token: Box<Account<'info, TokenAccount>>,
    
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>,
    #[account(mut, has_one = second_owner)]
    pub state_account: Box<Account<'info, StateAccount>>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_state: Account<'info, test_tokens::TokenStateAccount>,
    // Programs and Sysvars
    pub lending_program: Program<'info, TestTokens>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct UpdateConfigAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner)]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>
}
