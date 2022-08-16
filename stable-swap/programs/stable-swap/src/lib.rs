use anchor_lang::prelude::*;

use anchor_spl::{
    token::{self, Mint, MintTo, Burn, Token, TokenAccount, Transfer }
};

use anchor_lang::{ Result};

use std::f64;

mod states;
pub use states::*;

// use std::str::FromStr;
// use pyth_client;

declare_id!("EDvp5kbQi9ogJtRgnxWaX58V66xEeZqWXxy4QUrhNC2V");

#[program]
pub mod stable_swap {
    use super::*;

    pub fn create_stableswap(ctx: Context<CreateStableswap>,
        amount_a: u64,
        amount_b: u64,
        amp: u64,
        fee: u8,
    ) -> Result<()> {
        if amount_a == 0 || amount_b == 0 || amp == 0 || fee == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let stableswap_pool: &mut Account<StableswapPool> = &mut ctx.accounts.stable_swap_pool;
        let author: &Signer = &ctx.accounts.author;

        let token_a: &Account<Mint> = &ctx.accounts.token_a;
        let token_b: &Account<Mint> = &ctx.accounts.token_b;
        let token_lp: &Account<Mint> = &ctx.accounts.token_lp;

        let author_ata_a: &Account<TokenAccount> = &ctx.accounts.author_ata_a;
        let author_ata_b: &Account<TokenAccount> = &ctx.accounts.author_ata_b;
        let author_ata_lp: &Account<TokenAccount> = &ctx.accounts.author_ata_lp;

        let pool_ata_a: &Account<TokenAccount> = &ctx.accounts.pool_ata_a;
        let pool_ata_b: &Account<TokenAccount> = &ctx.accounts.pool_ata_b;

        let token_program: &Program<Token> = &ctx.accounts.token_program;

        stableswap_pool.author = author.key();

        stableswap_pool.token_a = token_a.key();
        stableswap_pool.token_b = token_b.key();
        stableswap_pool.token_lp = token_lp.key();

        stableswap_pool.amp = amp;
        stableswap_pool.fee = fee;

        stableswap_pool.amount_a = amount_a;
        stableswap_pool.amount_b = amount_b;

        let amount_a_f = amount_a as f64;
        let amount_b_f = amount_b as f64;
        let lp_rewards = (((amount_a_f * amount_b_f).sqrt()) / 100000.0) as u64;

        stableswap_pool.total_lp_amount = lp_rewards;
        //-------- Add Liquidity Token A : Author -> POOL -----------------------------
        let cpi_accounts_a = Transfer {
            from: author_ata_a.to_account_info(),
            to: pool_ata_a.to_account_info(),
            authority: author.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_a = CpiContext::new(cpi_program, cpi_accounts_a);
        token::transfer(cpi_ctx_a, amount_a)?;
        //-------- Add Liquidity Token B : Author -> POOL -----------------------------
        let cpi_accounts_b = Transfer {
            from: author_ata_b.to_account_info(),
            to: pool_ata_b.to_account_info(),
            authority: author.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_b = CpiContext::new(cpi_program, cpi_accounts_b);
        token::transfer(cpi_ctx_b, amount_b)?;
        //-------- Check PDA --------------------------------
        let (_stableswap_pool_pda, stableswap_pool_bump) = Pubkey::find_program_address(
            &[
                PREFIX_POOL.as_bytes(),
                stableswap_pool.token_a.as_ref(),
                stableswap_pool.token_b.as_ref(),
                stableswap_pool.author.as_ref(),
            ],
            ctx.program_id
        );
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_POOL.as_bytes(),
            stableswap_pool.token_a.as_ref(),
            stableswap_pool.token_b.as_ref(),
            stableswap_pool.author.as_ref(),
            &[stableswap_pool_bump]
        ];
        let signer = &[&seeds[..]];
        //------ LP Tokens Mint to Authority ATA LP ------------------------
        let cpi_accounts_lp = MintTo {
            mint: token_lp.to_account_info(),
            to: author_ata_lp.to_account_info(),
            authority: stableswap_pool.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_lp = CpiContext::new_with_signer(cpi_program, cpi_accounts_lp, signer);
        token::mint_to(cpi_ctx_lp, lp_rewards)?;

        Ok(())
    }

    pub fn add_liquidity_stableswap(ctx: Context<AddLiquidityStableswap>,
        amount_a: u64,
    ) -> Result<()> {
        if amount_a == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let stableswap_pool: &mut Account<StableswapPool> = &mut ctx.accounts.stable_swap_pool;
        let adder: &Signer = &ctx.accounts.adder;

        let token_lp: &Account<Mint> = &ctx.accounts.token_lp;
        if token_lp.key() != stableswap_pool.token_lp {
            return Err(ErrorCode::LpTokenError.into());
        }

        let adder_ata_a: &Account<TokenAccount> = &ctx.accounts.adder_ata_a;
        let adder_ata_b: &Account<TokenAccount> = &ctx.accounts.adder_ata_b;
        let adder_ata_lp: &Account<TokenAccount> = &ctx.accounts.adder_ata_lp;

        let pool_ata_a: &Account<TokenAccount> = &ctx.accounts.pool_ata_a;
        let pool_ata_b: &Account<TokenAccount> = &ctx.accounts.pool_ata_b;

        let token_program: &Program<Token> = &ctx.accounts.token_program;

        let amount_a_f = amount_a as f64;
        let pool_amount_a_f = stableswap_pool.amount_a as f64;
        let pool_amount_b_f = stableswap_pool.amount_b as f64;
        let amount_b_f = (pool_amount_b_f / pool_amount_a_f) * amount_a_f;
        let amount_b = amount_b_f as u64;
        let lp_rewards = (((amount_a_f * amount_b_f).sqrt()) / 100000.0) as u64;

        stableswap_pool.amount_a += amount_a;
        stableswap_pool.amount_b += amount_b;
        stableswap_pool.total_lp_amount += lp_rewards;
        //-------- Add Liquidity Token A : Author -> POOL -----------------------------
        let cpi_accounts_a = Transfer {
            from: adder_ata_a.to_account_info(),
            to: pool_ata_a.to_account_info(),
            authority: adder.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_a = CpiContext::new(cpi_program, cpi_accounts_a);
        token::transfer(cpi_ctx_a, amount_a)?;
        //-------- Add Liquidity Token B : Author -> POOL -----------------------------
        let cpi_accounts_b = Transfer {
            from: adder_ata_b.to_account_info(),
            to: pool_ata_b.to_account_info(),
            authority: adder.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_b = CpiContext::new(cpi_program, cpi_accounts_b);
        token::transfer(cpi_ctx_b, amount_b)?;
        //-------- Check PDA --------------------------------
        let (_stableswap_pool_pda, stableswap_pool_bump) = Pubkey::find_program_address(
            &[
                PREFIX_POOL.as_bytes(),
                stableswap_pool.token_a.as_ref(),
                stableswap_pool.token_b.as_ref(),
                stableswap_pool.author.as_ref(),
            ],
            ctx.program_id
        );
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_POOL.as_bytes(),
            stableswap_pool.token_a.as_ref(),
            stableswap_pool.token_b.as_ref(),
            stableswap_pool.author.as_ref(),
            &[stableswap_pool_bump]
        ];
        let signer = &[&seeds[..]];
        //------ LP Tokens Mint to Authority ATA LP ------------------------
        let cpi_accounts_lp = MintTo {
            mint: token_lp.to_account_info(),
            to: adder_ata_lp.to_account_info(),
            authority: stableswap_pool.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_lp = CpiContext::new_with_signer(cpi_program, cpi_accounts_lp, signer);
        token::mint_to(cpi_ctx_lp, lp_rewards)?;

        Ok(())
    }

    pub fn remove_liquidity_stableswap(ctx: Context<RemoveLiquidityStableswap>,
        amount_lp: u64,
    ) -> Result<()> {
        if amount_lp == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let stableswap_pool: &mut Account<StableswapPool> = &mut ctx.accounts.stable_swap_pool;
        let taker: &Signer = &ctx.accounts.taker;

        let token_lp: &Account<Mint> = &ctx.accounts.token_lp;
        if token_lp.key() != stableswap_pool.token_lp {
            return Err(ErrorCode::LpTokenError.into());
        }
        let taker_ata_a: &Account<TokenAccount> = &ctx.accounts.taker_ata_a;
        let taker_ata_b: &Account<TokenAccount> = &ctx.accounts.taker_ata_b;
        let taker_ata_lp: &Account<TokenAccount> = &ctx.accounts.taker_ata_lp;

        let pool_ata_a: &Account<TokenAccount> = &ctx.accounts.pool_ata_a;
        let pool_ata_b: &Account<TokenAccount> = &ctx.accounts.pool_ata_b;

        let token_program: &Program<Token> = &ctx.accounts.token_program;

        let amount_lp_f = amount_lp as f64;
        let total_amount_lp_f = stableswap_pool.total_lp_amount as f64;
        let pool_amount_a_f = stableswap_pool.amount_a as f64;
        let pool_amount_b_f = stableswap_pool.amount_b as f64;

        let amount_a_f = (amount_lp_f / total_amount_lp_f) * pool_amount_a_f;
        let amount_b_f = (amount_lp_f / total_amount_lp_f) * pool_amount_b_f;

        let amount_a = amount_a_f as u64;
        let amount_b = amount_b_f as u64;
        
        stableswap_pool.amount_a -= amount_a;
        stableswap_pool.amount_b -= amount_b;
        stableswap_pool.total_lp_amount -= amount_lp;

        //-------- Check PDA --------------------------------
        let (_stableswap_pool_pda, stableswap_pool_bump) = Pubkey::find_program_address(
            &[
                PREFIX_POOL.as_bytes(),
                stableswap_pool.token_a.as_ref(),
                stableswap_pool.token_b.as_ref(),
                stableswap_pool.author.as_ref(),
            ],
            ctx.program_id
        );
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_POOL.as_bytes(),
            stableswap_pool.token_a.as_ref(),
            stableswap_pool.token_b.as_ref(),
            stableswap_pool.author.as_ref(),
            &[stableswap_pool_bump]
        ];
        let signer = &[&seeds[..]];
        //-------- Remove Liquidity Token A : Pool -> Author -----------------------------
        let cpi_accounts_a = Transfer {
            from: pool_ata_a.to_account_info(),
            to: taker_ata_a.to_account_info(),
            authority: stableswap_pool.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_a = CpiContext::new_with_signer(cpi_program, cpi_accounts_a, signer);
        token::transfer(cpi_ctx_a, amount_a)?;
        //-------- Remove Liquidity Token B : Pool -> Author -----------------------------
        let cpi_accounts_b = Transfer {
            from: pool_ata_b.to_account_info(),
            to: taker_ata_b.to_account_info(),
            authority: stableswap_pool.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_b = CpiContext::new_with_signer(cpi_program, cpi_accounts_b, signer);
        token::transfer(cpi_ctx_b, amount_b)?;
        //------ LP Tokens Burn From Author ATA LP ------------------------
        let cpi_accounts_lp = Burn {
            mint: token_lp.to_account_info(),
            from: taker_ata_lp.to_account_info(),
            authority: taker.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_lp = CpiContext::new(cpi_program, cpi_accounts_lp);
        token::burn(cpi_ctx_lp, amount_lp)?;

        Ok(())
    }

    pub fn stableswap_tokens(ctx: Context<StableswapTokens>,
        amount_src: u64,
    ) -> Result<u64> {
        if amount_src == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let stableswap_pool: &mut Account<StableswapPool> = &mut ctx.accounts.stable_swap_pool;
        let user: &Signer = &ctx.accounts.user;

        let token_src: &Account<Mint> = &ctx.accounts.token_src;
        let token_dest: &Account<Mint> = &ctx.accounts.token_dest;

        let user_ata_src: &Account<TokenAccount> = &ctx.accounts.user_ata_src;
        let user_ata_dest: &Account<TokenAccount> = &ctx.accounts.user_ata_dest;
        let pool_ata_src: &Account<TokenAccount> = &ctx.accounts.pool_ata_src;
        let pool_ata_dest: &Account<TokenAccount> = &ctx.accounts.pool_ata_dest;

        let token_program: &Program<Token> = &ctx.accounts.token_program;

        let amount_return;
        if (token_src.key() == stableswap_pool.token_a) && (token_dest.key() == stableswap_pool.token_b) {
            let amp_f = stableswap_pool.amp as f64;
            let d_f = (stableswap_pool.amount_a + stableswap_pool.amount_b) as f64;
            let amount_a_f = (stableswap_pool.amount_a + amount_src) as f64;
    
            let a: f64 = 16.0 * amp_f * amount_a_f;
            let b: f64 = 16.0 * amp_f * amount_a_f * amount_a_f - 4.0 * d_f * (4.0 * amp_f - 1.0 ) * amount_a_f;
            let c: f64 = -1.0 * d_f * d_f * d_f;
    
            let amount_b_f: f64 = (-1.0*b+(b*b-4.0*a*c).sqrt())/2.0/a;
            let amount_return_f =stableswap_pool.amount_b as f64 - amount_b_f;
            amount_return = (amount_return_f * (100.0 - stableswap_pool.fee as f64 / 10.0) / 100.0) as u64;
    
            stableswap_pool.amount_a += amount_src;
            stableswap_pool.amount_b -= amount_return;
        }else if (token_src.key() == stableswap_pool.token_b) && (token_dest.key() == stableswap_pool.token_a) {
            let amp_f = stableswap_pool.amp as f64;
            let d_f = (stableswap_pool.amount_a + stableswap_pool.amount_b) as f64;
            let amount_b_f = (stableswap_pool.amount_b + amount_src) as f64;
    
            let a: f64 = 16.0 * amp_f * amount_b_f;
            let b: f64 = 16.0 * amp_f * amount_b_f * amount_b_f - 4.0 * d_f * (4.0 * amp_f - 1.0 ) * amount_b_f;
            let c: f64 = -1.0 * d_f * d_f * d_f;
    
            let amount_a_f: f64 = (-1.0*b+(b*b-4.0*a*c).sqrt())/2.0/a;
            let amount_return_f =stableswap_pool.amount_a as f64 - amount_a_f;
            amount_return = (amount_return_f * (100.0 - stableswap_pool.fee as f64 / 10.0) / 100.0) as u64;
    
            stableswap_pool.amount_a -= amount_return;
            stableswap_pool.amount_b += amount_src;
        }else {
            return Err(ErrorCode::TokenError.into());
        }

        //-------- Transfer Token Src : User -> POOL -----------------------------
        let cpi_accounts_src = Transfer {
            from: user_ata_src.to_account_info(),
            to: pool_ata_src.to_account_info(),
            authority: user.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_src = CpiContext::new(cpi_program, cpi_accounts_src);
        token::transfer(cpi_ctx_src, amount_src)?;
        //-------- Check PDA --------------------------------
        let (_stableswap_pool_pda, stableswap_pool_bump) = Pubkey::find_program_address(
            &[
                PREFIX_POOL.as_bytes(),
                stableswap_pool.token_a.as_ref(),
                stableswap_pool.token_b.as_ref(),
                stableswap_pool.author.as_ref(),
            ],
            ctx.program_id
        );
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_POOL.as_bytes(),
            stableswap_pool.token_a.as_ref(),
            stableswap_pool.token_b.as_ref(),
            stableswap_pool.author.as_ref(),
            &[stableswap_pool_bump]
        ];
        let signer = &[&seeds[..]];
        //-------- Transfer Token Dest : POOL -> User -----------------------------
        let cpi_accounts_dest = Transfer {
            from: pool_ata_dest.to_account_info(),
            to: user_ata_dest.to_account_info(),
            authority: stableswap_pool.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_dest = CpiContext::new_with_signer(cpi_program, cpi_accounts_dest, signer);
        token::transfer(cpi_ctx_dest, amount_return)?;

        Ok(amount_return)
    }

}

#[error_code]
pub enum ErrorCode {
    #[msg("error -> Invalid amount(zero).")]
    AmountZeroError,
    #[msg("error -> Lp Token Error.")]
    LpTokenError,
    #[msg("error -> Token Error.")]
    TokenError,
}