use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction };
use pyth_client;
use anchor_spl::token::{self, Transfer };

mod states;
pub use states::*;

use lpfinance_accounts::cpi::accounts::AddFromCbsProgram;
use lpfinance_accounts::program::LpfinanceAccounts;
use lpfinance_accounts::{self, WhiteList};

use lpfinance_tokens::cpi::accounts::MintLpToken;
use lpfinance_tokens::program::LpfinanceTokens;
use lpfinance_tokens::{self, TokenStateAccount};

use solend::program::Solend;
use solend::{self};

use apricot::program::Apricot;
use apricot::{self};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const LTV:u128 = 85;
const DOMINATOR:u128 = 100;
const PREFIX: &str = "cbsprotocol3";

const LENDING_PERCENT: u64 = 10;

const LENDING_DENOMINATOR: u128 = 10000000;
const W_THRESHHOLD: u64 = 90;
const S_THRESHHOLD: u64 = 75;

pub fn get_price(pyth_account: AccountInfo) -> Result<u128> {
    let pyth_price_info = &pyth_account;
    let pyth_price_data = &pyth_price_info.try_borrow_data()?;
    let pyth_price = pyth_client::cast::<pyth_client::Price>(pyth_price_data);
    let price = pyth_price.agg.price as u128;
    Ok(price)
}

#[program]
pub mod cbs_protocol {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>
    ) -> Result<()> {
        msg!("INITIALIZE PROTOCAL");

        let state_account = &mut ctx.accounts.state_account;
        let config = &mut ctx.accounts.config;

        state_account.owner = ctx.accounts.authority.key();
        state_account.liquidation_run = false;

        config.state_account = ctx.accounts.state_account.key();

        // token mint
        config.ray_mint = ctx.accounts.ray_mint.key();
        config.ray_mint = ctx.accounts.ray_mint.key();
        config.msol_mint = ctx.accounts.msol_mint.key();
        config.ust_mint = ctx.accounts.ust_mint.key();
        config.srm_mint = ctx.accounts.srm_mint.key();
        config.scnsol_mint = ctx.accounts.scnsol_mint.key();
        config.stsol_mint = ctx.accounts.stsol_mint.key();
        
        config.lpusd_mint = ctx.accounts.lpusd_mint.key();
        config.lpsol_mint = ctx.accounts.lpsol_mint.key();
        config.lpray_mint = ctx.accounts.lpray_mint.key();
        config.lpeth_mint = ctx.accounts.lpeth_mint.key();

        // lptoken pool
        config.pool_lpsol = ctx.accounts.pool_lpsol.key();
        config.pool_lpusd = ctx.accounts.pool_lpusd.key();

        // borrowed amount
        config.total_borrowed_lpsol = 0;
        config.total_borrowed_lpusd = 0;
        config.total_borrowed_lpeth = 0;
        config.total_borrowed_lpbtc = 0;

        // deposited amount
        config.total_deposited_sol = 0;
        config.total_deposited_usdc = 0;
        config.total_deposited_btc = 0;
        config.total_deposited_eth = 0;
        config.total_deposited_msol = 0;
        config.total_deposited_ust = 0;
        config.total_deposited_srm = 0;
        config.total_deposited_scnsol = 0;
        config.total_deposited_stsol = 0;
        config.total_deposited_usdt = 0;

        config.total_deposited_lpsol = 0;
        config.total_deposited_lpusd = 0;
        config.total_deposited_lpeth = 0;
        config.total_deposited_lpbtc = 0;        

        Ok(())
    }

    pub fn initialize_pool(
        ctx: Context<InitializePool>
    ) -> Result<()> {
        msg!("INITIALIZE POOL");

        let state_account = &mut ctx.accounts.state_account;
        let config = &mut ctx.accounts.config;

        if state_account.owner != ctx.accounts.authority.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        config.pool_btc = ctx.accounts.pool_btc.key();
        config.pool_usdc = ctx.accounts.pool_usdc.key();
        config.pool_msol = ctx.accounts.pool_msol.key();
        config.pool_eth = ctx.accounts.pool_eth.key();
        config.pool_ust = ctx.accounts.pool_ust.key();
        config.pool_srm = ctx.accounts.pool_srm.key();
        config.pool_scnsol = ctx.accounts.pool_scnsol.key();
        config.pool_stsol = ctx.accounts.pool_stsol.key();
        config.pool_usdt = ctx.accounts.pool_usdt.key();    

        Ok(())
    }

    // Init user account
    pub fn init_user_account(
        ctx: Context<InitUserAccount>, 
        bump: u8
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.owner = ctx.accounts.user_authority.key();
        user_account.bump = 0;
        Ok(())
    }

    pub fn deposit_collateral(
        ctx: Context<DepositCollateral>,
        amount: u64
    )-> Result<()> {        
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        if ctx.accounts.user_collateral.amount < amount {
            return Err(ErrorCode::InsufficientUserAmount.into());
        }

        // While initial depositing, need to send 10% to lending protocol.
        let lending_amount = (amount as u128 * LENDING_PERCENT as u128 / 100) as u64;
        let pool_amount = amount - lending_amount;

        // Transfer collateral from user account to pool
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_collateral.to_account_info(),
            to: ctx.accounts.collateral_pool.to_account_info(),
            authority: ctx.accounts.user_authority.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        let user_account =&mut ctx.accounts.user_account;

        if user_account.bump > 0 && user_account.bump < 6 {
            return Err(ErrorCode::InLiquidating.into());
        }
        
        let config = &mut ctx.accounts.config;

        // == GET signer started == //
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
        // == GET signer ended == //

        if ctx.accounts.user_collateral.mint == config.ray_mint || 
           ctx.accounts.user_collateral.mint == config.msol_mint || 
           ctx.accounts.user_collateral.mint == config.ray_mint || 
           ctx.accounts.user_collateral.mint == config.eth_mint || 
           ctx.accounts.user_collateral.mint == config.ust_mint || 
           ctx.accounts.user_collateral.mint == config.srm_mint || 
           ctx.accounts.user_collateral.mint == config.scnsol_mint || 
           ctx.accounts.user_collateral.mint == config.stsol_mint || 
           ctx.accounts.user_collateral.mint == config.usdt_mint {

            // If solend APY rate is higher than apricot APY rate, return true;
            let mut solend_higher = false;

            if ctx.accounts.user_collateral.mint == config.ray_mint {
                if ctx.accounts.solend_config.ray_rate > ctx.accounts.apricot_config.ray_rate {
                    solend_higher = true;
                    user_account.lending_btc_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.solend_config.ray_rate as u128) as u64;
                } else {
                    user_account.lending_btc_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.apricot_config.ray_rate as u128) as u64;
                }

            } else if ctx.accounts.user_collateral.mint == config.ray_mint {
                if ctx.accounts.solend_config.wsol_rate > ctx.accounts.apricot_config.wsol_rate {
                    solend_higher = true;

                    user_account.lending_usdc_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.solend_config.wsol_rate as u128) as u64;
                } else {

                    user_account.lending_usdc_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.apricot_config.wsol_rate as u128) as u64;
                }

            } else if ctx.accounts.user_collateral.mint == config.msol_mint {
                if ctx.accounts.solend_config.msol_rate > ctx.accounts.apricot_config.msol_rate {
                    solend_higher = true;
                    user_account.lending_msol_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.solend_config.msol_rate as u128) as u64;
                } else {
                    user_account.lending_msol_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.apricot_config.msol_rate as u128) as u64;
                }
            } else if ctx.accounts.user_collateral.mint == config.srm_mint {
                if ctx.accounts.solend_config.srm_rate > ctx.accounts.apricot_config.srm_rate {
                    solend_higher = true;
                    user_account.lending_srm_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.solend_config.srm_rate as u128) as u64;
                } else {
                    user_account.lending_srm_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.apricot_config.srm_rate as u128) as u64;

                }
            } else if ctx.accounts.user_collateral.mint == config.scnsol_mint {
                if ctx.accounts.solend_config.scnsol_rate > ctx.accounts.apricot_config.scnsol_rate {
                    solend_higher = true;
                    user_account.lending_scnsol_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.solend_config.scnsol_rate as u128) as u64;
                } else {
                    user_account.lending_scnsol_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.apricot_config.scnsol_rate as u128) as u64;

                }
            } else if ctx.accounts.user_collateral.mint == config.stsol_mint {
                if ctx.accounts.solend_config.stsol_rate > ctx.accounts.apricot_config.stsol_rate {
                    solend_higher = true;
                    user_account.lending_stsol_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.solend_config.stsol_rate as u128) as u64;
                } else {
                    user_account.lending_stsol_amount += (lending_amount as u128 * LENDING_DENOMINATOR / ctx.accounts.solend_config.stsol_rate as u128) as u64;

                }
            }

            if solend_higher {
                let cpi_program = ctx.accounts.solend_program.to_account_info();
                let cpi_accounts = solend::cpi::accounts::DepositToken {
                    authority: ctx.accounts.state_account.to_account_info(),
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
                let cpi_program = ctx.accounts.apricot_program.to_account_info();
                let cpi_accounts = apricot::cpi::accounts::DepositToken {
                    authority: ctx.accounts.state_account.to_account_info(),
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

        if ctx.accounts.user_collateral.mint == config.ray_mint {
            user_account.btc_amount = user_account.btc_amount + pool_amount;
            config.total_deposited_btc = config.total_deposited_btc + amount;
        }

        if ctx.accounts.user_collateral.mint == config.ray_mint {
            user_account.usdc_amount = user_account.usdc_amount + pool_amount;
            config.total_deposited_usdc = config.total_deposited_usdc + amount;
        }

        if ctx.accounts.user_collateral.mint == config.msol_mint {
            user_account.msol_amount = user_account.msol_amount + pool_amount;
            config.total_deposited_msol = config.total_deposited_msol + amount;
        }
        
        if ctx.accounts.user_collateral.mint == config.eth_mint {
            user_account.eth_amount = user_account.eth_amount + pool_amount;
            config.total_deposited_eth = config.total_deposited_eth + amount;
        }

        if ctx.accounts.user_collateral.mint == config.ust_mint {
            user_account.ust_amount = user_account.ust_amount + pool_amount;
            config.total_deposited_ust = config.total_deposited_ust + amount;
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

        if ctx.accounts.user_collateral.mint == config.usdt_mint {
            user_account.usdt_amount = user_account.usdt_amount + pool_amount;
            config.total_deposited_usdt = config.total_deposited_usdt + amount;
        }

        if ctx.accounts.user_collateral.mint == config.lpusd_mint {
            user_account.lpusd_amount = user_account.lpusd_amount + amount;
            config.total_deposited_lpusd = config.total_deposited_lpusd + amount;
        }

        if ctx.accounts.user_collateral.mint == config.lpsol_mint {
            user_account.lpsol_amount = user_account.lpsol_amount + amount;
            config.total_deposited_lpsol = config.total_deposited_lpsol + amount;
        }

        if ctx.accounts.user_collateral.mint == config.lpeth_mint {
            user_account.lpeth_amount = user_account.lpeth_amount + amount;
            config.total_deposited_lpeth = config.total_deposited_lpeth + amount;
        }

        if ctx.accounts.user_collateral.mint == config.lpray_mint {
            user_account.lpbtc_amount = user_account.lpbtc_amount + amount;
            config.total_deposited_lpbtc = config.total_deposited_lpbtc + amount;
        }

        // let whitelist = ctx.accounts.whitelist.load_mut()?;
        if ctx.accounts.whitelist.load_mut()?.addresses.contains(&ctx.accounts.user_authority.key()) {
            msg!("Already Exist");
        } else {

            let cpi_program = ctx.accounts.accounts_program.to_account_info();
            let cpi_accounts = AddFromCbsProgram {
                config: ctx.accounts.accounts_config.to_account_info(),
                whitelist: ctx.accounts.whitelist.to_account_info(),
                cbsprogram: ctx.accounts.state_account.to_account_info()
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

            let new_wallet = ctx.accounts.user_authority.key();
            lpfinance_accounts::cpi::add_from_cbs_program(cpi_ctx, new_wallet)?;
        }
        
        Ok(())
    }

    pub fn deposit_sol(
        ctx: Context<DepositSOL>,
        amount: u64
    ) -> Result<()> {
        msg!("Deposit SOL");

        if **ctx.accounts.user_authority.lamports.borrow() < amount {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        invoke(
            &system_instruction::transfer(
                ctx.accounts.user_authority.key,
                ctx.accounts.state_account.to_account_info().key,
                amount
            ),
            &[
                ctx.accounts.user_authority.to_account_info().clone(),
                ctx.accounts.state_account.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone()
            ]
        )?;

        let user_account = &mut ctx.accounts.user_account;

        if user_account.bump > 0 && user_account.bump < 6 {
            return Err(ErrorCode::InLiquidating.into());
        }

        let config = &mut ctx.accounts.config;

        user_account.sol_amount = user_account.sol_amount + amount;
        config.total_deposited_sol = config.total_deposited_sol + amount;

        // let whitelist = ctx.accounts.whitelist.load_mut()?;
        if ctx.accounts.whitelist.load_mut()?.addresses.contains(&ctx.accounts.user_authority.key()) {
            msg!("Already Exist");
        } else {

            let cpi_program = ctx.accounts.accounts_program.to_account_info();
            let cpi_accounts = AddFromCbsProgram {
                config: ctx.accounts.whitelist_config.to_account_info(),
                whitelist: ctx.accounts.whitelist.to_account_info(),
                cbsprogram: ctx.accounts.state_account.to_account_info()
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

            let new_wallet = ctx.accounts.user_authority.key();
            lpfinance_accounts::cpi::add_from_cbs_program(cpi_ctx, new_wallet)?;
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
        // Borrowable TotalPrice. Need to be calculated with LTV
        let mut total_price: u128 = 0;
        let mut total_borrowed_price: u128 = 0;
        let user_account = &mut ctx.accounts.user_account;

        if user_account.bump > 0 && user_account.bump < 6 {
            return Err(ErrorCode::InLiquidating.into());
        }

        let config = &mut ctx.accounts.config;

        // BTC price        
        let btc_price: u128 = get_price(ctx.accounts.pyth_btc_account.to_account_info())?;    
        total_price += btc_price * (user_account.btc_amount + user_account.lending_btc_amount )as u128;

        // SOL price
        let sol_price: u128 = get_price(ctx.accounts.pyth_sol_account.to_account_info())?;    
        total_price += sol_price * (user_account.sol_amount + user_account.lending_sol_amount ) as u128;

        // USDC price
        let usdc_price: u128 = get_price(ctx.accounts.pyth_usdc_account.to_account_info())?;
        total_price += usdc_price * (user_account.usdc_amount + user_account.lending_usdc_amount ) as u128;

        // mSOL price
        let msol_price: u128 = get_price(ctx.accounts.pyth_msol_account.to_account_info())?;
        total_price += msol_price * (user_account.msol_amount + user_account.lending_msol_amount ) as u128;

        // ETH price
        let eth_price: u128 = get_price(ctx.accounts.pyth_eth_account.to_account_info())?;
        total_price += eth_price * (user_account.eth_amount + user_account.lending_eth_amount ) as u128;


        // ust price        
        let ust_price: u128 = get_price(ctx.accounts.pyth_ust_account.to_account_info())?;    
        total_price += ust_price * (user_account.ust_amount + user_account.lending_ust_amount ) as u128;

        // srm price
        let srm_price: u128 = get_price(ctx.accounts.pyth_srm_account.to_account_info())?;    
        total_price += srm_price * (user_account.srm_amount + user_account.lending_srm_amount ) as u128;

        // scnsol price
        let scnsol_price: u128 = get_price(ctx.accounts.pyth_scnsol_account.to_account_info())?;
        total_price += scnsol_price * (user_account.scnsol_amount + user_account.lending_scnsol_amount ) as u128;

        // stsol price
        let stsol_price: u128 = get_price(ctx.accounts.pyth_stsol_account.to_account_info())?;
        total_price += stsol_price * (user_account.stsol_amount + user_account.lending_stsol_amount ) as u128;

        // usdt price
        let usdt_price: u128 = get_price(ctx.accounts.pyth_usdt_account.to_account_info())?;
        total_price += usdt_price * (user_account.usdt_amount + user_account.lending_usdt_amount ) as u128;

        // LpUSD price
        let lpusd_price = usdc_price;        
        total_price += lpusd_price * user_account.lpusd_amount as u128;

        // LpSOL price
        let lpsol_price = sol_price;
        total_price += lpsol_price * user_account.lpsol_amount as u128;

        // LpBTC price
        let lpbtc_price = btc_price;        
        total_price += lpbtc_price * user_account.lpbtc_amount as u128;

        // LpETH price
        let lpeth_price: u128 = eth_price;
        total_price += lpeth_price * user_account.lpeth_amount as u128;

        // Total Borrowed AMount
        total_borrowed_price += lpusd_price * user_account.borrowed_lpusd as u128;
        total_borrowed_price += lpsol_price * user_account.borrowed_lpsol as u128;
        total_borrowed_price += lpbtc_price * user_account.borrowed_lpbtc as u128;
        total_borrowed_price += lpeth_price * user_account.borrowed_lpeth as u128;

        let mut borrow_value: u128 = amount as u128;
        
        if ctx.accounts.collateral_mint.key() == config.lpusd_mint {
            borrow_value = borrow_value * lpusd_price;

            config.total_borrowed_lpusd = config.total_borrowed_lpusd + amount;
            user_account.borrowed_lpusd = user_account.borrowed_lpusd + amount;
        } else if ctx.accounts.collateral_mint.key() == config.lpsol_mint {
            borrow_value = borrow_value * lpsol_price;

            config.total_borrowed_lpsol = config.total_borrowed_lpsol + amount;
            user_account.borrowed_lpsol = user_account.borrowed_lpsol + amount;
        } else if ctx.accounts.collateral_mint.key() == config.lpray_mint {
            borrow_value = borrow_value * lpbtc_price;

            config.total_borrowed_lpbtc = config.total_borrowed_lpbtc + amount;
            user_account.borrowed_lpbtc = user_account.borrowed_lpbtc + amount;
        } else if ctx.accounts.collateral_mint.key() == config.lpeth_mint {
            borrow_value = borrow_value * lpeth_price;

            config.total_borrowed_lpeth = config.total_borrowed_lpeth + amount;
            user_account.borrowed_lpeth = user_account.borrowed_lpeth + amount;
        } else {
            return Err(ErrorCode::InvalidToken.into());
        }

        let borrable_total = total_price * LTV / DOMINATOR - total_borrowed_price;

        if borrable_total > borrow_value {
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

            // Mint
            let cpi_program = ctx.accounts.lptokens_program.to_account_info();
            let cpi_accounts = MintLpToken {
                signer: ctx.accounts.state_account.to_account_info(),
                state_account: ctx.accounts.tokens_state.to_account_info(),
                config: ctx.accounts.lptoken_config.to_account_info(),
                lptoken_mint: ctx.accounts.collateral_mint.to_account_info(),
                user_lptoken: ctx.accounts.user_collateral.to_account_info(),
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

    pub fn liquidate_lptoken_collateral(
        ctx: Context<LiquidateLpTokenCollateral>
    ) -> Result<()> {
        msg!("liquidate_collateral started");

        let user_account = &mut ctx.accounts.user_account;


        let lpusd_amount = user_account.lpusd_amount;
        let lpsol_amount = user_account.lpsol_amount;
        let lpbtc_amount = user_account.lpbtc_amount;
        let lpeth_amount = user_account.lpeth_amount;


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

        if lpbtc_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_lpbtc.to_account_info(),
                to: ctx.accounts.auction_lpbtc.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, lpbtc_amount)?;
        }

        if lpeth_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_lpeth.to_account_info(),
                to: ctx.accounts.auction_lpeth.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, lpeth_amount)?;
        }

        user_account.bump = 3;
        // user_account.lpusd_amount = 0;
        // user_account.lpsol_amount = 0;
        // user_account.lpbtc_amount = 0;
        // user_account.lpeth_amount = 0;
        
        Ok(())
    }


    pub fn liquidate_collateral(
        ctx: Context<LiquidateCollateral>
    ) -> Result<()> {
        msg!("liquidate_collateral started");

        let user_account = &mut ctx.accounts.user_account;

        let sol_amount = user_account.sol_amount;
        let btc_amount = user_account.btc_amount;
        let msol_amount = user_account.msol_amount;
        let usdc_amount = user_account.usdc_amount;
        let eth_amount = user_account.eth_amount;

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

        if btc_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_btc.to_account_info(),
                to: ctx.accounts.auction_btc.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, btc_amount)?;
        }

        if usdc_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_usdc.to_account_info(),
                to: ctx.accounts.auction_usdc.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, usdc_amount)?;
        }

        if eth_amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_eth.to_account_info(),
                to: ctx.accounts.auction_eth.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, eth_amount)?;
        }

        msg!("sol_amount started");

        if sol_amount > 0 {
            **ctx.accounts.state_account.to_account_info().try_borrow_mut_lamports()? -= sol_amount;
            **ctx.accounts.auction_account.try_borrow_mut_lamports()? += sol_amount;
        }
        msg!("sol_amount ended");

        user_account.bump = 1;
        // user_account.sol_amount = 0;

        // user_account.usdc_amount = 0;
        // user_account.btc_amount = 0;
        // user_account.msol_amount = 0;
        // user_account.eth_amount = 0;      
        Ok(())
    }

    pub fn liquidate_second_collateral(
        ctx: Context<LiquidateSecondCollateral>
    ) -> Result<()> {
        msg!("liquidate_collateral started");

        let user_account = &mut ctx.accounts.user_account;

        let ust_amount = user_account.ust_amount;
        let srm_amount = user_account.srm_amount;
        let scnsol_amount = user_account.scnsol_amount;
        let stsol_amount = user_account.stsol_amount;
        let usdt_amount = user_account.usdt_amount;

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

        if ust_amount > 0 {
            msg!("liquidate_collateral ust_amount");
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_ust.to_account_info(),
                to: ctx.accounts.auction_ust.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, ust_amount)?;
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

        if usdt_amount > 0 {
            msg!("liquidate_collateral usdt_amount");
            let cpi_accounts = Transfer {
                from: ctx.accounts.cbs_usdt.to_account_info(),
                to: ctx.accounts.auction_usdt.to_account_info(),
                authority: ctx.accounts.state_account.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, usdt_amount)?;
        }

        user_account.bump = 2;
        // user_account.ust_amount = 0;
        // user_account.srm_amount = 0;
        // user_account.scnsol_amount = 0;
        // user_account.stsol_amount = 0;
        // user_account.usdt_amount = 0;
        
        Ok(())
    }


    pub fn withdraw_sol(
        ctx: Context<WithdrawSOL>,
        amount: u64
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;

        if user_account.bump > 0 && user_account.bump < 6 {
            return Err(ErrorCode::InLiquidating.into());
        }

        let sol_amount = user_account.sol_amount as u128;
        let btc_amount = user_account.btc_amount as u128;
        let usdc_amount = user_account.usdc_amount as u128;
        let msol_amount = user_account.msol_amount as u128;
        let eth_amount = user_account.eth_amount as u128;
        let ust_amount = user_account.ust_amount as u128;
        let srm_amount = user_account.srm_amount as u128;
        let scnsol_amount = user_account.scnsol_amount as u128;
        let stsol_amount = user_account.stsol_amount as u128;
        let usdt_amount = user_account.usdt_amount as u128;

        let lpsol_amount = user_account.lpsol_amount as u128;
        let lpusd_amount = user_account.lpusd_amount as u128;
        let lpbtc_amount = user_account.lpbtc_amount as u128;
        let lpeth_amount = user_account.lpeth_amount as u128;

        let borrowed_lpusd = user_account.borrowed_lpusd as u128;
        let borrowed_lpsol = user_account.borrowed_lpsol as u128;
        let borrowed_lpbtc = user_account.borrowed_lpbtc as u128;
        let borrowed_lpeth = user_account.borrowed_lpeth as u128;

        let mut total_price: u128 = 0;

        // BTC price
        let btc_price: u128 = get_price(ctx.accounts.pyth_btc_account.to_account_info())?;
        total_price += btc_price * btc_amount;

        // SOL price
        let sol_price: u128 = get_price(ctx.accounts.pyth_sol_account.to_account_info())?; 
        total_price += sol_price * sol_amount;

        // USDC price
        let usdc_price: u128 = get_price(ctx.accounts.pyth_usdc_account.to_account_info())?;      
        total_price += usdc_price * usdc_amount;

        // mSOL price
        let msol_price: u128 = get_price(ctx.accounts.pyth_msol_account.to_account_info())?;
        total_price += msol_price * msol_amount;

        // ETH price
        let eth_price: u128 = get_price(ctx.accounts.pyth_eth_account.to_account_info())?;
        total_price += eth_price * eth_amount;

        // ust price
        let ust_price: u128 = get_price(ctx.accounts.pyth_ust_account.to_account_info())?;
        total_price += ust_price * ust_amount;

        // srm price
        let srm_price: u128 = get_price(ctx.accounts.pyth_srm_account.to_account_info())?; 
        total_price += srm_price * srm_amount;

        // scnsol price
        let scnsol_price: u128 = get_price(ctx.accounts.pyth_scnsol_account.to_account_info())?;      
        total_price += scnsol_price * scnsol_amount;

        // stsol price
        let stsol_price: u128 = get_price(ctx.accounts.pyth_stsol_account.to_account_info())?;
        total_price += stsol_price * stsol_amount;

        // usdt price
        let usdt_price: u128 = get_price(ctx.accounts.pyth_usdt_account.to_account_info())?;
        total_price += usdt_price * usdt_amount;

        // lpETH price
        let lpeth_price: u128 = eth_price;
        total_price += lpeth_price * lpeth_amount;

        // LpUSD price
        let lpusd_price = usdc_price;        
        total_price += lpusd_price * lpusd_amount;

        // LpUSD price
        let lpbtc_price = btc_price;        
        total_price += lpbtc_price * lpbtc_amount;

        // LpSOL price
        let lpsol_price = sol_price;
        total_price += lpsol_price * lpsol_amount;

        let mut borrowed_total: u128 = 0;
        borrowed_total += borrowed_lpsol * lpsol_price;
        borrowed_total += borrowed_lpusd * lpusd_price;
        borrowed_total += borrowed_lpbtc * lpbtc_price;
        borrowed_total += borrowed_lpeth * lpeth_price;

        if total_price * LTV < borrowed_total * DOMINATOR {
            return Err(ErrorCode::InvalidAmount.into());
        }
        
        if amount > sol_amount as u64 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        let borrowable_amount = (total_price - borrowed_total * DOMINATOR / LTV) / sol_price;
        if amount > borrowable_amount as u64{
            return Err(ErrorCode::InvalidAmount.into());
        }
        
        **ctx.accounts.state_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user_authority.try_borrow_mut_lamports()? += amount;

        user_account.sol_amount -= amount;
        ctx.accounts.config.total_deposited_sol -= amount;

        Ok(())
    }

    pub fn withdraw_token(
        ctx: Context<WithdrawToken>,
        amount: u64
    ) -> Result<()> {
        msg!("Withdraw Token");

        let user_account = &mut ctx.accounts.user_account;

        if user_account.bump > 0 && user_account.bump < 6 {
            return Err(ErrorCode::InLiquidating.into());
        }

        let solend_config = &mut ctx.accounts.solend_config;
        let apricot_config = &mut ctx.accounts.apricot_config;

        let sol_amount = user_account.sol_amount as u128;
        let btc_amount = user_account.btc_amount as u128;
        let usdc_amount = user_account.usdc_amount as u128;
        let msol_amount = user_account.msol_amount as u128;
        let eth_amount = user_account.eth_amount as u128;
        let ust_amount = user_account.ust_amount as u128;
        let srm_amount = user_account.srm_amount as u128;
        let scnsol_amount = user_account.scnsol_amount as u128;
        let stsol_amount = user_account.stsol_amount as u128;
        let usdt_amount = user_account.usdt_amount as u128;

        let lending_sol_amount = user_account.lending_sol_amount as u128;
        let lending_btc_amount = user_account.lending_btc_amount as u128;
        let lending_usdc_amount = user_account.lending_usdc_amount as u128;
        let lending_msol_amount = user_account.lending_msol_amount as u128;
        let lending_eth_amount = user_account.lending_eth_amount as u128;
        let lending_ust_amount = user_account.lending_ust_amount as u128;
        let lending_srm_amount = user_account.lending_srm_amount as u128;
        let lending_scnsol_amount = user_account.lending_scnsol_amount as u128;
        let lending_stsol_amount = user_account.lending_stsol_amount as u128;
        let lending_usdt_amount = user_account.lending_usdt_amount as u128;

        let lpsol_amount = user_account.lpsol_amount as u128;
        let lpusd_amount = user_account.lpusd_amount as u128;
        let lpbtc_amount = user_account.lpbtc_amount as u128;
        let lpeth_amount = user_account.lpeth_amount as u128;

        let borrowed_lpusd = user_account.borrowed_lpusd as u128;
        let borrowed_lpsol = user_account.borrowed_lpsol as u128;
        let borrowed_lpbtc = user_account.borrowed_lpbtc as u128;
        let borrowed_lpeth = user_account.borrowed_lpeth as u128;

        let mut total_price: u128 = 0;

        // BTC price
        let btc_price: u128 = get_price(ctx.accounts.pyth_btc_account.to_account_info())?;     
        total_price += btc_price * (btc_amount + lending_btc_amount);

        // SOL price
        let sol_price: u128 = get_price(ctx.accounts.pyth_sol_account.to_account_info())?;     
        total_price += sol_price * (sol_amount + lending_sol_amount);

        // USDC price
        let usdc_price: u128 = get_price(ctx.accounts.pyth_usdc_account.to_account_info())?;     
        total_price += usdc_price * (usdc_amount + lending_usdc_amount);

        // mSOL price
        let msol_price: u128 = get_price(ctx.accounts.pyth_msol_account.to_account_info())?;
        total_price += msol_price * (msol_amount + lending_msol_amount);

        // ETH price
        let eth_price: u128 = get_price(ctx.accounts.pyth_eth_account.to_account_info())?;   
        total_price += eth_price * (eth_amount + lending_eth_amount);

        // ust price
        let ust_price: u128 = get_price(ctx.accounts.pyth_ust_account.to_account_info())?;     
        total_price += ust_price * (ust_amount + lending_ust_amount);

        // srm price
        let srm_price: u128 = get_price(ctx.accounts.pyth_srm_account.to_account_info())?;     
        total_price += srm_price * (srm_amount + lending_srm_amount);

        // scnsol price
        let scnsol_price: u128 = get_price(ctx.accounts.pyth_scnsol_account.to_account_info())?;     
        total_price += scnsol_price * (scnsol_amount + lending_scnsol_amount);

        // stsol price
        let stsol_price: u128 = get_price(ctx.accounts.pyth_stsol_account.to_account_info())?;
        total_price += stsol_price * (stsol_amount + lending_stsol_amount);

        // usdt price
        let usdt_price: u128 = get_price(ctx.accounts.pyth_usdt_account.to_account_info())?;   
        total_price += usdt_price * (usdt_amount + lending_usdt_amount);

        // LpUSD price
        let lpusd_price = usdc_price;        
        total_price += lpusd_price * lpusd_amount;

        // LpSOL price
        let lpsol_price = sol_price;
        total_price += lpsol_price * lpsol_amount;

        // LpETH price
        let lpeth_price: u128 = eth_price;   
        total_price += lpeth_price * lpeth_amount;

        // LpBTC price
        let lpbtc_price = btc_price;
        total_price += lpbtc_price * lpbtc_amount;

        let mut borrowed_total: u128 = 0;
        borrowed_total += borrowed_lpsol * lpsol_price;
        borrowed_total += borrowed_lpusd * lpusd_price;
        borrowed_total += borrowed_lpbtc * lpbtc_price;
        borrowed_total += borrowed_lpeth * lpeth_price;

        if total_price * LTV < borrowed_total * DOMINATOR {
            return Err(ErrorCode::InvalidAmount.into());
        }        
        
        let mut dest_price:u128 = 0;
        let mut owned_amount:u128 = 0;
        let mut require_lending_amount: u64 = 0;

        if ctx.accounts.dest_mint.key() == ctx.accounts.config.ray_mint {
            if solend_config.wsol_rate > apricot_config.wsol_rate {
                owned_amount = usdc_amount + solend_config.wsol_rate as u128 * lending_usdc_amount / LENDING_DENOMINATOR;
                require_lending_amount = (solend_config.wsol_rate as u128 * lending_usdc_amount / LENDING_DENOMINATOR) as u64;
            } else {
                owned_amount = usdc_amount + apricot_config.wsol_rate as u128 * lending_usdc_amount / LENDING_DENOMINATOR;
                require_lending_amount = (apricot_config.wsol_rate as u128 * lending_usdc_amount / LENDING_DENOMINATOR) as u64;
            }
            dest_price = usdc_price;
            user_account.usdc_amount = owned_amount as u64 - amount;
            user_account.lending_usdc_amount = 0;
            ctx.accounts.config.total_deposited_usdc -= amount;
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
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.ray_mint {
            if solend_config.ray_rate > apricot_config.ray_rate {
                owned_amount = btc_amount + solend_config.ray_rate as u128 * lending_btc_amount / LENDING_DENOMINATOR;
                require_lending_amount = (solend_config.ray_rate as u128 * lending_btc_amount / LENDING_DENOMINATOR) as u64;
            } else {
                owned_amount = btc_amount + apricot_config.ray_rate as u128 * lending_btc_amount / LENDING_DENOMINATOR;
                require_lending_amount = (apricot_config.ray_rate as u128 * lending_btc_amount / LENDING_DENOMINATOR) as u64;
            }
            dest_price = btc_price;
            user_account.btc_amount = owned_amount as u64 - amount;
            ctx.accounts.config.total_deposited_btc -= amount;
            user_account.lending_btc_amount = 0;
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
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.lpray_mint {
            dest_price = lpbtc_price;
            owned_amount = lpbtc_amount;
            user_account.lpbtc_amount -= amount;
            ctx.accounts.config.total_deposited_lpbtc -= amount;
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.lpeth_mint {
            dest_price = lpeth_price;
            owned_amount = lpeth_amount;
            user_account.lpeth_amount -= amount;
            ctx.accounts.config.total_deposited_lpeth -= amount;
        } else if ctx.accounts.dest_mint.key() == ctx.accounts.config.lpusd_mint {
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

        if amount > owned_amount as u64 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        let borrowable_amount = (total_price - borrowed_total * DOMINATOR / LTV) / dest_price;
        if amount > borrowable_amount as u64{
            return Err(ErrorCode::InvalidAmount.into());
        }
        
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

        let config = &mut ctx.accounts.config;
        if require_lending_amount > 0 && (ctx.accounts.user_dest.mint == config.ray_mint || 
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
            } else if ctx.accounts.user_dest.mint == config.ray_mint {
                if ctx.accounts.solend_config.wsol_rate > ctx.accounts.apricot_config.wsol_rate {
                    solend_higher = true;
                }
            } else if ctx.accounts.user_dest.mint == config.msol_mint {
                if ctx.accounts.solend_config.msol_rate > ctx.accounts.apricot_config.msol_rate {
                    solend_higher = true;
                }
            } else if ctx.accounts.user_dest.mint == config.ust_mint {
                solend_higher = true;
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
                msg!("Withdraw from solend");
                let cpi_program = ctx.accounts.solend_program.to_account_info();
                let cpi_accounts = solend::cpi::accounts::WithdrawToken {
                    authority: ctx.accounts.state_account.to_account_info(),
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
                    authority: ctx.accounts.state_account.to_account_info(),
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
            authority: ctx.accounts.state_account.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn repay_token(
        ctx: Context<RepayToken>,
        amount: u64
    ) -> Result<()> {
        if ctx.accounts.user_dest.amount < amount {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        let user_account =&mut ctx.accounts.user_account;

        if user_account.bump > 0 && user_account.bump < 6 {
            return Err(ErrorCode::InLiquidating.into());
        }

        let config = &mut ctx.accounts.config;


        if ctx.accounts.user_dest.mint != config.ray_mint && 
            ctx.accounts.user_dest.mint != config.eth_mint &&
            ctx.accounts.user_dest.mint != config.ray_mint &&
            ctx.accounts.user_dest.mint != config.lpusd_mint &&
            ctx.accounts.user_dest.mint != config.lpeth_mint &&
            ctx.accounts.user_dest.mint != config.lpray_mint &&
            ctx.accounts.user_dest.mint != config.lpsol_mint
        {
            return Err(ErrorCode::InvalidToken.into());
        }

        if ctx.accounts.user_dest.mint == config.ray_mint ||
            ctx.accounts.user_dest.mint == config.eth_mint ||
            ctx.accounts.user_dest.mint == config.ray_mint       
        {
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
            ctx.accounts.user_dest.mint == config.lpeth_mint ||
            ctx.accounts.user_dest.mint == config.lpsol_mint 
        {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.dest_mint.to_account_info(),
                    to: ctx.accounts.user_dest.to_account_info(),
                    authority: ctx.accounts.user_authority.to_account_info()
                }
            );

            token::burn(cpi_ctx, amount)?;
        }

        if ctx.accounts.user_dest.mint == config.ray_mint || ctx.accounts.user_dest.mint == config.lpusd_mint{  
            if user_account.borrowed_lpusd < amount || config.total_borrowed_lpusd < amount {
                return Err(ErrorCode::RepayFinished.into());
            }

            user_account.borrowed_lpusd = user_account.borrowed_lpusd - amount;
            config.total_borrowed_lpusd = config.total_borrowed_lpusd - amount;  
        } else if ctx.accounts.user_dest.mint == config.lpsol_mint {
            if user_account.borrowed_lpsol < amount || config.total_borrowed_lpsol < amount {
                return Err(ErrorCode::RepayFinished.into());
            }
            user_account.borrowed_lpsol = user_account.borrowed_lpsol - amount;
            config.total_borrowed_lpsol = config.total_borrowed_lpsol - amount;            
        } else if ctx.accounts.user_dest.mint == config.lpray_mint || ctx.accounts.user_dest.mint == config.ray_mint{
            if user_account.borrowed_lpbtc < amount || config.total_borrowed_lpbtc < amount {
                return Err(ErrorCode::RepayFinished.into());
            }

            user_account.borrowed_lpbtc = user_account.borrowed_lpbtc - amount;
            config.total_borrowed_lpbtc = config.total_borrowed_lpbtc - amount;            
        } else if ctx.accounts.user_dest.mint == config.lpeth_mint || ctx.accounts.user_dest.mint == config.eth_mint{
            if user_account.borrowed_lpeth < amount || config.total_borrowed_lpeth < amount {
                return Err(ErrorCode::RepayFinished.into());
            }

            user_account.borrowed_lpeth = user_account.borrowed_lpeth - amount;
            config.total_borrowed_lpeth = config.total_borrowed_lpeth - amount;            
        }

        Ok(())
    }

    pub fn repay_sol(
        ctx: Context<RepaySOL>,
        amount: u64
    ) -> Result<()> {
        if **ctx.accounts.user_authority.lamports.borrow() < amount {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        if amount > ctx.accounts.user_account.borrowed_lpsol || amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        invoke(
            &system_instruction::transfer(
                ctx.accounts.user_authority.key,
                ctx.accounts.state_account.to_account_info().key,
                amount
            ),
            &[
                ctx.accounts.user_authority.to_account_info().clone(),
                ctx.accounts.state_account.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone()
            ]
        )?;

        let user_account = &mut ctx.accounts.user_account;

        if user_account.bump > 0 && user_account.bump < 6 {
            return Err(ErrorCode::InLiquidating.into());
        }

        let config = &mut ctx.accounts.config;

        if user_account.borrowed_lpsol < amount || config.total_borrowed_lpsol < amount {
            return Err(ErrorCode::RepayFinished.into());
        }

        user_account.borrowed_lpsol = user_account.borrowed_lpsol - amount;
        config.total_borrowed_lpsol = config.total_borrowed_lpsol - amount;

        Ok(())
    }

    pub fn update_user_account(
        ctx: Context<UpdateUserAccount>,
        step: u8
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.bump = step;

        // Need to reset everything
        if step == 10 {
            user_account.bump = 0;
            user_account.lpusd_amount = 0;
            user_account.lpsol_amount = 0;
            user_account.lpbtc_amount = 0;
            user_account.lpeth_amount = 0;

            user_account.sol_amount = 0;
            user_account.usdc_amount = 0;
            user_account.btc_amount = 0;
            user_account.msol_amount = 0;
            user_account.eth_amount = 0;
            user_account.ust_amount = 0;
            user_account.srm_amount = 0;
            user_account.scnsol_amount = 0;
            user_account.stsol_amount = 0;
            user_account.usdt_amount = 0;

            user_account.borrowed_lpusd = 0;
            user_account.borrowed_lpsol = 0;
            user_account.borrowed_lpbtc = 0;
            user_account.borrowed_lpeth = 0;


            user_account.lending_btc_amount = 0;
            user_account.lending_sol_amount = 0;
            user_account.lending_usdc_amount = 0;
            user_account.lending_eth_amount = 0;
            user_account.lending_msol_amount = 0;
            user_account.lending_ust_amount = 0;
            user_account.lending_srm_amount = 0;
            user_account.lending_scnsol_amount = 0;
            user_account.lending_stsol_amount = 0;
            user_account.lending_usdt_amount = 0;
        }
        Ok(())
    }

    pub fn fix_user_account(
        ctx: Context<UpdateUserAccount>,
        amount: u64
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.borrowed_lpusd = 0;
        user_account.borrowed_lpsol = 0;
        user_account.borrowed_lpbtc = 0;
        user_account.borrowed_lpeth = 0;

        Ok(())
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>
    ) -> Result<()> {
        msg!("Update Config");

        let config = &mut ctx.accounts.config;

        
        config.ray_mint = ctx.accounts.ray_mint.key();
        config.ray_mint = ctx.accounts.ray_mint.key();
        config.eth_mint = ctx.accounts.eth_mint.key();
        config.msol_mint = ctx.accounts.msol_mint.key();     
        config.ust_mint = ctx.accounts.ust_mint.key();
        config.srm_mint = ctx.accounts.srm_mint.key();
        config.scnsol_mint = ctx.accounts.scnsol_mint.key();
        config.stsol_mint = ctx.accounts.stsol_mint.key();
        config.usdt_mint = ctx.accounts.usdt_mint.key();      
        config.lpusd_mint = ctx.accounts.lpusd_mint.key();
        config.lpsol_mint = ctx.accounts.lpsol_mint.key();
        config.lpray_mint = ctx.accounts.lpray_mint.key();
        config.lpeth_mint = ctx.accounts.lpeth_mint.key();
        config.pool_btc = ctx.accounts.pool_btc.key();
        config.pool_usdc = ctx.accounts.pool_usdc.key();
        config.pool_msol = ctx.accounts.pool_msol.key();
        config.pool_eth = ctx.accounts.pool_eth.key();
        config.pool_ust = ctx.accounts.pool_ust.key();
        config.pool_scnsol = ctx.accounts.pool_scnsol.key();
        config.pool_stsol = ctx.accounts.pool_stsol.key();
        config.pool_srm = ctx.accounts.pool_srm.key();
        config.pool_usdt = ctx.accounts.pool_usdt.key();
        config.pool_lpsol = ctx.accounts.pool_lpsol.key();
        config.pool_lpusd = ctx.accounts.pool_lpusd.key();
        config.pool_lpbtc = ctx.accounts.pool_lpbtc.key();
        config.pool_lpeth = ctx.accounts.pool_lpeth.key();


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
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Invalid Token")]
    InvalidToken,
    #[msg("Invalid Owner")]
    InvalidOwner,
    #[msg("In Liquidating")]
    ProgressInLiquidate,
    #[msg("Repay finished for the selected token")]
    RepayFinished,
    #[msg("Progress in Liquidating")]
    InLiquidating
}
