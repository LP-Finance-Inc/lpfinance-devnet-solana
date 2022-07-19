use anchor_lang::prelude::*;
use pyth_client;
use anchor_spl::token::{self, Transfer };

mod states;
pub use states::*;

use swap_base::{self, Pool};

use lpfinance_tokens::cpi::accounts::MintLpToken;
use lpfinance_tokens::{self};

use solend::{self};
use apricot::{self};

declare_id!("8NSpbuD66CrveJYufKZWiJPneVak7Ri74115qpiP8xw4");

const LTV:u128 = 85;
const DOMINATOR:u128 = 100;

const LENDING_PERCENT: u64 = 10; // 10%
// In solend, apricot
// APR is set as multiplier 100000
// This means APR could be 0.00001 as accurate rate.
const LENDING_DENOMINATOR: u128 = 10000000; // 100,00000

// const W_THRESHHOLD: u64 = 90;
// const S_THRESHHOLD: u64 = 75;

const PRICE_DENOMINATOR: u128 = 100000000;

const PRICE_UNIT: u64 = 1000000000; // 10^9

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
pub mod cbs_protocol {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>
    ) -> Result<()> {
        msg!("INITIALIZE CBS PROTOCAL");

        let config = &mut ctx.accounts.config;
        config.owner = ctx.accounts.authority.key();
        config.liquidation_run = false;

        // borrowed amount
        config.total_borrowed_lpsol = 0;
        config.total_borrowed_lpusd = 0;

        // deposited amount
        config.total_deposited_wsol = 0;
        config.total_deposited_ray = 0;
        config.total_deposited_msol = 0;
        config.total_deposited_srm = 0;
        config.total_deposited_scnsol = 0;
        config.total_deposited_stsol = 0;

        config.total_deposited_lpsol = 0;
        config.total_deposited_lpusd = 0;
        config.total_deposited_lpfi = 0;       

        Ok(())
    }
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

    // Create solend user account
    pub fn create_solend_cbs_account(
        ctx: Context<CreateSolendCBSAccount>
    ) -> Result<()> {
         //-------- PDA Generate --------------------------------
         let (program_authority, program_authority_bump) = 
         Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
 
         if program_authority != ctx.accounts.cbs_pda.to_account_info().key() {
             return Err(ErrorCode::InvalidCBSOwner.into());
         }
 
         let seeds = &[
             PREFIX.as_bytes(),
             &[program_authority_bump]
         ];
         let signer = &[&seeds[..]];
         // == GET signer ended == //

        msg!("Solend CBS account create");
        let cpi_program = ctx.accounts.solend_program.to_account_info();
        let cpi_accounts = solend::cpi::accounts::InitUserAccount {
            user_account: ctx.accounts.solend_account.to_account_info(),
            user: ctx.accounts.cbs_pda.to_account_info(),
            user_authority: ctx.accounts.cbs_pda.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        solend::cpi::init_user_account(cpi_ctx)?;
        Ok(())
    }

    // Create apricot user account
    pub fn create_apricot_cbs_account(
        ctx: Context<CreateApricotCBSAccount>
    ) -> Result<()> {
         //-------- PDA Generate --------------------------------
         let (program_authority, program_authority_bump) = 
         Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
 
         if program_authority != ctx.accounts.cbs_pda.to_account_info().key() {
             return Err(ErrorCode::InvalidCBSOwner.into());
         }
 
         let seeds = &[
             PREFIX.as_bytes(),
             &[program_authority_bump]
         ];
         let signer = &[&seeds[..]];
         // == GET signer ended == //

        msg!("Apricot CBS account starts");
        let cpi_program = ctx.accounts.apricot_program.to_account_info();
        let cpi_accounts = apricot::cpi::accounts::InitUserAccount {
            user_account: ctx.accounts.apricot_account.to_account_info(),
            user: ctx.accounts.cbs_pda.to_account_info(),
            user_authority: ctx.accounts.cbs_pda.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        apricot::cpi::init_user_account(cpi_ctx)?;
        Ok(())
    }

    // Init user account
    pub fn init_user_account(
        ctx: Context<InitUserAccount>
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.owner = ctx.accounts.user_authority.key();
        user_account.step_num = 0;
        Ok(())
    }

    // Close user account
    pub fn delete_user_account(_ctx: Context<DeleteUserAccount>) -> Result<()> {
        Ok(())
    }

    // Deposit collateral tokens
    pub fn deposit_collateral(
        ctx: Context<DepositCollateral>,
        amount: u64
    )-> Result<()> {        
        msg!("Deposit collateral started");
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        if ctx.accounts.user_collateral.amount < amount {
            return Err(ErrorCode::InsufficientUserAmount.into());
        } 


        let user_account: &mut Account<UserAccount> =&mut ctx.accounts.user_account;
        let config: &mut Account<Config> = &mut ctx.accounts.config;

        // Need to check if the current user is in Liquidating.
        // If user account is in liquidating, user cannot make deposit tx
        if user_account.step_num > 0 && user_account.step_num < 6 {
            return Err(ErrorCode::ProgressInLiquidate.into());
        }

        let amount_f: f64 = amount as f64;
        let lending_percent_f: f64 = LENDING_PERCENT as f64;

        // While initial depositing, need to send 10% to lending protocol.
        let lending_amount: u64 = (amount_f * lending_percent_f / 100.0) as u64;
        let pool_amount: u64 = amount - lending_amount;

        //--------Transfer Collateral Token USER_ATA -> CBS_ATA
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_collateral.to_account_info(),
            to: ctx.accounts.collateral_pool.to_account_info(),
            authority: ctx.accounts.user_authority.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        //-------- PDA Generate --------------------------------
        let (program_authority, program_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);

        if program_authority != ctx.accounts.cbs_pda.to_account_info().key() {
            return Err(ErrorCode::InvalidCBSOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];
        // == GET signer ended == //

        //--------Transfer Collateral Token CBS_ATA -> SOLEND_ATA, APRICOT_ATA
        // In case of normal tokens to be able to deposit into lending protocol but not lpfinace tokens
        if ctx.accounts.user_collateral.mint == config.wsol_mint || 
           ctx.accounts.user_collateral.mint == config.msol_mint || 
           ctx.accounts.user_collateral.mint == config.ray_mint || 
           ctx.accounts.user_collateral.mint == config.srm_mint || 
           ctx.accounts.user_collateral.mint == config.scnsol_mint || 
           ctx.accounts.user_collateral.mint == config.stsol_mint {

            // If solend APY rate is higher than apricot APY rate, return true;
            let mut solend_higher = false;
            let lending_amount_f: f64 = lending_amount as f64;
            let lending_denominator_f: f64 = LENDING_DENOMINATOR as f64;
            let mut lending_rate_f: f64 = 0.0;

            if ctx.accounts.user_collateral.mint == config.ray_mint {
                if ctx.accounts.solend_config.ray_rate > ctx.accounts.apricot_config.ray_rate {
                    solend_higher = true;
                    lending_rate_f = ctx.accounts.solend_config.ray_rate as f64;
                } else {
                    lending_rate_f = ctx.accounts.apricot_config.ray_rate as f64;
                }
                user_account.lending_ray_amount += (lending_amount_f * lending_denominator_f / lending_rate_f) as u64;
            } else if ctx.accounts.user_collateral.mint == config.wsol_mint {
                if ctx.accounts.solend_config.wsol_rate > ctx.accounts.apricot_config.wsol_rate {
                    solend_higher = true;
                    lending_rate_f = ctx.accounts.solend_config.wsol_rate as f64;
                } else {
                    lending_rate_f = ctx.accounts.apricot_config.wsol_rate as f64;
                }
                user_account.lending_wsol_amount += (lending_amount_f * lending_denominator_f / lending_rate_f) as u64;

            } else if ctx.accounts.user_collateral.mint == config.msol_mint {
                if ctx.accounts.solend_config.msol_rate > ctx.accounts.apricot_config.msol_rate {
                    solend_higher = true;
                    lending_rate_f = ctx.accounts.solend_config.msol_rate as f64;
                } else {
                    lending_rate_f = ctx.accounts.apricot_config.msol_rate as f64;
                }
                user_account.lending_msol_amount += (lending_amount_f * lending_denominator_f / lending_rate_f) as u64;
            } else if ctx.accounts.user_collateral.mint == config.srm_mint {
                if ctx.accounts.solend_config.srm_rate > ctx.accounts.apricot_config.srm_rate {
                    solend_higher = true;
                    lending_rate_f = ctx.accounts.solend_config.srm_rate as f64;
                } else {
                    lending_rate_f = ctx.accounts.apricot_config.srm_rate as f64;
                }
                user_account.lending_srm_amount += (lending_amount_f * lending_denominator_f / lending_rate_f) as u64;

            } else if ctx.accounts.user_collateral.mint == config.scnsol_mint {
                if ctx.accounts.solend_config.scnsol_rate > ctx.accounts.apricot_config.scnsol_rate {
                    solend_higher = true;
                    lending_rate_f = ctx.accounts.solend_config.scnsol_rate as f64;
                } else {
                    lending_rate_f = ctx.accounts.apricot_config.scnsol_rate as f64;
                }
                user_account.lending_scnsol_amount += (lending_amount_f * lending_denominator_f / lending_rate_f) as u64;

            } else if ctx.accounts.user_collateral.mint == config.stsol_mint {
                if ctx.accounts.solend_config.stsol_rate > ctx.accounts.apricot_config.stsol_rate {
                    solend_higher = true;
                    lending_rate_f = ctx.accounts.solend_config.stsol_rate as f64;
                } else {
                    lending_rate_f = ctx.accounts.apricot_config.stsol_rate as f64;
                }
                user_account.lending_stsol_amount += (lending_amount_f * lending_denominator_f / lending_rate_f) as u64;
            }
            msg!("LendingRate: {}", lending_rate_f as u64);

            if solend_higher {
                msg!("Solend Deposit");
                
                let cpi_program = ctx.accounts.solend_program.to_account_info();
                let cpi_accounts = solend::cpi::accounts::DepositToken {
                    authority: ctx.accounts.cbs_pda.to_account_info(),
                    user_token: ctx.accounts.collateral_pool.to_account_info(),
                    token_mint: ctx.accounts.collateral_mint.to_account_info(),
                    pool_token: ctx.accounts.solend_pool.to_account_info(),
                    config: ctx.accounts.solend_config.to_account_info(),
                    user_account: ctx.accounts.solend_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                };
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
                solend::cpi::deposit_token(cpi_ctx, lending_amount)?;
            } else {
                msg!("Apricot Deposit");         
                let cpi_program = ctx.accounts.apricot_program.to_account_info();
                let cpi_accounts = apricot::cpi::accounts::DepositToken {
                    authority: ctx.accounts.cbs_pda.to_account_info(),
                    user_token: ctx.accounts.collateral_pool.to_account_info(),
                    token_mint: ctx.accounts.collateral_mint.to_account_info(),
                    pool_token: ctx.accounts.apricot_pool.to_account_info(),
                    config: ctx.accounts.apricot_config.to_account_info(),
                    user_account: ctx.accounts.apricot_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                };
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
                apricot::cpi::deposit_token(cpi_ctx, lending_amount)?;
            }
        }

        //----- Normal Tokens ------
        if ctx.accounts.user_collateral.mint == config.ray_mint {
            user_account.ray_amount = user_account.ray_amount + pool_amount;
            config.total_deposited_ray = config.total_deposited_ray + amount;
        }

        if ctx.accounts.user_collateral.mint == config.msol_mint {
            user_account.msol_amount = user_account.msol_amount + pool_amount;
            config.total_deposited_msol = config.total_deposited_msol + amount;
        }        

        if ctx.accounts.user_collateral.mint == config.srm_mint {
            user_account.srm_amount = user_account.srm_amount + pool_amount;
            config.total_deposited_srm = config.total_deposited_srm + amount;
        }

        if ctx.accounts.user_collateral.mint == config.scnsol_mint {
            user_account.scnsol_amount = user_account.scnsol_amount + pool_amount;
            config.total_deposited_scnsol = config.total_deposited_scnsol + amount;
        }
        
        if ctx.accounts.user_collateral.mint == config.stsol_mint {
            user_account.stsol_amount = user_account.stsol_amount + pool_amount;
            config.total_deposited_stsol = config.total_deposited_stsol + amount;
        }
        
        //----- LpFinance Tokens ------
        if ctx.accounts.user_collateral.mint == config.lpusd_mint {
            user_account.lpusd_amount = user_account.lpusd_amount + amount;
            config.total_deposited_lpusd = config.total_deposited_lpusd + amount;
        }

        if ctx.accounts.user_collateral.mint == config.lpsol_mint {
            user_account.lpsol_amount = user_account.lpsol_amount + amount;
            config.total_deposited_lpsol = config.total_deposited_lpsol + amount;
        }

        if ctx.accounts.user_collateral.mint == config.lpfi_mint {
            user_account.lpfi_amount = user_account.lpfi_amount + amount;
            config.total_deposited_lpfi = config.total_deposited_lpfi + amount;
        }
        
        Ok(())
    }

    pub fn borrow_lptoken(
        ctx: Context<BorrowLpToken>,
        amount: u64
    ) -> Result<()> {
        msg!("Borrow LpToken");

        if amount < 1 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        // Deposited Collateral Tokens' TotalPrice. Need to be calculated with LTV
        let mut total_price: u128 = 0;
        let mut total_borrowed_price: u128 = 0;
        let user_account = &mut ctx.accounts.user_account;


        if user_account.step_num > 0 && user_account.step_num < 6 {
            return Err(ErrorCode::ProgressInLiquidate.into());
        }

        let config = &mut ctx.accounts.config;

        
        let lpusd_swap_amount: u64 = ctx.accounts.stable_lpusd_pool.get_swap_rate(PRICE_UNIT)?;
        let lpsol_swap_amount: u64 = ctx.accounts.stable_lpsol_pool.get_swap_rate(PRICE_UNIT)?;

        // RAY price        
        let ray_price: u128 = get_price(ctx.accounts.pyth_ray_account.to_account_info())?;    
        total_price += ray_price * (user_account.ray_amount + user_account.lending_ray_amount )as u128;

        // wSOL price
        let wsol_price: u128 = get_price(ctx.accounts.pyth_sol_account.to_account_info())?;    
        total_price += wsol_price * (user_account.wsol_amount + user_account.lending_wsol_amount) as u128;


        // mSOL price
        let msol_price: u128 = get_price(ctx.accounts.pyth_msol_account.to_account_info())?;
        total_price += msol_price * (user_account.msol_amount + user_account.lending_msol_amount ) as u128;

        // srm price
        let srm_price: u128 = get_price(ctx.accounts.pyth_srm_account.to_account_info())?;    
        total_price += srm_price * (user_account.srm_amount + user_account.lending_srm_amount ) as u128;

        // scnsol price
        let scnsol_price: u128 = get_price(ctx.accounts.pyth_scnsol_account.to_account_info())?;
        total_price += scnsol_price * (user_account.scnsol_amount + user_account.lending_scnsol_amount ) as u128;

        // stsol price
        let stsol_price: u128 = get_price(ctx.accounts.pyth_stsol_account.to_account_info())?;
        total_price += stsol_price * (user_account.stsol_amount + user_account.lending_stsol_amount ) as u128;

        // LpFi price
        let lpfi_price: u128 = get_price(ctx.accounts.pyth_usdc_account.to_account_info())? * ctx.accounts.liquidity_pool.get_price()? / PRICE_DENOMINATOR;
        total_price += lpfi_price * user_account.lpfi_amount as u128;

        // LpUSD price
        let usdc_price: u128 = get_price(ctx.accounts.pyth_usdc_account.to_account_info())?;
        let lpusd_price = usdc_price * lpusd_swap_amount as u128/ PRICE_UNIT as u128;        
        total_price += lpusd_price * user_account.lpusd_amount as u128;

        // LpSOL price
        let lpsol_price = wsol_price * lpsol_swap_amount as u128 / PRICE_UNIT as u128;
        total_price += lpsol_price * user_account.lpsol_amount as u128;

        msg!("LpFinance token price: {}, {} ", lpusd_price, lpsol_price);

        // Total Borrowed AMount
        total_borrowed_price += lpusd_price * user_account.borrowed_lpusd as u128;
        total_borrowed_price += lpsol_price * user_account.borrowed_lpsol as u128;

        let mut borrow_value: u128 = amount as u128;
        
        if ctx.accounts.lptoken_mint.key() == config.lpusd_mint {
            borrow_value = borrow_value * lpusd_price;

            config.total_borrowed_lpusd = config.total_borrowed_lpusd + amount;
            user_account.borrowed_lpusd = user_account.borrowed_lpusd + amount;
        } else if ctx.accounts.lptoken_mint.key() == config.lpsol_mint {
            borrow_value = borrow_value * lpsol_price;

            config.total_borrowed_lpsol = config.total_borrowed_lpsol + amount;
            user_account.borrowed_lpsol = user_account.borrowed_lpsol + amount;
        } else {
            return Err(ErrorCode::InvalidToken.into());
        }

        let borrable_total = total_price * LTV / DOMINATOR - total_borrowed_price;

        if borrable_total > borrow_value {
            let (program_authority, program_authority_bump) = 
            Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
        
            if program_authority != ctx.accounts.cbs_pda.to_account_info().key() {
                return Err(ErrorCode::InvalidCBSOwner.into());
            }

            let seeds = &[
                PREFIX.as_bytes(),
                &[program_authority_bump]
            ];
            let signer = &[&seeds[..]];

            // Mint
            let cpi_program = ctx.accounts.lptokens_program.to_account_info();
            let cpi_accounts = MintLpToken {
                signer: ctx.accounts.cbs_pda.to_account_info(),
                state_account: ctx.accounts.tokens_state.to_account_info(),
                config: ctx.accounts.lptoken_config.to_account_info(),
                lptoken_mint: ctx.accounts.lptoken_mint.to_account_info(),
                user_lptoken: ctx.accounts.user_lptoken.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info()
            };

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            lpfinance_tokens::cpi::mint_lptoken(cpi_ctx, amount)?;
        } else {
            return Err(ErrorCode::BorrowExceed.into());
        }

        Ok(())
    }

    pub fn withdraw_token(
        ctx: Context<WithdrawToken>,
        amount: u64
    ) -> Result<()> {
        msg!("Withdraw Token");

        let user_account = &mut ctx.accounts.user_account;

        if user_account.step_num > 0 && user_account.step_num < 6 {
            return Err(ErrorCode::ProgressInLiquidate.into());
        }

        let solend_config = &mut ctx.accounts.solend_config;
        let apricot_config = &mut ctx.accounts.apricot_config;

        let lpusd_swap_amount: u64 = ctx.accounts.stable_lpusd_pool.get_swap_rate(PRICE_UNIT)?;
        let lpsol_swap_amount: u64 = ctx.accounts.stable_lpsol_pool.get_swap_rate(PRICE_UNIT)?;

        let wsol_amount = user_account.wsol_amount as u128;
        let ray_amount = user_account.ray_amount as u128;
        let msol_amount = user_account.msol_amount as u128;
        let srm_amount = user_account.srm_amount as u128;
        let scnsol_amount = user_account.scnsol_amount as u128;
        let stsol_amount = user_account.stsol_amount as u128;

        let lending_wsol_amount = user_account.lending_wsol_amount as u128;
        let lending_ray_amount = user_account.lending_ray_amount as u128;
        let lending_msol_amount = user_account.lending_msol_amount as u128;
        let lending_srm_amount = user_account.lending_srm_amount as u128;
        let lending_scnsol_amount = user_account.lending_scnsol_amount as u128;
        let lending_stsol_amount = user_account.lending_stsol_amount as u128;

        let lpsol_amount = user_account.lpsol_amount as u128;
        let lpusd_amount = user_account.lpusd_amount as u128;
        let lpfi_amount = user_account.lpfi_amount as u128;

        let borrowed_lpusd = user_account.borrowed_lpusd as u128;
        let borrowed_lpsol = user_account.borrowed_lpsol as u128;

        let mut total_price: u128 = 0;

        // RAY price
        let ray_price: u128 = get_price(ctx.accounts.pyth_ray_account.to_account_info())?;     
        total_price += ray_price * (ray_amount + lending_ray_amount);
        if ray_price <= 0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }
        msg!("Ray price: {}, {} ", ray_price, total_price);

        // SOL price
        let sol_price: u128 = get_price(ctx.accounts.pyth_sol_account.to_account_info())?;     
        total_price += sol_price * (wsol_amount + lending_wsol_amount);
        if sol_price <= 0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        msg!("wSOL price: {}", sol_price);
        // mSOL price
        let msol_price: u128 = get_price(ctx.accounts.pyth_msol_account.to_account_info())?;
        total_price += msol_price * (msol_amount + lending_msol_amount);
        if msol_price <= 0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        // srm price
        let srm_price: u128 = get_price(ctx.accounts.pyth_srm_account.to_account_info())?;     
        total_price += srm_price * (srm_amount + lending_srm_amount);
        if srm_price <= 0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        // scnsol price
        let scnsol_price: u128 = get_price(ctx.accounts.pyth_scnsol_account.to_account_info())?;     
        total_price += scnsol_price * (scnsol_amount + lending_scnsol_amount);
        if scnsol_price <= 0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        // stsol price
        let stsol_price: u128 = get_price(ctx.accounts.pyth_stsol_account.to_account_info())?;
        total_price += stsol_price * (stsol_amount + lending_stsol_amount);
        if stsol_price <= 0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        // LpUSD price
        let usdc_price: u128 = get_price(ctx.accounts.pyth_usdc_account.to_account_info())?;
        let lpusd_price = usdc_price * lpusd_swap_amount as u128/ PRICE_UNIT as u128;        
        total_price += lpusd_price * lpusd_amount;
        if lpusd_price <= 0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        // LpSOL price
        let lpsol_price = sol_price * lpsol_swap_amount as u128 / PRICE_UNIT as u128;
        total_price += lpsol_price * lpsol_amount;
        if lpsol_price <= 0 {
            return Err(ErrorCode::InvalidPythPrice.into());
        }

        // LpFi price
        let lpfi_price: u128 = usdc_price * ctx.accounts.liquidity_pool.get_price()? / PRICE_DENOMINATOR;
        total_price += lpfi_price * lpfi_amount;

        let mut borrowed_total: u128 = 0;
        borrowed_total += borrowed_lpsol * lpsol_price;
        borrowed_total += borrowed_lpusd * lpusd_price;

        msg!("Lpfi price: {}, {}, {} ", lpfi_price, total_price, borrowed_total);

        if total_price * LTV < borrowed_total * DOMINATOR {
            return Err(ErrorCode::LTVAlreadyExceed.into());
        }        
        
        let mut dest_price:u128 = 0;
        let mut owned_amount:u128 = 0;
        let mut require_lending_amount: u64 = 0;

        if ctx.accounts.dest_mint.key() == ctx.accounts.config.ray_mint {
            msg!("Ray ----");
            if solend_config.ray_rate > apricot_config.ray_rate {
                owned_amount = ray_amount + solend_config.ray_rate as u128 * lending_ray_amount / LENDING_DENOMINATOR;
                require_lending_amount = (solend_config.ray_rate as u128 * lending_ray_amount / LENDING_DENOMINATOR) as u64;
            } else {
                owned_amount = ray_amount + apricot_config.ray_rate as u128 * lending_ray_amount / LENDING_DENOMINATOR;
                require_lending_amount = (apricot_config.ray_rate as u128 * lending_ray_amount / LENDING_DENOMINATOR) as u64;
            }
            dest_price = ray_price;
            user_account.ray_amount = owned_amount as u64 - amount;
            user_account.lending_ray_amount = 0;
            ctx.accounts.config.total_deposited_ray -= amount;
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.wsol_mint {
            if solend_config.wsol_rate > apricot_config.wsol_rate {
                owned_amount = wsol_amount + solend_config.wsol_rate as u128 * lending_wsol_amount / LENDING_DENOMINATOR;
                require_lending_amount = (solend_config.wsol_rate as u128 * lending_wsol_amount / LENDING_DENOMINATOR) as u64;
            } else {
                owned_amount = wsol_amount + apricot_config.wsol_rate as u128 * lending_wsol_amount / LENDING_DENOMINATOR;
                require_lending_amount = (apricot_config.wsol_rate as u128 * lending_wsol_amount / LENDING_DENOMINATOR) as u64;
            }
            dest_price = sol_price;
            user_account.wsol_amount = owned_amount as u64 - amount;
            ctx.accounts.config.total_deposited_wsol -= amount;

            user_account.lending_wsol_amount = 0;
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.msol_mint {
            if solend_config.msol_rate > apricot_config.msol_rate {
                owned_amount = msol_amount + solend_config.msol_rate as u128 * lending_msol_amount / LENDING_DENOMINATOR;
                require_lending_amount = (solend_config.msol_rate as u128 * lending_msol_amount / LENDING_DENOMINATOR) as u64;
            } else {
                owned_amount = msol_amount + apricot_config.msol_rate as u128 * lending_msol_amount / LENDING_DENOMINATOR;
                require_lending_amount = (apricot_config.msol_rate as u128 * lending_msol_amount / LENDING_DENOMINATOR) as u64;
            }
            dest_price = msol_price;
            user_account.msol_amount = owned_amount as u64 - amount;
            ctx.accounts.config.total_deposited_msol -= amount;

            user_account.lending_msol_amount = 0;
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.srm_mint {
            if solend_config.srm_rate > apricot_config.srm_rate {
                owned_amount = srm_amount + solend_config.srm_rate as u128 * lending_srm_amount / LENDING_DENOMINATOR;
                require_lending_amount = (solend_config.srm_rate as u128 * lending_srm_amount / LENDING_DENOMINATOR) as u64;
            } else {
                owned_amount = srm_amount + apricot_config.srm_rate as u128 * lending_srm_amount / LENDING_DENOMINATOR;
                require_lending_amount = (apricot_config.srm_rate as u128 * lending_srm_amount / LENDING_DENOMINATOR) as u64;
            }
            dest_price = srm_price;
            user_account.srm_amount = owned_amount as u64 - amount;
            ctx.accounts.config.total_deposited_srm -= amount;
            user_account.lending_srm_amount = 0;
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.scnsol_mint {
            if solend_config.scnsol_rate > apricot_config.scnsol_rate {
                owned_amount = scnsol_amount + solend_config.scnsol_rate as u128 * lending_scnsol_amount / LENDING_DENOMINATOR;
                require_lending_amount = (solend_config.scnsol_rate as u128 * lending_scnsol_amount / LENDING_DENOMINATOR) as u64;
            } else {
                owned_amount = scnsol_amount + apricot_config.scnsol_rate as u128 * lending_scnsol_amount / LENDING_DENOMINATOR;
                require_lending_amount = (apricot_config.scnsol_rate as u128 * lending_scnsol_amount / LENDING_DENOMINATOR) as u64;
            }
            dest_price = scnsol_price;
            user_account.scnsol_amount = owned_amount as u64 - amount;
            ctx.accounts.config.total_deposited_scnsol -= amount;
            user_account.lending_scnsol_amount = 0;
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.stsol_mint {
            if solend_config.stsol_rate > apricot_config.stsol_rate {
                owned_amount = stsol_amount + solend_config.stsol_rate as u128 * lending_stsol_amount / LENDING_DENOMINATOR;
                require_lending_amount = (solend_config.stsol_rate as u128 * lending_stsol_amount / LENDING_DENOMINATOR) as u64;
            } else {
                owned_amount = stsol_amount + apricot_config.stsol_rate as u128 * lending_stsol_amount / LENDING_DENOMINATOR;
                require_lending_amount = (apricot_config.stsol_rate as u128 * lending_stsol_amount / LENDING_DENOMINATOR) as u64;
            }
            dest_price = stsol_price;
            user_account.stsol_amount = owned_amount as u64 - amount;
            ctx.accounts.config.total_deposited_stsol -= amount;
            user_account.lending_stsol_amount = 0;
        } 
        // else if ctx.accounts.dest_mint.key() == ctx.accounts.config.lpfi_mint {
        //     dest_price = lpfi_price;
        //     owned_amount = lpfi_amount;
        //     user_account.lpfi_amount -= amount;
        //     ctx.accounts.config.total_deposited_lpfi -= amount;
        // } 
        else if ctx.accounts.dest_mint.key() == ctx.accounts.config.lpusd_mint {
            dest_price = lpusd_price;
            owned_amount = lpusd_amount;
            user_account.lpusd_amount -= amount;
            ctx.accounts.config.total_deposited_lpusd -= amount;
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.lpsol_mint {
            dest_price = lpsol_price;
            owned_amount = lpsol_amount;
            user_account.lpsol_amount -= amount;
            ctx.accounts.config.total_deposited_lpsol -= amount;
        } else {
            return Err(ErrorCode::InvalidToken.into());
        }        

        let borrowable_amount = (total_price * LTV / DOMINATOR - borrowed_total) / dest_price;
        if amount > borrowable_amount as u64{
            return Err(ErrorCode::InvalidAmount.into());
        }
        
        let (program_authority, program_authority_bump) = 
            Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
        
        if program_authority != ctx.accounts.cbs_pda.to_account_info().key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let config = &mut ctx.accounts.config;
        if require_lending_amount > 0 && (ctx.accounts.user_dest.mint == config.wsol_mint || 
           ctx.accounts.user_dest.mint == config.msol_mint || 
           ctx.accounts.user_dest.mint == config.ray_mint || 
           ctx.accounts.user_dest.mint == config.srm_mint || 
           ctx.accounts.user_dest.mint == config.scnsol_mint || 
           ctx.accounts.user_dest.mint == config.stsol_mint) {

            let mut solend_higher = false;

            if ctx.accounts.user_dest.mint == config.ray_mint {
                if ctx.accounts.solend_config.ray_rate > ctx.accounts.apricot_config.ray_rate {
                    solend_higher = true;
                }
            } else if ctx.accounts.user_dest.mint == config.wsol_mint {
                if ctx.accounts.solend_config.wsol_rate > ctx.accounts.apricot_config.wsol_rate {
                    solend_higher = true;
                }
            } else if ctx.accounts.user_dest.mint == config.msol_mint {
                if ctx.accounts.solend_config.msol_rate > ctx.accounts.apricot_config.msol_rate {
                    solend_higher = true;
                }
            } else if ctx.accounts.user_dest.mint == config.srm_mint {
                if ctx.accounts.solend_config.srm_rate > ctx.accounts.apricot_config.srm_rate {
                    solend_higher = true;
                }
            } else if ctx.accounts.user_dest.mint == config.scnsol_mint {
                if ctx.accounts.solend_config.scnsol_rate > ctx.accounts.apricot_config.scnsol_rate {
                    solend_higher = true;
                }
            } else if ctx.accounts.user_dest.mint == config.stsol_mint {
                if ctx.accounts.solend_config.stsol_rate > ctx.accounts.apricot_config.stsol_rate {
                    solend_higher = true;
                }
            } 
            
            if solend_higher {
                msg!("Withdraw from solend {}", require_lending_amount);
                let cpi_program = ctx.accounts.solend_program.to_account_info();
                let cpi_accounts = solend::cpi::accounts::WithdrawToken {
                    authority: ctx.accounts.cbs_pda.to_account_info(),
                    user_token: ctx.accounts.dest_pool.to_account_info(),
                    token_mint: ctx.accounts.dest_mint.to_account_info(),
                    pool_token: ctx.accounts.solend_pool.to_account_info(),
                    config: ctx.accounts.solend_config.to_account_info(),
                    user_account: ctx.accounts.solend_account.to_account_info(),
                    state_account: ctx.accounts.solend_state_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                };
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
                solend::cpi::withdraw_token(cpi_ctx, require_lending_amount)?;
            } else {
                msg!("Withdraw from apricot");
                let cpi_program = ctx.accounts.apricot_program.to_account_info();
                let cpi_accounts = apricot::cpi::accounts::WithdrawToken {
                    authority: ctx.accounts.cbs_pda.to_account_info(),
                    user_token: ctx.accounts.dest_pool.to_account_info(),
                    token_mint: ctx.accounts.dest_mint.to_account_info(),
                    pool_token: ctx.accounts.apricot_pool.to_account_info(),
                    state_account: ctx.accounts.apricot_state_account.to_account_info(),
                    config: ctx.accounts.apricot_config.to_account_info(),
                    user_account: ctx.accounts.apricot_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                };
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
                apricot::cpi::withdraw_token(cpi_ctx, require_lending_amount)?;
            }
        }


        msg!("Witndraw from cbs");
        let cpi_accounts = Transfer {
            from: ctx.accounts.dest_pool.to_account_info(),
            to: ctx.accounts.user_dest.to_account_info(),
            authority: ctx.accounts.cbs_pda.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    // Typeless payback
    pub fn repay_token(
        ctx: Context<RepayToken>,
        amount: u64
    ) -> Result<()> {
        if ctx.accounts.user_dest.amount < amount {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        let user_account =&mut ctx.accounts.user_account;

        if user_account.step_num > 0 && user_account.step_num < 6 {
            return Err(ErrorCode::ProgressInLiquidate.into());
        }

        let config = &mut ctx.accounts.config;

        // Validate Token
        if ctx.accounts.user_dest.mint != config.wsol_mint && 
            ctx.accounts.user_dest.mint != config.lpusd_mint &&
            ctx.accounts.user_dest.mint != config.lpsol_mint
        {
            return Err(ErrorCode::InvalidToken.into());
        }

        if ctx.accounts.user_dest.mint == config.wsol_mint   
        {
            // Swap wsol to LpSOL and burn LpSOL
            let cpi_accounts = Transfer {
                from: ctx.accounts.user_dest.to_account_info(),
                to: ctx.accounts.dest_pool.to_account_info(),
                authority: ctx.accounts.user_authority.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, amount)?;
        }

        if ctx.accounts.user_dest.mint == config.lpusd_mint ||
            ctx.accounts.user_dest.mint == config.lpsol_mint 
        {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.dest_mint.to_account_info(),
                    from: ctx.accounts.user_dest.to_account_info(),
                    authority: ctx.accounts.user_authority.to_account_info()
                }
            );

            token::burn(cpi_ctx, amount)?;
        }

        if ctx.accounts.user_dest.mint == config.lpusd_mint{  
            if user_account.borrowed_lpusd < amount || config.total_borrowed_lpusd < amount {
                return Err(ErrorCode::RepayFinished.into());
            }

            user_account.borrowed_lpusd = user_account.borrowed_lpusd - amount;
            config.total_borrowed_lpusd = config.total_borrowed_lpusd - amount;  
        } else if ctx.accounts.user_dest.mint == config.lpsol_mint || ctx.accounts.user_dest.mint == config.wsol_mint {
            if user_account.borrowed_lpsol < amount || config.total_borrowed_lpsol < amount {
                return Err(ErrorCode::RepayFinished.into());
            }
            user_account.borrowed_lpsol = user_account.borrowed_lpsol - amount;
            config.total_borrowed_lpsol = config.total_borrowed_lpsol - amount;            
        }

        Ok(())
    }

    // The first step 
    pub fn liquidate_collateral(
        ctx: Context<LiquidateCollateral>
    ) -> Result<()> {
        msg!("liquidate_collateral started");

        let user_account = &mut ctx.accounts.user_account;

        let wsol_amount = user_account.wsol_amount;
        let ray_amount = user_account.ray_amount;
        let msol_amount = user_account.msol_amount;
        let srm_amount = user_account.srm_amount;
        let scnsol_amount = user_account.scnsol_amount;
        let stsol_amount = user_account.stsol_amount;

        let (program_authority, program_authority_bump) = 
            Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
        
        if program_authority != ctx.accounts.state_account.to_account_info().key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];


        if msol_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_msol.to_account_info(),
                to: ctx.accounts.auction_msol.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, msol_amount)?;
        }

        if ray_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_ray.to_account_info(),
                to: ctx.accounts.auction_ray.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, ray_amount)?;
        }

        if wsol_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_wsol.to_account_info(),
                to: ctx.accounts.auction_wsol.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, wsol_amount)?;
        }

        
        if srm_amount > 0 {
            msg!("liquidate_collateral srm_amount");
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_srm.to_account_info(),
                to: ctx.accounts.auction_srm.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, srm_amount)?;
        }

        if scnsol_amount > 0 {
            msg!("liquidate_collateral scnsol_amount");
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_scnsol.to_account_info(),
                to: ctx.accounts.auction_scnsol.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, scnsol_amount)?;
        }

        if stsol_amount > 0 {
            msg!("liquidate_collateral stsol_amount");
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_stsol.to_account_info(),
                to: ctx.accounts.auction_stsol.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, stsol_amount)?;
        }

        user_account.step_num = 1;

        // user_account.wsol_amount = 0;
        // user_account.ray_amount = 0;
        // user_account.msol_amount = 0;
        // user_account.srm_amount = 0;
        // user_account.scnsol_amount = 0;
        // user_account.stsol_amount = 0;

        Ok(())
    }

    // Transfer collateral tokens to auction pool from cbs
    pub fn liquidate_lptoken_collateral(
        ctx: Context<LiquidateLpTokenCollateral>
    ) -> Result<()> {
        msg!("liquidate_collateral started");

        let user_account = &mut ctx.accounts.user_account;


        let lpusd_amount = user_account.lpusd_amount;
        let lpsol_amount = user_account.lpsol_amount;
        let lpfi_amount = user_account.lpfi_amount;


        let (program_authority, program_authority_bump) = 
            Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
        
        if program_authority != ctx.accounts.state_account.to_account_info().key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];

        msg!("Lpusd amount: !!{:?}!!", lpusd_amount.to_string());

        if lpusd_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_lpusd.to_account_info(),
                to: ctx.accounts.auction_lpusd.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, lpusd_amount)?;
        }

        if lpsol_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_lpsol.to_account_info(),
                to: ctx.accounts.auction_lpsol.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, lpsol_amount)?;
        }

        if lpfi_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_lpfi.to_account_info(),
                to: ctx.accounts.auction_lpfi.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, lpfi_amount)?;
        }

        user_account.step_num = 3;
        // user_account.lpusd_amount = 0;
        // user_account.lpsol_amount = 0;
        // user_account.lpfi_amount = 0;
        
        Ok(())
    }

    pub fn update_user_account(
        ctx: Context<UpdateUserAccount>,
        step: u8
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.step_num = step;

        // Need to reset everything
        if step == 10 {
            user_account.step_num = 0;

            user_account.lpusd_amount = 0;
            user_account.lpsol_amount = 0;
            user_account.lpfi_amount = 0;

            user_account.wsol_amount = 0;
            user_account.ray_amount = 0;
            user_account.msol_amount = 0;
            user_account.srm_amount = 0;
            user_account.scnsol_amount = 0;
            user_account.stsol_amount = 0;

            user_account.borrowed_lpusd = 0;
            user_account.borrowed_lpsol = 0;

            user_account.lending_ray_amount = 0;
            user_account.lending_wsol_amount = 0;
            user_account.lending_msol_amount = 0;
            user_account.lending_srm_amount = 0;
            user_account.lending_scnsol_amount = 0;
            user_account.lending_stsol_amount = 0;
        }
        Ok(())
    }

    pub fn apply_dsf(
        ctx: Context<UpdateUserAccount>,
        lpusd_rate: u64,
        lpsol_rate: u64
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.borrowed_lpusd = user_account.borrowed_lpusd * (100 + lpusd_rate) / 100;
        user_account.borrowed_lpsol = user_account.borrowed_lpsol * (100 + lpsol_rate) / 100;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient Amount From User Account")]
    InsufficientUserAmount,
    #[msg("Insufficient Amount")]
    InsufficientAmount,
    #[msg("Borrow Failed")]
    BorrowFailed,
    #[msg("Borrow Exceed")]
    BorrowExceed,
    #[msg("LTV Already Exceed")]
    LTVAlreadyExceed,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Invalid Token")]
    InvalidToken,
    #[msg("Invalid Owner")]
    InvalidOwner,
    #[msg("Invalid CBS Owner")]
    InvalidCBSOwner,
    #[msg("In Liquidating")]
    ProgressInLiquidate,
    #[msg("Repay finished for the selected token")]
    RepayFinished,
    #[msg("Invalid pyth price")]
    InvalidPythPrice,
}
