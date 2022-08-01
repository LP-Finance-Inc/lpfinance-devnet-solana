use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };
use anchor_spl::associated_token::AssociatedToken;

use std::cmp;

pub const PREFIX: &str = "lpfiswap0";
pub const PRICE_MULTIPLIER: u128 = 100000000; // 10**8 
// space size
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const U64_LENGTH: usize = 8;
const U8_LENGTH: usize =1;
const TITLE_LENGTH: usize = 4*2; // Title -> pool

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

    pub lpfi_mint: Box<Account<'info, Mint>>,
    pub usdc_mint: Box<Account<'info, Mint>>,
    
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

    pub token_mint: Box<Account<'info, Mint>>,
    // Token POOL
    #[account(
        init,
        token::mint = token_mint,
        token::authority = state_account,
        seeds = [PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump,
        payer = authority
    )]
    pub token_pool: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct CreatePair<'info> {
    // Pool owner
    #[account(mut)]
    pub creator: Signer<'info>,
    // Quote token
    pub tokena_mint: Box<Account<'info, Mint>>,
    // Owner's token
    pub tokenb_mint: Box<Account<'info, Mint>>,
    // If it was already created, throw error
    #[account(
        init, 
        payer = creator, 
        space = PoolInfo::LEN
    )]
    pub liquidity_pool: Box<Account<'info, PoolInfo>>,
    #[account(
        init,
        payer = creator,
        mint::decimals = 5,
        mint::authority = creator,
    )]    
    pub token_lp: Account<'info, Mint>,
    #[account(
        init,
        payer = creator,
        token::mint = token_lp,
        token::authority = creator,
    )]
    pub token_acc_lp: Account<'info, TokenAccount>,
    // LpFi Pool
    pub token_acc_a: Box<Account<'info, TokenAccount>>,
    // USDC pool
    pub token_acc_b: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct InitLiquidity<'info> {
    #[account(mut, has_one = creator)]
    pub pool: Account<'info, PoolInfo>,
    #[account(mut)]
    pub creator: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub creator_acc_a: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub creator_acc_b: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub token_acc_a: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub token_acc_b: AccountInfo<'info>,
    #[account(
        init,
        payer = creator,
        associated_token::mint = token_lp,
        associated_token::authority = creator,
    )]
    pub ata_creator_lp: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = token_lp
    )]
    pub token_acc_lp: Box<Account<'info, TokenAccount>>,
    pub token_lp: Box<Account<'info, Mint>>,
    /// CHECK: This is safe
    #[account(seeds = [PREFIX.as_ref()], bump)]
    pub pool_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DeletePool<'info> {
    #[account(mut, has_one = creator, close = creator)]
    pub pool: Account<'info, PoolInfo>,
    pub creator: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    // Pool owner
    #[account(mut)]
    pub authority: Signer<'info>,
    // Quote token
    pub tokena_mint: Box<Account<'info, Mint>>,
    // Owner's token
    pub tokenb_mint: Box<Account<'info, Mint>>,
    // If it was already created, throw error
    #[account(mut)]
    pub liquidity_pool: Box<Account<'info, PoolInfo>>,
    #[account(mut)]
    pub user_tokena: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_tokenb: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub tokena_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub tokenb_pool: Box<Account<'info, TokenAccount>>,
    pub token_lp: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_lp,
        associated_token::authority = authority,
    )]
    pub ata_adder_lp: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is safe
    #[account(mut)]
    pub token_acc_lp: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(seeds = [PREFIX.as_ref()], bump)]
    pub pool_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, PoolInfo>,
    #[account(mut)]
    pub remover: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub remover_acc_a: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub remover_acc_b: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub token_acc_a: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub token_acc_b: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub ata_remover_lp: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub token_acc_lp: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(seeds = [PREFIX.as_ref()], bump)]
    pub pool_pda: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

// This swap not allow user to LpFi swap
#[derive(Accounts)]
pub struct SwapTokenToToken<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub state_account: Box<Account<'info, StateAccount>>,

    #[account(
        mut,
        constraint = user_quote.owner == user_authority.key(),
        constraint = user_quote.mint == quote_mint.key()
    )]
    pub user_quote : Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = quote_pool.owner == state_account.key(),
        constraint = quote_pool.mint == quote_mint.key()
    )]
    pub quote_pool : Box<Account<'info, TokenAccount>>,
    // Should not same 
    #[account(mut, 
        constraint = quote_mint.key() != dest_mint.key()
    )]
    pub quote_mint: Box<Account<'info,Mint>>,
    // This could be LpFi mint
    #[account(mut)]
    pub dest_mint: Box<Account<'info,Mint>>,
    #[account(
        init_if_needed,
        payer = user_authority,
        associated_token::mint = dest_mint,
        associated_token::authority = user_authority
    )]
    pub user_dest : Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub dest_pool : Box<Account<'info, TokenAccount>>,
    // For now, this is lpfi<->usdc pair pool
    #[account(mut)]
    pub liquidity_pool: Box<Account<'info, PoolInfo>>,
    /// Pyth account could be USDC in case of LpFi swapping
    /// CHECK: pyth network
    pub pyth_quote_account: AccountInfo<'info>,
    /// CHECK: pyth network
    pub pyth_dest_account: AccountInfo<'info>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}


#[derive(Accounts)]
pub struct LiquidateToken<'info>{
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub state_account: Box<Account<'info, StateAccount>>,

    #[account(
        mut,
        constraint = auction_pool.mint == dest_mint.key()
    )]
    pub auction_pool : Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = swap_pool.owner == state_account.key(),
        constraint = swap_pool.mint == dest_mint.key()
    )]
    pub swap_pool : Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub dest_mint: Account<'info,Mint>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[account]
#[derive(Default)]
pub struct StateAccount {
    pub owner: Pubkey,
    pub lpfi_mint: Pubkey,
    pub usdc_mint: Pubkey
}

impl StateAccount {
    pub const LEN: usize = 3 * 32 + 8;
}

#[account]
#[derive(Default)]
pub struct PoolInfo {
    pub title: String,
    pub creator: Pubkey, // pool creator
    pub tokena_mint: Pubkey,
    pub tokenb_mint: Pubkey,
    pub token_lp: Pubkey,
    pub token_acc_a: Pubkey,
    pub token_acc_b: Pubkey,
    pub token_acc_lp: Pubkey,
    pub tokena_amount: u64,
    pub tokenb_amount: u64,
    pub min_lp_amount: u64,
    pub total_lp_amount: u64,
    pub state: u8,
    pub fee: u8             // real fee = (fee / 100)
}

impl PoolInfo {
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
        + PUBLIC_KEY_LENGTH * 7 
        + U64_LENGTH * 4 
        + U8_LENGTH * 2
        + TITLE_LENGTH;

    // Return the price of token b
    // For example, LpFi and USDC pool
    // If pass LpFi, USDC, return USDC price
    // Else pass USDC, LpFi, return LpFi price
    pub fn get_token_price(&self, tokena: Pubkey, tokenb: Pubkey) -> Result<u128> {
        if tokena == self.tokena_mint && tokenb == self.tokenb_mint {
            let price = PRICE_MULTIPLIER * self.tokena_amount as u128 / self.tokenb_amount as u128;
            Ok( price )
        } else if tokena == self.tokenb_mint && tokenb == self.tokena_mint {
            let price = PRICE_MULTIPLIER * self.tokenb_amount as u128 / self.tokena_amount as u128;
            Ok( price )
        } else {
            Ok(0)
        }
    }   
    
    // Price of token A
    pub fn get_price(&self) -> Result<u128> {
        let price = PRICE_MULTIPLIER * self.tokenb_amount as u128 / self.tokena_amount as u128;
        Ok( price )
    }   
    
    // Price of token B
    pub fn get_reverse_price(&self) -> Result<u128> {
        let price = PRICE_MULTIPLIER * self.tokena_amount as u128 / self.tokenb_amount as u128;
        Ok( price )
    }  
    
    // Get LpToken amount to mint as reward
    pub fn get_lptoken_amount(&self, amount_a: u64, amount_b: u64) -> Result<u64> {
        let input_amount_af = amount_a as f64;
        let input_amount_bf = amount_b as f64;

        let total_lp_amount_f = self.total_lp_amount as f64;
        let reserve_a_f = self.tokena_amount as f64;
        let reserve_b_f = self.tokenb_amount as f64;

        if self.total_lp_amount == 0 {
            let liquidity = (input_amount_af * input_amount_bf).sqrt() as u64 - self.min_lp_amount;
            Ok( liquidity )
        } else {
            let optimal_a_f = input_amount_af * total_lp_amount_f / reserve_a_f;
            let optimal_b_f = input_amount_bf * total_lp_amount_f / reserve_b_f;
            let liquidity = cmp::min(optimal_a_f as u64, optimal_b_f as u64);
            Ok( liquidity )
        }
    }
}