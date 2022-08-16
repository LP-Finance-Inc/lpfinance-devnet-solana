use anchor_lang::prelude::*;
// use pyth_client;
use anchor_spl::token::{self, Transfer, TokenAccount };

mod states;
pub use states::*;

mod oracle;
pub use oracle::*;

use cbs_protocol::cpi::accounts::{ UpdateUserAccount };
use cbs_protocol::{self};

use stable_swap::{self, StableswapPool};
use uniswap::{self, UniswapPool};

// use lpfinance_swap::cpi::accounts::LiquidateToken;
// use lpfinance_swap::program::LpfinanceSwap;
// use lpfinance_swap::{self};

use lpfinance_tokens::cpi::accounts::{BurnLpToken};
use lpfinance_tokens::{self};

declare_id!("DbQju5NRVunuGz5aKdaqAaUfWSMRsy6hdZQ2zFDkGL9y");


pub const PRICE_DENOMINATOR: u128 = 100000000; // 10 ^ 8
// which means token 1
pub const PRICE_UNIT: u64 = 1000000000; // 10^9

#[program]
pub mod lpusd_auction {
    use super::*;
    // Initialize auction program with config
    pub fn initialize(
        ctx: Context<Initialize>,
    ) -> Result<()> {
        msg!("INITIALIZE Auction");

        let config = &mut ctx.accounts.config;
        config.owner = ctx.accounts.authority.key();

        config.total_deposited_lpusd = 0;        
        config.total_lpusd = 0;
        config.total_percent = DENOMINATOR;
        config.epoch_duration = 7 * 24 * 3600;
        config.last_epoch_percent = 0;
        config.last_epoch_profit = 0;

        Ok(())
    }

    // Create LpToken's ATA for auction pool
    pub fn create_lptoken_ata(
        ctx: Context<CreateLpTokenATA>
    ) -> Result<()> {
        msg!("INITIALIZE LpToken ATAs");

        let config = &mut ctx.accounts.config;

        if config.owner != ctx.accounts.authority.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // token mint        
        config.lpusd_mint = ctx.accounts.lpusd_mint.key();
        config.lpsol_mint = ctx.accounts.lpsol_mint.key();
        config.lpfi_mint = ctx.accounts.lpfi_mint.key();

        // lptoken pool
        config.pool_lpsol = ctx.accounts.pool_lpsol.key();
        config.pool_lpusd = ctx.accounts.pool_lpusd.key();
        config.pool_lpfi = ctx.accounts.pool_lpfi.key();  

        Ok(())
    }

    // Create Token's ATA for auction pool
    pub fn create_token_ata(
        ctx: Context<CreateTokenATA>
    ) -> Result<()> {
        msg!("INITIALIZE Token ATAs");

        let config = &mut ctx.accounts.config;

        if config.owner != ctx.accounts.authority.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // token mint
        config.wsol_mint = ctx.accounts.wsol_mint.key();
        config.ray_mint = ctx.accounts.ray_mint.key();
        config.msol_mint = ctx.accounts.msol_mint.key();
        config.srm_mint = ctx.accounts.srm_mint.key();
        config.scnsol_mint = ctx.accounts.scnsol_mint.key();
        config.stsol_mint = ctx.accounts.stsol_mint.key();

        config.pool_ray = ctx.accounts.pool_ray.key();
        config.pool_wsol = ctx.accounts.pool_wsol.key();
        config.pool_msol = ctx.accounts.pool_msol.key();
        config.pool_srm = ctx.accounts.pool_srm.key();
        config.pool_scnsol = ctx.accounts.pool_scnsol.key();
        config.pool_stsol = ctx.accounts.pool_stsol.key();

        Ok(())
    }

    // Init user account
    pub fn init_user_account(
        ctx: Context<InitUserAccount>
    ) -> Result<()> {
        msg!("INITIALIZE User Account");
        let user_account = &mut ctx.accounts.user_account;
        user_account.owner = ctx.accounts.user_authority.key();

        user_account.lpusd_amount = 0;
        Ok(())
    }

    // Close user account
    pub fn delete_user_account(_ctx: Context<DeleteUserAccount>) -> Result<()> {
        Ok(())
    }

    // Deposit LpUSD token into auction pool
    pub fn deposit_lpusd(
        ctx: Context<DepositLpUSD>,
        amount: u64
    ) -> Result<()> {
        if amount < 1 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        if ctx.accounts.user_lpusd.amount < amount {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_lpusd.to_account_info(),
            to: ctx.accounts.pool_lpusd.to_account_info(),
            authority: ctx.accounts.user_authority.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        let user_account: &mut Account<UserAccount> = &mut ctx.accounts.user_account;
        let config: &mut Account<Config> = &mut ctx.accounts.config;

        let cur_amount_f: f64 = user_account.lpusd_amount as f64;
        let amount_f: f64 = amount as f64;
        let denominator_f: f64 = DENOMINATOR as f64;
        let withdraw_percent_f: f64 = config.total_percent as f64;

        let result_f = cur_amount_f + amount_f * denominator_f / withdraw_percent_f;
        if result_f < 0.0 {
            return Err(ErrorCode::InvalidResult.into());            
        }

        user_account.lpusd_amount = result_f as u64;

        let config = &mut ctx.accounts.config;
        config.total_lpusd = config.total_lpusd + amount;
        Ok(())
    }

    // Withdraw LpUSD with interest amount
    pub fn withdraw_lpusd(        
        ctx: Context<WithdrawLpUSD>,
        amount: u64
    ) -> Result<()> {
        // NOTE: check if able to withdraw
        if amount < 1 {
            return Err(ErrorCode::InvalidAmount.into());
        }        

        if ctx.accounts.pool_lpusd.amount < amount {
            return Err(ErrorCode::InsufficientPoolAmount.into());
        }


        let user_account: &mut Account<UserAccount> = &mut ctx.accounts.user_account;
        let config: &mut Account<Config> = &mut ctx.accounts.config;

        let cur_amount_f: f64 = user_account.lpusd_amount as f64;
        let amount_f: f64 = amount as f64;
        let denominator_f: f64 = DENOMINATOR as f64;
        let withdraw_percent_f: f64 = config.total_percent as f64;

        let total_withdrawable_amount: f64 = cur_amount_f * withdraw_percent_f / denominator_f;
        msg!("Total withdraw amount: !!{:?}!!", total_withdrawable_amount.to_string());

        if amount > total_withdrawable_amount as u64 {
            return Err(ErrorCode::ExceedAmount.into());
        }

        let (program_authority, program_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if program_authority != ctx.accounts.auction_pda.to_account_info().key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_lpusd.to_account_info(),
            to: ctx.accounts.user_lpusd.to_account_info(),
            authority: ctx.accounts.auction_pda.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;
        
        // LpUSD amount after removed amount from user account data
        let remain_amount_f: f64 = cur_amount_f - amount_f * denominator_f / withdraw_percent_f;

        // update user account
        if remain_amount_f > 0.0 {
            user_account.lpusd_amount = remain_amount_f as u64;
        } else {
            user_account.lpusd_amount = 0;
        }
        // update config
        config.total_lpusd = config.total_lpusd - amount;

        Ok(())
    }

    // Liquidate starts
    pub fn burn_lpusd_liquidate(
        ctx: Context<BurnLpUSDForLiquidate>
    ) -> Result<()> {

        let user_account: &mut Account<UserAccount> = &mut ctx.accounts.user_account;
        let cbs_account: &mut Account<cbs_protocol::UserAccount> = &mut ctx.accounts.cbs_account;
        let lpusd_ata: &mut Account<TokenAccount> = &mut ctx.accounts.lpusd_ata;

        let pyth_ray_account: &AccountInfo = &ctx.accounts.pyth_ray_account;
        let pyth_usdc_account: &AccountInfo = &ctx.accounts.pyth_usdc_account;
        let pyth_sol_account: &AccountInfo = &ctx.accounts.pyth_sol_account;
        let pyth_msol_account: &AccountInfo = &ctx.accounts.pyth_msol_account;
        let pyth_srm_account: &AccountInfo = &ctx.accounts.pyth_srm_account;
        let pyth_scnsol_account: &AccountInfo = &ctx.accounts.pyth_scnsol_account;
        let pyth_stsol_account: &AccountInfo = &ctx.accounts.pyth_stsol_account;

        let liquidity_pool: &Account<UniswapPool> = &ctx.accounts.liquidity_pool;
        let stable_lpusd_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpusd_pool;
        let stable_lpsol_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpsol_pool;
        // let config = &mut ctx.accounts.config;

        
        let is_liquidatable = cbs_account.check_liquidatable()?;

        if is_liquidatable == false {
            return Err(ErrorCode::ReadyErrorForLiquidate.into());
        }

        // borrowed
        let borrowed_lpusd: u64 = cbs_account.borrowed_lpusd;
        let borrowed_lpsol: u64 = cbs_account.borrowed_lpsol;

        if cbs_account.is_empty_account()? == true
        {
            return Err(ErrorCode::EmptyAccount.into());
        }

        let mut _ltv: u64 = 0;
        let mut _total_price: f64 = 0.0;
        let mut _borrowed_total: f64 = 0.0;

        (_ltv, _total_price, _borrowed_total) = cbs_account.get_ltv_from_auction(
            liquidity_pool,
            stable_lpusd_pool,
            stable_lpsol_pool,
            pyth_ray_account,
            pyth_usdc_account,
            pyth_sol_account,
            pyth_msol_account,
            pyth_srm_account,
            pyth_scnsol_account,
            pyth_stsol_account
        )?;        

        msg!("LTV {} Camount {} Bamount {}", _ltv, _total_price, _borrowed_total);

        // If LTV < 94, not be able to liquidate
        // if _ltv < LTV_PERMISSION {
        //     return Err(ErrorCode::NotEnoughLTV.into());
        // }

        // Burn token users' deposited LpUSD
        if user_account.lpusd_amount > lpusd_ata.amount {
            return Err(ErrorCode::InsufficientPoolAmount.into());
        }

        // Get signer
        let (program_authority, program_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if program_authority != ctx.accounts.auction_pda.to_account_info().key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];
        // End to get signer

        // Burn
        if borrowed_lpusd > 0 {
            let cpi_program = ctx.accounts.lptokens_program.to_account_info();
            let cpi_accounts = BurnLpToken {
                owner: ctx.accounts.auction_pda.to_account_info(),
                token_mint: ctx.accounts.lpusd_mint.to_account_info(),
                user_token: ctx.accounts.lpusd_ata.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info()
            };
    
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            lpfinance_tokens::cpi::burn_lptoken(cpi_ctx, borrowed_lpusd)?;
        }

        
        let cpi_program = ctx.accounts.cbs_program.to_account_info();
        let cpi_accounts = UpdateUserAccount {
            user_account: cbs_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        if borrowed_lpsol > 0 {
            cbs_protocol::cpi::liquidate_step1(cpi_ctx, borrowed_lpusd)?;
        } else {
            cbs_protocol::cpi::liquidate_step2(cpi_ctx, 0)?;
        }
        
        Ok(())
    }

    // Liquidate starts
    pub fn burn_lpsol_liquidate(
        ctx: Context<BurnLpSOLForLiquidate>
    ) -> Result<()> {
        let cbs_account: &mut Account<cbs_protocol::UserAccount> = &mut ctx.accounts.cbs_account;

        let stable_lpusd_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpusd_pool;
        let stable_lpsol_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpsol_pool;
        // let config = &mut ctx.accounts.config;

        let swap_escrow = &mut ctx.accounts.swap_escrow;
        let stable_swap_pool = &mut ctx.accounts.stable_swap_pool;
        let token_state_account = &mut ctx.accounts.token_state_account;
        let token_lpsol = &mut ctx.accounts.token_lpsol;
        let token_wsol = &mut ctx.accounts.token_wsol;
        let token_lpusd = &mut ctx.accounts.token_lpusd;
        let token_usdc = &mut ctx.accounts.token_usdc;
        let pyth_usdc = &mut ctx.accounts.pyth_usdc;
        let pyth_wsol = &mut ctx.accounts.pyth_wsol;
        let auction_ata_lpsol = &ctx.accounts.auction_ata_lpsol;
        let auction_ata_lpusd = &ctx.accounts.auction_ata_lpusd;
        let stableswap_pool_ata_lpsol = &ctx.accounts.stableswap_pool_ata_lpsol;
        let stableswap_pool_ata_lpusd = &ctx.accounts.stableswap_pool_ata_lpusd;
        let stableswap_pool_ata_wsol = &ctx.accounts.stableswap_pool_ata_wsol;
        let stableswap_pool_ata_usdc = &ctx.accounts.stableswap_pool_ata_usdc;
        let escrow_ata_lpsol = &ctx.accounts.escrow_ata_lpsol;
        let escrow_ata_lpusd = &ctx.accounts.escrow_ata_lpusd;
        let escrow_ata_wsol = &ctx.accounts.escrow_ata_wsol;
        let escrow_ata_usdc = &ctx.accounts.escrow_ata_usdc;
        let stableswap_program = &ctx.accounts.stableswap_program;
        let testtokens_program = &ctx.accounts.testtokens_program;
        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let associated_token_program = &ctx.accounts.associated_token_program;
        let rent = &ctx.accounts.rent;

        if cbs_account.step_num != 1 {
            return Err(ErrorCode::InvalidLiquidateNum.into());
        }
        
        // borrowed
        let borrowed_lpsol: f64 = cbs_account.borrowed_lpsol as f64;
        let lpusd_swap_amount: f64 = stable_lpusd_pool.get_swap_rate(PRICE_UNIT)? as f64;
        let lpsol_swap_amount: f64 = stable_lpsol_pool.get_swap_rate(PRICE_UNIT)? as f64;
        let swap_amount = (borrowed_lpsol * lpsol_swap_amount / lpusd_swap_amount) as u64;

        if swap_amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        // Get signer
        let (program_authority, program_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);

        if program_authority != ctx.accounts.auction_pda.to_account_info().key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];
        // End to get signer

        {
            let cpi_program = ctx.accounts.swaprouter_program.to_account_info();
            let cpi_accounts = swap_router::cpi::accounts::SwapLpusdToLpsolStep1 {
                user: ctx.accounts.auction_pda.to_account_info(),
                swap_escrow: swap_escrow.to_account_info(),
                stable_swap_pool: stable_swap_pool.to_account_info(),
                token_state_account: token_state_account.to_account_info(),
                token_lpusd: token_lpusd.to_account_info(),
                token_wsol: token_wsol.to_account_info(),
                token_usdc: token_usdc.to_account_info(),
                pyth_wsol: pyth_wsol.to_account_info(),
                pyth_usdc: pyth_usdc.to_account_info(),
                user_ata_lpusd: auction_ata_lpusd.to_account_info(),
                stableswap_pool_ata_lpusd: stableswap_pool_ata_lpusd.to_account_info(),
                stableswap_pool_ata_usdc: stableswap_pool_ata_usdc.to_account_info(),
                escrow_ata_lpusd: escrow_ata_lpusd.to_account_info(),
                escrow_ata_wsol: escrow_ata_wsol.to_account_info(),
                escrow_ata_usdc: escrow_ata_usdc.to_account_info(),
                stableswap_program: stableswap_program.to_account_info(),
                testtokens_program: testtokens_program.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                rent: rent.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);    
            swap_router::cpi::swap_lpusd_to_lpsol_step1(cpi_ctx, swap_amount)?;
        }
        

        {
            let cpi_program = ctx.accounts.swaprouter_program.to_account_info();
            let cpi_accounts = swap_router::cpi::accounts::SwapLpusdToLpsolStep2 {
                user: ctx.accounts.auction_pda.to_account_info(),
                swap_escrow: swap_escrow.to_account_info(),
                stable_swap_pool: stable_swap_pool.to_account_info(),
                token_lpsol: token_lpsol.to_account_info(),
                token_wsol: token_wsol.to_account_info(),
                user_ata_lpsol: auction_ata_lpsol.to_account_info(),
                stableswap_pool_ata_lpsol: stableswap_pool_ata_lpsol.to_account_info(),
                stableswap_pool_ata_wsol: stableswap_pool_ata_wsol.to_account_info(),
                escrow_ata_lpsol: escrow_ata_lpsol.to_account_info(),
                escrow_ata_wsol: escrow_ata_wsol.to_account_info(),
                stableswap_program: stableswap_program.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                rent: rent.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
            let tx = swap_router::cpi::swap_lpusd_to_lpsol_step2(cpi_ctx)?;
            let lpsol_amount = tx.get();

            if lpsol_amount > 0 {
                // swap LpUSD -> LpSOL missing point
    
                // Burn LpSOL
                let cpi_program = ctx.accounts.lptokens_program.to_account_info();
                let cpi_accounts = BurnLpToken {
                    owner: ctx.accounts.auction_pda.to_account_info(),
                    token_mint: ctx.accounts.token_lpsol.to_account_info(),
                    user_token: ctx.accounts.auction_ata_lpsol.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                };
    
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
                lpfinance_tokens::cpi::burn_lptoken(cpi_ctx, lpsol_amount)?;    
                
            }  
        }
            
        let cpi_program = ctx.accounts.cbs_program.to_account_info();
        let cpi_accounts = UpdateUserAccount {
            user_account: cbs_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        cbs_protocol::cpi::liquidate_step2(cpi_ctx, swap_amount)?;
        
        Ok(())
    }

    
    pub fn distribute_reward_from_liquidate(        
        ctx: Context<DistributeRewardFromLiquidate>
    ) -> Result<()> {
        let config  = &mut ctx.accounts.config;
        let cbs_account  = &mut ctx.accounts.cbs_account;
        let reward_amount: i64 = cbs_account.escrow_lpusd_amount;
        let total_lpusd_added: i64 = config.total_lpusd as i64 + reward_amount;
        if total_lpusd_added < 0 {
            return Err(ErrorCode::InsufficientPoolAmount.into());
        }

        let auction_percent = config.total_percent as f64 * total_lpusd_added as f64 / config.total_lpusd as f64;

        config.last_epoch_percent = (total_lpusd_added as f64 * DENOMINATOR as f64 / config.total_lpusd as f64) as i64;
        config.last_epoch_profit = reward_amount;
        config.total_lpusd = total_lpusd_added as u64;
        config.total_percent = auction_percent as u64;

        // Get signer
        let (program_authority, program_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);

        if program_authority != ctx.accounts.auction_pda.to_account_info().key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];
        // End to get signer

        let cpi_program = ctx.accounts.cbs_program.to_account_info();
        let cpi_accounts = UpdateUserAccount {
            user_account: cbs_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        cbs_protocol::cpi::finalize_liquidate(cpi_ctx)?;
        Ok(())
    }

    // Update config params by onlyOwner
    // pub fn update_config(        
    //     ctx: Context<UpdateConfig>,
    //     total_percent: u64,
    //     last_epoch_percent: i64
    // ) -> Result<()> {
    //     let _config  = &mut ctx.accounts.config;        
    //     Ok(())
    // }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient User's Amount")]
    InsufficientAmount,
    #[msg("Insufficient Pool's Amount")]
    InsufficientPoolAmount,
    #[msg("Invalid Owner")]
    InvalidOwner,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Invalid Result")]
    InvalidResult,
    #[msg("Exceed Amount")]
    ExceedAmount,
    #[msg("Not Enough For LTV")]
    NotEnoughLTV,
    #[msg("Not Borrowed LpToken")]
    NotBorrowedLpToken,
    #[msg("PREV Liquidate Not Finished")]
    FinishPrevLiquidate,
    #[msg("Invalid pyth price")]
    InvalidPythPrice,
    #[msg("Empty Account")]
    EmptyAccount,
    #[msg("Withdraw collateral tokens before Liquidate")]
    ReadyErrorForLiquidate,
    #[msg("Invalid step num for liquidate")]
    InvalidLiquidateNum,
}