use anchor_lang::prelude::*;
// use pyth_client;
use anchor_spl::token::{ Mint, Token, TokenAccount };

use cbs_protocol::program::CbsProtocol;
use cbs_protocol::{self};

use lpfinance_swap::program::LpfinanceSwap;
use lpfinance_swap::{self};

// Actually DENOMINATOR should be 100 (%)
// But to calculate percent in more clearly, we consider DECIMAL 4. For example, 101.0014
pub const DENOMINATOR: u64 = 1000000;
pub const LTV_PERMISSION:u128 = 94;

pub const PREFIX: &str = "lpusd-auction";

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const U64_LENGTH: usize = 8;
const I64_LENGTH: usize = 8;
const U8_LENGTH: usize = 1;
const BOOL_LENGTH: usize =1;
// const TITLE_LENGTH: usize = 4*2;

#[derive(Accounts)]
pub struct Initialize <'info>{
    // Auction program deployer
    #[account(mut)]
    pub authority: Signer<'info>,

    // Config Accounts
    #[account(init,
        payer = authority,
        space = Config::LEN
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
    pub lpfi_mint: Box<Account<'info, Mint>>,
    /// CHECK: This is safe
    #[account(seeds = [PREFIX.as_ref()], bump)]
    pub auction_pda: AccountInfo<'info>,
    // LpSOL POOL
    #[account(
        init,
        token::mint = lpsol_mint,
        token::authority = auction_pda,
        payer = authority
    )]
    pub pool_lpsol: Box<Account<'info, TokenAccount>>,

    // LpUSD POOL
    #[account(
        init,
        token::mint = lpusd_mint,
        token::authority = auction_pda,
        payer = authority
    )]
    pub pool_lpusd: Box<Account<'info, TokenAccount>>,
    // LpFi POOL
    #[account(
        init,
        token::mint = lpfi_mint,
        token::authority = auction_pda,
        payer = authority
    )]
    pub pool_lpfi: Box<Account<'info, TokenAccount>>,    

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct CreateTokenATA<'info> {
    // Token program authority
    #[account(mut)]
    pub authority: Signer<'info>,

    // Config Accounts
    #[account(mut,
        constraint = config.owner == authority.key()
    )]
    pub config: Box<Account<'info, Config>>,
    
    // Tokens
    pub wsol_mint: Box<Account<'info, Mint>>,
    pub ray_mint: Box<Account<'info, Mint>>,
    pub msol_mint: Box<Account<'info, Mint>>,
    pub srm_mint: Box<Account<'info, Mint>>,
    pub scnsol_mint: Box<Account<'info, Mint>>,
    pub stsol_mint: Box<Account<'info, Mint>>,

    /// CHECK: This is safe
    #[account(seeds = [PREFIX.as_ref()], bump)]
    pub auction_pda: AccountInfo<'info>,

    // wSOL POOL
    #[account(
        init,
        token::mint = wsol_mint,
        token::authority = auction_pda,
        payer = authority
    )]
    pub pool_wsol: Box<Account<'info, TokenAccount>>,
    // Ray POOL
    #[account(
        init,
        token::mint = ray_mint,
        token::authority = auction_pda,
        payer = authority
    )]
    pub pool_ray: Box<Account<'info, TokenAccount>>,
    // mSOL POOL
    #[account(
        init,
        token::mint = msol_mint,
        token::authority = auction_pda,
        payer = authority
    )]
    pub pool_msol: Box<Account<'info, TokenAccount>>,
    // srm POOL
    #[account(
        init,
        token::mint = srm_mint,
        token::authority = auction_pda,
        payer = authority
    )]
    pub pool_srm: Box<Account<'info, TokenAccount>>,
    // scnsol POOL
    #[account(
        init,
        token::mint = scnsol_mint,
        token::authority = auction_pda,
        payer = authority
    )]
    pub pool_scnsol: Box<Account<'info, TokenAccount>>,
    // stsol POOL
    #[account(
        init,
        token::mint = stsol_mint,
        token::authority = auction_pda,
        payer = authority
    )]
    pub pool_stsol: Box<Account<'info, TokenAccount>>,    

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

// #[derive(Accounts)]
// pub struct LiquidateFromCBS<'info> {
//     #[account(mut)]
//     pub user_authority: Signer<'info>,
//     #[account(mut,
//         seeds = [PREFIX.as_ref()],
//         bump
//     )]
//     pub state_account: Box<Account<'info, StateAccount>>,
//     #[account(mut, has_one = state_account)]
//     pub config: Box<Account<'info, Config>>,
//     // UserAccount from CBS protocol
//     #[account(mut)]
//     pub liquidator: Box<Account<'info, cbs_protocol::UserAccount>>,
//     #[account(mut)]
//     pub cbs_account: Box<Account<'info, cbs_protocol::StateAccount>>,
//     pub cbs_program: Program<'info, CbsProtocol>,

//     #[account(mut)]
//     pub auction_btc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_msol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_usdc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_eth: Box<Account<'info, TokenAccount>>,

//     #[account(mut)]
//     pub cbs_btc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_msol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_usdc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_eth: Box<Account<'info, TokenAccount>>,
    
//     // Programs and Sysvars
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub rent: Sysvar<'info, Rent>
// }


// #[derive(Accounts)]
// pub struct LiquidateSecondFromCBS<'info> {
//     #[account(mut)]
//     pub user_authority: Signer<'info>,
//     #[account(mut)]
//     pub config: Box<Account<'info, Config>>,
//     // UserAccount from CBS protocol
//     #[account(mut)]
//     pub liquidator: Box<Account<'info, cbs_protocol::UserAccount>>,
//     #[account(mut)]
//     pub cbs_account: Box<Account<'info, cbs_protocol::StateAccount>>,
//     pub cbs_program: Program<'info, CbsProtocol>,

//     #[account(mut)]
//     pub auction_ust: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_srm: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_scnsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_stsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_usdt: Box<Account<'info, TokenAccount>>,

//     #[account(mut)]
//     pub cbs_ust: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_srm: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_scnsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_stsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_usdt: Box<Account<'info, TokenAccount>>,
    
//     // Programs and Sysvars
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub rent: Sysvar<'info, Rent>
// }

// #[derive(Accounts)]
// pub struct LiquidateLpTokenFromCBS<'info> {
//     #[account(mut)]
//     pub user_authority: Signer<'info>,
//     #[account(mut,
//         seeds = [PREFIX.as_ref()],
//         bump
//     )]
//     pub state_account: Box<Account<'info, StateAccount>>,
//     #[account(mut, has_one = state_account)]
//     pub config: Box<Account<'info, Config>>,
//     // UserAccount from CBS protocol
//     #[account(mut)]
//     pub liquidator: Box<Account<'info, cbs_protocol::UserAccount>>,
//     #[account(mut)]
//     pub cbs_account: Box<Account<'info, cbs_protocol::StateAccount>>,
//     pub cbs_program: Program<'info, CbsProtocol>,

//     #[account(mut)]
//     pub auction_lpusd: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_lpsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_lpbtc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_lpeth: Box<Account<'info, TokenAccount>>,

//     #[account(mut)]
//     pub cbs_lpusd: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_lpsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_lpbtc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_lpeth: Box<Account<'info, TokenAccount>>,
    
//     // Programs and Sysvars
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub rent: Sysvar<'info, Rent>
// }


// #[derive(Accounts)]
// pub struct Liquidate<'info> {
//     #[account(mut,
//         seeds = [PREFIX.as_ref()],
//         bump
//     )]
//     pub state_account: Box<Account<'info, StateAccount>>,
//     #[account(mut, has_one = state_account)]
//     pub config: Box<Account<'info, Config>>,
//     // UserAccount from CBS protocol
//     #[account(mut)]
//     pub liquidator: Box<Account<'info, cbs_protocol::UserAccount>>,

//     pub cbs_program: Program<'info, CbsProtocol>,
//     pub swap_program: Program<'info, LpfinanceSwap>,

//     #[account(mut)]
//     pub auction_lpusd: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_lpsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_lpbtc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_lpeth: Box<Account<'info, TokenAccount>>,

//     #[account(mut)]
//     pub lpbtc_mint: Box<Account<'info,Mint>>,
//     #[account(mut)]
//     pub lpsol_mint: Box<Account<'info,Mint>>,

//     #[account(mut)]
//     pub cbs_lpusd: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_lpsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_lpbtc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub cbs_lpeth: Box<Account<'info, TokenAccount>>,
    
//     #[account(mut)]
//     pub swap_account: Box<Account<'info, lpfinance_swap::StateAccount>>,
//     // pyth
//     pub pyth_btc_account: AccountInfo<'info>,
//     pub pyth_usdc_account: AccountInfo<'info>,
//     pub pyth_sol_account: AccountInfo<'info>,
//     pub pyth_msol_account: AccountInfo<'info>,
//     pub pyth_ust_account: AccountInfo<'info>,
//     pub pyth_srm_account: AccountInfo<'info>,
//     pub pyth_scnsol_account: AccountInfo<'info>,
//     pub pyth_stsol_account: AccountInfo<'info>,
//     pub pyth_usdt_account: AccountInfo<'info>,
//     pub pyth_eth_account: AccountInfo<'info>,
//     // Programs and Sysvars
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub rent: Sysvar<'info, Rent>
// }

// #[derive(Accounts)]
// pub struct LiquidateSwap<'info> {
//     #[account(mut)]
//     pub user_authority: Signer<'info>,
//     #[account(mut,
//         seeds = [PREFIX.as_ref()],
//         bump
//     )]
//     pub state_account: Box<Account<'info, StateAccount>>,
//     #[account(mut, has_one = state_account)]
//     pub config: Box<Account<'info, Config>>,
//     // UserAccount from CBS protocol
//     #[account(mut)]
//     pub liquidator: Box<Account<'info, cbs_protocol::UserAccount>>,
//     #[account(mut)]
//     pub swap_account: Box<Account<'info, lpfinance_swap::StateAccount>>,
//     pub cbs_program: Program<'info, CbsProtocol>,
//     pub swap_program: Program<'info, LpfinanceSwap>,

//     #[account(mut)]
//     pub lpusd_mint: Box<Account<'info,Mint>>,

//     #[account(mut)]
//     pub swap_btc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_usdc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_msol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_eth: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_ust: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_srm: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_scnsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_stsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_usdt: Box<Account<'info, TokenAccount>>,

//     #[account(mut)]
//     pub auction_btc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_usdc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_msol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_eth: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_ust: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_srm: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_scnsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_stsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_usdt: Box<Account<'info, TokenAccount>>,

//     // Programs and Sysvars
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub rent: Sysvar<'info, Rent>
// }

// #[derive(Accounts)]
// pub struct LiquidateSecondSwap<'info> {
//     #[account(mut)]
//     pub user_authority: Signer<'info>,
//     #[account(mut,
//         seeds = [PREFIX.as_ref()],
//         bump
//     )]
//     pub state_account: Box<Account<'info, StateAccount>>,
//     #[account(mut, has_one = state_account)]
//     pub config: Box<Account<'info, Config>>,
//     // UserAccount from CBS protocol
//     #[account(mut)]
//     pub liquidator: Box<Account<'info, cbs_protocol::UserAccount>>,
//     #[account(mut)]
//     pub swap_account: Box<Account<'info, lpfinance_swap::StateAccount>>,
//     pub cbs_program: Program<'info, CbsProtocol>,
//     pub swap_program: Program<'info, LpfinanceSwap>,

//     #[account(mut)]
//     pub lpusd_mint: Box<Account<'info,Mint>>,

//     #[account(mut)]
//     pub swap_lpusd: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_lpsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_lpbtc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub swap_lpeth: Box<Account<'info, TokenAccount>>,

//     #[account(mut)]
//     pub auction_lpusd: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_lpsol: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_lpbtc: Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub auction_lpeth: Box<Account<'info, TokenAccount>>,

//     // Programs and Sysvars
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub rent: Sysvar<'info, Rent>
// }




#[account]
#[derive(Default)]
pub struct Config {
    pub owner: Pubkey,

    pub wsol_mint: Pubkey,
    pub msol_mint: Pubkey,
    pub ray_mint: Pubkey,
    pub srm_mint: Pubkey,
    pub scnsol_mint: Pubkey,
    pub stsol_mint: Pubkey,

    pub lpsol_mint: Pubkey,
    pub lpusd_mint: Pubkey,
    pub lpfi_mint: Pubkey,

    pub pool_wsol: Pubkey,      // Auction ATA
    pub pool_ray: Pubkey,       // Auction ATA
    pub pool_msol: Pubkey,      // Auction ATA
    pub pool_srm: Pubkey,       // Auction ATA
    pub pool_scnsol: Pubkey,    // Auction ATA
    pub pool_stsol: Pubkey,     // Auction ATA

    pub pool_lpsol: Pubkey,     // Auction ATA
    pub pool_lpusd: Pubkey,     // Auction ATA
    pub pool_lpfi: Pubkey,      // Auction ATA

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

impl Config {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 19 // pubkey
        + U64_LENGTH * 4 
        + I64_LENGTH * 2;
}


#[account]
#[derive(Default)]
pub struct UserAccount {
    pub owner: Pubkey,
    // deposited lpusd
    // NOTE: only lpusd is able to be deposited
    pub lpusd_amount: u64
}

impl UserAccount {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH  // owner pubkey
        + U64_LENGTH;        // Deposited LpUSD amount
}


