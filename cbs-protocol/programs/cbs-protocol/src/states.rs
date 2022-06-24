
//! Accounts state.

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount }
};

use lpfinance_accounts::{self, WhiteList};
use lpfinance_tokens::{self, TokenStateAccount};
use lpfinance_accounts::program::LpfinanceAccounts;
use lpfinance_tokens::program::LpfinanceTokens;

use solend::program::Solend;
use solend::{self};

use apricot::program::Apricot;
use apricot::{self};

const PREFIX: &str = "cbsprotocol3";

#[derive(Accounts)]
pub struct RepayToken<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(mut)]
    pub user_dest : Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub dest_mint: Account<'info,Mint>,
    // state account for user's wallet
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut,has_one = state_account)]
    pub config: Box<Account<'info, Config>>,
    #[account(mut)]
    pub dest_pool: Box<Account<'info, TokenAccount>>,
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
pub struct RepaySOL<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,

    // state account for user's wallet
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
pub struct WithdrawSOL<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut, has_one= state_account)]
    pub config: Box<Account<'info, Config>>,
    // state account for user's wallet
    #[account(
        mut,
        constraint = user_account.owner == user_authority.key()
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    pub pyth_btc_account: AccountInfo<'info>,
    pub pyth_usdc_account: AccountInfo<'info>,
    pub pyth_sol_account: AccountInfo<'info>,
    pub pyth_eth_account: AccountInfo<'info>,
    pub pyth_msol_account: AccountInfo<'info>,
    pub pyth_ust_account: AccountInfo<'info>,
    pub pyth_srm_account: AccountInfo<'info>,
    pub pyth_scnsol_account: AccountInfo<'info>,
    pub pyth_stsol_account: AccountInfo<'info>,
    pub pyth_usdt_account: AccountInfo<'info>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    // state account for user's wallet
    #[account(
        mut,
        constraint = user_account.owner == user_authority.key()
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub user_dest : Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub dest_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub dest_mint: Account<'info,Mint>,
    pub pyth_btc_account: AccountInfo<'info>,
    pub pyth_usdc_account: AccountInfo<'info>,
    pub pyth_sol_account: AccountInfo<'info>,
    pub pyth_eth_account: AccountInfo<'info>,
    pub pyth_msol_account: AccountInfo<'info>,
    pub pyth_ust_account: AccountInfo<'info>,
    pub pyth_srm_account: AccountInfo<'info>,
    pub pyth_scnsol_account: AccountInfo<'info>,
    pub pyth_stsol_account: AccountInfo<'info>,
    pub pyth_usdt_account: AccountInfo<'info>,

    #[account(mut)]
    pub solend_config: Box<Account<'info, solend::Config>>,
    #[account(mut)]
    pub solend_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub solend_account: Box<Account<'info, solend::UserAccount>>,
    #[account(mut)]
    pub solend_state_account: Box<Account<'info, solend::StateAccount>>,
    #[account(mut)]
    pub apricot_config: Box<Account<'info, apricot::Config>>,
    #[account(mut)]
    pub apricot_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub apricot_account: Box<Account<'info, apricot::UserAccount>>,
    #[account(mut)]
    pub apricot_state_account: Box<Account<'info, apricot::StateAccount>>,
    pub solend_program: Program<'info, Solend>,
    pub apricot_program: Program<'info, Apricot>,

    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Token program authority
    #[account(mut)]
    pub authority: Signer<'info>,
    // State Accounts
    #[account(init,
        seeds = [PREFIX.as_bytes()],
        bump,
        space = StateAccount::LEN,
        payer = authority
    )]
    pub state_account: Box<Account<'info, StateAccount>>,

    // Config Accounts
    #[account(init,
        payer = authority,
        space = 32 * 27 + 24 * 20 + 8
    )]
    pub config: Box<Account<'info, Config>>,

    pub wsol_mint: Box<Account<'info, Mint>>,
    pub ray_mint: Box<Account<'info, Mint>>,
    pub msol_mint: Box<Account<'info, Mint>>,
    pub eth_mint: Box<Account<'info, Mint>>,
    pub ust_mint: Box<Account<'info, Mint>>,
    pub srm_mint: Box<Account<'info, Mint>>,
    pub scnsol_mint: Box<Account<'info, Mint>>,
    pub stsol_mint: Box<Account<'info, Mint>>,
    pub usdt_mint: Box<Account<'info, Mint>>,

    pub lpsol_mint: Box<Account<'info, Mint>>,   
    pub lpusd_mint: Box<Account<'info, Mint>>,
    pub lpray_mint: Box<Account<'info, Mint>>,
    pub lpeth_mint: Box<Account<'info, Mint>>,

    // LpSOL POOL
    #[account(
        init,
        token::mint = lpsol_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_lpsol".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_lpsol: Box<Account<'info, TokenAccount>>,
    // LpUSD POOL
    #[account(
        init,
        token::mint = lpusd_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_lpusd".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_lpusd: Box<Account<'info, TokenAccount>>,
    // LpBTC POOL
    #[account(
        init,
        token::mint = lpray_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_lpbtc".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_lpbtc: Box<Account<'info, TokenAccount>>,
    // LpETH POOL
    #[account(
        init,
        token::mint = lpeth_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_lpeth".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_lpeth: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    // Token program authority
    #[account(mut)]
    pub authority: Signer<'info>,
    // State Accounts
    #[account(mut)]
    pub state_account: Box<Account<'info, StateAccount>>,

    // Config Accounts
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>,

    pub wsol_mint: Box<Account<'info, Mint>>,
    pub ray_mint: Box<Account<'info, Mint>>,
    pub msol_mint: Box<Account<'info, Mint>>,
    pub eth_mint: Box<Account<'info, Mint>>,
    pub ust_mint: Box<Account<'info, Mint>>,
    pub srm_mint: Box<Account<'info, Mint>>,
    pub scnsol_mint: Box<Account<'info, Mint>>,
    pub stsol_mint: Box<Account<'info, Mint>>,
    pub usdt_mint: Box<Account<'info, Mint>>,
    // USDC POOL
    #[account(
        init,
        token::mint = wsol_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_wsol".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_wsol: Box<Account<'info, TokenAccount>>,
    // USDC POOL
    #[account(
        init,
        token::mint = eth_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_eth".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_eth: Box<Account<'info, TokenAccount>>,
    // BTC POOL
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
    // UST POOL
    #[account(
        init,
        token::mint = ust_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_ust".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_ust: Box<Account<'info, TokenAccount>>,
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
    // usdt POOL
    #[account(
        init,
        token::mint = usdt_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"pool_usdt".as_ref()],
        bump,
        payer = authority
    )]
    pub pool_usdt: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
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
        space = UserAccount::LEN,
        payer = user_authority
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    // Contract Authority accounts
    #[account(mut)]
    pub user_authority: Signer<'info>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    // Program signer
    #[account(mut)]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>,  

    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(
        mut,
        constraint = user_collateral.owner == user_authority.key(),
        constraint = user_collateral.mint == collateral_mint.key()
    )]
    pub user_collateral : Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub collateral_mint: Account<'info,Mint>,
    #[account(
        mut,
        constraint = collateral_pool.mint == collateral_mint.key(),
        constraint = collateral_pool.owner == state_account.key()
    )]
    pub collateral_pool: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = user_account.owner == user_authority.key()
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    
    #[account(mut)]
    pub solend_config: Box<Account<'info, solend::Config>>,
    #[account(mut)]
    pub solend_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub solend_account: Box<Account<'info, solend::UserAccount>>,
    #[account(mut)]
    pub apricot_config: Box<Account<'info, apricot::Config>>,
    #[account(mut)]
    pub apricot_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub apricot_account: Box<Account<'info, apricot::UserAccount>>,

    pub solend_program: Program<'info, Solend>,
    pub apricot_program: Program<'info, Apricot>,

    #[account(mut)]
    pub whitelist: AccountLoader<'info, WhiteList>,
    #[account(mut)]
    pub accounts_config: Box<Account<'info, lpfinance_accounts::Config>>,
    pub accounts_program: Program<'info, LpfinanceAccounts>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct DepositSOL<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub state_account: Box<Account<'info, StateAccount>>,
    // state account for user's wallet
    #[account(
        mut,
        constraint = user_account.owner == user_authority.key()
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    #[account(mut)]
    pub whitelist: AccountLoader<'info, WhiteList>,

    #[account(mut)]
    pub whitelist_config: Box<Account<'info, lpfinance_accounts::Config>>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,
    pub accounts_program: Program<'info, LpfinanceAccounts>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct BorrowLpToken<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    // state account for user's wallet
    #[account(
        mut,
        constraint = user_account.owner == user_authority.key()
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut)]
    pub tokens_state: Box<Account<'info, TokenStateAccount>>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,
    #[account(mut)]
    pub lptoken_config: Box<Account<'info, lpfinance_tokens::Config>>,
    #[account(
        init_if_needed,
        payer = user_authority,
        associated_token::mint = collateral_mint,
        associated_token::authority = user_authority
    )]
    pub user_collateral : Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub collateral_mint: Account<'info,Mint>,
    pub pyth_btc_account: AccountInfo<'info>,
    pub pyth_eth_account: AccountInfo<'info>,
    pub pyth_usdc_account: AccountInfo<'info>,
    pub pyth_sol_account: AccountInfo<'info>,
    pub pyth_msol_account: AccountInfo<'info>,
    pub pyth_ust_account: AccountInfo<'info>,
    pub pyth_srm_account: AccountInfo<'info>,
    pub pyth_scnsol_account: AccountInfo<'info>,
    pub pyth_stsol_account: AccountInfo<'info>,
    pub pyth_usdt_account: AccountInfo<'info>,
    // Programs and Sysvars
    pub lptokens_program: Program<'info, LpfinanceTokens>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}


#[derive(Accounts)]
pub struct LiquidateCollateral<'info> {
    #[account(mut)]
    pub user_account: Box<Account<'info, UserAccount>>,
    #[account(mut)]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut)]
    pub auction_account: AccountInfo<'info>,

    #[account(mut)]
    pub auction_msol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_btc: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_usdc: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_eth: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub cbs_msol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_btc: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_usdc: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_eth: Box<Account<'info, TokenAccount>>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct LiquidateSecondCollateral<'info> {
    #[account(mut)]
    pub user_account: Box<Account<'info, UserAccount>>,
    #[account(mut)]
    pub state_account: Box<Account<'info, StateAccount>>,

    #[account(mut)]
    pub auction_ust: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_srm: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_scnsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_stsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_usdt: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub cbs_ust: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_srm: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_scnsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_stsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_usdt: Box<Account<'info, TokenAccount>>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct LiquidateLpTokenCollateral<'info> {
    #[account(mut)]
    pub user_account: Box<Account<'info, UserAccount>>,
    #[account(mut)]
    pub state_account: Box<Account<'info, StateAccount>>,

    #[account(mut)]
    pub auction_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_lpbtc: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_lpeth: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub cbs_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_lpbtc: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_lpeth: Box<Account<'info, TokenAccount>>,

    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}


#[derive(Accounts)]
pub struct UpdateUserAccount<'info> {
    #[account(mut)]
    pub user_account: Box<Account<'info, UserAccount>>
}

#[account]
#[derive(Default)]
pub struct StateAccount {
    pub owner: Pubkey,
    pub liquidation_run: bool
}

impl StateAccount {
    pub const LEN: usize = 32 + 1 + 8;
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>,
    #[account(mut, has_one = owner)]
    pub state_account: Box<Account<'info, StateAccount>>,

    #[account(mut)]
    pub ray_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub wsol_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub eth_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub msol_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub ust_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub srm_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub scnsol_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub stsol_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub usdt_mint: Box<Account<'info,Mint>>,

    #[account(mut)]
    pub lpsol_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub lpusd_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub lpray_mint: Box<Account<'info,Mint>>,
    #[account(mut)]
    pub lpeth_mint: Box<Account<'info,Mint>>,

    #[account(mut)]
    pub pool_ray: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_wsol: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_msol: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_eth: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_ust: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_srm: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_scnsol: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_stsol: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_usdt: Box<Account<'info,TokenAccount>>,

    #[account(mut)]
    pub pool_lpsol: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_lpusd: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_lpbtc: Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub pool_lpeth: Box<Account<'info,TokenAccount>>,

    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}

#[account]
#[derive(Default)]
pub struct Config {
    pub state_account: Pubkey,

    pub total_borrowed_lpusd: u64,
    pub total_borrowed_lpsol: u64,
    pub total_borrowed_lpbtc: u64,
    pub total_borrowed_lpeth: u64,

    pub total_deposited_sol: u64,
    pub total_deposited_usdc: u64,
    pub total_deposited_btc: u64,
    pub total_deposited_eth: u64,
    pub total_deposited_msol: u64,
    pub total_deposited_ust: u64,
    pub total_deposited_srm: u64,
    pub total_deposited_scnsol: u64,
    pub total_deposited_stsol: u64,
    pub total_deposited_usdt: u64,
    pub total_deposited_lpsol: u64,
    pub total_deposited_lpusd: u64,
    pub total_deposited_lpbtc: u64,
    pub total_deposited_lpeth: u64,

    pub lpsol_mint: Pubkey,
    pub lpusd_mint: Pubkey,
    pub lpray_mint: Pubkey,
    pub lpeth_mint: Pubkey,

    pub ray_mint: Pubkey,
    pub wsol_mint: Pubkey,
    pub msol_mint: Pubkey,
    pub ust_mint: Pubkey,
    pub srm_mint: Pubkey,
    pub scnsol_mint: Pubkey,
    pub stsol_mint: Pubkey,

    pub pool_ray: Pubkey,
    pub pool_wsol: Pubkey,
    pub pool_msol: Pubkey,
    pub pool_eth: Pubkey,
    pub pool_srm: Pubkey,
    pub pool_scnsol: Pubkey,
    pub pool_stsol: Pubkey,
    pub pool_lpsol: Pubkey,
    pub pool_lpusd: Pubkey
}

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub borrowed_lpusd: u64,
    pub borrowed_lpsol: u64,
    pub borrowed_lpbtc: u64,
    pub borrowed_lpeth: u64,
    // deposited amount
    pub btc_amount: u64,
    pub sol_amount: u64,
    pub usdc_amount: u64,
    pub eth_amount: u64,
    pub msol_amount: u64,
    pub ust_amount: u64,
    pub srm_amount: u64,
    pub scnsol_amount: u64,
    pub stsol_amount: u64,
    pub usdt_amount: u64,

    pub lpsol_amount: u64,
    pub lpusd_amount: u64,
    pub lpeth_amount: u64,
    pub lpbtc_amount: u64,

    // solend & apricot
    pub lending_btc_amount: u64,
    pub lending_sol_amount: u64,
    pub lending_usdc_amount: u64,
    pub lending_eth_amount: u64,
    pub lending_msol_amount: u64,
    pub lending_ust_amount: u64,
    pub lending_srm_amount: u64,
    pub lending_scnsol_amount: u64,
    pub lending_stsol_amount: u64,
    pub lending_usdt_amount: u64,

    pub owner: Pubkey,
    pub bump: u8
}

impl UserAccount {
    pub const LEN: usize = 8 * 28 + 32 + 1 + 8;
}