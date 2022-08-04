
//! Accounts state.

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount }
};

use swap_base::{self, Pool};

use lpfinance_swap::{self, PoolInfo};

use lpfinance_tokens::{self, TokenStateAccount};
use lpfinance_tokens::program::LpfinanceTokens;

use solend::program::Solend;
use solend::{self};

use apricot::program::Apricot;
use apricot::{self};

pub const PREFIX: &str = "cbs-pda";
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const U64_LENGTH: usize = 8;
const U8_LENGTH: usize = 1;
const BOOL_LENGTH: usize =1;
// const TITLE_LENGTH: usize = 4*2;

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Token program authority
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
    // Token program authority
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
    pub cbs_pda: AccountInfo<'info>,
    // LpSOL POOL
    #[account(
        init,
        token::mint = lpsol_mint,
        token::authority = cbs_pda,
        payer = authority
    )]
    pub pool_lpsol: Box<Account<'info, TokenAccount>>,

    // LpUSD POOL
    #[account(
        init,
        token::mint = lpusd_mint,
        token::authority = cbs_pda,
        payer = authority
    )]
    pub pool_lpusd: Box<Account<'info, TokenAccount>>,
    // LpFi POOL
    #[account(
        init,
        token::mint = lpfi_mint,
        token::authority = cbs_pda,
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
    pub cbs_pda: AccountInfo<'info>,

    // wSOL POOL
    #[account(
        init,
        token::mint = wsol_mint,
        token::authority = cbs_pda,
        payer = authority
    )]
    pub pool_wsol: Box<Account<'info, TokenAccount>>,
    // Ray POOL
    #[account(
        init,
        token::mint = ray_mint,
        token::authority = cbs_pda,
        payer = authority
    )]
    pub pool_ray: Box<Account<'info, TokenAccount>>,
    // mSOL POOL
    #[account(
        init,
        token::mint = msol_mint,
        token::authority = cbs_pda,
        payer = authority
    )]
    pub pool_msol: Box<Account<'info, TokenAccount>>,
    // srm POOL
    #[account(
        init,
        token::mint = srm_mint,
        token::authority = cbs_pda,
        payer = authority
    )]
    pub pool_srm: Box<Account<'info, TokenAccount>>,
    // scnsol POOL
    #[account(
        init,
        token::mint = scnsol_mint,
        token::authority = cbs_pda,
        payer = authority
    )]
    pub pool_scnsol: Box<Account<'info, TokenAccount>>,
    // stsol POOL
    #[account(
        init,
        token::mint = stsol_mint,
        token::authority = cbs_pda,
        payer = authority
    )]
    pub pool_stsol: Box<Account<'info, TokenAccount>>,    

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct CreateSolendCBSAccount<'info> {
    /// CHECK: This is safe
    #[account(mut, seeds = [PREFIX.as_ref()], bump)]
    pub cbs_pda: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub solend_account: AccountInfo<'info>,
    pub solend_program: Program<'info, Solend>,
    #[account(mut, has_one = owner)]
    pub config: Box<Account<'info, Config>>,  
    // Signer
    #[account(mut)]
    pub owner: Signer<'info>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateApricotCBSAccount<'info> {
    /// CHECK: This is safe
    #[account(mut, seeds = [PREFIX.as_ref()], bump)]
    pub cbs_pda: AccountInfo<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub apricot_account: AccountInfo<'info>,
    pub apricot_program: Program<'info, Apricot>,
    #[account(mut, has_one = owner)]
    pub config: Box<Account<'info, Config>>,  
    // Signer
    #[account(mut)]
    pub owner: Signer<'info>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
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
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,  
    #[account(mut)]
    pub user_authority: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX.as_ref()], bump)]
    pub cbs_pda: AccountInfo<'info>,

    // User token account for collateral
    #[account(
        mut,
        constraint = user_collateral.owner == user_authority.key(),
        constraint = user_collateral.mint == collateral_mint.key()
    )]
    pub user_collateral : Box<Account<'info,TokenAccount>>,
    // Collateral token address
    #[account(mut)]
    pub collateral_mint: Account<'info,Mint>,
    // CBS protocol pool
    #[account(
        mut,
        constraint = collateral_pool.mint == collateral_mint.key(),
        constraint = collateral_pool.owner == cbs_pda.key()
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
    /// CHECK: this is safe
    #[account(mut, seeds = [PREFIX.as_bytes()], bump)]
    pub cbs_pda: AccountInfo<'info>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,
    // Token program's Signer
    #[account(mut)]
    pub tokens_state: Box<Account<'info, TokenStateAccount>>,
    #[account(mut)]
    pub lptoken_config: Box<Account<'info, lpfinance_tokens::Config>>,
    #[account(
        init_if_needed,
        payer = user_authority,
        associated_token::mint = lptoken_mint,
        associated_token::authority = user_authority
    )]
    pub user_lptoken : Box<Account<'info,TokenAccount>>,
    // LpUSD-USDC stableswap pool
    pub stable_lpusd_pool: Box<Account<'info, Pool>>,
    // LpSOL-wSOL stableswap pool
    pub stable_lpsol_pool: Box<Account<'info, Pool>>,
    #[account(mut)]
    pub lptoken_mint: Box<Account<'info,Mint>>,
    /// CHECK: pyth
    pub pyth_ray_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_usdc_account: AccountInfo<'info>,
    // Price feed for wSOL
    /// CHECK: pyth
    pub pyth_sol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_msol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_srm_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_scnsol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_stsol_account: AccountInfo<'info>,
    // LpFi<->USDC pool
    pub liquidity_pool: Box<Account<'info, PoolInfo>>,
    #[account(mut)]
    pub solend_config: Box<Account<'info, solend::Config>>,
    #[account(mut)]
    pub apricot_config: Box<Account<'info, apricot::Config>>,
    // Programs and Sysvars
    pub lptokens_program: Program<'info, LpfinanceTokens>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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
    /// CHECK: this is safe
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub cbs_pda: AccountInfo<'info>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub user_dest : Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub dest_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub dest_mint: Box<Account<'info,Mint>>,
    // LpUSD-USDC stableswap pool
    pub stable_lpusd_pool: Box<Account<'info, Pool>>,
    // LpSOL-wSOL stableswap pool
    pub stable_lpsol_pool: Box<Account<'info, Pool>>,
    /// CHECK: pyth
    pub pyth_ray_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_usdc_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_sol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_msol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_srm_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_scnsol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_stsol_account: AccountInfo<'info>,
    // LpFi<->USDC pool
    pub liquidity_pool: Box<Account<'info, PoolInfo>>,

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
pub struct WithdrawLending<'info> {
     #[account(mut)]
    pub user_authority: Signer<'info>,
    // state account for user's wallet
    #[account(
        mut,
        constraint = user_account.owner == user_authority.key()
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// CHECK: this is safe
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub cbs_pda: AccountInfo<'info>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,

    // LpUSD-USDC stableswap pool
    pub stable_lpusd_pool: Box<Account<'info, Pool>>,
    // LpSOL-wSOL stableswap pool
    pub stable_lpsol_pool: Box<Account<'info, Pool>>,
    /// CHECK: pyth
    pub pyth_ray_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_usdc_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_sol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_msol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_srm_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_scnsol_account: AccountInfo<'info>,
    /// CHECK: pyth
    pub pyth_stsol_account: AccountInfo<'info>,
    // LpFi<->USDC pool
    pub liquidity_pool: Box<Account<'info, PoolInfo>>,
    
    #[account(mut)]
    pub dest_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub cbs_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub solend_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub apricot_pool: Box<Account<'info, TokenAccount>>,
    
    #[account(mut)]
    pub solend_config: Box<Account<'info, solend::Config>>,
    #[account(mut)]
    pub solend_account: Box<Account<'info, solend::UserAccount>>,
    #[account(mut)]
    pub solend_state_account: Box<Account<'info, solend::StateAccount>>,
    #[account(mut)]
    pub apricot_config: Box<Account<'info, apricot::Config>>,
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
pub struct LiquidateCollateral<'info> {
    #[account(mut)]
    pub user_account: Box<Account<'info, UserAccount>>,
    #[account(mut)]
    pub state_account: Box<Account<'info, StateAccount>>,
    /// CHECK: auction
    #[account(mut)]
    pub auction_account: AccountInfo<'info>,

    #[account(mut)]
    pub auction_msol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_ray: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_wsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_srm: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_scnsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub auction_stsol: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub cbs_msol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_ray: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_wsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_srm: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_scnsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_stsol: Box<Account<'info, TokenAccount>>,

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
    pub auction_lpfi: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub cbs_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub cbs_lpfi: Box<Account<'info, TokenAccount>>,

    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct RepayToken<'info> {
    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(mut)]
    pub user_dest : Box<Account<'info,TokenAccount>>,
    #[account(mut)]
    pub dest_mint: Box<Account<'info,Mint>>,
    // state account for user's wallet
    #[account(mut,
        seeds = [PREFIX.as_bytes()],
        bump
    )]
    pub state_account: Box<Account<'info, StateAccount>>,
    #[account(mut)]
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

#[account]
#[derive(Default)]
pub struct Config {
    pub total_borrowed_lpusd: u64,
    pub total_borrowed_lpsol: u64,

    pub total_deposited_wsol: u64,
    pub total_deposited_ray: u64,
    pub total_deposited_msol: u64,
    pub total_deposited_srm: u64,
    pub total_deposited_scnsol: u64,
    pub total_deposited_stsol: u64,

    pub total_deposited_lpsol: u64,
    pub total_deposited_lpusd: u64,
    pub total_deposited_lpfi: u64,

    pub lpsol_mint: Pubkey,
    pub lpusd_mint: Pubkey,
    pub lpfi_mint: Pubkey,

    pub ray_mint: Pubkey,
    pub wsol_mint: Pubkey,
    pub msol_mint: Pubkey,
    pub srm_mint: Pubkey,
    pub scnsol_mint: Pubkey,
    pub stsol_mint: Pubkey,

    pub pool_ray: Pubkey,
    pub pool_wsol: Pubkey,
    pub pool_msol: Pubkey,
    pub pool_srm: Pubkey,
    pub pool_scnsol: Pubkey,
    pub pool_stsol: Pubkey,
    pub pool_lpsol: Pubkey,
    pub pool_lpusd: Pubkey,
    pub pool_lpfi: Pubkey,

    pub owner: Pubkey,
    pub liquidation_run: bool,

}

impl Config {
    pub const LEN:usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 19
        + U64_LENGTH * 11
        + BOOL_LENGTH;

    pub fn exist_token (&self, dest_mint: Pubkey) -> Result<bool> {
        if dest_mint == self.wsol_mint || 
           dest_mint == self.msol_mint || 
           dest_mint == self.ray_mint || 
           dest_mint == self.srm_mint || 
           dest_mint == self.scnsol_mint || 
           dest_mint == self.stsol_mint {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_key_borrowed_amount (
        &self, 
        dest_mint: Pubkey
    ) -> Result<u64> {
        let mut _deposited_amount = 0;

        if dest_mint.key() == self.lpsol_mint {
            _deposited_amount = self.total_borrowed_lpsol;

        }else if dest_mint.key() == self.lpusd_mint {
            _deposited_amount = self.total_borrowed_lpusd;
        }

        Ok(_deposited_amount)
    }

    pub fn get_key_deposited_amount (
        &self, 
        dest_mint: Pubkey
    ) -> Result<u64> {
        let mut _deposited_amount = 0;
        if dest_mint.key() == self.ray_mint {
            _deposited_amount = self.total_deposited_ray;
        } else if dest_mint.key() == self.wsol_mint {
            _deposited_amount = self.total_deposited_wsol;
        } else if dest_mint.key() == self.msol_mint {
            _deposited_amount = self.total_deposited_msol;
        } else if dest_mint.key() == self.srm_mint {
            _deposited_amount = self.total_deposited_srm;
        } else if dest_mint.key() == self.scnsol_mint {
            _deposited_amount = self.total_deposited_scnsol;
        } else if dest_mint.key() == self.stsol_mint {
            _deposited_amount = self.total_deposited_stsol;

        } else if dest_mint.key() == self.lpsol_mint {
            _deposited_amount = self.total_deposited_lpsol;

        }else if dest_mint.key() == self.lpusd_mint {
            _deposited_amount = self.total_deposited_lpusd;

        }else if dest_mint.key() == self.lpfi_mint {
            _deposited_amount = self.total_deposited_lpfi;
        }

        Ok(_deposited_amount)
    }

    pub fn update_borrowed_amount (
        &mut self, 
        amount: u64,
        dest_mint: Pubkey
    ) -> Result<bool> {
        if dest_mint.key() == self.lpsol_mint {
            self.total_borrowed_lpsol = amount;
        }else if dest_mint.key() == self.lpusd_mint {
            self.total_borrowed_lpusd = amount;
        }

        Ok(true)
    }

    pub fn update_total_deposited_amount (
        &mut self, 
        amount: u64,
        dest_mint: Pubkey
    ) -> Result<bool> {
        if dest_mint.key() == self.ray_mint {
            self.total_deposited_ray = amount;
        } else if dest_mint.key() == self.wsol_mint {
            self.total_deposited_wsol = amount;
        } else if dest_mint.key() == self.msol_mint {
            self.total_deposited_msol = amount;
        } else if dest_mint.key() == self.srm_mint {
            self.total_deposited_srm = amount;

        } else if dest_mint.key() == self.scnsol_mint {
            self.total_deposited_scnsol = amount;

        } else if dest_mint.key() == self.stsol_mint {
            self.total_deposited_stsol = amount;
        }

        Ok(true)
    }

    pub fn update_total_lp_deposited_amount (
        &mut self, 
        amount: u64,
        dest_mint: Pubkey
    ) -> Result<bool> {
        if dest_mint.key() == self.lpusd_mint {
            self.total_deposited_lpusd = amount;
        } else if dest_mint.key() == self.lpsol_mint {
            self.total_deposited_lpsol = amount;
        } else if dest_mint.key() == self.lpfi_mint {
            self.total_deposited_lpfi = amount;
        } 
        Ok(true)
    }
}

#[account]
#[derive(Default)]
pub struct OracleConfig {
    pub owner: Pubkey,

    pub pyth_ray_account: Pubkey,
    pub pyth_usdc_account: Pubkey,
    pub pyth_sol_account: Pubkey,

    pub pyth_msol_account: Pubkey,
    pub pyth_srm_account: Pubkey,
    pub pyth_scnsol_account: Pubkey,
    pub pyth_stsol_account: Pubkey
}

impl OracleConfig {
    pub const LEN:usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 8;
}

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub owner: Pubkey,
    // Number to present the current Liquidate process
    // NOTE: need to check solend & apricot amount
    // 0: status that be able to deposit & borrow & withdraw & repay
    // 1: Burn LpUSD from Auction
    // 2: Swap LpToken (LpSOL, LpFi) to LpUSD
    // 3: Swap Tokens (Ray, wSOL, mSOL, stSOL, scnSOL, srm) to LpUSD
    // 4: Transfer LpUSD from CBS to Auction
    pub step_num: u8,

    pub borrowed_lpusd: u64,
    pub borrowed_lpsol: u64,
    // deposited amount
    pub ray_amount: u64,
    pub wsol_amount: u64,
    pub msol_amount: u64,
    pub srm_amount: u64,
    pub scnsol_amount: u64,
    pub stsol_amount: u64,

    pub lpsol_amount: u64,
    pub lpusd_amount: u64,
    pub lpfi_amount: u64,

    // solend & apricot
    pub lending_ray_amount: u64,
    pub lending_wsol_amount: u64,
    pub lending_msol_amount: u64,
    pub lending_srm_amount: u64,
    pub lending_scnsol_amount: u64,
    pub lending_stsol_amount: u64
}

impl UserAccount {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + U64_LENGTH * 28 
        + PUBLIC_KEY_LENGTH // owner pubkey
        + U8_LENGTH;        // Liquidate process


    pub fn update_borrowed_amount (
        &mut self, 
        amount: u64,
        dest_mint: Pubkey,
        config: &mut Account<Config>
    ) -> Result<bool> {
        if dest_mint.key() == config.lpsol_mint {
            self.borrowed_lpsol = amount;
        }else if dest_mint.key() == config.lpusd_mint {
            self.borrowed_lpusd = amount;
        }

        Ok(true)
    }

    pub fn get_key_borrowed_amount (
        &self, 
        dest_mint: Pubkey,
        config: &mut Account<Config>
    ) -> Result<u64> {
        let mut _deposited_amount = 0;

        if dest_mint.key() == config.lpsol_mint {
            _deposited_amount = config.total_borrowed_lpsol;

        }else if dest_mint.key() == config.lpusd_mint {
            _deposited_amount = config.total_borrowed_lpusd;
        }

        Ok(_deposited_amount)
    }

    pub fn get_key_lending_amount (
        &self, 
        dest_mint: Pubkey,
        config: &mut Account<Config>
    ) -> Result<u64> {
        let mut _lending_amount = 0;
        if dest_mint.key() == config.ray_mint {
            _lending_amount = self.lending_ray_amount;
        } else if dest_mint.key() == config.wsol_mint {
            _lending_amount = self.lending_wsol_amount;
        } else if dest_mint.key() == config.msol_mint {
            _lending_amount = self.lending_msol_amount;
        } else if dest_mint.key() == config.srm_mint {
            _lending_amount = self.lending_srm_amount;

        } else if dest_mint.key() == config.scnsol_mint {
            _lending_amount = self.lending_scnsol_amount;

        } else if dest_mint.key() == config.stsol_mint {
            _lending_amount = self.lending_stsol_amount;

        } 

        Ok(_lending_amount)
    }

    pub fn get_key_amount (
        &self, 
        dest_mint: Pubkey,
        config: &mut Account<Config>
    ) -> Result<u64> {
        let mut _amount = 0;
        if dest_mint.key() == config.ray_mint {
            _amount = self.ray_amount;
        } else if dest_mint.key() == config.wsol_mint {
            _amount = self.wsol_amount;
        } else if dest_mint.key() == config.msol_mint {
            _amount = self.msol_amount;
        } else if dest_mint.key() == config.srm_mint {
            _amount = self.srm_amount;

        } else if dest_mint.key() == config.scnsol_mint {
            _amount = self.scnsol_amount;

        } else if dest_mint.key() == config.stsol_mint {
            _amount = self.stsol_amount;

        } else if dest_mint.key() == config.lpfi_mint {
            _amount = self.lpfi_amount;
        } else if dest_mint.key() == config.lpsol_mint {
            _amount = self.lpsol_amount;
        } else if dest_mint.key() == config.lpusd_mint {
            _amount = self.lpusd_amount;
        }

        Ok(_amount)
    }

    pub fn update_lending_amount (
        &mut self, 
        amount: u64,
        dest_mint: Pubkey,
        config: &mut Account<Config>
    ) -> Result<bool> {
        if dest_mint.key() == config.ray_mint {
            self.lending_ray_amount = amount;
        } else if dest_mint.key() == config.wsol_mint {
            self.lending_wsol_amount = amount;
        } else if dest_mint.key() == config.msol_mint {
            self.lending_msol_amount = amount;
        } else if dest_mint.key() == config.srm_mint {
            self.lending_srm_amount = amount;

        } else if dest_mint.key() == config.scnsol_mint {
            self.lending_scnsol_amount = amount;

        } else if dest_mint.key() == config.stsol_mint {
            self.lending_stsol_amount = amount;
        }

        Ok(true)
    }

    pub fn update_deposited_amount (
        &mut self, 
        amount: u64,
        dest_mint: Pubkey,
        config: &mut Account<Config>
    ) -> Result<bool> {
        if dest_mint.key() == config.ray_mint {
            self.ray_amount = amount;
        } else if dest_mint.key() == config.wsol_mint {
            self.wsol_amount = amount;
        } else if dest_mint.key() == config.msol_mint {
            self.msol_amount = amount;
        } else if dest_mint.key() == config.srm_mint {
            self.srm_amount = amount;

        } else if dest_mint.key() == config.scnsol_mint {
            self.scnsol_amount = amount;

        } else if dest_mint.key() == config.stsol_mint {
            self.stsol_amount = amount;
        }

        Ok(true)
    }

    pub fn update_lp_deposited_amount (
        &mut self, 
        amount: u64,
        dest_mint: Pubkey,
        config: &mut Account<Config>
    ) -> Result<bool> {
        if dest_mint.key() == config.lpusd_mint {
            self.lpusd_amount = amount;
        } else if dest_mint.key() == config.lpsol_mint {
            self.lpsol_amount = amount;
        } else if dest_mint.key() == config.lpfi_mint {
            self.lpfi_amount = amount;
        }

        Ok(true)
    }
}