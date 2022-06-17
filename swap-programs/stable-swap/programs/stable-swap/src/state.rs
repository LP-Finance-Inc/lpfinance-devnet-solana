
//! Accounts state.

use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount }
};
use stable_swap_anchor::{StableSwap, SwapInfo};


#[derive(Accounts)]
pub struct CreatePool<'info> {
    // Token program authority
    // #[account(mut)]
    // pub authority: Signer<'info>,
    /// The swap.
    #[account(mut)]
    pub swap: Box<Account<'info, SwapInfo>>,
    /// The authority of the swap.    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub swap_authority: UncheckedAccount<'info>,
    /// The admin of the swap.
    pub admin: Box<Account<'info, TokenAccount>>,
    /// The A token of the swap.
    pub token_a: InitToken<'info>,
    /// The B token of the swap.
    pub token_b: InitToken<'info>,
    /// The pool mint of the swap.
    pub pool_mint: Box<Account<'info, Mint>>,
    /// The output account for LP tokens.
    pub output_lp: Box<Account<'info, TokenAccount>>,
    /// The spl_token program.
    pub token_program: Program<'info, Token>,
    pub swap_program: Program<'info, StableSwap>,
    pub system_program: Program<'info, System>,
}

/// Token accounts for initializing a [crate::SwapInfo].
#[derive(Accounts)]
pub struct InitToken<'info> {
    /// The token account for the pool's reserves of this token.
    pub reserve: Box<Account<'info, TokenAccount>>,
    /// The token account for the fees associated with the token.
    pub fees: Box<Account<'info, TokenAccount>>,
    /// The mint of the token.
    pub mint: Box<Account<'info, Mint>>,
}

/// Fees struct
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct SwapFees {
    /// Admin trade fee numerator
    pub admin_trade_fee_numerator: u64,
    /// Admin trade fee denominator
    pub admin_trade_fee_denominator: u64,
    /// Admin withdraw fee numerator
    pub admin_withdraw_fee_numerator: u64,
    /// Admin withdraw fee denominator
    pub admin_withdraw_fee_denominator: u64,
    /// Trade fee numerator
    pub trade_fee_numerator: u64,
    /// Trade fee denominator
    pub trade_fee_denominator: u64,
    /// Withdraw fee numerator
    pub withdraw_fee_numerator: u64,
    /// Withdraw fee denominator
    pub withdraw_fee_denominator: u64,
}

impl SwapFees {
    /// Number of bytes in a serialized [SwapFees].
    pub const LEN: usize = 8 * 8;
}

impl From<SwapFees> for stable_swap_client::fees::Fees {
    fn from(e: SwapFees) -> Self {
        let SwapFees {
            admin_trade_fee_numerator,
            admin_trade_fee_denominator,
            admin_withdraw_fee_numerator,
            admin_withdraw_fee_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            withdraw_fee_numerator,
            withdraw_fee_denominator,
        } = e;
        Self {
            admin_trade_fee_numerator,
            admin_trade_fee_denominator,
            admin_withdraw_fee_numerator,
            admin_withdraw_fee_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            withdraw_fee_numerator,
            withdraw_fee_denominator,
        }
    }
}
