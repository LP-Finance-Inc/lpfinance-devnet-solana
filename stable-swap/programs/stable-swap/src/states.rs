use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };
use anchor_spl::associated_token::AssociatedToken;

use std::mem::size_of;

pub const PREFIX_POOL: &str = "stable-swap";

#[derive(Accounts)]
pub struct CreateStableswap<'info> {
    #[account(
        init, 
        seeds = [
            PREFIX_POOL.as_bytes(),
            token_a.key().as_ref(),
            token_b.key().as_ref(),
            author.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<StableswapPool>(),
        payer = author
    )]
    pub stable_swap_pool: Box<Account<'info, StableswapPool>>,
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
        associated_token::authority = stable_swap_pool,
    )]    
    pub pool_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = author,
        associated_token::mint = token_b,
        associated_token::authority = stable_swap_pool,
    )]    
    pub pool_ata_b: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = author,
        mint::decimals = 5,
        mint::authority = stable_swap_pool,
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
pub struct AddLiquidityStableswap<'info> {
    #[account(mut)]
    pub stable_swap_pool: Account<'info, StableswapPool>,
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
        associated_token::authority = stable_swap_pool,
    )]    
    pub pool_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_b,
        associated_token::authority = stable_swap_pool,
    )]
    pub pool_ata_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        mint::decimals = 5,
        mint::authority = stable_swap_pool,
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
pub struct RemoveLiquidityStableswap<'info> {
    #[account(mut)]
    pub stable_swap_pool: Account<'info, StableswapPool>,
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
        associated_token::authority = stable_swap_pool,
    )]    
    pub pool_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_b,
        associated_token::authority = stable_swap_pool,
    )]
    pub pool_ata_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        mint::decimals = 5,
        mint::authority = stable_swap_pool,
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
pub struct StableswapTokens<'info> {
    #[account(mut)]
    pub stable_swap_pool: Box<Account<'info, StableswapPool>>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub token_src: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_dest: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
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
        associated_token::authority = stable_swap_pool,
    )]
    pub pool_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_dest,
        associated_token::authority = stable_swap_pool,
    )]
    pub pool_ata_dest: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct StableswapPool {
    pub author: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub token_lp: Pubkey,
    pub total_lp_amount: u64,
    pub amp: u64,
    pub fee: u8,
}

impl StableswapPool {
    // swap rate of Lptoken -> quote token
    pub fn get_swap_rate(&self, amount_swap: u64) -> Result<u64> {
        let amp_f = self.amp as f64;
        let d_f = (self.amount_a + self.amount_b) as f64;
        let amount_a_f = (self.amount_a + amount_swap) as f64;

        let a: f64 = 16.0 * amp_f * amount_a_f;
        let b: f64 = 16.0 * amp_f * amount_a_f * amount_a_f - 4.0 * d_f * (4.0 * amp_f - 1.0 ) * amount_a_f;
        let c: f64 = -1.0 * d_f * d_f * d_f;

        let amount_b_f: f64 = (-1.0*b+(b*b-4.0*a*c).sqrt())/2.0/a;
        let amount_return_f =self.amount_b as f64 - amount_b_f;
        let amount_return = (amount_return_f * (100.0 - self.fee as f64 / 10.0) / 100.0) as u64;

        Ok( amount_return )
    }   
}