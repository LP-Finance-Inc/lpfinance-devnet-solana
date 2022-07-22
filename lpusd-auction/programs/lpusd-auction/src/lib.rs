use anchor_lang::prelude::*;
// use pyth_client;
use anchor_spl::token::{self, Mint, Transfer, Token, TokenAccount };

mod states;
pub use states::*;

use cbs_protocol::cpi::accounts::{LiquidateCollateral, UpdateUserAccount, LiquidateLpTokenCollateral};
use cbs_protocol::program::CbsProtocol;
use cbs_protocol::{self};

use lpfinance_swap::cpi::accounts::LiquidateToken;
use lpfinance_swap::program::LpfinanceSwap;
use lpfinance_swap::{self};

use lpfinance_tokens::cpi::accounts::{BurnLpToken};
use lpfinance_tokens::{self};

declare_id!("DbQju5NRVunuGz5aKdaqAaUfWSMRsy6hdZQ2zFDkGL9y");


pub const PRICE_DENOMINATOR: u128 = 100000000; // 10 ^ 8
// which means token 1
pub const PRICE_UNIT: u64 = 1000000000; // 10^9

pub fn get_price(pyth_account: AccountInfo) -> Result<u128> {
    let pyth_price_info = &pyth_account;
    let pyth_price_data = &pyth_price_info.try_borrow_data()?;
    let pyth_price = pyth_client::cast::<pyth_client::Price>(pyth_price_data);
    if pyth_price.agg.price <= 0 {
        Ok(0)
    } else {
        let price = pyth_price.agg.price as u128;
        Ok(price)
    }
}

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
    pub fn burn_for_liquidate(
        ctx: Context<BurnForLiquidate>
    ) -> Result<()> {

        let user_account: &mut Account<UserAccount> = &mut ctx.accounts.user_account;
        let cbs_account: &mut Account<cbs_protocol::UserAccount> = &mut ctx.accounts.cbs_account;
        let lpusd_ata: &mut Account<TokenAccount> = &mut ctx.accounts.lpusd_ata;
        // let config = &mut ctx.accounts.config;

        let lpusd_swap_amount_f: f64 = ctx.accounts.stable_lpusd_pool.get_swap_rate(PRICE_UNIT)? as f64;
        let lpsol_swap_amount_f: f64 = ctx.accounts.stable_lpsol_pool.get_swap_rate(PRICE_UNIT)? as f64;
        let price_denominator_f: f64 = PRICE_DENOMINATOR as f64;
        let price_unit_f: f64 = PRICE_UNIT as f64;

        // deposited
        let ray_amount_f: f64 = cbs_account.ray_amount as f64;
        let wsol_amount_f: f64 = cbs_account.wsol_amount as f64;
        let msol_amount_f: f64 = cbs_account.msol_amount as f64;
        let srm_amount_f: f64 = cbs_account.srm_amount as f64;
        let scnsol_amount_f: f64 = cbs_account.scnsol_amount as f64;
        let stsol_amount_f: f64 = cbs_account.stsol_amount as f64;
        let lpfi_amount_f: f64 = cbs_account.lpfi_amount as f64;
        let lpusd_amount_f: f64 = cbs_account.lpusd_amount as f64;
        let lpsol_amount_f: f64 = cbs_account.lpsol_amount as f64;
        // borrowed
        let borrowed_lpusd_f: f64 = cbs_account.borrowed_lpusd as f64;
        let borrowed_lpsol_f: f64 = cbs_account.borrowed_lpsol as f64;

        if ray_amount_f == 0.0 &&
           wsol_amount_f == 0.0 &&
           msol_amount_f == 0.0 &&
           srm_amount_f == 0.0 &&
           scnsol_amount_f == 0.0 &&
           stsol_amount_f == 0.0 &&
           lpfi_amount_f == 0.0 &&
           lpusd_amount_f == 0.0 &&
           lpsol_amount_f == 0.0 
        {
            return Err(ErrorCode::EmptyAccount.into());
        }

        // RAY price        
        let ray_price: f64 = get_price(ctx.accounts.pyth_ray_account.to_account_info())? as f64; 
        if ray_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }   
        // wSOL price
        let wsol_price: f64 = get_price(ctx.accounts.pyth_sol_account.to_account_info())? as f64;   
        if wsol_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }
        // mSOL price
        let msol_price: f64 = get_price(ctx.accounts.pyth_msol_account.to_account_info())? as f64; 
        if msol_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }
        // srm price
        let srm_price: f64 = get_price(ctx.accounts.pyth_srm_account.to_account_info())? as f64;    
        if srm_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }
        // scnsol price
        let scnsol_price: f64 = get_price(ctx.accounts.pyth_scnsol_account.to_account_info())? as f64;
        if scnsol_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }
        // stsol price
        let stsol_price: f64 = get_price(ctx.accounts.pyth_stsol_account.to_account_info())? as f64;
        if stsol_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }
        // USDC price
        let usdc_price: f64  = get_price(ctx.accounts.pyth_usdc_account.to_account_info())? as f64;
        if usdc_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        // LpFi price
        let lpfi_usdc_rate: f64 = ctx.accounts.liquidity_pool.get_price()? as f64; // LpFi = x * USDC here x = rate
        let lpfi_price: f64 = usdc_price * lpfi_usdc_rate / price_denominator_f;
        if lpfi_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        // LpUSD price
        let lpusd_price: f64 = usdc_price * lpusd_swap_amount_f / price_unit_f;   
        if lpusd_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }     

        // LpSOL price
        let lpsol_price = wsol_price * lpsol_swap_amount_f / price_unit_f;
        if lpsol_price <= 0.0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        // Deposited Collateral Tokens' TotalPrice. Need to be calculated with LTV
        let mut total_deposited_price_f: f64 = 0.0;
        let mut total_borrowed_price_f: f64 = 0.0;

        // Total deposited cTokens' price
        total_deposited_price_f += ray_price * ray_amount_f; // + cbs_account.lending_ray_amount
        total_deposited_price_f += wsol_price * wsol_amount_f; // + cbs_account.lending_wsol_amount
        total_deposited_price_f += msol_price * msol_amount_f; // + cbs_account.lending_msol_amount
        total_deposited_price_f += srm_price * srm_amount_f; // + cbs_account.lending_srm_amount
        total_deposited_price_f += scnsol_price * scnsol_amount_f; // + cbs_account.lending_scnsol_amount
        total_deposited_price_f += stsol_price * stsol_amount_f; // + cbs_account.lending_stsol_amount
        total_deposited_price_f += lpfi_price * lpfi_amount_f;
        total_deposited_price_f += lpusd_price * lpusd_amount_f;
        total_deposited_price_f += lpsol_price * lpsol_amount_f;

        // Total Borrowed LpTokens' price
        total_borrowed_price_f += lpusd_price * borrowed_lpusd_f;
        total_borrowed_price_f += lpsol_price * borrowed_lpsol_f;

        let ltv_permission_f: f64 = LTV_PERMISSION as f64;
        // If LTV < 94, not be able to liquidate
        if total_borrowed_price_f * 100.0 < total_deposited_price_f * ltv_permission_f {
            return Err(ErrorCode::NotEnoughLTV.into());
        }

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
        if borrowed_lpusd_f >= 0.0 {
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
            lpfinance_tokens::cpi::burn_lptoken(cpi_ctx, borrowed_lpusd_f as u64)?;
        }

        if borrowed_lpsol_f >= 0.0 {
            // swap LpUSD -> LpSOL
            // Burn LpSOL
            let cpi_program = ctx.accounts.lptokens_program.to_account_info();
            let cpi_accounts = BurnLpToken {
                owner: ctx.accounts.auction_pda.to_account_info(),
                token_mint: ctx.accounts.lpsol_mint.to_account_info(),
                user_token: ctx.accounts.lpsol_ata.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info()
            };
    
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            lpfinance_tokens::cpi::burn_lptoken(cpi_ctx, borrowed_lpsol_f as u64)?;
        }
        
        Ok(())
    }



    // pub fn liquidate_from_cbs(
    //     ctx: Context<LiquidateFromCBS>
    // ) -> Result<()> {
    //     // Transfer all collaterals from cbs to auction
    //     msg!("Start LiquidateFromCBS");
    //     if ctx.accounts.liquidator.bump > 0 && ctx.accounts.liquidator.bump < 6 {
    //         return Err(ErrorCode::FinishPrevLiquidate.into());
    //     }
    //     let borrowed_lpusd = ctx.accounts.liquidator.borrowed_lpusd;       
    //     let borrowed_lpsol = ctx.accounts.liquidator.borrowed_lpsol;
    //     let borrowed_lpbtc = ctx.accounts.liquidator.borrowed_lpbtc;       
    //     let borrowed_lpeth = ctx.accounts.liquidator.borrowed_lpeth;

    //     if borrowed_lpusd == 0 && borrowed_lpsol == 0 && borrowed_lpbtc == 0 && borrowed_lpeth == 0{
    //         return Err(ErrorCode::NotBorrowedLpToken.into());
    //     }

    //     let cpi_program = ctx.accounts.cbs_program.to_account_info();
    //     let cpi_accounts = LiquidateCollateral {
    //         user_account: ctx.accounts.liquidator.to_account_info(),
    //         state_account: ctx.accounts.cbs_account.to_account_info(),
    //         auction_account: ctx.accounts.state_account.to_account_info(),

    //         auction_msol: ctx.accounts.auction_msol.to_account_info(),
    //         auction_btc: ctx.accounts.auction_btc.to_account_info(),
    //         auction_usdc: ctx.accounts.auction_usdc.to_account_info(),
    //         auction_eth: ctx.accounts.auction_eth.to_account_info(),

    //         cbs_btc: ctx.accounts.cbs_btc.to_account_info(),
    //         cbs_msol: ctx.accounts.cbs_msol.to_account_info(),
    //         cbs_usdc: ctx.accounts.cbs_usdc.to_account_info(),
    //         cbs_eth: ctx.accounts.cbs_eth.to_account_info(),

    //         system_program: ctx.accounts.system_program.to_account_info(),
    //         token_program: ctx.accounts.token_program.to_account_info(),
    //         rent: ctx.accounts.rent.to_account_info()
    //     };
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     cbs_protocol::cpi::liquidate_collateral(cpi_ctx)?;

    //     Ok(())
    // }

    // pub fn liquidate_second_from_cbs(
    //     ctx: Context<LiquidateSecondFromCBS>
    // ) -> Result<()> {
    //     // Transfer all collaterals from cbs to auction
    //     msg!("Start LiquidateSecondFromCBS");
    //     if ctx.accounts.liquidator.bump != 1 {
    //         return Err(ErrorCode::FinishPrevLiquidate.into());
    //     }

    //     let borrowed_lpusd = ctx.accounts.liquidator.borrowed_lpusd;       
    //     let borrowed_lpsol = ctx.accounts.liquidator.borrowed_lpsol;
    //     let borrowed_lpbtc = ctx.accounts.liquidator.borrowed_lpbtc;       
    //     let borrowed_lpeth = ctx.accounts.liquidator.borrowed_lpeth;

    //     if borrowed_lpusd == 0 && borrowed_lpsol == 0 && borrowed_lpbtc == 0 && borrowed_lpeth == 0{
    //         return Err(ErrorCode::NotBorrowedLpToken.into());
    //     }

    //     let cpi_program = ctx.accounts.cbs_program.to_account_info();
    //     let cpi_accounts = LiquidateSecondCollateral {
    //         user_account: ctx.accounts.liquidator.to_account_info(),
    //         state_account: ctx.accounts.cbs_account.to_account_info(),

    //         auction_ust: ctx.accounts.auction_ust.to_account_info(),
    //         auction_srm: ctx.accounts.auction_srm.to_account_info(),
    //         auction_scnsol: ctx.accounts.auction_scnsol.to_account_info(),
    //         auction_stsol: ctx.accounts.auction_stsol.to_account_info(),
    //         auction_usdt: ctx.accounts.auction_usdt.to_account_info(),

    //         cbs_ust: ctx.accounts.cbs_ust.to_account_info(),
    //         cbs_srm: ctx.accounts.cbs_srm.to_account_info(),
    //         cbs_scnsol: ctx.accounts.cbs_scnsol.to_account_info(),
    //         cbs_stsol: ctx.accounts.cbs_stsol.to_account_info(),
    //         cbs_usdt: ctx.accounts.cbs_usdt.to_account_info(),

    //         system_program: ctx.accounts.system_program.to_account_info(),
    //         token_program: ctx.accounts.token_program.to_account_info(),
    //         rent: ctx.accounts.rent.to_account_info()
    //     };
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     cbs_protocol::cpi::liquidate_second_collateral(cpi_ctx)?;

    //     Ok(())
    // }

    // pub fn liquidate_lptoken_from_cbs(
    //     ctx: Context<LiquidateLpTokenFromCBS>
    // ) -> Result<()> {
    //     // Transfer all collaterals from cbs to auction
    //     msg!("Start LiquidateLpTokenFromCBS");

    //     if ctx.accounts.liquidator.bump != 2 {
    //         return Err(ErrorCode::FinishPrevLiquidate.into());
    //     }
        
    //     let borrowed_lpusd = ctx.accounts.liquidator.borrowed_lpusd;       
    //     let borrowed_lpsol = ctx.accounts.liquidator.borrowed_lpsol;
    //     let borrowed_lpbtc = ctx.accounts.liquidator.borrowed_lpbtc;       
    //     let borrowed_lpeth = ctx.accounts.liquidator.borrowed_lpeth;

    //     if borrowed_lpusd == 0 && borrowed_lpsol == 0 && borrowed_lpbtc == 0 && borrowed_lpeth == 0{
    //         return Err(ErrorCode::NotBorrowedLpToken.into());
    //     }

    //     let cpi_program = ctx.accounts.cbs_program.to_account_info();
    //     let cpi_accounts = LiquidateLpTokenCollateral {
    //         user_account: ctx.accounts.liquidator.to_account_info(),
    //         state_account: ctx.accounts.cbs_account.to_account_info(),

    //         auction_lpusd: ctx.accounts.auction_lpusd.to_account_info(),
    //         auction_lpsol: ctx.accounts.auction_lpsol.to_account_info(),
    //         auction_lpbtc: ctx.accounts.auction_lpbtc.to_account_info(),
    //         auction_lpeth: ctx.accounts.auction_lpeth.to_account_info(),

    //         cbs_lpusd: ctx.accounts.cbs_lpusd.to_account_info(),
    //         cbs_lpsol: ctx.accounts.cbs_lpsol.to_account_info(),
    //         cbs_lpbtc: ctx.accounts.cbs_lpbtc.to_account_info(),
    //         cbs_lpeth: ctx.accounts.cbs_lpeth.to_account_info(),

    //         system_program: ctx.accounts.system_program.to_account_info(),
    //         token_program: ctx.accounts.token_program.to_account_info(),
    //         rent: ctx.accounts.rent.to_account_info()
    //     };
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     cbs_protocol::cpi::liquidate_lptoken_collateral(cpi_ctx)?;

    //     Ok(())
    // }

    // pub fn liquidate (
    //     ctx: Context<Liquidate>
    // ) -> Result<()> {
    //     msg!("Started liquidate");

    //     let liquidator = &mut ctx.accounts.liquidator;

    //     if liquidator.bump != 3 {
    //         return Err(ErrorCode::FinishPrevLiquidate.into());
    //     }

    //     let borrowed_lpusd = liquidator.borrowed_lpusd;       
    //     let borrowed_lpsol = liquidator.borrowed_lpsol;
    //     let borrowed_lpbtc = liquidator.borrowed_lpbtc;       
    //     let borrowed_lpeth = liquidator.borrowed_lpeth;

    //     if borrowed_lpusd == 0 && borrowed_lpsol == 0 && borrowed_lpbtc == 0 && borrowed_lpeth == 0{
    //         return Err(ErrorCode::NotBorrowedLpToken.into());
    //     }

    //     // Fetch the price
    //     let sol_price: u128 = get_price(ctx.accounts.pyth_sol_account.to_account_info())? as u128;
    //     let usdc_price: u128 = get_price(ctx.accounts.pyth_usdc_account.to_account_info())? as u128;
    //     let btc_price: u128 = get_price(ctx.accounts.pyth_btc_account.to_account_info())? as u128;
    //     let msol_price: u128 = get_price(ctx.accounts.pyth_msol_account.to_account_info())? as u128;
    //     let eth_price: u128 = get_price(ctx.accounts.pyth_eth_account.to_account_info())? as u128;
    //     let ust_price: u128 = get_price(ctx.accounts.pyth_ust_account.to_account_info())? as u128;
    //     let srm_price: u128 = get_price(ctx.accounts.pyth_srm_account.to_account_info())? as u128;
    //     let scnsol_price: u128 = get_price(ctx.accounts.pyth_scnsol_account.to_account_info())? as u128;
    //     let stsol_price: u128 = get_price(ctx.accounts.pyth_stsol_account.to_account_info())? as u128;
    //     let usdt_price: u128 = get_price(ctx.accounts.pyth_usdt_account.to_account_info())? as u128;

    //     // Total Deposited Price
    //     let mut total_price: u128 = 0;
    //     total_price += sol_price * (liquidator.sol_amount) as u128; //  + liquidator.lending_sol_amount
    //     total_price += btc_price * (liquidator.btc_amount) as u128; //  + liquidator.lending_btc_amount
    //     total_price += usdc_price * (liquidator.usdc_amount) as u128; //  + liquidator.lending_usdc_amount
    //     total_price += eth_price * (liquidator.eth_amount) as u128; //  + liquidator.lending_eth_amount
    //     total_price += msol_price * (liquidator.msol_amount) as u128; //  + liquidator.lending_msol_amount
    //     total_price += ust_price * (liquidator.ust_amount) as u128; //  + liquidator.lending_ust_amount
    //     total_price += srm_price * (liquidator.srm_amount) as u128; //  + liquidator.lending_srm_amount
    //     total_price += scnsol_price * (liquidator.scnsol_amount) as u128; //  + liquidator.lending_scnsol_amount
    //     total_price += stsol_price * (liquidator.stsol_amount) as u128; //  + liquidator.lending_stsol_amount
    //     total_price += usdt_price * (liquidator.usdt_amount) as u128; //  + liquidator.lending_usdt_amount

    //     total_price += sol_price * liquidator.lpsol_amount as u128;
    //     total_price += btc_price * liquidator.lpbtc_amount as u128;
    //     total_price += eth_price * liquidator.lpeth_amount as u128;
    //     total_price += usdc_price * liquidator.lpusd_amount as u128;

    //     // Total Borrowed Price 
    //     let total_borrowed_price:u128 = borrowed_lpusd as u128 * usdc_price + 
    //         borrowed_lpsol as u128 * sol_price + 
    //         borrowed_lpbtc as u128 * btc_price + 
    //         borrowed_lpeth as u128 * eth_price;

    //     // LTV should be > 94
    //     // Formula: LTV = (total_borrowed_price / total_price) * 100 > 94
    //     // if total_price * LTV_PERMISSION as u128 >= total_borrowed_price * 100{
    //     //     return Err(ErrorCode::NotEnoughLTV.into());
    //     // }
        
    //     // Get signer
    //     let (program_authority, program_authority_bump) = 
    //     Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
    //     if program_authority != ctx.accounts.state_account.to_account_info().key() {
    //         return Err(ErrorCode::InvalidOwner.into());
    //     }

    //     let seeds = &[
    //         PREFIX.as_bytes(),
    //         &[program_authority_bump]
    //     ];
    //     let signer = &[&seeds[..]];
    //     // End to get signer

    //     if borrowed_lpusd > 0 {
    //         if borrowed_lpusd > ctx.accounts.auction_lpusd.amount {
    //             return Err(ErrorCode::InsufficientPoolAmount.into());            
    //         }
    //         // Transfer lpusd from auction to cbs
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_lpusd.to_account_info(),
    //             to: ctx.accounts.cbs_lpusd.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, borrowed_lpusd)?;
    //     }

    //     // Liquidate LpSOL (Swap LpUSD to LpSOL and transfer LpSOL to CBS)
    //     if borrowed_lpsol > 0 {            
    //         msg!("Started request LpSOL from swap");
    //         // Request LpSOL from SWAP for sending LpSOL back to CBS
    //         let cpi_program = ctx.accounts.swap_program.to_account_info();
    //         let cpi_accounts = LiquidateToken {
    //             state_account: ctx.accounts.swap_account.to_account_info(),
    //             auction_pool: ctx.accounts.cbs_lpsol.to_account_info(),
    //             swap_pool: ctx.accounts.swap_lpsol.to_account_info(),
    //             dest_mint: ctx.accounts.lpsol_mint.to_account_info(),
    //             system_program: ctx.accounts.system_program.to_account_info(),
    //             token_program: ctx.accounts.token_program.to_account_info(),
    //             rent: ctx.accounts.rent.to_account_info()
    //         };
    //         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //         lpfinance_swap::cpi::liquidate_token(cpi_ctx, borrowed_lpsol)?;
            
    //     }
  
    //     // Liquidate LpBTC (Swap LpUSD to LpBTC and transfer LpBTC to CBS)
    //     if borrowed_lpbtc > 0 {            

    //         msg!("Started request LpBTC from swap");
    //         // Request LpBTC from SWAP for sending LpBTC back to CBS
    //         let cpi_program = ctx.accounts.swap_program.to_account_info();
    //         let cpi_accounts = LiquidateToken {
    //             state_account: ctx.accounts.swap_account.to_account_info(),
    //             auction_pool: ctx.accounts.cbs_lpbtc.to_account_info(),
    //             swap_pool: ctx.accounts.swap_lpbtc.to_account_info(),
    //             dest_mint: ctx.accounts.lpbtc_mint.to_account_info(),
    //             system_program: ctx.accounts.system_program.to_account_info(),
    //             token_program: ctx.accounts.token_program.to_account_info(),
    //             rent: ctx.accounts.rent.to_account_info()
    //         };
    //         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //         lpfinance_swap::cpi::liquidate_token(cpi_ctx, borrowed_lpbtc)?;
    //     }
        
    //     // Liquidate LpETH (Swap LpUSD to LpETH and transfer LpETH to CBS)
    //     if borrowed_lpeth > 0 {            

    //         msg!("Started request LpETH from swap");
    //         // Request LpETH from SWAP for sending LpETH back to CBS
    //         let cpi_program = ctx.accounts.swap_program.to_account_info();
    //         let cpi_accounts = LiquidateToken {
    //             state_account: ctx.accounts.swap_account.to_account_info(),
    //             auction_pool: ctx.accounts.cbs_lpeth.to_account_info(),
    //             swap_pool: ctx.accounts.swap_lpeth.to_account_info(),
    //             dest_mint: ctx.accounts.lpeth_mint.to_account_info(),
    //             system_program: ctx.accounts.system_program.to_account_info(),
    //             token_program: ctx.accounts.token_program.to_account_info(),
    //             rent: ctx.accounts.rent.to_account_info()
    //         };
    //         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //         lpfinance_swap::cpi::liquidate_token(cpi_ctx, borrowed_lpeth)?;
    //     }

    //     let reward: i128 = (total_price as i128 - total_borrowed_price as i128) / usdc_price as i128;
    //     msg!("Total Price and Total Borrowed Price {} {}", reward, total_borrowed_price);
    //     msg!("Reward {}", reward);
    //     let config = &mut ctx.accounts.config;
    //     msg!("Total Auction Lpusd {}", config.total_lpusd);

    //     let total_amount_temp: i128 = config.total_lpusd as i128 + reward;
    //     if total_amount_temp < 0 {
    //         return Err(ErrorCode::InsufficientPoolAmount.into());
    //     }
    //     let total_amount = total_amount_temp as u128;
    //     let auction_percent = config.total_percent * total_amount / config.total_lpusd;

    //     config.last_epoch_percent = (total_amount * DENOMINATOR / config.total_lpusd) as i64;
    //     config.last_epoch_profit = reward;
    //     config.total_lpusd = total_amount;
    //     config.total_percent = auction_percent;        

    //     let cpi_program = ctx.accounts.cbs_program.to_account_info();
    //     let cpi_accounts = UpdateUserAccount {
    //         user_account: ctx.accounts.liquidator.to_account_info()
    //     };
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     cbs_protocol::cpi::update_user_account(cpi_ctx, 4)?;

    //     Ok(())
    // }

    // pub fn liquidate_swap(
    //     ctx: Context<LiquidateSwap>
    // ) -> Result<()> {
        
    //     msg!("Started liquidate swap");

    //     let liquidator = &mut ctx.accounts.liquidator;
    //     if liquidator.bump != 4 {
    //         return Err(ErrorCode::FinishPrevLiquidate.into());
    //     }
        
    //     let btc_amount = liquidator.btc_amount; // + liquidator.lending_btc_amount;
    //     let sol_amount = liquidator.sol_amount; // + liquidator.lending_sol_amount;
    //     let usdc_amount = liquidator.usdc_amount; // + liquidator.lending_usdc_amount;
    //     let msol_amount = liquidator.msol_amount; // + liquidator.lending_msol_amount;
    //     let eth_amount = liquidator.eth_amount; // + liquidator.lending_eth_amount;
    //     let ust_amount = liquidator.ust_amount; // + liquidator.lending_ust_amount;
    //     let srm_amount = liquidator.srm_amount; // + liquidator.lending_srm_amount;
    //     let scnsol_amount = liquidator.scnsol_amount; // + liquidator.lending_scnsol_amount;
    //     let stsol_amount = liquidator.stsol_amount; // + liquidator.lending_stsol_amount;
    //     let usdt_amount = liquidator.usdt_amount; // + liquidator.lending_usdt_amount;

    //     let (program_authority, program_authority_bump) = 
    //     Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
    //     if program_authority != ctx.accounts.state_account.to_account_info().key() {
    //         return Err(ErrorCode::InvalidOwner.into());
    //     }

    //     let seeds = &[
    //         PREFIX.as_bytes(),
    //         &[program_authority_bump]
    //     ];
    //     let signer = &[&seeds[..]];

    //     // BTC
    //     if btc_amount > 0 {
    //         msg!("BTC sending");
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_btc.to_account_info(),
    //             to: ctx.accounts.swap_btc.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, btc_amount)?;
    //     }
        
    //     // mSOL
    //     if msol_amount > 0 {
    //         msg!("mSOL sending");
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_msol.to_account_info(),
    //             to: ctx.accounts.swap_msol.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, msol_amount)?;
    //     }
        
    //     // USDC 
    //     if usdc_amount > 0 {
    //         msg!("USDC sending");
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_usdc.to_account_info(),
    //             to: ctx.accounts.swap_usdc.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, usdc_amount)?;
    //     }

    //     // eth
    //     if eth_amount > 0 {
    //         msg!("ETH sending");
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_eth.to_account_info(),
    //             to: ctx.accounts.swap_eth.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, eth_amount)?;
    //     }

        
    //     // ust
    //     if ust_amount > 0 {
    //         msg!("UST sending");
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_ust.to_account_info(),
    //             to: ctx.accounts.swap_ust.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, ust_amount)?;
    //     }
        
    //     // srm 
    //     if srm_amount > 0 {
    //         msg!("SRN sending");
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_srm.to_account_info(),
    //             to: ctx.accounts.swap_srm.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, srm_amount)?;
    //     }

    //     // scnsol
    //     if scnsol_amount > 0 {
    //         msg!("SCNSOL sending");
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_scnsol.to_account_info(),
    //             to: ctx.accounts.swap_scnsol.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, scnsol_amount)?;
    //     }

        
    //     // stsol
    //     if stsol_amount > 0 {
    //         msg!("STSOL sending");
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_stsol.to_account_info(),
    //             to: ctx.accounts.swap_stsol.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, stsol_amount)?;
    //     }
        
    //     // usdt 
    //     if usdt_amount > 0 {
    //         msg!("USDT sending");
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_usdt.to_account_info(),
    //             to: ctx.accounts.swap_usdt.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, usdt_amount)?;
    //     }

    //     // SOL transfer
    //     if sol_amount > 0 {
    //         msg!("SOL sending");
    //         **ctx.accounts.state_account.to_account_info().try_borrow_mut_lamports()? -= sol_amount;
    //         **ctx.accounts.swap_account.to_account_info().try_borrow_mut_lamports()? += sol_amount;
    //     }

    //     let cpi_program = ctx.accounts.cbs_program.to_account_info();
    //     let cpi_accounts = UpdateUserAccount {
    //         user_account: ctx.accounts.liquidator.to_account_info()
    //     };
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     cbs_protocol::cpi::update_user_account(cpi_ctx, 5)?;

    //     Ok(())
    // }


    // pub fn liquidate_second_swap(
    //     ctx: Context<LiquidateSecondSwap>
    // ) -> Result<()> {
        
    //     msg!("Started liquidate second swap");

    //     let liquidator = &mut ctx.accounts.liquidator;
    //     if liquidator.bump != 5 {
    //         return Err(ErrorCode::FinishPrevLiquidate.into());
    //     }
        

    //     let lpsol_amount = liquidator.lpsol_amount;
    //     // let lpusd_amount = liquidator.lpusd_amount;
    //     let lpbtc_amount = liquidator.lpbtc_amount;
    //     let lpeth_amount = liquidator.lpeth_amount;

    //     let (program_authority, program_authority_bump) = 
    //     Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
    //     if program_authority != ctx.accounts.state_account.to_account_info().key() {
    //         return Err(ErrorCode::InvalidOwner.into());
    //     }

    //     let seeds = &[
    //         PREFIX.as_bytes(),
    //         &[program_authority_bump]
    //     ];
    //     let signer = &[&seeds[..]];

    //     // LpSOL
    //     if lpsol_amount > 0 {
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_lpsol.to_account_info(),
    //             to: ctx.accounts.swap_lpsol.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, lpsol_amount)?;
    //     }
    //     // LpBTC
    //     if lpbtc_amount > 0 {
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_lpbtc.to_account_info(),
    //             to: ctx.accounts.swap_lpbtc.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, lpbtc_amount)?;
    //     }
    //     // LpETH
    //     if lpeth_amount > 0 {
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_lpeth.to_account_info(),
    //             to: ctx.accounts.swap_lpeth.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, lpeth_amount)?;
    //     }


    //     let transfer_amount = ctx.accounts.config.last_epoch_profit;
    //     if transfer_amount > 0 {
    //         let cpi_program = ctx.accounts.swap_program.to_account_info();
    //         let cpi_accounts = LiquidateToken {
    //             state_account: ctx.accounts.swap_account.to_account_info(),
    //             auction_pool: ctx.accounts.auction_lpusd.to_account_info(),
    //             swap_pool: ctx.accounts.swap_lpusd.to_account_info(),
    //             dest_mint: ctx.accounts.lpusd_mint.to_account_info(),
    //             system_program: ctx.accounts.system_program.to_account_info(),
    //             token_program: ctx.accounts.token_program.to_account_info(),
    //             rent: ctx.accounts.rent.to_account_info()
    //         };
    //         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //         lpfinance_swap::cpi::liquidate_token(cpi_ctx, transfer_amount as u64)?;
    //     } else if transfer_amount < 0 {
    //         let reversed_amount = transfer_amount * -1;
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.auction_lpusd.to_account_info(),
    //             to: ctx.accounts.swap_lpusd.to_account_info(),
    //             authority: ctx.accounts.state_account.to_account_info()
    //         };
    
    //         let cpi_program = ctx.accounts.token_program.to_account_info();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, reversed_amount as u64)?;
    //     }

    //     let cpi_program = ctx.accounts.cbs_program.to_account_info();
    //     let cpi_accounts = UpdateUserAccount {
    //         user_account: ctx.accounts.liquidator.to_account_info()
    //     };
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     cbs_protocol::cpi::update_user_account(cpi_ctx, 10)?;


    //     Ok(())
    // }


    // Update config params by onlyOwner
    pub fn update_config(        
        ctx: Context<UpdateConfig>,
        total_percent: u64,
        last_epoch_percent: i64
    ) -> Result<()> {
        let _config  = &mut ctx.accounts.config;        
        Ok(())
    }
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
}