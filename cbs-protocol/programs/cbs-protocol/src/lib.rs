use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Mint, Burn };

mod states;
pub use states::*;

use lpfinance_tokens::cpi::accounts::MintLpToken;
use lpfinance_tokens::{self};

use stable_swap::{self, StableswapPool};
use uniswap::{self, UniswapPool};
use uniswap::cpi::accounts::UniswapTokens;

use solend::{self};
use apricot::{self};

declare_id!("8NSpbuD66CrveJYufKZWiJPneVak7Ri74115qpiP8xw4");

const LTV: f64 = 85.0;
const DOMINATOR: f64 = 100.0;

const LENDING_PERCENT: u64 = 10; // 10%

// const W_THRESHHOLD: u64 = 90;
// const S_THRESHHOLD: u64 = 75;



/// Return: 
/// true if solend APR is bigger than apricot
/// APR value
pub fn get_lending_protocol_info(
    dest_mint: &mut Account<Mint>,
    config: &mut Account<Config>,
    solend_config: &mut Account<solend::Config>,
    apricot_config: &mut Account<apricot::Config>
) -> (bool, u64, u64, u64) {
    let dest_mint_key = dest_mint.key();
    let mut _is_solend_rate_higher = false;
    let mut _lending_rate = 0;
    let mut _solend_rate = 0;
    let mut _apricot_rate = 0;
    if dest_mint_key == config.ray_mint {
        _solend_rate = solend_config.ray_rate;
        _apricot_rate = apricot_config.ray_rate;
    } else if dest_mint_key == config.wsol_mint {
        _solend_rate = solend_config.wsol_rate;
        _apricot_rate = apricot_config.wsol_rate;
    } else if dest_mint_key == config.msol_mint {
        _solend_rate = solend_config.msol_rate;
        _apricot_rate = apricot_config.msol_rate;
    } else if dest_mint_key == config.srm_mint {
        _solend_rate = solend_config.srm_rate;
        _apricot_rate = apricot_config.srm_rate;
    } else if dest_mint_key == config.scnsol_mint {
        _solend_rate = solend_config.scnsol_rate;
        _apricot_rate = apricot_config.scnsol_rate;
    } else if dest_mint_key == config.stsol_mint {
        _solend_rate = solend_config.stsol_rate;
        _apricot_rate = apricot_config.stsol_rate;
    }

    if _solend_rate > _apricot_rate {
        _is_solend_rate_higher = true;
        _lending_rate = _solend_rate;
    } else {
        _lending_rate = _apricot_rate;
    }
    return (_is_solend_rate_higher, _lending_rate, _solend_rate, _apricot_rate);
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

    pub fn create_token_ata1(
        ctx: Context<CreateTokenATA1>
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

        // token account
        config.pool_ray = ctx.accounts.pool_ray.key();
        config.pool_wsol = ctx.accounts.pool_wsol.key();
        config.pool_msol = ctx.accounts.pool_msol.key();

        Ok(())
    }

    pub fn create_token_ata2(
        ctx: Context<CreateTokenATA2>
    ) -> Result<()> {
        msg!("INITIALIZE Token ATAs");

        let config = &mut ctx.accounts.config;

        if config.owner != ctx.accounts.authority.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // token mint
        config.srm_mint = ctx.accounts.srm_mint.key();
        config.scnsol_mint = ctx.accounts.scnsol_mint.key();
        config.stsol_mint = ctx.accounts.stsol_mint.key();

        // token account
        config.pool_srm = ctx.accounts.pool_srm.key();
        config.pool_scnsol = ctx.accounts.pool_scnsol.key();
        config.pool_stsol = ctx.accounts.pool_stsol.key();

        Ok(())
    }


    pub fn create_usdc_escrow(
        _ctx: Context<CreateEscrowUSDC>
    ) -> Result<()> {
        msg!("INITIALIZE USDC Escrow");

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
        let solend_config: &mut Account<solend::Config> = &mut ctx.accounts.solend_config;
        let apricot_config: &mut Account<apricot::Config> = &mut ctx.accounts.apricot_config;
        let dest_mint: &mut Account<Mint> = &mut ctx.accounts.collateral_mint;

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
        if config.is_normal_token(dest_mint.key())? == true {

            // If solend APY rate is higher than apricot APY rate, return true;
            let mut _solend_higher = false;
            let lending_amount_f: f64 = lending_amount as f64;
            let lending_denominator_f: f64 = LENDING_DENOMINATOR as f64;
            let mut _lending_rate = 0;
            let mut _lending_rate_f: f64 = 0.0;

            let mut _solend_rate = 0;
            let mut _apricot_rate = 0;

            (_solend_higher, _lending_rate, _solend_rate, _apricot_rate) = get_lending_protocol_info(dest_mint, config, solend_config, apricot_config);

            _lending_rate_f = _lending_rate as f64;

            let additional_lending_amount: u64 = (lending_amount_f * lending_denominator_f / _lending_rate_f) as u64;
            let total_lending_amount = user_account.get_key_lending_amount(dest_mint.key(), config)? + additional_lending_amount;
            user_account.update_lending_amount(total_lending_amount, dest_mint.key(), config)?;
            msg!("LendingRate: {}, {}, {}, {}", _lending_rate, _solend_higher, _solend_rate, _apricot_rate );

            if _solend_higher {
                msg!("Solend Deposit");
                
                let cpi_program = ctx.accounts.solend_program.to_account_info();
                let cpi_accounts = solend::cpi::accounts::DepositToken {
                    authority: ctx.accounts.cbs_pda.to_account_info(),
                    user_token: ctx.accounts.collateral_pool.to_account_info(),
                    token_mint: dest_mint.to_account_info(),
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
                    token_mint: dest_mint.to_account_info(),
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

        // update user account
        let key_amount = user_account.get_key_amount(dest_mint.key(), config)?;
        user_account.update_deposited_amount(key_amount + pool_amount, dest_mint.key(), config)?;
        user_account.update_lp_deposited_amount(key_amount + amount, dest_mint.key(), config)?;

        // update config account
        let config_key_amount = config.get_key_deposited_amount(dest_mint.key())?;
        config.update_total_deposited_amount(config_key_amount + amount, dest_mint.key())?;
        config.update_total_lp_deposited_amount(config_key_amount + amount, dest_mint.key())?;        
        
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
        let user_account = &mut ctx.accounts.user_account;

        if user_account.step_num > 0 && user_account.step_num < 6 {
            return Err(ErrorCode::ProgressInLiquidate.into());
        }

        let solend_config : &mut Account<solend::Config>= &mut ctx.accounts.solend_config;
        let apricot_config : &mut Account<apricot::Config>= &mut ctx.accounts.apricot_config;
        let lptoken_mint: &mut Account<Mint> = &mut ctx.accounts.lptoken_mint;
        let config: &mut Account<Config> = &mut ctx.accounts.config;

        let liquidity_pool: &Account<UniswapPool> = &ctx.accounts.liquidity_pool;
        let stable_lpusd_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpusd_pool;
        let stable_lpsol_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpsol_pool;

        let pyth_ray_account: &AccountInfo = &ctx.accounts.pyth_ray_account;
        let pyth_usdc_account: &AccountInfo = &ctx.accounts.pyth_usdc_account;
        let pyth_sol_account: &AccountInfo = &ctx.accounts.pyth_sol_account;
        let pyth_msol_account: &AccountInfo = &ctx.accounts.pyth_msol_account;
        let pyth_srm_account: &AccountInfo = &ctx.accounts.pyth_srm_account;
        let pyth_scnsol_account: &AccountInfo = &ctx.accounts.pyth_scnsol_account;
        let pyth_stsol_account: &AccountInfo = &ctx.accounts.pyth_stsol_account;

        if config.exist_token(lptoken_mint.key())? == false {
            msg!("Invalid token===");
            return Err(ErrorCode::InvalidToken.into());
        }

        let mut _ltv: u64 = 0;
        let mut _dest_price: u64 = 0;
        let mut _total_price: f64 = 0.0;
        let mut _borrowed_total: f64 = 0.0;

        (_ltv, _dest_price, _total_price, _borrowed_total) = user_account.get_ltv(
            lptoken_mint.key(),
            config,
            solend_config,
            apricot_config,
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

        let mut _borrow_value: f64 = amount as f64 * _dest_price as f64;
        let total_borrowed_amount = config.get_key_borrowed_amount(lptoken_mint.key())? + amount;
        let borrowed_amount = user_account.get_key_borrowed_amount(lptoken_mint.key(), config)? + amount;

        config.update_borrowed_amount(total_borrowed_amount, lptoken_mint.key())?;
        user_account.update_borrowed_amount(borrowed_amount, lptoken_mint.key(), config)?;
        
        let borrable_total: f64 = _total_price * LTV / DOMINATOR - _borrowed_total;

        if borrable_total > _borrow_value {
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

        let solend_config : &mut Account<solend::Config>= &mut ctx.accounts.solend_config;
        let apricot_config : &mut Account<apricot::Config>= &mut ctx.accounts.apricot_config;
        let dest_mint: &mut Account<Mint> = &mut ctx.accounts.dest_mint;
        let config: &mut Account<Config> = &mut ctx.accounts.config;

        let liquidity_pool: &Account<UniswapPool> = &ctx.accounts.liquidity_pool;
        let stable_lpusd_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpusd_pool;
        let stable_lpsol_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpsol_pool;

        let pyth_ray_account: &AccountInfo = &ctx.accounts.pyth_ray_account;
        let pyth_usdc_account: &AccountInfo = &ctx.accounts.pyth_usdc_account;
        let pyth_sol_account: &AccountInfo = &ctx.accounts.pyth_sol_account;
        let pyth_msol_account: &AccountInfo = &ctx.accounts.pyth_msol_account;
        let pyth_srm_account: &AccountInfo = &ctx.accounts.pyth_srm_account;
        let pyth_scnsol_account: &AccountInfo = &ctx.accounts.pyth_scnsol_account;
        let pyth_stsol_account: &AccountInfo = &ctx.accounts.pyth_stsol_account;

        if config.exist_token(dest_mint.key())? == false {
            return Err(ErrorCode::InvalidToken.into());
        }

        let mut _ltv: u64 = 0;
        let mut _dest_price: u64 = 0;
        let mut _total_price: f64 = 0.0;
        let mut _borrowed_total: f64 = 0.0;

        (_ltv, _dest_price, _total_price, _borrowed_total) = user_account.get_ltv(
            dest_mint.key(),
            config,
            solend_config,
            apricot_config,
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

        msg!("LTV {} {} {} {}", _ltv, _dest_price, _total_price, _borrowed_total);

        if _ltv > LTV as u64{
            return Err(ErrorCode::LTVAlreadyExceed.into());
        }                  

        let mut _solend_higher = false;
        let mut _lending_rate = 0;
        let mut _solend_rate = 0;
        let mut _apricot_rate = 0;

        (_solend_higher, _lending_rate, _solend_rate, _apricot_rate) = get_lending_protocol_info(
            dest_mint, 
            config, 
            solend_config, 
            apricot_config
        );


        let key_lending_amount = user_account.get_key_lending_amount(dest_mint.key(), config)?;
        let key_amount = user_account.get_key_amount(dest_mint.key(), config)?;
        let key_total_deposited_amount = config.get_key_deposited_amount(dest_mint.key())?;

        let withdrawable_amount: f64 = (_total_price - _borrowed_total * DOMINATOR / LTV) / _dest_price as f64;
        msg!("Borrowable {} {}", withdrawable_amount, _solend_higher);

        if amount > withdrawable_amount as u64{
            return Err(ErrorCode::InsufficientAmount.into());
        }
        
        let mut _solend_withdraw_amount: u64 = 0;
        let mut _apricot_withdraw_amount: u64 = 0;

        let mut _user_lending_amount_for = 0;

        if amount > key_amount {
            let withdraw_amount_from_lending: u64 = amount - key_amount;

            let solend_rate_f: f64 = _solend_rate as f64;
            let apricot_rate_f: f64 = _apricot_rate as f64;
            // let key_lending_amount_f: f64 = withdraw_amount_from_lending as f64;
            let denominator_f: f64 = LENDING_DENOMINATOR as f64;

            let solend_key_amount: u64 = ctx.accounts.solend_account.get_key_amount(dest_mint.key(), solend_config)?;
            let apricot_key_amount: u64 = ctx.accounts.apricot_account.get_key_amount(dest_mint.key(), apricot_config)?;

            let solend_total_amount: u64 =  (solend_rate_f * solend_key_amount as f64 / denominator_f) as u64;
            let apricot_total_amount: u64 =  (apricot_rate_f * apricot_key_amount as f64 / denominator_f) as u64;


            if _solend_higher {

                if withdraw_amount_from_lending<= solend_total_amount {
                    _solend_withdraw_amount = withdraw_amount_from_lending;

                    _user_lending_amount_for = (_solend_withdraw_amount as f64 * denominator_f / solend_rate_f) as u64;
                } else {
                    _solend_withdraw_amount = solend_total_amount;
                    _apricot_withdraw_amount = withdraw_amount_from_lending- solend_total_amount;
                    
                    if _apricot_withdraw_amount > apricot_total_amount {
                        return Err(ErrorCode::InsufficientAmount.into());
                    }

                    _user_lending_amount_for = solend_key_amount;
                    _user_lending_amount_for += (_apricot_withdraw_amount as f64 * denominator_f / apricot_rate_f) as u64;
                }

            } else {
                if withdraw_amount_from_lending<= apricot_total_amount {
                    _apricot_withdraw_amount = withdraw_amount_from_lending;

                    _user_lending_amount_for = (_apricot_withdraw_amount as f64 * denominator_f / apricot_rate_f) as u64;
                } else {
                    _apricot_withdraw_amount = apricot_total_amount;

                    _solend_withdraw_amount = withdraw_amount_from_lending- apricot_total_amount;
                    if _solend_withdraw_amount > solend_total_amount {
                        return Err(ErrorCode::InsufficientAmount.into());
                    }

                    _user_lending_amount_for = apricot_key_amount;
                    _user_lending_amount_for += (_solend_withdraw_amount as f64 * denominator_f / solend_rate_f) as u64;
                }
            }
            msg!("Lending: {} {}", _solend_withdraw_amount, _apricot_withdraw_amount );
        }
        

        let owned_amount: u64 = if amount > key_amount { 0} else { key_amount - amount };
        let total_deposited_amount = key_total_deposited_amount - amount;

        msg!("Lending: {} {}", key_lending_amount - _user_lending_amount_for, owned_amount );

        user_account.update_lending_amount(key_lending_amount - _user_lending_amount_for, dest_mint.key(), config)?;
        user_account.update_deposited_amount(owned_amount, dest_mint.key(), config)?;  
        config.update_total_deposited_amount(total_deposited_amount , dest_mint.key())?;

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
            
        if _solend_higher {
            msg!("Withdraw from solend {}", _solend_withdraw_amount);

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

            solend::cpi::withdraw_token(cpi_ctx, _solend_withdraw_amount)?;
        } 

        if _apricot_withdraw_amount > 0 {
            msg!("Withdraw from apricot {}", _apricot_withdraw_amount);
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

            apricot::cpi::withdraw_token(cpi_ctx, _apricot_withdraw_amount)?;
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

    pub fn get_ltv(ctx: Context<GetLTV>) -> Result<Vec<u64>> {
        let user_account = &ctx.accounts.user_account;

        let solend_config : &Account<solend::Config>= &ctx.accounts.solend_config;
        let apricot_config : &Account<apricot::Config>= &ctx.accounts.apricot_config;

        let liquidity_pool: &Account<UniswapPool> = &ctx.accounts.liquidity_pool;
        let stable_lpusd_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpusd_pool;
        let stable_lpsol_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpsol_pool;

        let pyth_ray_account: &AccountInfo = &ctx.accounts.pyth_ray_account;
        let pyth_usdc_account: &AccountInfo = &ctx.accounts.pyth_usdc_account;
        let pyth_sol_account: &AccountInfo = &ctx.accounts.pyth_sol_account;
        let pyth_msol_account: &AccountInfo = &ctx.accounts.pyth_msol_account;
        let pyth_srm_account: &AccountInfo = &ctx.accounts.pyth_srm_account;
        let pyth_scnsol_account: &AccountInfo = &ctx.accounts.pyth_scnsol_account;
        let pyth_stsol_account: &AccountInfo = &ctx.accounts.pyth_stsol_account;


        let user_ltv: LtvData = user_account.get_ltv_view(
            solend_config,
            apricot_config,
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
        
        let r_values: Vec<u64> = vec![user_ltv.ltv, user_ltv.r_total, user_ltv.r_borrow];

        Ok(r_values)
    }

    pub fn withdraw_lending(
        ctx: Context<WithdrawLending>
    ) -> Result<()> {
        msg!("Withdraw LendingToken");

        let user_account = &mut ctx.accounts.user_account;

        if user_account.step_num > 0 && user_account.step_num < 6 {
            return Err(ErrorCode::ProgressInLiquidate.into());
        }

        let solend_config : &mut Account<solend::Config>= &mut ctx.accounts.solend_config;
        let apricot_config : &mut Account<apricot::Config>= &mut ctx.accounts.apricot_config;
        let dest_mint: &mut Account<Mint> = &mut ctx.accounts.dest_mint;
        let config: &mut Account<Config> = &mut ctx.accounts.config;

        let liquidity_pool: &Account<UniswapPool> = &ctx.accounts.liquidity_pool;
        let stable_lpusd_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpusd_pool;
        let stable_lpsol_pool: &Account<StableswapPool> = &ctx.accounts.stable_lpsol_pool;

        let pyth_ray_account: &AccountInfo = &ctx.accounts.pyth_ray_account;
        let pyth_usdc_account: &AccountInfo = &ctx.accounts.pyth_usdc_account;
        let pyth_sol_account: &AccountInfo = &ctx.accounts.pyth_sol_account;
        let pyth_msol_account: &AccountInfo = &ctx.accounts.pyth_msol_account;
        let pyth_srm_account: &AccountInfo = &ctx.accounts.pyth_srm_account;
        let pyth_scnsol_account: &AccountInfo = &ctx.accounts.pyth_scnsol_account;
        let pyth_stsol_account: &AccountInfo = &ctx.accounts.pyth_stsol_account;

        if config.exist_token(dest_mint.key())? == false {
            return Err(ErrorCode::InvalidToken.into());
        }

        let key_lending_amount = user_account.get_key_lending_amount(dest_mint.key(), config)?;
        let key_amount = user_account.get_key_amount(dest_mint.key(), config)?;

        if key_lending_amount <= 0 {
            Ok(())
        } else {

            let mut _ltv: u64 = 0;
            let mut _dest_price: u64 = 0;
            let mut _total_price: f64 = 0.0;
            let mut _borrowed_total: f64 = 0.0;

            (_ltv, _dest_price, _total_price, _borrowed_total) = user_account.get_ltv(
                dest_mint.key(),
                config,
                solend_config,
                apricot_config,
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
    
            // Threshold 90
            // if _ltv < 90 {
            //     return Err(ErrorCode::LTVAlreadyExceed.into());
            // }        
            
            let mut _solend_higher = false;
            let mut _lending_rate = 0;
            let mut _solend_rate = 0;
            let mut _apricot_rate = 0;

            (_solend_higher, _lending_rate, _solend_rate, _apricot_rate) = get_lending_protocol_info(
                dest_mint, 
                config, 
                solend_config, 
                apricot_config
            );
        
            let solend_rate_f: f64 = _solend_rate as f64;
            let apricot_rate_f: f64 = _apricot_rate as f64;
            let key_lending_amount_f: f64 = key_lending_amount as f64;
            let denominator_f: f64 = LENDING_DENOMINATOR as f64;


            let mut _solend_withdraw_amount: u64 = 0;
            let mut _apricot_withdraw_amount: u64 = 0;

            if _solend_higher {
                if key_lending_amount <= ctx.accounts.solend_account.get_key_amount(dest_mint.key(), solend_config)? {
                    _solend_withdraw_amount = (solend_rate_f * key_lending_amount_f / denominator_f) as u64;
                } else {
                    // The max value to be able to withdraw from solend
                    let solend_withdraw_amount = ctx.accounts.solend_account.get_key_amount(dest_mint.key(), solend_config)?;
                    let solend_withdraw_amount_f: f64 = solend_withdraw_amount as f64;
                    _solend_withdraw_amount = (solend_rate_f * solend_withdraw_amount_f / denominator_f) as u64;

                    let apricot_withdraw_amount = key_lending_amount - solend_withdraw_amount;
                    if apricot_withdraw_amount > ctx.accounts.apricot_account.get_key_amount(dest_mint.key(), apricot_config)? {
                        return Err(ErrorCode::InsufficientAmount.into());
                    }

                    let apricot_withdraw_amount_f: f64 = _apricot_withdraw_amount as f64;
                    _apricot_withdraw_amount = (apricot_rate_f * apricot_withdraw_amount_f / denominator_f) as u64;
                }
            } else {
                if key_lending_amount <= ctx.accounts.apricot_account.get_key_amount(dest_mint.key(), apricot_config)? {
                    _apricot_withdraw_amount = (apricot_rate_f * key_lending_amount_f / denominator_f) as u64;
                } else {
                    // The max value to be able to withdraw from apricot
                    let apricot_withdraw_amount = ctx.accounts.apricot_account.get_key_amount(dest_mint.key(), apricot_config)?;
                    let apricot_withdraw_amount_f: f64 = apricot_withdraw_amount as f64;
                    _apricot_withdraw_amount = (apricot_rate_f * apricot_withdraw_amount_f / denominator_f) as u64;

                    let solend_withdraw_amount = key_lending_amount - apricot_withdraw_amount;
                    if solend_withdraw_amount > ctx.accounts.solend_account.get_key_amount(dest_mint.key(), solend_config)? {
                        return Err(ErrorCode::InsufficientAmount.into());
                    }

                    let solend_withdraw_amount_f: f64 = _solend_withdraw_amount as f64;
                    _solend_withdraw_amount = (solend_rate_f * solend_withdraw_amount_f / denominator_f) as u64;
                }
            }

            let require_lending_amount: u64 = _solend_withdraw_amount + _apricot_withdraw_amount;
            let owned_amount: u64 = key_amount + require_lending_amount;
    
            //== update User account
            user_account.update_lending_amount(0, dest_mint.key(), config)?;
            user_account.update_deposited_amount(owned_amount, dest_mint.key(), config)?;  
    
            
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
    
            if _solend_withdraw_amount > 0 {
                msg!("Withdraw from solend {}", _solend_withdraw_amount);
                let cpi_program = ctx.accounts.solend_program.to_account_info();
                let cpi_accounts = solend::cpi::accounts::WithdrawToken {
                    authority: ctx.accounts.cbs_pda.to_account_info(),
                    user_token: ctx.accounts.cbs_pool.to_account_info(),
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
    
                solend::cpi::withdraw_token(cpi_ctx, _solend_withdraw_amount)?;
            } 

            if _apricot_withdraw_amount > 0 {
                msg!("Withdraw from apricot {}", _apricot_withdraw_amount);
                let cpi_program = ctx.accounts.apricot_program.to_account_info();
                let cpi_accounts = apricot::cpi::accounts::WithdrawToken {
                    authority: ctx.accounts.cbs_pda.to_account_info(),
                    user_token: ctx.accounts.cbs_pool.to_account_info(),
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
    
                apricot::cpi::withdraw_token(cpi_ctx, _apricot_withdraw_amount)?;
            }
    
            Ok(())
        }
        
    }

    // Typeless payback
    pub fn repay_token(
        ctx: Context<RepayToken>,
        amount: u64
    ) -> Result<()> {
        if ctx.accounts.user_ata_src.amount < amount {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        let user_account =&mut ctx.accounts.user_account;
        let config = &mut ctx.accounts.config;

        if user_account.step_num > 0 && user_account.step_num < 6 {
            return Err(ErrorCode::ProgressInLiquidate.into());
        }

        if (user_account.borrowed_lpsol < amount && ctx.accounts.token_src.key() == config.lpsol_mint) ||
            (user_account.borrowed_lpusd < amount && ctx.accounts.token_src.key() == config.lpusd_mint) ||
            amount == 0
        {
            return Err(ErrorCode::InvalidAmount.into());
        }

        // Validate Token
        if ctx.accounts.user_ata_src.mint != config.lpusd_mint &&
            ctx.accounts.user_ata_src.mint != config.lpsol_mint
        {
            return Err(ErrorCode::InvalidToken.into());
        }

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.token_src.to_account_info(),
                from: ctx.accounts.user_ata_src.to_account_info(),
                authority: ctx.accounts.user_authority.to_account_info()
            }
        );

        token::burn(cpi_ctx, amount)?;

        if ctx.accounts.user_ata_src.mint == config.lpusd_mint{  
            if user_account.borrowed_lpusd < amount || config.total_borrowed_lpusd < amount {
                return Err(ErrorCode::RepayFinished.into());
            }

            user_account.borrowed_lpusd = user_account.borrowed_lpusd - amount;
            config.total_borrowed_lpusd = config.total_borrowed_lpusd - amount;  
        } else if ctx.accounts.user_ata_src.mint == config.lpsol_mint {
            if user_account.borrowed_lpsol < amount || config.total_borrowed_lpsol < amount {
                return Err(ErrorCode::RepayFinished.into());
            }
            user_account.borrowed_lpsol = user_account.borrowed_lpsol - amount;
            config.total_borrowed_lpsol = config.total_borrowed_lpsol - amount;            
        }

        Ok(())
    }

    // Typeless payback
    pub fn repay_wsol(
        ctx: Context<RepayTokenWithWSOL>,
        amount: u64
    ) -> Result<()> {
        if ctx.accounts.user_ata_src.amount < amount {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        let user_account =&mut ctx.accounts.user_account;
        let config = &mut ctx.accounts.config;

        if user_account.step_num > 0 && user_account.step_num < 6 {
            return Err(ErrorCode::ProgressInLiquidate.into());
        }

        if user_account.borrowed_lpsol < amount || config.total_borrowed_lpsol < amount {
            return Err(ErrorCode::RepayFinished.into());
        }
        
        {
            // Swap wsol to LpSOL and burn LpSOL            
            let cpi_accounts = Transfer {
                from: ctx.accounts.user_ata_src.to_account_info(),
                to: ctx.accounts.cbs_ata_src.to_account_info(),
                authority: ctx.accounts.user_authority.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, amount)?;
        }

        let (program_authority, program_authority_bump) = 
            Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
        
        if program_authority != ctx.accounts.cbs_pda.to_account_info().key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        msg!("Repay with wSOL token");
        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_program = ctx.accounts.stableswap_program.to_account_info();
        let cpi_accounts_swap = stable_swap::cpi::accounts::StableswapTokens{
            stable_swap_pool: ctx.accounts.stable_swap_pool.to_account_info(),
            user: ctx.accounts.cbs_pda.to_account_info(),
            token_src: ctx.accounts.token_src.to_account_info(),
            token_dest: ctx.accounts.token_dest.to_account_info(),
            user_ata_src: ctx.accounts.cbs_ata_src.to_account_info(),
            user_ata_dest: ctx.accounts.cbs_ata_dest.to_account_info(),
            pool_ata_src: ctx.accounts.swap_ata_src.to_account_info(),
            pool_ata_dest: ctx.accounts.swap_ata_dest.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info()
        };
        let cpi_swap = CpiContext::new_with_signer(cpi_program, cpi_accounts_swap, signer);
        let tx = stable_swap::cpi::stableswap_tokens(cpi_swap, amount)?;

        {
            let return_amount: u64 = tx.get();
            msg!("Return value: {}", return_amount);

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.token_dest.to_account_info(),
                    from: ctx.accounts.cbs_ata_dest.to_account_info(),
                    authority: ctx.accounts.cbs_pda.to_account_info()
                },
                signer
            );
            token::burn(cpi_ctx, return_amount)?;            
        }


        user_account.borrowed_lpsol = user_account.borrowed_lpsol - amount;
        config.total_borrowed_lpsol = config.total_borrowed_lpsol - amount;             

        Ok(())
    }

    // Burn LpUSD
    // STEP: 1
    pub fn liquidate_step1 (ctx: Context<UpdateUserAccount>, burn_amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.borrowed_lpusd = 0;
        user_account.update_current_step(1)?;
        user_account.escrow_lpusd_amount -= burn_amount as i64;
        Ok(())
    }
    // Swap LpUSD -> USDC, Burn USDC, Mint wSOL
    // STEP: 2
    pub fn liquidate_step2 (ctx: Context<UpdateUserAccount>, burn_amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.escrow_lpusd_amount -= burn_amount as i64;
        user_account.update_current_step(2)?;
        Ok(())
    }

    // Burn LpSOL
    // STEP: 3
    pub fn liquidate_step3 (ctx: Context<UpdateUserAccount>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.borrowed_lpsol = 0;
        user_account.update_current_step(3)?;
        Ok(())
    }

    // Liquidate normal tokens
    // STEP: 4
    pub fn liquidate_swap_normaltoken(
        ctx: Context<LiquidateNormalSwap>,
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let cbs_pda = &mut ctx.accounts.cbs_pda;
        // LpUSD - USDC pool
        let stable_swap_pool = &mut ctx.accounts.stable_swap_pool;
        let token_state_account = &mut ctx.accounts.token_state_account;
        let token_src = &mut ctx.accounts.token_src;
        let token_lpusd = &mut ctx.accounts.token_lpusd;
        let token_usdc = &mut ctx.accounts.token_usdc;
        let pyth_src = &mut ctx.accounts.pyth_src;
        let pyth_usdc = &mut ctx.accounts.pyth_usdc;
        let cbs_ata_src = &ctx.accounts.cbs_ata_src;
        let cbs_ata_usdc = &ctx.accounts.cbs_ata_usdc;
        let cbs_ata_lpusd = &ctx.accounts.cbs_ata_lpusd;
        let auction_ata_lpusd = &ctx.accounts.auction_ata_lpusd;
        let stableswap_pool_ata_lpusd = &ctx.accounts.stableswap_pool_ata_lpusd;
        let stableswap_pool_ata_usdc = &ctx.accounts.stableswap_pool_ata_usdc;
        let stableswap_program = &ctx.accounts.stableswap_program;
        let testtokens_program = &ctx.accounts.testtokens_program;
        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let config = &mut ctx.accounts.config;
        let associated_token_program = &ctx.accounts.associated_token_program;
        let rent = &ctx.accounts.rent;

        if user_account.step_num != 3 && user_account.step_num != 4{
            return Err(ErrorCode::InvalidLiquidateNum.into());
        }

        let amount_src: u64 = user_account.get_key_amount(token_src.key(), config)?;

        let (program_authority, program_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if program_authority != cbs_pda.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let src_price = get_price(pyth_src)?;
        let usdc_price = get_price(pyth_usdc)?;
        let usdc_amount = (src_price as f64 * amount_src as f64 / usdc_price as f64) as u64;

        {
            msg!("Burn src token {}", amount_src);
    
            let cpi_accounts_usdc = Burn {
                mint: token_src.to_account_info(),
                from: cbs_ata_src.to_account_info(),
                authority: cbs_pda.to_account_info()
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
            token::burn(cpi_ctx_usdc, amount_src)?;

        }

        {
            msg!("Mint usdc {}", usdc_amount);
    
            let cpi_accounts_usdc = test_tokens::cpi::accounts::MintToken {
                owner: cbs_pda.to_account_info(),
                state_account: token_state_account.to_account_info(),
                user_token: cbs_ata_usdc.to_account_info(),
                token_mint: token_usdc.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                rent: rent.to_account_info()
            };
            let cpi_program = testtokens_program.to_account_info();
            let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
            test_tokens::cpi::mint_token(cpi_ctx_usdc, usdc_amount)?;

        }

        let cpi_program = stableswap_program.to_account_info();
        let cpi_accounts = stable_swap::cpi::accounts::StableswapTokens {
            user: cbs_pda.to_account_info(),
            stable_swap_pool: stable_swap_pool.to_account_info(),
            token_src: token_usdc.to_account_info(),
            token_dest: token_lpusd.to_account_info(),
            user_ata_src: cbs_ata_usdc.to_account_info(),
            user_ata_dest: cbs_ata_lpusd.to_account_info(),
            pool_ata_src: stableswap_pool_ata_usdc.to_account_info(),
            pool_ata_dest: stableswap_pool_ata_lpusd.to_account_info(),                
            system_program: system_program.to_account_info(),
            token_program: token_program.to_account_info(),
            associated_token_program: associated_token_program.to_account_info(),
            rent: rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        let tx = stable_swap::cpi::stableswap_tokens(cpi_ctx, usdc_amount)?;
        let lpusd_amount = tx.get();

        if lpusd_amount > 0 {
            msg!("Liquidate LpUSD {}", lpusd_amount);
            let cpi_accounts = Transfer {
                from: cbs_ata_lpusd.to_account_info(),
                to: auction_ata_lpusd.to_account_info(),
                authority: cbs_pda.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, lpusd_amount)?;

            user_account.escrow_lpusd_amount += lpusd_amount as i64;
        }

        user_account.update_current_step(4)?;
        Ok(())
    }

    // Liquidate LpSOL -> LpUSD tokens
    // STEP: 5
    pub fn liquidate_swap_lpsoltoken1(
        ctx: Context<LiquidateLpSOLTokenSwap1>
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;

        let config = &mut ctx.accounts.config;
        let cbs_pda = &mut ctx.accounts.cbs_pda;
        let stable_swap_pool = &mut ctx.accounts.stable_swap_pool;
        let token_state_account = &mut ctx.accounts.token_state_account;
        let token_lpsol = &mut ctx.accounts.token_lpsol;
        let token_wsol = &mut ctx.accounts.token_wsol;
        let token_usdc = &mut ctx.accounts.token_usdc;
        let pyth_usdc = &mut ctx.accounts.pyth_usdc;
        let pyth_wsol = &mut ctx.accounts.pyth_wsol;
        let cbs_ata_lpsol = &ctx.accounts.cbs_ata_lpsol;
        let cbs_ata_wsol = &ctx.accounts.cbs_ata_wsol;
        let cbs_ata_usdc = &ctx.accounts.cbs_ata_usdc;
        let stableswap_pool_ata_lpsol = &ctx.accounts.stableswap_pool_ata_lpsol;
        let stableswap_pool_ata_wsol = &ctx.accounts.stableswap_pool_ata_wsol;
        let stableswap_program = &ctx.accounts.stableswap_program;
        let testtokens_program = &ctx.accounts.testtokens_program;
        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let associated_token_program = &ctx.accounts.associated_token_program;
        let rent = &ctx.accounts.rent;

        if user_account.step_num != 4 {
            return Err(ErrorCode::InvalidLiquidateNum.into());
        }

        if user_account.ray_amount != 0 ||
            user_account.wsol_amount != 0 ||
            user_account.srm_amount != 0 ||
            user_account.stsol_amount != 0 ||
            user_account.scnsol_amount != 0 ||
            user_account.msol_amount != 0 {
                return Err(ErrorCode::LiquidateNormalTokens.into());
            }

        let (program_authority, program_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if program_authority != cbs_pda.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let amount_src: u64 = user_account.get_key_amount(token_lpsol.key(), config)?;
        let mut _wsol: u64 = 0;
        {
            msg!("LpSOL -> SOL {}", amount_src);
            let cpi_program = stableswap_program.to_account_info();
            let cpi_accounts = stable_swap::cpi::accounts::StableswapTokens {
                user: ctx.accounts.cbs_pda.to_account_info(),
                stable_swap_pool: stable_swap_pool.to_account_info(),
                token_src: token_lpsol.to_account_info(),
                token_dest: token_wsol.to_account_info(),
                user_ata_src: cbs_ata_lpsol.to_account_info(),
                user_ata_dest: cbs_ata_wsol.to_account_info(),
                pool_ata_src: stableswap_pool_ata_lpsol.to_account_info(),
                pool_ata_dest: stableswap_pool_ata_wsol.to_account_info(),                
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                rent: rent.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);    
            let tx = stable_swap::cpi::stableswap_tokens(cpi_ctx, amount_src)?;
            _wsol = tx.get();
        }

        let mut _usdc_swap_amount = 0;
        {
            // Pyth swap
            let usdc_price = get_price(pyth_usdc)?;
            let wsol_price = get_price(pyth_wsol)?;
            if usdc_price <= 0 || wsol_price <= 0 {
                return Err(ErrorCode::InvalidPythPrice.into());
            }

            // Burn wSOL
            msg!("Burn wSOL {}", _wsol);

            let cpi_accounts_wsol = Burn {
                mint: token_wsol.to_account_info(),
                from: cbs_ata_wsol.to_account_info(),
                authority: ctx.accounts.cbs_pda.to_account_info()
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx_wsol = CpiContext::new_with_signer(cpi_program, cpi_accounts_wsol, signer);
            token::burn(cpi_ctx_wsol, _wsol)?;

            _usdc_swap_amount = (wsol_price as f64 * _wsol as f64 /  usdc_price as f64) as u64;

            msg!("Mint USDC {}", _usdc_swap_amount);
            let cpi_accounts_usdc = test_tokens::cpi::accounts::MintToken {
                owner: ctx.accounts.cbs_pda.to_account_info(),
                state_account: token_state_account.to_account_info(),
                user_token: cbs_ata_usdc.to_account_info(),
                token_mint: token_usdc.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                rent: rent.to_account_info()
            };
            let cpi_program = testtokens_program.to_account_info();
            let cpi_ctx_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_usdc, signer);
            test_tokens::cpi::mint_token(cpi_ctx_usdc, _usdc_swap_amount)?;
        }

        user_account.escrow_usdc_amount += _usdc_swap_amount;        
        user_account.update_current_step(5)?;

        Ok(())
    }

    // STEP: 6
    pub fn liquidate_swap_lpsoltoken2(
        ctx: Context<LiquidateLpSOLTokenSwap2>
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let cbs_pda = &mut ctx.accounts.cbs_pda;
        let stable_swap_pool = &mut ctx.accounts.stable_swap_pool;
        let token_lpusd = &mut ctx.accounts.token_lpusd;
        let token_usdc = &mut ctx.accounts.token_usdc;
        let cbs_ata_lpusd = &ctx.accounts.cbs_ata_lpusd;
        let cbs_ata_usdc = &ctx.accounts.cbs_ata_usdc;
        let stableswap_pool_ata_lpusd = &ctx.accounts.stableswap_pool_ata_lpusd;
        let stableswap_pool_ata_usdc = &ctx.accounts.stableswap_pool_ata_usdc;
        let stableswap_program = &ctx.accounts.stableswap_program;
        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let associated_token_program = &ctx.accounts.associated_token_program;
        let rent = &ctx.accounts.rent;

        if user_account.step_num != 5 {
            return Err(ErrorCode::InvalidLiquidateNum.into());
        }

        let (program_authority, program_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if program_authority != cbs_pda.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let usdc_swap_amount = user_account.escrow_usdc_amount;
        msg!("USDC -> LpUSD {}", usdc_swap_amount);

        let cpi_program = stableswap_program.to_account_info();
        let cpi_accounts = stable_swap::cpi::accounts::StableswapTokens {
            user: ctx.accounts.cbs_pda.to_account_info(),
            stable_swap_pool: stable_swap_pool.to_account_info(),
            token_src: token_usdc.to_account_info(),
            token_dest: token_lpusd.to_account_info(),
            user_ata_src: cbs_ata_usdc.to_account_info(),
            user_ata_dest: cbs_ata_lpusd.to_account_info(),
            pool_ata_src: stableswap_pool_ata_usdc.to_account_info(),
            pool_ata_dest: stableswap_pool_ata_lpusd.to_account_info(),                
            system_program: system_program.to_account_info(),
            token_program: token_program.to_account_info(),
            associated_token_program: associated_token_program.to_account_info(),
            rent: rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);    
        let tx = stable_swap::cpi::stableswap_tokens(cpi_ctx, usdc_swap_amount)?;
        let lpusd_amount = tx.get();

        if lpusd_amount > 0 {
            msg!("Transfer LpUSD to auction {}", lpusd_amount);
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_ata_lpusd.to_account_info(),
                to: ctx.accounts.auction_lpusd.to_account_info(),
                authority: ctx.accounts.cbs_pda.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, lpusd_amount)?;

            user_account.escrow_lpusd_amount += lpusd_amount as i64;
        }    
        
        user_account.escrow_usdc_amount = 0;
        user_account.update_current_step(6)?;
        Ok(())
    }

    // Liquidate LpFI and LpUSD tokens
    // STEP: 7
    pub fn liquidate_swap_lpfitoken(
        ctx: Context<LiquidateLpFITokenSwap>,
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let cbs_pda = &mut ctx.accounts.cbs_pda;
        let stable_swap_pool = &mut ctx.accounts.stable_swap_pool;

        let uniswap_pool = &mut ctx.accounts.uniswap_pool;
        let token_lpfi = &mut ctx.accounts.token_lpfi;
        let token_lpusd = &mut ctx.accounts.token_lpusd;
        let token_usdc = &mut ctx.accounts.token_usdc;
        let cbs_ata_lpfi = &ctx.accounts.cbs_ata_lpfi;
        let cbs_ata_lpusd = &ctx.accounts.cbs_ata_lpusd;
        let stableswap_pool_ata_lpusd = &ctx.accounts.stableswap_pool_ata_lpusd;
        let stableswap_pool_ata_usdc = &ctx.accounts.stableswap_pool_ata_usdc;
        let uniswap_pool_ata_lpfi = &ctx.accounts.uniswap_pool_ata_lpfi;
        let uniswap_pool_ata_usdc = &ctx.accounts.uniswap_pool_ata_usdc;
        let escrow_ata_usdc = &ctx.accounts.escrow_ata_usdc;
        let stableswap_program = &ctx.accounts.stableswap_program;
        let uniswap_program = &ctx.accounts.uniswap_program;

        let system_program = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let associated_token_program = &ctx.accounts.associated_token_program;
        let rent = &ctx.accounts.rent;

        if user_account.step_num != 6 {
            return Err(ErrorCode::InvalidLiquidateNum.into());
        }
        if user_account.lpsol_amount != 0 {
            return Err(ErrorCode::LiquidateLpSOLTokens.into());
        }

        let amount_lpfi: u64 = user_account.lpfi_amount;
        let (program_authority, program_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if program_authority != cbs_pda.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let seeds = &[
            PREFIX.as_bytes(),
            &[program_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let mut _usdc_swap_amount: u64 = 0;
        {
            msg!("LpFI -> USDC {}", amount_lpfi);
            //---------- Cross-Calling Uniswap Program ----------------
            let cpi_accounts_uniswap_lpfi_to_usdc = UniswapTokens {
                uniswap_pool: uniswap_pool.to_account_info(),
                user: cbs_pda.to_account_info(),
                token_src: token_lpfi.to_account_info(),
                token_dest: token_usdc.to_account_info(),
                user_ata_src: cbs_ata_lpfi.to_account_info(),
                user_ata_dest: escrow_ata_usdc.to_account_info(),
                pool_ata_src: uniswap_pool_ata_lpfi.to_account_info(),
                pool_ata_dest: uniswap_pool_ata_usdc.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                rent: rent.to_account_info()
            };
            let cpi_program = uniswap_program.to_account_info();
            let cpi_swap_lpfi_to_usdc = CpiContext::new_with_signer(cpi_program, cpi_accounts_uniswap_lpfi_to_usdc, signer);
            let tx = uniswap::cpi::uniswap_tokens(cpi_swap_lpfi_to_usdc, amount_lpfi)?;
            _usdc_swap_amount = tx.get();
        }

        {
            msg!("USDC -> LpUSD {}", _usdc_swap_amount);

            let cpi_program = stableswap_program.to_account_info();
            let cpi_accounts = stable_swap::cpi::accounts::StableswapTokens {
                user: ctx.accounts.cbs_pda.to_account_info(),
                stable_swap_pool: stable_swap_pool.to_account_info(),
                token_src: token_usdc.to_account_info(),
                token_dest: token_lpusd.to_account_info(),
                user_ata_src: escrow_ata_usdc.to_account_info(),
                user_ata_dest: cbs_ata_lpusd.to_account_info(),
                pool_ata_src: stableswap_pool_ata_usdc.to_account_info(),
                pool_ata_dest: stableswap_pool_ata_lpusd.to_account_info(),                
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
                associated_token_program: associated_token_program.to_account_info(),
                rent: rent.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);    
            let tx = stable_swap::cpi::stableswap_tokens(cpi_ctx, _usdc_swap_amount)?;
            let lpusd_amount = tx.get();


            if lpusd_amount > 0 {
                msg!("Transfer LpUSD to auction {}", lpusd_amount);
                let cpi_accounts = Transfer {
                    from: ctx.accounts.cbs_ata_lpusd.to_account_info(),
                    to: ctx.accounts.auction_lpusd.to_account_info(),
                    authority: ctx.accounts.cbs_pda.to_account_info()
                };
        
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
                token::transfer(cpi_ctx, lpusd_amount)?;

                user_account.escrow_lpusd_amount += lpusd_amount as i64;
            }    
        }        

        user_account.update_current_step(7)?;
        Ok(())
    }

    // STEP 8
    pub fn finalize_liquidate(
        ctx: Context<UpdateUserAccount>
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        if user_account.step_num != 6 {
            return Err(ErrorCode::InvalidLiquidateNum.into());
        }

        user_account.escrow_lpusd_amount = 0;
        user_account.update_current_step(0)?;
        Ok(())
    }

    pub fn apply_dsf(
        ctx: Context<UpdateUserAccount>,
        lpusd_rate: u64,
        lpsol_rate: u64
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        if lpusd_rate > 0 {
            user_account.borrowed_lpusd = user_account.borrowed_lpusd * (10000 + lpusd_rate) / 10000;
        }
        if lpsol_rate > 0 {
            user_account.borrowed_lpsol = user_account.borrowed_lpsol * (10000 + lpsol_rate) / 10000;
        }
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient Amount From User Account")]
    InsufficientUserAmount,
    #[msg("CBS: Insufficient Amount")]
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
    #[msg("Invalid pyth account")]
    InvalidPythAccount,
    #[msg("Invalid step num for liquidate")]
    InvalidLiquidateNum,
    #[msg("Liquidate normal token before lpsol")]
    LiquidateNormalTokens,
    #[msg("Liquidate LpSOL token before LpFI")]
    LiquidateLpSOLTokens,
}
