use anchor_lang::prelude::*;
// use pyth_client;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount }
};
use std::mem::size_of;

use cbs_protocol::{self};

use stable_swap::{self, StableswapPool};
use uniswap::{self, UniswapPool};
use test_tokens::{self, TokenStateAccount};

// Actually DENOMINATOR should be 100 (%)
// But to calculate percent in more clearly, we consider DECIMAL 4. For example, 101.0014
pub const DENOMINATOR: u64 = 1000000;
pub const LTV_PERMISSION: u64 = 94;

pub const PREFIX: &str = "lpusd-auction";

const DISCRIMINATOR_LENGTH: usize = 8;
// const PUBLIC_KEY_LENGTH: usize = 32;
// const U64_LENGTH: usize = 8;
// const I64_LENGTH: usize = 8;
// const U8_LENGTH: usize = 1;
// const BOOL_LENGTH: usize =1;
// const TITLE_LENGTH: usize = 4*2;

#[derive(Accounts)]
pub struct Initialize <'info>{
    // Auction program deployer
    #[account(mut)]
    pub authority: Signer<'info>,

    // Config Accounts
    #[account(init,
        payer = authority,
        space = size_of::<Config>() + DISCRIMINATOR_LENGTH
    )]
    pub config: Box<Account<'info, Config>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct CreateLpTokenATA<'info> {
    // Auction program deployer
    #[account(mut)]
    pub authority: Signer<'info>,

    // Config Accounts
    #[account(mut,
        constraint = config.owner == authority.key()
    )]
    pub config: Box<Account<'info, Config>>,

    pub lpsol_mint: Box<Account<'info, Mint>>,   
    pub lpusd_mint: Box<Account<'info, Mint>>,
    /// CHECK: This is safe
    #[account(seeds = [PREFIX.as_ref()], bump)]
    pub auction_pda: AccountInfo<'info>,
    // LpSOL POOL
    #[account(
        init_if_needed,
        associated_token::mint = lpsol_mint,
        associated_token::authority = auction_pda,
        payer = authority
    )]
    pub pool_lpsol: Box<Account<'info, TokenAccount>>,

    // LpUSD POOL
    #[account(
        init_if_needed,
        associated_token::mint = lpusd_mint,
        associated_token::authority = auction_pda,
        payer = authority
    )]
    pub pool_lpusd: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct CreateNormalTokenATA<'info> {
    // Auction program deployer
    #[account(mut)]
    pub authority: Signer<'info>,

    // Config Accounts
    #[account(mut,
        constraint = config.owner == authority.key()
    )]
    pub config: Box<Account<'info, Config>>,

    pub usdc_mint: Box<Account<'info, Mint>>,   
    pub wsol_mint: Box<Account<'info, Mint>>,
    /// CHECK: This is safe
    #[account(seeds = [PREFIX.as_ref()], bump)]
    pub auction_pda: AccountInfo<'info>,
    // usdc POOL
    #[account(
        init_if_needed,
        associated_token::mint = usdc_mint,
        associated_token::authority = auction_pda,
        payer = authority
    )]
    pub pool_usdc: Box<Account<'info, TokenAccount>>,

    // wsol POOL
    #[account(
        init_if_needed,
        associated_token::mint = wsol_mint,
        associated_token::authority = auction_pda,
        payer = authority
    )]
    pub pool_wsol: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct InitUserAccount<'info> {
    // State account for each user/wallet
    #[account(
        init,
        seeds = [PREFIX.as_bytes(), user_authority.key().as_ref()],
        bump,
        space = size_of::<UserAccount>() + DISCRIMINATOR_LENGTH,
        payer = user_authority
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    // Signer
    #[account(mut)]
    pub user_authority: Signer<'info>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DeleteUserAccount<'info> {
    #[account(mut, has_one = owner, close = owner)]
    pub user_account: Box<Account<'info, UserAccount>>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DepositLpUSD<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    /// CHECK: This is safe
    #[account(seeds = [PREFIX.as_ref()], bump)]
    pub auction_pda: AccountInfo<'info>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,

    // LpUSD token mint
    #[account(
        constraint = config.lpusd_mint == lpusd_mint.key()
    )]
    pub lpusd_mint: Box<Account<'info, Mint>>,
    // user's LpUSD ATA
    #[account(
        mut,
        constraint = user_lpusd.owner == user_authority.key(),
        constraint = user_lpusd.mint == lpusd_mint.key()
    )]
    pub user_lpusd: Box<Account<'info, TokenAccount>>,
    // program's LpUSD ATA
    #[account(mut,
        constraint=pool_lpusd.mint == lpusd_mint.key(),
        constraint=pool_lpusd.owner == auction_pda.key()
    )]
    pub pool_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_account.owner == user_authority.key()
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct WithdrawLpUSD<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    /// CHECK: This is safe
    #[account(seeds = [PREFIX.as_ref()], bump)]
    pub auction_pda: AccountInfo<'info>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        constraint = config.lpusd_mint == lpusd_mint.key()
    )]
    pub lpusd_mint: Account<'info, Mint>,
    // user's LpUSD ATA
    #[account(
        mut,
        constraint = user_lpusd.owner == user_authority.key(),
        constraint = user_lpusd.mint == lpusd_mint.key()
    )]
    pub user_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint=pool_lpusd.mint == lpusd_mint.key(),
        constraint=pool_lpusd.owner == auction_pda.key()
    )]
    pub pool_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_account.owner == user_authority.key()
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {    
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner)]
    pub config: Box<Account<'info, Config>>,
}

#[derive(Accounts)]
pub struct BurnLpUSDForLiquidate<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    /// CHECK: this is safe
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    // Auction: user account
    #[account(
        init_if_needed,
        seeds = [PREFIX.as_bytes(), owner.key().as_ref()],
        bump,
        space = size_of::<UserAccount>() + DISCRIMINATOR_LENGTH,
        payer = user_authority
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(mut)]
    pub config: Box<Account<'info, Config>>,
    /// CHECK: This is safe
    #[account(mut, seeds = [PREFIX.as_ref()], bump)]
    pub auction_pda: AccountInfo<'info>,
    // CBS: user account
    #[account(mut, 
        constraint = cbs_account.step_num == 0
    )]
    pub cbs_account: Box<Account<'info, cbs_protocol::UserAccount>>,
    // LpUSD
    #[account(mut,
        constraint = lpusd_mint.key() == config.lpusd_mint
    )]
    pub lpusd_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        constraint = lpusd_ata.mint == config.lpusd_mint,
        constraint = lpusd_ata.owner == auction_pda.key()
    )]
    pub lpusd_ata: Box<Account<'info, TokenAccount>>,
    // LpSOL
    #[account(mut,
        constraint = lpsol_mint.key() == config.lpsol_mint
    )]
    pub lpsol_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        constraint = lpsol_ata.mint == config.lpsol_mint,
        constraint = lpsol_ata.owner == auction_pda.key()
    )]
    pub lpsol_ata: Box<Account<'info, TokenAccount>>,
    // LpUSD-USDC stableswap pool
    pub stable_lpusd_pool: Box<Account<'info, StableswapPool>>,
    // LpSOL-wSOL stableswap pool
    pub stable_lpsol_pool: Box<Account<'info, StableswapPool>>,

    /// CHECK: pyth
    pub pyth_ray_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_usdc_account: AccountInfo<'info>,
    // Price feed for wSOL
    /// CHECK: pyth
    pub pyth_sol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_msol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_srm_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_scnsol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_stsol_account: AccountInfo<'info>,
    // LpFi<->USDC pool
    pub liquidity_pool: Box<Account<'info, UniswapPool>>,
    /// CHECK: this is safe
    pub lptokens_program: AccountInfo<'info>,
    /// CHECK: this is safe
    pub cbs_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct BurnLpSOLForLiquidate1<'info> {
    /// CHECK: this is for cbs participatant
    pub owner: AccountInfo<'info>,
    // Auction: user account
    #[account(mut, constraint = user_account.owner == owner.key())]
    pub user_account: Box<Account<'info, UserAccount>>,
    // CBS: user account
    #[account(mut, constraint = cbs_account.owner == owner.key())]
    pub cbs_account: Box<Account<'info, cbs_protocol::UserAccount>>,
    /// CHECK: this is safe
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub auction_pda: AccountInfo<'info>,
    // LpUSD-USDC stableswap pool
    #[account(mut)]
    pub stable_lpusd_pool: Box<Account<'info, StableswapPool>>,
    // LpSOL-wSOL stableswap pool
    #[account(mut)]
    pub stable_lpsol_pool: Box<Account<'info, StableswapPool>>,
    // This would be used for burn usdc <-> mint wsol
    #[account(mut)]
    pub token_state_account: Box<Account<'info, TokenStateAccount>>,
    
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_lpusd: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_usdc: AccountInfo<'info>,
    /// CHECK:
    pub pyth_wsol: AccountInfo<'info>,

    #[account(mut)]
    pub auction_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_ata_usdc: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_ata_wsol: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub stableswap_pool_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub stableswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    /// CHECK: this is safe
    pub cbs_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct BurnLpSOLForLiquidate2<'info> {
    /// CHECK: this is safe
    pub owner: AccountInfo<'info>,
    // Auction: user account
    #[account(mut, constraint = user_account.owner == owner.key())]
    pub user_account: Box<Account<'info, UserAccount>>,
    // CBS: user account
    #[account(mut, 
        constraint = cbs_account.owner == owner.key()
    )]
    pub cbs_account: Box<Account<'info, cbs_protocol::UserAccount>>,
    /// CHECK: this is safe
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub auction_pda: AccountInfo<'info>,
    
    // LpSOL-wSOL stableswap pool
    #[account(mut)]
    pub stable_lpsol_pool: Box<Account<'info, StableswapPool>>,
    #[account(mut)]
    pub token_lpsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,
    
    #[account(mut)]
    pub auction_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_ata_wsol: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub stableswap_pool_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub stableswap_pool_ata_wsol: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub lptokens_program: AccountInfo<'info>,
    /// CHECK: this is safe
    pub cbs_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct DistributeRewardFromLiquidate<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>, 
    /// CHECK: this is safe
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub auction_pda: AccountInfo<'info>,
    // CBS: user account
    #[account(mut)]
    pub cbs_account: Box<Account<'info, cbs_protocol::UserAccount>>,
    /// CHECK: this is safe
    pub cbs_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>    
}

#[account]
#[derive(Default)]
pub struct Config {
    pub owner: Pubkey,

    pub lpsol_mint: Pubkey,
    pub lpusd_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub wsol_mint: Pubkey,

    pub pool_lpsol: Pubkey,     // Auction ATA
    pub pool_lpusd: Pubkey,     // Auction ATA
    pub pool_usdc: Pubkey,      // Auction ATA
    pub pool_wsol: Pubkey,      // Auction ATA

    pub total_deposited_lpusd: u64, // for now, dump
    // Current auction pool's balance of LpUSD including epoch profits
    pub total_lpusd: u64,
    // This percentage is for user withdraw rate
    pub total_percent: u64,
    // Liquidation speed
    pub epoch_duration: u64,
    // Profit percentage from last liquidation
    // (Last_reward + Total_lpusd) / Total_lpusd
    pub last_epoch_percent: i64,
    // Profit from last liquidation
    pub last_epoch_profit: i64
}

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub owner: Pubkey,
    // deposited lpusd
    // NOTE: only lpusd is able to be deposited
    pub lpusd_amount: u64,
    // LpUSD -> USDC -> Wsol -> LpSOL
    // escrow
    pub escrow_wsol_amount: u64
}

