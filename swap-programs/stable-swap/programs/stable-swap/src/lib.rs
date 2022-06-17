use anchor_lang::prelude::*;
// use stable_swap_anchor::{StableSwap, SwapInfo};

mod state;

pub use state::*;

declare_id!("7W99dMuo82w7DzKZrB7ye6YoaGmdsE3t3XAyuRK3rjvr");

#[program]
pub mod stable_swap {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        nonce: u8, 
        amp_factor: u64, 
        fees: SwapFees
    ) -> Result<()> {
        msg!("INITIALIZE Pool");
        let cpi_program = ctx.accounts.swap_program.to_account_info();

        msg!("INITIALIZE Pool2");
        let cpi_accounts = stable_swap_anchor::Initialize {
            token_program: ctx.accounts.token_program.to_account_info(),
            swap: ctx.accounts.swap.to_account_info(),
            swap_authority: ctx.accounts.swap_authority.to_account_info(),
            admin: ctx.accounts.admin.to_account_info(),
            token_a:  stable_swap_anchor::InitToken{
                reserve: ctx.accounts.token_a.reserve.to_account_info(),
                fees: ctx.accounts.token_a.fees.to_account_info(),
                mint: ctx.accounts.token_a.mint.to_account_info()
            },
            token_b: stable_swap_anchor::InitToken{
                reserve: ctx.accounts.token_b.reserve.to_account_info(),
                fees: ctx.accounts.token_b.fees.to_account_info(),
                mint: ctx.accounts.token_b.mint.to_account_info()
            },
            pool_mint: ctx.accounts.pool_mint.to_account_info(),
            output_lp: ctx.accounts.output_lp.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        stable_swap_anchor::initialize(cpi_ctx, nonce, amp_factor, fees.into())?;
        Ok(())
    }
}

