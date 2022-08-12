use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Mint, Burn, TokenAccount, Transfer },
};

use anchor_lang::{ Result};
use spl_token::state;

use std::f64;

mod states;
pub use states::*;

use stable_swap::cpi::accounts::StableswapTokens;
use uniswap::cpi::accounts::UniswapTokens;
use test_tokens::cpi::accounts::MintToken;

declare_id!("APGnD1z3h8nei6uvZ4gGEhRhyqBy7axFdhJGM3LB61jF");

#[program]
pub mod swap_router {
    use anchor_lang::solana_program::program_pack::Pack;

    use super::*;

    pub fn swap_stableswap(ctx: Context<SwapStableswap>, amount_src: u64) -> Result<u64> {
        if amount_src == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let user = &ctx.accounts.user;
        let user_pubkey = user.key();
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        let stableswap_pool = &mut ctx.accounts.stable_swap_pool;

        let token_src: &Account<Mint> = &ctx.accounts.token_src;
        let token_dest: &Account<Mint> = &ctx.accounts.token_dest;

        let user_ata_src: &Account<TokenAccount> = &ctx.accounts.user_ata_src;
        let user_ata_dest: &Account<TokenAccount> = &ctx.accounts.user_ata_dest;
        let pool_ata_src: &Account<TokenAccount> = &ctx.accounts.pool_ata_src;
        let pool_ata_dest: &Account<TokenAccount> = &ctx.accounts.pool_ata_dest;
        let escrow_ata_src: &Account<TokenAccount> = &ctx.accounts.escrow_ata_src;
        let escrow_ata_dest: &Account<TokenAccount> = &ctx.accounts.escrow_ata_dest;

        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let associated_token_program = &ctx.accounts.associated_token_program;
        let rent = &ctx.accounts.rent;
        //-------- Transfer Token Src User -> Escrow -----------------------------
        let cpi_accounts_src = Transfer {
            from: user_ata_src.to_account_info(),
            to: escrow_ata_src.to_account_info(),
            authority: user.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_src = CpiContext::new(cpi_program, cpi_accounts_src);
        token::transfer(cpi_ctx_src, amount_src)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                user_pubkey.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            user_pubkey.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_accounts_swap = StableswapTokens{
            stable_swap_pool: stableswap_pool.to_account_info(),
            user: swap_escrow.to_account_info(),
            token_src: token_src.to_account_info(),
            token_dest: token_dest.to_account_info(),
            user_ata_src: escrow_ata_src.to_account_info(),
            user_ata_dest: escrow_ata_dest.to_account_info(),
            pool_ata_src: pool_ata_src.to_account_info(),
            pool_ata_dest: pool_ata_dest.to_account_info(),
            system_program: system_program.to_account_info(),
            token_program: token_program.to_account_info(),
            associated_token_program: associated_token_program.to_account_info(),
            rent: rent.to_account_info()
        };
        let cpi_swap = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap, amount_src)?;
        //--------- Get amount dest of escrow_ata_dest ----------------------------
        let escrow_ata_dest_info = state::Account::unpack(&escrow_ata_dest.to_account_info().data.borrow())?;
        let amount_dest = escrow_ata_dest_info.amount;
        //-------- Transfer Token Dest Escrow -> User -----------------------------
        let cpi_accounts_dest = Transfer {
            from: escrow_ata_dest.to_account_info(),
            to: user_ata_dest.to_account_info(),
            authority: swap_escrow.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_dest = CpiContext::new_with_signer(cpi_program, cpi_accounts_dest, signer);
        token::transfer(cpi_ctx_dest, amount_dest)?;

        Ok(amount_dest)
    }

    pub fn swap_uniswap(ctx: Context<SwapUniswap>, amount_src: u64) -> Result<()> {
        if amount_src == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let user = &ctx.accounts.user;
        let user_pubkey = user.key();
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        let uniswap_pool = &mut ctx.accounts.uniswap_pool;

        let token_src: &Account<Mint> = &ctx.accounts.token_src;
        let token_dest: &Account<Mint> = &ctx.accounts.token_dest;

        let user_ata_src: &Account<TokenAccount> = &ctx.accounts.user_ata_src;
        let user_ata_dest: &Account<TokenAccount> = &ctx.accounts.user_ata_dest;
        let pool_ata_src: &Account<TokenAccount> = &ctx.accounts.pool_ata_src;
        let pool_ata_dest: &Account<TokenAccount> = &ctx.accounts.pool_ata_dest;
        let escrow_ata_src: &Account<TokenAccount> = &ctx.accounts.escrow_ata_src;
        let escrow_ata_dest: &Account<TokenAccount> = &ctx.accounts.escrow_ata_dest;

        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let associated_token_program = &ctx.accounts.associated_token_program;
        let rent = &ctx.accounts.rent;
        //-------- Transfer Token Src User -> Escrow -----------------------------
        let cpi_accounts_src = Transfer {
            from: user_ata_src.to_account_info(),
            to: escrow_ata_src.to_account_info(),
            authority: user.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_src = CpiContext::new(cpi_program, cpi_accounts_src);
        token::transfer(cpi_ctx_src, amount_src)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                user_pubkey.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            user_pubkey.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Uniswap Program ----------------
        let cpi_program = ctx.accounts.uniswap_program.to_account_info();
        let cpi_accounts_swap = UniswapTokens {
            uniswap_pool: uniswap_pool.to_account_info(),
            user: swap_escrow.to_account_info(),
            token_src: token_src.to_account_info(),
            token_dest: token_dest.to_account_info(),
            user_ata_src: escrow_ata_src.to_account_info(),
            user_ata_dest: escrow_ata_dest.to_account_info(),
            pool_ata_src: pool_ata_src.to_account_info(),
            pool_ata_dest: pool_ata_dest.to_account_info(),
            system_program: system_program.to_account_info(),
            token_program: token_program.to_account_info(),
            associated_token_program: associated_token_program.to_account_info(),
            rent: rent.to_account_info()
        };
        let cpi_swap = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap, signer);
        uniswap::cpi::uniswap_tokens(cpi_swap, amount_src)?;
        //--------- Get amount dest of escrow_ata_dest ----------------------------
        let escrow_ata_dest_info = state::Account::unpack(&escrow_ata_dest.to_account_info().data.borrow())?;
        let amount_dest = escrow_ata_dest_info.amount;
        //-------- Transfer Token Dest Escrow -> User -----------------------------
        let cpi_accounts_dest = Transfer {
            from: escrow_ata_dest.to_account_info(),
            to: user_ata_dest.to_account_info(),
            authority: swap_escrow.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_dest = CpiContext::new_with_signer(cpi_program, cpi_accounts_dest, signer);
        token::transfer(cpi_ctx_dest, amount_dest)?;

        Ok(())
    }

    pub fn swap_pyth(ctx: Context<SwapPyth>,
        amount_src: u64
    ) -> Result<()> {
        // let fee = 0.5 as f64;   // 0.5%
        let user: &Signer = &ctx.accounts.user;
        let token_src = &ctx.accounts.token_src;
        let token_dest = &ctx.accounts.token_dest;
        if token_src.key() == token_dest.key() {
            return Err(ErrorCode::TokenError.into());
        }
        let user_ata_src = &ctx.accounts.user_ata_src;
        let user_ata_dest = &ctx.accounts.user_ata_dest;
        let token_state_acc = &ctx.accounts.token_state_account;
        let testtokens_program = &ctx.accounts.testtokens_program;
        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let associated_token_program = &ctx.accounts.associated_token_program;
        let ret = &ctx.accounts.rent;

        let pyth_price_info_src = &ctx.accounts.pyth_src;
        let pyth_price_data_src = &pyth_price_info_src.try_borrow_data()?;
        let pyth_price_src = pyth_client::cast::<pyth_client::Price>(pyth_price_data_src);
        let token_price_src = pyth_price_src.agg.price as f64;

        let pyth_price_info_dest = &ctx.accounts.pyth_dest;
        let pyth_price_data_dest = &pyth_price_info_dest.try_borrow_data()?;
        let pyth_price_dest = pyth_client::cast::<pyth_client::Price>(pyth_price_data_dest);
        let token_price_dest = pyth_price_dest.agg.price as f64;

        let amount_src_f = amount_src as f64;
        let amount_dest_f = ( token_price_src / token_price_dest ) * amount_src_f;
        let amount_return = amount_dest_f as u64;
        // let amount_return = (amount_dest_f * (100.0 - fee) / 100.0) as u64;
        // let amount_fee = (amount_dest_f as u64) - amount_return;

        //-------- Burn Token Src From User -----------------------------
        let cpi_accounts_src = Burn {
            mint: token_src.to_account_info(),
            from: user_ata_src.to_account_info(),
            authority: user.to_account_info()
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx_src = CpiContext::new(cpi_program, cpi_accounts_src);
        token::burn(cpi_ctx_src, amount_src)?;
        //-------- Mint Token Dest To User -----------------------------
        let cpi_accounts_dest = MintToken {
            owner: user.to_account_info(),
            state_account: token_state_acc.to_account_info(),
            user_token: user_ata_dest.to_account_info(),
            token_mint: token_dest.to_account_info(),
            system_program: system_program.to_account_info(),
            token_program: token_program.to_account_info(),
            associated_token_program: associated_token_program.to_account_info(),
            rent: ret.to_account_info()
        };
        let cpi_program = testtokens_program.to_account_info();
        let cpi_ctx_dest = CpiContext::new(cpi_program, cpi_accounts_dest);
        test_tokens::cpi::mint_token(cpi_ctx_dest, amount_return)?;

        // -------- Mint Token ( fee ) -----------------------------
        // //-------- Check PDA --------------------------------
        // let (pyth_swap_pool_pda, pyth_swap_pool_bump) = Pubkey::find_program_address(
        //     &[
        //         PREFIX_POOL.as_bytes(),
        //         pyth_swap_pool.author.as_ref()
        //     ],
        //     ctx.program_id
        // );
        // if pyth_swap_pool_pda != pyth_swap_pool.key() {
        //     return Err(ErrorCode::PDAError.into());
        // }
        // //-------- Generate Signer ---------------------------
        // let seeds = &[
        //     PREFIX_POOL.as_bytes(),
        //     pyth_swap_pool.author.as_ref(),
        //     &[pyth_swap_pool_bump]
        // ];
        // let signer = &[&seeds[..]];        
        // //-------- Mint Token Dest To Pool ( fee ) -----------------------------
        // let cpi_accounts_dest_fee = MintToken {
        //     owner: pyth_swap_pool.to_account_info(),
        //     state_account: token_state_acc.to_account_info(),
        //     user_token: pool_ata_dest.to_account_info(),
        //     token_mint: token_dest.to_account_info(),
        //     system_program: system_program.to_account_info(),
        //     token_program: token_program.to_account_info(),
        //     associated_token_program: associated_token_program.to_account_info(),
        //     rent: ret.to_account_info()
        // };
        // let cpi_program = testtokens_program.to_account_info();
        // let cpi_ctx_dest_fee = CpiContext::new_with_signer(cpi_program, cpi_accounts_dest_fee, signer);
        // test_tokens::cpi::mint_token(cpi_ctx_dest_fee, amount_fee)?;

        Ok(())
    }

    pub fn swap_lpusd_to_lpfi(ctx: Context<SwapLpusdToLpfi>,
        amount_lpusd: u64
    ) -> Result<()> {
        if amount_lpusd == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Transfer Token Lpusd User -> Escrow -----------------------------
        let cpi_accounts_lpusd = Transfer {
            from: ctx.accounts.user_ata_lpusd.to_account_info(),
            to: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpusd = CpiContext::new(cpi_program, cpi_accounts_lpusd);
        token::transfer(cpi_ctx_lpusd, amount_lpusd)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != ctx.accounts.swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_lpusd_to_usdc = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_lpusd.to_account_info(),
            token_dest: ctx.accounts.token_usdc.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_usdc.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_lpusd.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_lpusd_to_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_lpusd_to_usdc, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_lpusd_to_usdc, amount_lpusd)?;
        //--------- Get amount USDC of escrow_ata_dest ----------------------------
        let escrow_ata_usdc_info = state::Account::unpack(&ctx.accounts.escrow_ata_usdc.to_account_info().data.borrow())?;
        let amount_usdc = escrow_ata_usdc_info.amount;
        //---------- Cross-Calling Uniswap Program ----------------
        let cpi_accounts_uniswap_usdc_to_lpfi = UniswapTokens {
            uniswap_pool: ctx.accounts.uniswap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_usdc.to_account_info(),
            token_dest: ctx.accounts.token_lpfi.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_usdc.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            pool_ata_src: ctx.accounts.uniswap_pool_ata_usdc.to_account_info(),
            pool_ata_dest: ctx.accounts.uniswap_pool_ata_lpfi.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.uniswap_program.to_account_info();
        let cpi_swap_usdc_to_lpfi = CpiContext::new_with_signer(cpi_program, cpi_accounts_uniswap_usdc_to_lpfi, signer);
        uniswap::cpi::uniswap_tokens(cpi_swap_usdc_to_lpfi, amount_usdc)?;
        //--------- Get amount LpFI of escrow_ata_lpfi ----------------------------
        let escrow_ata_lpfi_info = state::Account::unpack(&ctx.accounts.escrow_ata_lpfi.to_account_info().data.borrow())?;
        let amount_lpfi = escrow_ata_lpfi_info.amount;
        //-------- Transfer Token Lpfi Escrow -> User -----------------------------
        let cpi_accounts_lpfi = Transfer {
            from: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            to: ctx.accounts.user_ata_lpfi.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpfi = CpiContext::new_with_signer(cpi_program, cpi_accounts_lpfi, signer);
        token::transfer(cpi_ctx_lpfi, amount_lpfi)?;

        Ok(())
    }

    pub fn swap_lpfi_to_lpusd(ctx: Context<SwapLpfiToLpusd>,
        amount_lpfi: u64
    ) -> Result<u64> {
        if amount_lpfi == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Transfer Token Lpfi User -> Escrow -----------------------------
        let cpi_accounts_lpfi = Transfer {
            from: ctx.accounts.user_ata_lpfi.to_account_info(),
            to: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpfi = CpiContext::new(cpi_program, cpi_accounts_lpfi);
        token::transfer(cpi_ctx_lpfi, amount_lpfi)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != ctx.accounts.swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Uniswap Program ----------------
        let cpi_accounts_uniswap_lpfi_to_usdc = UniswapTokens {
            uniswap_pool: ctx.accounts.uniswap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_lpfi.to_account_info(),
            token_dest: ctx.accounts.token_usdc.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_usdc.to_account_info(),
            pool_ata_src: ctx.accounts.uniswap_pool_ata_lpfi.to_account_info(),
            pool_ata_dest: ctx.accounts.uniswap_pool_ata_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.uniswap_program.to_account_info();
        let cpi_swap_lpfi_to_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_uniswap_lpfi_to_usdc, signer);
        uniswap::cpi::uniswap_tokens(cpi_swap_lpfi_to_usdc, amount_lpfi)?;
        //--------- Get amount USDC of escrow_ata_dest ----------------------------
        let escrow_ata_usdc_info = state::Account::unpack(&ctx.accounts.escrow_ata_usdc.to_account_info().data.borrow())?;
        let amount_usdc = escrow_ata_usdc_info.amount;
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_usdc_to_lpusd = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_usdc.to_account_info(),
            token_dest: ctx.accounts.token_lpusd.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_usdc.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_usdc.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_lpusd.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_usdc_to_lpusd = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_usdc_to_lpusd, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_usdc_to_lpusd, amount_usdc)?;
        //--------- Get amount LpFI of escrow_ata_lpfi ----------------------------
        let escrow_ata_lpusd_info = state::Account::unpack(&ctx.accounts.escrow_ata_lpusd.to_account_info().data.borrow())?;
        let amount_lpusd = escrow_ata_lpusd_info.amount;
        //-------- Transfer Token Lpusd Escrow -> User -----------------------------
        let cpi_accounts_lpusd = Transfer {
            from: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            to: ctx.accounts.user_ata_lpusd.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpusd = CpiContext::new_with_signer(cpi_program, cpi_accounts_lpusd, signer);
        token::transfer(cpi_ctx_lpusd, amount_lpusd)?;

        Ok(amount_lpusd)
    }

    pub fn swap_lpusd_to_lpsol_step1(ctx: Context<SwapLpusdToLpsolStep1>,
        amount_lpusd: u64
    ) -> Result<()> {
        if amount_lpusd == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        //-------- Transfer Token Lpusd User -> Escrow -----------------------------
        let cpi_accounts_lpusd = Transfer {
            from: ctx.accounts.user_ata_lpusd.to_account_info(),
            to: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpusd = CpiContext::new(cpi_program, cpi_accounts_lpusd);
        token::transfer(cpi_ctx_lpusd, amount_lpusd)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_lpusd_to_usdc = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_lpusd.to_account_info(),
            token_dest: ctx.accounts.token_usdc.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_usdc.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_lpusd.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_lpusd_to_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_lpusd_to_usdc, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_lpusd_to_usdc, amount_lpusd)?;
        //--------- Get amount usdc of escrow_ata_usdc ----------------------------
        let escrow_ata_usdc_info = state::Account::unpack(&ctx.accounts.escrow_ata_usdc.to_account_info().data.borrow())?;
        let amount_usdc = escrow_ata_usdc_info.amount;
        //-------- Burn Token USDC From Escrow -----------------------------
        let cpi_accounts_usdc = Burn {
            mint: ctx.accounts.token_usdc.to_account_info(),
            from: ctx.accounts.escrow_ata_usdc.to_account_info(),
            authority: swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
        token::burn(cpi_ctx_usdc, amount_usdc)?;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_usdc = &ctx.accounts.pyth_usdc;
        let pyth_price_data_usdc = &pyth_price_info_usdc.try_borrow_data()?;
        let pyth_price_usdc = pyth_client::cast::<pyth_client::Price>(pyth_price_data_usdc);
        let token_price_usdc = pyth_price_usdc.agg.price as f64;

        let pyth_price_info_wsol = &ctx.accounts.pyth_wsol;
        let pyth_price_data_wsol = &pyth_price_info_wsol.try_borrow_data()?;
        let pyth_price_wsol = pyth_client::cast::<pyth_client::Price>(pyth_price_data_wsol);
        let token_price_wsol = pyth_price_wsol.agg.price as f64;

        let amount_usdc_f = amount_usdc as f64;
        let amount_wsol_f = ( token_price_usdc / token_price_wsol ) * amount_usdc_f;
        let amount_wsol = amount_wsol_f as u64;
        //-------- Mint Token Wsol To Escrow -----------------------------
        let cpi_accounts_wsol = MintToken {
            owner: swap_escrow.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.escrow_ata_wsol.to_account_info(),
            token_mint: ctx.accounts.token_wsol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_wsol, signer);
        test_tokens::cpi::mint_token(cpi_ctx_wsol, amount_wsol)?;

        Ok(())
    }

    pub fn swap_lpusd_to_lpsol_step2(ctx: Context<SwapLpusdToLpsolStep2>) -> Result<u64> {
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        //--------- Get amount WSOL of escrow_ata_wsol ----------------------------
        let escrow_ata_wsol_info = state::Account::unpack(&ctx.accounts.escrow_ata_wsol.to_account_info().data.borrow())?;
        let amount_wsol = escrow_ata_wsol_info.amount;
        if amount_wsol == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_wsol_to_lpsol = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_wsol.to_account_info(),
            token_dest: ctx.accounts.token_lpsol.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_wsol.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_wsol.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_lpsol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_wsol_to_lpsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_wsol_to_lpsol, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_wsol_to_lpsol, amount_wsol)?;
        //--------- Get amount LPSOL of escrow_ata_lpsol ----------------------------
        let escrow_ata_lpsol_info = state::Account::unpack(&ctx.accounts.escrow_ata_lpsol.to_account_info().data.borrow())?;
        let amount_lpsol = escrow_ata_lpsol_info.amount;
        //-------- Transfer Token LpSOL Escrow -> User -----------------------------
        let cpi_accounts_lpsol = Transfer {
            from: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            to: ctx.accounts.user_ata_lpsol.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_lpsol, signer);
        token::transfer(cpi_ctx_lpsol, amount_lpsol)?;

        Ok(amount_lpsol)
    }

    pub fn swap_lpsol_to_lpusd_step1(ctx: Context<SwapLpsolToLpusdStep1>,
        amount_lpsol: u64
    ) -> Result<()> {
        if amount_lpsol == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        //-------- Transfer Token Lpsol User -> Escrow -----------------------------
        let cpi_accounts_lpsol = Transfer {
            from: ctx.accounts.user_ata_lpsol.to_account_info(),
            to: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpsol = CpiContext::new(cpi_program, cpi_accounts_lpsol);
        token::transfer(cpi_ctx_lpsol, amount_lpsol)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_lpsol_to_wsol = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_lpsol.to_account_info(),
            token_dest: ctx.accounts.token_wsol.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_wsol.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_lpsol.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_wsol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_lpsol_to_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_lpsol_to_wsol, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_lpsol_to_wsol, amount_lpsol)?;
        //--------- Get amount wsol of escrow_ata_wsol ----------------------------
        let escrow_ata_wsol_info = state::Account::unpack(&ctx.accounts.escrow_ata_wsol.to_account_info().data.borrow())?;
        let amount_wsol = escrow_ata_wsol_info.amount;
        //-------- Burn Token wsol From Escrow -----------------------------
        let cpi_accounts_wsol = Burn {
            mint: ctx.accounts.token_wsol.to_account_info(),
            from: ctx.accounts.escrow_ata_wsol.to_account_info(),
            authority: swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_wsol, signer);
        token::burn(cpi_ctx_wsol, amount_wsol)?;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_usdc = &ctx.accounts.pyth_usdc;
        let pyth_price_data_usdc = &pyth_price_info_usdc.try_borrow_data()?;
        let pyth_price_usdc = pyth_client::cast::<pyth_client::Price>(pyth_price_data_usdc);
        let token_price_usdc = pyth_price_usdc.agg.price as f64;

        let pyth_price_info_wsol = &ctx.accounts.pyth_wsol;
        let pyth_price_data_wsol = &pyth_price_info_wsol.try_borrow_data()?;
        let pyth_price_wsol = pyth_client::cast::<pyth_client::Price>(pyth_price_data_wsol);
        let token_price_wsol = pyth_price_wsol.agg.price as f64;

        let amount_wsol_f = amount_wsol as f64;
        let amount_usdc_f = ( token_price_wsol / token_price_usdc ) * amount_wsol_f;
        let amount_usdc = amount_usdc_f as u64;
        //-------- Mint Token USDC To Escrow -----------------------------
        let cpi_accounts_usdc = MintToken {
            owner: swap_escrow.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.escrow_ata_usdc.to_account_info(),
            token_mint: ctx.accounts.token_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
        test_tokens::cpi::mint_token(cpi_ctx_usdc, amount_usdc)?;

        Ok(())
    }

    pub fn swap_lpsol_to_lpusd_step2(ctx: Context<SwapLpsolToLpusdStep2>) -> Result<u64> {
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        //--------- Get amount USDC of escrow_ata_usdc ----------------------------
        let escrow_ata_usdc_info = state::Account::unpack(&ctx.accounts.escrow_ata_usdc.to_account_info().data.borrow())?;
        let amount_usdc = escrow_ata_usdc_info.amount;
        if amount_usdc == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_usdc_to_lpusd = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_usdc.to_account_info(),
            token_dest: ctx.accounts.token_lpusd.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_usdc.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_usdc.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_lpusd.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_usdc_to_lpusd = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_usdc_to_lpusd, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_usdc_to_lpusd, amount_usdc)?;
        //--------- Get amount LPUSD of escrow_ata_lpusd ----------------------------
        let escrow_ata_lpusd_info = state::Account::unpack(&ctx.accounts.escrow_ata_lpusd.to_account_info().data.borrow())?;
        let amount_lpusd = escrow_ata_lpusd_info.amount;
        //-------- Transfer Token LpUSD Escrow -> User -----------------------------
        let cpi_accounts_lpusd = Transfer {
            from: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            to: ctx.accounts.user_ata_lpusd.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpusd = CpiContext::new_with_signer(cpi_program, cpi_accounts_lpusd, signer);
        token::transfer(cpi_ctx_lpusd, amount_lpusd)?;
    
        Ok(amount_lpusd)
    }
    
    pub fn swap_lpusd_to_normal(ctx: Context<SwapLpusdToNormal>,
        amount_lpusd: u64
    ) -> Result<()> {
        if amount_lpusd == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Transfer Token Lpusd User -> Escrow -----------------------------
        let cpi_accounts_lpusd = Transfer {
            from: ctx.accounts.user_ata_lpusd.to_account_info(),
            to: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpusd = CpiContext::new(cpi_program, cpi_accounts_lpusd);
        token::transfer(cpi_ctx_lpusd, amount_lpusd)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != ctx.accounts.swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_lpusd_to_usdc = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_lpusd.to_account_info(),
            token_dest: ctx.accounts.token_usdc.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_usdc.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_lpusd.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_lpusd_to_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_lpusd_to_usdc, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_lpusd_to_usdc, amount_lpusd)?;
        //--------- Get amount USDC of escrow_ata_dest ----------------------------
        let escrow_ata_usdc_info = state::Account::unpack(&ctx.accounts.escrow_ata_usdc.to_account_info().data.borrow())?;
        let amount_usdc = escrow_ata_usdc_info.amount;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_usdc = &ctx.accounts.pyth_usdc;
        let pyth_price_data_usdc = &pyth_price_info_usdc.try_borrow_data()?;
        let pyth_price_usdc = pyth_client::cast::<pyth_client::Price>(pyth_price_data_usdc);
        let token_price_usdc = pyth_price_usdc.agg.price as f64;

        let pyth_price_info_dest = &ctx.accounts.pyth_dest;
        let pyth_price_data_dest = &pyth_price_info_dest.try_borrow_data()?;
        let pyth_price_dest = pyth_client::cast::<pyth_client::Price>(pyth_price_data_dest);
        let token_price_dest = pyth_price_dest.agg.price as f64;

        let amount_usdc_f = amount_usdc as f64;
        let amount_dest_f = ( token_price_usdc / token_price_dest ) * amount_usdc_f;
        let amount_dest = amount_dest_f as u64;
        //-------- Burn Token USDC From Escrow -----------------------------
        let cpi_accounts_usdc = Burn {
            mint: ctx.accounts.token_usdc.to_account_info(),
            from: ctx.accounts.escrow_ata_usdc.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
        token::burn(cpi_ctx_usdc, amount_usdc)?;
        //-------- Mint Token Dest To User -----------------------------
        let cpi_accounts_dest = MintToken {
            owner: ctx.accounts.user.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.user_ata_dest.to_account_info(),
            token_mint: ctx.accounts.token_dest.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_dest = CpiContext::new(cpi_program, cpi_accounts_dest);
        test_tokens::cpi::mint_token(cpi_ctx_dest, amount_dest)?;

        Ok(())
    }

    pub fn swap_normal_to_lpusd(ctx: Context<SwapNormalToLpusd>,
        amount_src: u64
    ) -> Result<u64> {
        if amount_src == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Burn Token SRC From User -----------------------------
        let cpi_accounts_src = Burn {
            mint: ctx.accounts.token_src.to_account_info(),
            from: ctx.accounts.user_ata_src.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_src = CpiContext::new(cpi_program, cpi_accounts_src);
        token::burn(cpi_ctx_src, amount_src)?;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_src = &ctx.accounts.pyth_src;
        let pyth_price_data_src = &pyth_price_info_src.try_borrow_data()?;
        let pyth_price_src = pyth_client::cast::<pyth_client::Price>(pyth_price_data_src);
        let token_price_src = pyth_price_src.agg.price as f64;

        let pyth_price_info_usdc = &ctx.accounts.pyth_usdc;
        let pyth_price_data_usdc = &pyth_price_info_usdc.try_borrow_data()?;
        let pyth_price_usdc = pyth_client::cast::<pyth_client::Price>(pyth_price_data_usdc);
        let token_price_usdc = pyth_price_usdc.agg.price as f64;

        let amount_src_f = amount_src as f64;
        let amount_usdc_f = ( token_price_src / token_price_usdc ) * amount_src_f;
        let amount_usdc = amount_usdc_f as u64;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != ctx.accounts.swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //-------- Mint Token Usdc To Escrow -----------------------------
        let cpi_accounts_usdc = MintToken {
            owner: ctx.accounts.swap_escrow.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.escrow_ata_usdc.to_account_info(),
            token_mint: ctx.accounts.token_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
        test_tokens::cpi::mint_token(cpi_ctx_usdc, amount_usdc)?;
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_usdc_to_lpusd = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_usdc.to_account_info(),
            token_dest: ctx.accounts.token_lpusd.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_usdc.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_usdc.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_lpusd.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_usdc_to_lpusd = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_usdc_to_lpusd, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_usdc_to_lpusd, amount_usdc)?;
        //--------- Get amount USDC of escrow_ata_dest ----------------------------
        let escrow_ata_lpusd_info = state::Account::unpack(&ctx.accounts.escrow_ata_lpusd.to_account_info().data.borrow())?;
        let amount_lpusd = escrow_ata_lpusd_info.amount;
        //-------- Transfer Token lpusd Escrow -> User -----------------------------
        let cpi_accounts_lpusd = Transfer {
            from: ctx.accounts.escrow_ata_lpusd.to_account_info(),
            to: ctx.accounts.user_ata_lpusd.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpusd = CpiContext::new_with_signer(cpi_program, cpi_accounts_lpusd, signer);
        token::transfer(cpi_ctx_lpusd, amount_lpusd)?;

        Ok(amount_lpusd)
    }

    pub fn swap_lpsol_to_normal(ctx: Context<SwapLpsolToNormal>,
        amount_lpsol: u64
    ) -> Result<()> {
        if amount_lpsol == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Transfer Token Lpsol User -> Escrow -----------------------------
        let cpi_accounts_lpsol = Transfer {
            from: ctx.accounts.user_ata_lpsol.to_account_info(),
            to: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpsol = CpiContext::new(cpi_program, cpi_accounts_lpsol);
        token::transfer(cpi_ctx_lpsol, amount_lpsol)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != ctx.accounts.swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_lpsol_to_wsol = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_lpsol.to_account_info(),
            token_dest: ctx.accounts.token_wsol.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_wsol.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_lpsol.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_wsol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_lpsol_to_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_lpsol_to_wsol, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_lpsol_to_wsol, amount_lpsol)?;
        //--------- Get amount WSOL of escrow_ata_wsol ----------------------------
        let escrow_ata_wsol_info = state::Account::unpack(&ctx.accounts.escrow_ata_wsol.to_account_info().data.borrow())?;
        let amount_wsol = escrow_ata_wsol_info.amount;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_wsol = &ctx.accounts.pyth_wsol;
        let pyth_price_data_wsol = &pyth_price_info_wsol.try_borrow_data()?;
        let pyth_price_wsol = pyth_client::cast::<pyth_client::Price>(pyth_price_data_wsol);
        let token_price_wsol = pyth_price_wsol.agg.price as f64;

        let pyth_price_info_dest = &ctx.accounts.pyth_dest;
        let pyth_price_data_dest = &pyth_price_info_dest.try_borrow_data()?;
        let pyth_price_dest = pyth_client::cast::<pyth_client::Price>(pyth_price_data_dest);
        let token_price_dest = pyth_price_dest.agg.price as f64;

        let amount_wsol_f = amount_wsol as f64;
        let amount_dest_f = ( token_price_wsol / token_price_dest ) * amount_wsol_f;
        let amount_dest = amount_dest_f as u64;
        //-------- Burn Token WSOL From Escrow -----------------------------
        let cpi_accounts_wsol = Burn {
            mint: ctx.accounts.token_wsol.to_account_info(),
            from: ctx.accounts.escrow_ata_wsol.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_wsol, signer);
        token::burn(cpi_ctx_wsol, amount_wsol)?;
        //-------- Mint Token Dest To User -----------------------------
        let cpi_accounts_dest = MintToken {
            owner: ctx.accounts.user.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.user_ata_dest.to_account_info(),
            token_mint: ctx.accounts.token_dest.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_dest = CpiContext::new(cpi_program, cpi_accounts_dest);
        test_tokens::cpi::mint_token(cpi_ctx_dest, amount_dest)?;

        Ok(())
    }

    pub fn swap_normal_to_lpsol(ctx: Context<SwapNormalToLpsol>,
        amount_src: u64
    ) -> Result<()> {
        if amount_src == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Burn Token SRC From USER -----------------------------
        let cpi_accounts_src = Burn {
            mint: ctx.accounts.token_src.to_account_info(),
            from: ctx.accounts.user_ata_src.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_src = CpiContext::new(cpi_program, cpi_accounts_src);
        token::burn(cpi_ctx_src, amount_src)?;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_src = &ctx.accounts.pyth_src;
        let pyth_price_data_src = &pyth_price_info_src.try_borrow_data()?;
        let pyth_price_src = pyth_client::cast::<pyth_client::Price>(pyth_price_data_src);
        let token_price_src = pyth_price_src.agg.price as f64;

        let pyth_price_info_wsol = &ctx.accounts.pyth_wsol;
        let pyth_price_data_wsol = &pyth_price_info_wsol.try_borrow_data()?;
        let pyth_price_wsol = pyth_client::cast::<pyth_client::Price>(pyth_price_data_wsol);
        let token_price_wsol = pyth_price_wsol.agg.price as f64;

        let amount_src_f = amount_src as f64;
        let amount_wsol_f = ( token_price_src / token_price_wsol ) * amount_src_f;
        let amount_wsol = amount_wsol_f as u64;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != ctx.accounts.swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //-------- Mint Token Wsol To Escrow -----------------------------
        let cpi_accounts_wsol = MintToken {
            owner: ctx.accounts.swap_escrow.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.escrow_ata_wsol.to_account_info(),
            token_mint: ctx.accounts.token_wsol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_wsol, signer);
        test_tokens::cpi::mint_token(cpi_ctx_wsol, amount_wsol)?;
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_wsol_to_lpsol = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_wsol.to_account_info(),
            token_dest: ctx.accounts.token_lpsol.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_wsol.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_wsol.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_lpsol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_wsol_to_lpsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_wsol_to_lpsol, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_wsol_to_lpsol, amount_wsol)?;
        //--------- Get amount LPSOL of escrow_ata_lpsol ----------------------------
        let escrow_ata_lpsol_info = state::Account::unpack(&ctx.accounts.escrow_ata_lpsol.to_account_info().data.borrow())?;
        let amount_lpsol = escrow_ata_lpsol_info.amount;
        //-------- Transfer Token LpSOL Escrow -> User -----------------------------
        let cpi_accounts_lpsol = Transfer {
            from: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            to: ctx.accounts.user_ata_lpsol.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_lpsol, signer);
        token::transfer(cpi_ctx_lpsol, amount_lpsol)?;

        Ok(())
    }

    pub fn swap_lpfi_to_normal(ctx: Context<SwapLpfiToNormal>,
        amount_lpfi: u64
    ) -> Result<()> {
        if amount_lpfi == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Transfer Token Lpfi User -> Escrow -----------------------------
        let cpi_accounts_lpfi = Transfer {
            from: ctx.accounts.user_ata_lpfi.to_account_info(),
            to: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpfi = CpiContext::new(cpi_program, cpi_accounts_lpfi);
        token::transfer(cpi_ctx_lpfi, amount_lpfi)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != ctx.accounts.swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Uniswap Program ----------------
        let cpi_accounts_uniswap_lpfi_to_usdc = UniswapTokens {
            uniswap_pool: ctx.accounts.uniswap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_lpfi.to_account_info(),
            token_dest: ctx.accounts.token_usdc.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_usdc.to_account_info(),
            pool_ata_src: ctx.accounts.uniswap_pool_ata_lpfi.to_account_info(),
            pool_ata_dest: ctx.accounts.uniswap_pool_ata_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.uniswap_program.to_account_info();
        let cpi_swap_lpfi_to_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_uniswap_lpfi_to_usdc, signer);
        uniswap::cpi::uniswap_tokens(cpi_swap_lpfi_to_usdc, amount_lpfi)?;
        //--------- Get amount USDC of escrow_ata_dest ----------------------------
        let escrow_ata_usdc_info = state::Account::unpack(&ctx.accounts.escrow_ata_usdc.to_account_info().data.borrow())?;
        let amount_usdc = escrow_ata_usdc_info.amount;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_usdc = &ctx.accounts.pyth_usdc;
        let pyth_price_data_usdc = &pyth_price_info_usdc.try_borrow_data()?;
        let pyth_price_usdc = pyth_client::cast::<pyth_client::Price>(pyth_price_data_usdc);
        let token_price_usdc = pyth_price_usdc.agg.price as f64;

        let pyth_price_info_dest = &ctx.accounts.pyth_dest;
        let pyth_price_data_dest = &pyth_price_info_dest.try_borrow_data()?;
        let pyth_price_dest = pyth_client::cast::<pyth_client::Price>(pyth_price_data_dest);
        let token_price_dest = pyth_price_dest.agg.price as f64;

        let amount_usdc_f = amount_usdc as f64;
        let amount_dest_f = ( token_price_usdc / token_price_dest ) * amount_usdc_f;
        let amount_dest = amount_dest_f as u64;
        //-------- Burn Token USDC From Escrow -----------------------------
        let cpi_accounts_usdc = Burn {
            mint: ctx.accounts.token_usdc.to_account_info(),
            from: ctx.accounts.escrow_ata_usdc.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
        token::burn(cpi_ctx_usdc, amount_usdc)?;
        //-------- Mint Token Dest To User -----------------------------
        let cpi_accounts_dest = MintToken {
            owner: ctx.accounts.user.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.user_ata_dest.to_account_info(),
            token_mint: ctx.accounts.token_dest.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_dest = CpiContext::new(cpi_program, cpi_accounts_dest);
        test_tokens::cpi::mint_token(cpi_ctx_dest, amount_dest)?;

        Ok(())
    }

    pub fn swap_normal_to_lpfi(ctx: Context<SwapNormalToLpfi>,
        amount_src: u64
    ) -> Result<()> {
        if amount_src == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Burn Token SRC From USER -----------------------------
        let cpi_accounts_src = Burn {
            mint: ctx.accounts.token_src.to_account_info(),
            from: ctx.accounts.user_ata_src.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_src = CpiContext::new(cpi_program, cpi_accounts_src);
        token::burn(cpi_ctx_src, amount_src)?;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_src = &ctx.accounts.pyth_src;
        let pyth_price_data_src = &pyth_price_info_src.try_borrow_data()?;
        let pyth_price_src = pyth_client::cast::<pyth_client::Price>(pyth_price_data_src);
        let token_price_src = pyth_price_src.agg.price as f64;

        let pyth_price_info_usdc = &ctx.accounts.pyth_usdc;
        let pyth_price_data_usdc = &pyth_price_info_usdc.try_borrow_data()?;
        let pyth_price_usdc = pyth_client::cast::<pyth_client::Price>(pyth_price_data_usdc);
        let token_price_usdc = pyth_price_usdc.agg.price as f64;

        let amount_src_f = amount_src as f64;
        let amount_usdc_f = ( token_price_src / token_price_usdc ) * amount_src_f;
        let amount_usdc = amount_usdc_f as u64;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != ctx.accounts.swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //-------- Mint Token Wsol To Escrow -----------------------------
        let cpi_accounts_usdc = MintToken {
            owner: ctx.accounts.swap_escrow.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.escrow_ata_usdc.to_account_info(),
            token_mint: ctx.accounts.token_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
        test_tokens::cpi::mint_token(cpi_ctx_usdc, amount_usdc)?;
        //---------- Cross-Calling Uniswap Program ----------------
        let cpi_accounts_uniswap_usdc_to_lpfi = UniswapTokens {
            uniswap_pool: ctx.accounts.uniswap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_usdc.to_account_info(),
            token_dest: ctx.accounts.token_lpfi.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_usdc.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            pool_ata_src: ctx.accounts.uniswap_pool_ata_usdc.to_account_info(),
            pool_ata_dest: ctx.accounts.uniswap_pool_ata_lpfi.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.uniswap_program.to_account_info();
        let cpi_swap_usdc_to_lpfi = CpiContext::new_with_signer(cpi_program, cpi_accounts_uniswap_usdc_to_lpfi, signer);
        uniswap::cpi::uniswap_tokens(cpi_swap_usdc_to_lpfi, amount_usdc)?;
        //--------- Get amount LpFI of escrow_ata_lpfi ----------------------------
        let escrow_ata_lpfi_info = state::Account::unpack(&ctx.accounts.escrow_ata_lpfi.to_account_info().data.borrow())?;
        let amount_lpfi = escrow_ata_lpfi_info.amount;
        //-------- Transfer Token Lpfi Escrow -> User -----------------------------
        let cpi_accounts_lpfi = Transfer {
            from: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            to: ctx.accounts.user_ata_lpfi.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpfi = CpiContext::new_with_signer(cpi_program, cpi_accounts_lpfi, signer);
        token::transfer(cpi_ctx_lpfi, amount_lpfi)?;

        Ok(())
    }

    pub fn swap_lpsol_to_lpfi_step1(ctx: Context<SwapLpsolToLpfiStep1>,
        amount_lpsol: u64
    ) -> Result<()> {
        if amount_lpsol == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        //-------- Transfer Token Lpsol User -> Escrow -----------------------------
        let cpi_accounts_lpsol = Transfer {
            from: ctx.accounts.user_ata_lpsol.to_account_info(),
            to: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpsol = CpiContext::new(cpi_program, cpi_accounts_lpsol);
        token::transfer(cpi_ctx_lpsol, amount_lpsol)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_lpsol_to_wsol = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_lpsol.to_account_info(),
            token_dest: ctx.accounts.token_wsol.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_wsol.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_lpsol.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_wsol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_lpsol_to_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_lpsol_to_wsol, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_lpsol_to_wsol, amount_lpsol)?;
        //--------- Get amount wsol of escrow_ata_wsol ----------------------------
        let escrow_ata_wsol_info = state::Account::unpack(&ctx.accounts.escrow_ata_wsol.to_account_info().data.borrow())?;
        let amount_wsol = escrow_ata_wsol_info.amount;
        //-------- Burn Token wsol From Escrow -----------------------------
        let cpi_accounts_wsol = Burn {
            mint: ctx.accounts.token_wsol.to_account_info(),
            from: ctx.accounts.escrow_ata_wsol.to_account_info(),
            authority: swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_wsol, signer);
        token::burn(cpi_ctx_wsol, amount_wsol)?;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_usdc = &ctx.accounts.pyth_usdc;
        let pyth_price_data_usdc = &pyth_price_info_usdc.try_borrow_data()?;
        let pyth_price_usdc = pyth_client::cast::<pyth_client::Price>(pyth_price_data_usdc);
        let token_price_usdc = pyth_price_usdc.agg.price as f64;

        let pyth_price_info_wsol = &ctx.accounts.pyth_wsol;
        let pyth_price_data_wsol = &pyth_price_info_wsol.try_borrow_data()?;
        let pyth_price_wsol = pyth_client::cast::<pyth_client::Price>(pyth_price_data_wsol);
        let token_price_wsol = pyth_price_wsol.agg.price as f64;

        let amount_wsol_f = amount_wsol as f64;
        let amount_usdc_f = ( token_price_wsol / token_price_usdc ) * amount_wsol_f;
        let amount_usdc = amount_usdc_f as u64;
        //-------- Mint Token USDC To Escrow -----------------------------
        let cpi_accounts_usdc = MintToken {
            owner: swap_escrow.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.escrow_ata_usdc.to_account_info(),
            token_mint: ctx.accounts.token_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
        test_tokens::cpi::mint_token(cpi_ctx_usdc, amount_usdc)?;

        Ok(())
    }

    pub fn swap_lpsol_to_lpfi_step2(ctx: Context<SwapLpsolToLpfiStep2>) -> Result<()> {
        //--------- Get amount USDC of escrow_ata_dest ----------------------------
        let escrow_ata_usdc_info = state::Account::unpack(&ctx.accounts.escrow_ata_usdc.to_account_info().data.borrow())?;
        let amount_usdc = escrow_ata_usdc_info.amount;
        if amount_usdc == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Uniswap Program ----------------
        let cpi_accounts_uniswap_usdc_to_lpfi = UniswapTokens {
            uniswap_pool: ctx.accounts.uniswap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_usdc.to_account_info(),
            token_dest: ctx.accounts.token_lpfi.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_usdc.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            pool_ata_src: ctx.accounts.uniswap_pool_ata_usdc.to_account_info(),
            pool_ata_dest: ctx.accounts.uniswap_pool_ata_lpfi.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.uniswap_program.to_account_info();
        let cpi_swap_usdc_to_lpfi = CpiContext::new_with_signer(cpi_program, cpi_accounts_uniswap_usdc_to_lpfi, signer);
        uniswap::cpi::uniswap_tokens(cpi_swap_usdc_to_lpfi, amount_usdc)?;
        //--------- Get amount LpFI of escrow_ata_lpfi ----------------------------
        let escrow_ata_lpfi_info = state::Account::unpack(&ctx.accounts.escrow_ata_lpfi.to_account_info().data.borrow())?;
        let amount_lpfi = escrow_ata_lpfi_info.amount;
        //-------- Transfer Token Lpfi Escrow -> User -----------------------------
        let cpi_accounts_lpfi = Transfer {
            from: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            to: ctx.accounts.user_ata_lpfi.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpfi = CpiContext::new_with_signer(cpi_program, cpi_accounts_lpfi, signer);
        token::transfer(cpi_ctx_lpfi, amount_lpfi)?;
    
        Ok(())
    }
    
    pub fn swap_lpfi_to_lpsol_step1(ctx: Context<SwapLpfiToLpsolStep1>,
        amount_lpfi: u64
    ) -> Result<()> {
        if amount_lpfi == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        //-------- Transfer Token Lpfi User -> Escrow -----------------------------
        let cpi_accounts_lpfi = Transfer {
            from: ctx.accounts.user_ata_lpfi.to_account_info(),
            to: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpfi = CpiContext::new(cpi_program, cpi_accounts_lpfi);
        token::transfer(cpi_ctx_lpfi, amount_lpfi)?;
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Uniswap Program ----------------
        let cpi_accounts_uniswap_lpfi_to_usdc = UniswapTokens {
            uniswap_pool: ctx.accounts.uniswap_pool.to_account_info(),
            user: swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_lpfi.to_account_info(),
            token_dest: ctx.accounts.token_usdc.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_lpfi.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_usdc.to_account_info(),
            pool_ata_src: ctx.accounts.uniswap_pool_ata_lpfi.to_account_info(),
            pool_ata_dest: ctx.accounts.uniswap_pool_ata_usdc.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.uniswap_program.to_account_info();
        let cpi_swap_lpfi_to_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_uniswap_lpfi_to_usdc, signer);
        uniswap::cpi::uniswap_tokens(cpi_swap_lpfi_to_usdc, amount_lpfi)?;
        //--------- Get amount usdc of escrow_ata_usdc ----------------------------
        let escrow_ata_usdc_info = state::Account::unpack(&ctx.accounts.escrow_ata_usdc.to_account_info().data.borrow())?;
        let amount_usdc = escrow_ata_usdc_info.amount;
        //-------- Burn Token USDC From Escrow -----------------------------
        let cpi_accounts_usdc = Burn {
            mint: ctx.accounts.token_usdc.to_account_info(),
            from: ctx.accounts.escrow_ata_usdc.to_account_info(),
            authority: swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
        token::burn(cpi_ctx_usdc, amount_usdc)?;
        //--------- Pyth Price -----------------------------------------
        let pyth_price_info_usdc = &ctx.accounts.pyth_usdc;
        let pyth_price_data_usdc = &pyth_price_info_usdc.try_borrow_data()?;
        let pyth_price_usdc = pyth_client::cast::<pyth_client::Price>(pyth_price_data_usdc);
        let token_price_usdc = pyth_price_usdc.agg.price as f64;
    
        let pyth_price_info_wsol = &ctx.accounts.pyth_wsol;
        let pyth_price_data_wsol = &pyth_price_info_wsol.try_borrow_data()?;
        let pyth_price_wsol = pyth_client::cast::<pyth_client::Price>(pyth_price_data_wsol);
        let token_price_wsol = pyth_price_wsol.agg.price as f64;
    
        let amount_usdc_f = amount_usdc as f64;
        let amount_wsol_f = ( token_price_usdc / token_price_wsol ) * amount_usdc_f;
        let amount_wsol = amount_wsol_f as u64;
        //-------- Mint Token Wsol To Escrow -----------------------------
        let cpi_accounts_wsol = MintToken {
            owner: swap_escrow.to_account_info(),
            state_account: ctx.accounts.token_state_account.to_account_info(),
            user_token: ctx.accounts.escrow_ata_wsol.to_account_info(),
            token_mint: ctx.accounts.token_wsol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.testtokens_program.to_account_info();
        let cpi_ctx_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_wsol, signer);
        test_tokens::cpi::mint_token(cpi_ctx_wsol, amount_wsol)?;
    
        Ok(())
    }
    
    pub fn swap_lpfi_to_lpsol_step2(ctx: Context<SwapLpfiToLpsolStep2>) -> Result<()> {
        let swap_escrow = &mut ctx.accounts.swap_escrow;
        //--------- Get amount WSOL of escrow_ata_wsol ----------------------------
        let escrow_ata_wsol_info = state::Account::unpack(&ctx.accounts.escrow_ata_wsol.to_account_info().data.borrow())?;
        let amount_wsol = escrow_ata_wsol_info.amount;
        if amount_wsol == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }
        //-------- Check PDA --------------------------------
        let (swap_escrow_pda, swap_escrow_bump) = Pubkey::find_program_address(
            &[
                PREFIX_ESCROW.as_bytes(),
                ctx.accounts.user.key.as_ref()
            ],
            ctx.program_id
        );
        if swap_escrow_pda != swap_escrow.key() {
            return Err(ErrorCode::SwapEscrowPDAError.into());
        }
        //-------- Generate Signer ---------------------------
        let seeds = &[
            PREFIX_ESCROW.as_bytes(),
            ctx.accounts.user.key.as_ref(),
            &[swap_escrow_bump]
        ];
        let signer = &[&seeds[..]];
        //---------- Cross-Calling Stable Swap Program ----------------
        let cpi_accounts_swap_wsol_to_lpsol = StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.swap_escrow.to_account_info(),
            token_src: ctx.accounts.token_wsol.to_account_info(),
            token_dest: ctx.accounts.token_lpsol.to_account_info(),
            user_ata_src: ctx.accounts.escrow_ata_wsol.to_account_info(),
            user_ata_dest: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            pool_ata_src: ctx.accounts.stableswap_pool_ata_wsol.to_account_info(),
            pool_ata_dest: ctx.accounts.stableswap_pool_ata_lpsol.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_swap_wsol_to_lpsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap_wsol_to_lpsol, signer);
        stable_swap::cpi::stableswap_tokens(cpi_swap_wsol_to_lpsol, amount_wsol)?;
        //--------- Get amount LPSOL of escrow_ata_lpsol ----------------------------
        let escrow_ata_lpsol_info = state::Account::unpack(&ctx.accounts.escrow_ata_lpsol.to_account_info().data.borrow())?;
        let amount_lpsol = escrow_ata_lpsol_info.amount;
        //-------- Transfer Token LpSOL Escrow -> User -----------------------------
        let cpi_accounts_lpsol = Transfer {
            from: ctx.accounts.escrow_ata_lpsol.to_account_info(),
            to: ctx.accounts.user_ata_lpsol.to_account_info(),
            authority: ctx.accounts.swap_escrow.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_lpsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_lpsol, signer);
        token::transfer(cpi_ctx_lpsol, amount_lpsol)?;
    
        Ok(())
    }
    
}

#[error_code]
pub enum ErrorCode {
    #[msg("error -> Invalid token.")]
    TokenError,
    #[msg("error -> Invalid amount(zero).")]
    AmountZeroError,
    #[msg("error -> Invalid swap-escrow account.")]
    SwapEscrowPDAError,
    #[msg("error -> Amount Overflow")]
    AmountOverflow,
}