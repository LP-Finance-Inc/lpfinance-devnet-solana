use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };
use anchor_spl::associated_token::AssociatedToken;

use std::mem::size_of;
use std::cmp;

pub const PREFIX_POOL: &str = "uniswap";
pub const PRICE_MULTIPLIER: u128 = 100000000; // 10**8 

#[derive(Accounts)]
pub struct CreateUniswap<'info> {
    #[account(
        init, 
        seeds = [
            PREFIX_POOL.as_bytes(),
            token_a.key().as_ref(),
            token_b.key().as_ref(),
            author.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<UniswapPool>(),
        payer = author
    )]
    pub uniswap_pool: Box<Account<'info, UniswapPool>>,
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(mut)]
    pub token_a: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_b: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_a,
        associated_token::authority = author,
    )]    
    pub author_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_b,
        associated_token::authority = author,
    )]    
    pub author_ata_b: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = author,
        associated_token::mint = token_a,
        associated_token::authority = uniswap_pool,
    )]    
    pub pool_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = author,
        associated_token::mint = token_b,
        associated_token::authority = uniswap_pool,
    )]    
    pub pool_ata_b: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = author,
        mint::decimals = 5,
        mint::authority = uniswap_pool,
    )]
    pub token_lp: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = author,
        associated_token::mint = token_lp,
        associated_token::authority = author,
    )]
    pub author_ata_lp: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AddLiquidityUniswap<'info> {
    #[account(mut)]
    pub uniswap_pool: Account<'info, UniswapPool>,
    #[account(mut)]
    pub adder: Signer<'info>,

    #[account(mut)]
    pub token_a: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_b: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_a,
        associated_token::authority = adder,
    )]    
    pub adder_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_b,
        associated_token::authority = adder,
    )]    
    pub adder_ata_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_a,
        associated_token::authority = uniswap_pool,
    )]    
    pub pool_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_b,
        associated_token::authority = uniswap_pool,
    )]
    pub pool_ata_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        mint::decimals = 5,
        mint::authority = uniswap_pool,
    )]
    pub token_lp: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = adder,
        associated_token::mint = token_lp,
        associated_token::authority = adder,
    )]
    pub adder_ata_lp: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RemoveLiquidityUniswap<'info> {
    #[account(mut)]
    pub uniswap_pool: Account<'info, UniswapPool>,
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub token_a: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_b: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_a,
        associated_token::authority = taker,
    )]    
    pub taker_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_b,
        associated_token::authority = taker,
    )]    
    pub taker_ata_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_a,
        associated_token::authority = uniswap_pool,
    )]    
    pub pool_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_b,
        associated_token::authority = uniswap_pool,
    )]
    pub pool_ata_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        mint::decimals = 5,
        mint::authority = uniswap_pool,
    )]
    pub token_lp: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = token_lp,
        associated_token::authority = taker,
    )]
    pub taker_ata_lp: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UniswapTokens<'info> {
    #[account(mut)]
    pub uniswap_pool: Box<Account<'info, UniswapPool>>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub token_src: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_dest: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_src,
        associated_token::authority = user,
    )]    
    pub user_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_dest,
        associated_token::authority = user,
    )]    
    pub user_ata_dest: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_src,
        associated_token::authority = uniswap_pool,
    )]
    pub pool_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_dest,
        associated_token::authority = uniswap_pool,
    )]
    pub pool_ata_dest: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct UniswapPool {
    pub author: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub token_lp: Pubkey,
    pub total_lp_amount: u64,
    pub fee: u8,
}

impl UniswapPool {
    // Return the price of token b
    // For example, LpFi and USDC pool
    // If pass LpFi, USDC, return USDC price
    // Else pass USDC, LpFi, return LpFi price
    pub fn get_token_price(&self, tokena: Pubkey, tokenb: Pubkey) -> Result<u128> {
        if tokena == self.token_a && tokenb == self.token_b {
            let price = PRICE_MULTIPLIER * self.amount_a as u128 / self.amount_b as u128;
            Ok( price )
        } else if tokena == self.token_b && tokenb == self.token_a {
            let price = PRICE_MULTIPLIER * self.amount_b as u128 / self.amount_a as u128;
            Ok( price )
        } else {
            Ok(0)
        }
    }   
    
    // Price of token A
    pub fn get_price(&self) -> Result<u128> {
        let price = PRICE_MULTIPLIER * self.amount_b as u128 / self.amount_a as u128;
        Ok( price )
    }   
    
    // Price of token B
    pub fn get_reverse_price(&self) -> Result<u128> {
        let price = PRICE_MULTIPLIER * self.amount_a as u128 / self.amount_b as u128;
        Ok( price )
    }  
    
    // Get LpToken amount to mint as reward
    pub fn get_lptoken_amount(&self, amount_a: u64, amount_b: u64) -> Result<u64> {
        let input_amount_af = amount_a as f64;
        let input_amount_bf = amount_b as f64;

        let total_lp_amount_f = self.total_lp_amount as f64;
        let reserve_a_f = self.amount_a as f64;
        let reserve_b_f = self.amount_b as f64;

        if self.total_lp_amount == 0 {
            let liquidity = (input_amount_af * input_amount_bf).sqrt() as u64;
            Ok( liquidity )
        } else {
            let optimal_a_f = input_amount_af * total_lp_amount_f / reserve_a_f;
            let optimal_b_f = input_amount_bf * total_lp_amount_f / reserve_b_f;
            let liquidity = cmp::min(optimal_a_f as u64, optimal_b_f as u64);
            Ok( liquidity )
        }
    }
}