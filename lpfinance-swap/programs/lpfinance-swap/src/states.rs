use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };
use anchor_spl::associated_token::AssociatedToken;

pub const PREFIX: &str = "lpfiswap";
pub const PRICE_MULTIPLIER: u128 = 100000000; // 10**8 

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
    pub authority: Signer<'info>,
    // Quote token
    pub tokena_mint: Box<Account<'info, Mint>>,
    // Owner's token
    pub tokenb_mint: Box<Account<'info, Mint>>,
    // If it was already created, throw error
    #[account(
        init,
        seeds = [PREFIX.as_bytes(), tokena_mint.key().as_ref(), tokenb_mint.key().as_ref()],
        space = PoolInfo::LEN,
        bump,
        payer = authority
    )]
    pub liquidity_pool: Box<Account<'info, PoolInfo>>,
    #[account(mut)]
    pub user_tokena: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_tokenb: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub tokena_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub tokenb_pool: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
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
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
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
    pub const LEN: usize = 2 * 32 + 8;
}

#[account]
#[derive(Default)]
pub struct PoolInfo {
    pub owner: Pubkey,
    pub tokena_mint: Pubkey,
    pub tokenb_mint: Pubkey,
    pub tokena_amount: u64,
    pub tokenb_amount: u64,
    pub initial_tokena_amount: u64,
    pub initial_tokenb_amount: u64
}

impl PoolInfo {
    pub const LEN: usize = 3 * 32 + 8 * 2 + 8;

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
}