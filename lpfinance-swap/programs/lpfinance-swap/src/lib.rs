use anchor_lang::prelude::*;
use pyth_client;
use anchor_spl::token::{self, Transfer };

mod states;
pub use states::*;

declare_id!("87jyVePaEbZAYAcAjtGwQTcC4LU188KxJdynUzFWJKHA");

#[program]
pub mod lpfinance_swap {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>
    ) -> Result<()> {
        msg!("INITIALIZE SWAP");

        let state_account = &mut ctx.accounts.state_account;
        state_account.owner = ctx.accounts.authority.key();
        state_account.lpfi_mint = ctx.accounts.lpfi_mint.key();

        Ok(())
    }

    // Create token account for swap protocol
    pub fn initialize_pool(
        ctx: Context<InitializePool>
    ) -> Result<()> {
        let token_mint = &mut ctx.accounts.token_mint;
        msg!("INITIALIZE Pool {:?}", token_mint.key().to_string());

        Ok(())
    }

    // Create new token pair
    pub fn create_pair(
        ctx: Context<CreatePair>,
        tokena_amount: u64,
        tokenb_amount: u64
    ) -> Result<()> {
        if ctx.accounts.user_tokena.amount < tokena_amount || 
            ctx.accounts.user_tokenb.amount < tokenb_amount
        {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        {
            let cpi_accounts = Transfer {
                from: ctx.accounts.user_tokena.to_account_info(),
                to: ctx.accounts.tokena_pool.to_account_info(),
                authority: ctx.accounts.authority.to_account_info()
            };

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, tokena_amount)?;
        }

        {
            let cpi_accounts = Transfer {
                from: ctx.accounts.user_tokenb.to_account_info(),
                to: ctx.accounts.tokenb_pool.to_account_info(),
                authority: ctx.accounts.authority.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, tokenb_amount)?;
        }        

        let liquidity_pool = &mut ctx.accounts.liquidity_pool;
        liquidity_pool.tokena_amount = tokena_amount;
        liquidity_pool.tokenb_amount = tokenb_amount;
        liquidity_pool.initial_tokena_amount = tokena_amount;
        liquidity_pool.initial_tokenb_amount = tokenb_amount;
        liquidity_pool.tokena_mint = ctx.accounts.tokena_mint.key();
        liquidity_pool.tokenb_mint = ctx.accounts.tokenb_mint.key();

        Ok(())
    }

    // Create new token pair
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        tokena_amount: u64,
        tokenb_amount: u64
    ) -> Result<()> {
        
        if ctx.accounts.user_tokena.amount < tokena_amount || 
            ctx.accounts.user_tokenb.amount < tokenb_amount
        {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        let liquidity_pool = &mut ctx.accounts.liquidity_pool;
        // Need to keep initial price
        if liquidity_pool.initial_tokena_amount * tokenb_amount != 
            liquidity_pool.initial_tokenb_amount * tokena_amount 
        {   
            return Err(ErrorCode::InvalidQuoteAmount.into());
        }

        {
            let cpi_accounts = Transfer {
                from: ctx.accounts.user_tokena.to_account_info(),
                to: ctx.accounts.tokena_pool.to_account_info(),
                authority: ctx.accounts.authority.to_account_info()
            };

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, tokena_amount)?;
        }

        {
            let cpi_accounts = Transfer {
                from: ctx.accounts.user_tokenb.to_account_info(),
                to: ctx.accounts.tokenb_pool.to_account_info(),
                authority: ctx.accounts.authority.to_account_info()
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, tokenb_amount)?;
        }        

        liquidity_pool.tokena_amount += tokena_amount;
        liquidity_pool.tokenb_amount += tokenb_amount;

        Ok(())
    }

    // Swap token to token
    pub fn swap_token_to_token(
        ctx: Context<SwapTokenToToken>,
        quote_amount: u64
    ) -> Result<()> {

        if quote_amount == 0 {
            return Err(ErrorCode::InvalidQuoteAmount.into());
        }

        if ctx.accounts.user_quote.amount < quote_amount {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        let pyth_price_info = &ctx.accounts.pyth_quote_account;
        let pyth_price_data = &pyth_price_info.try_borrow_data()?;
        let pyth_price = pyth_client::cast::<pyth_client::Price>(pyth_price_data);

        let quote_price = pyth_price.agg.price as u128;
        let mut quote_total: u128 = quote_price * quote_amount as u128;

        // If quote token is LpFi Dao token, swap LpFi-> USDC and then USDC -> dest token
        if ctx.accounts.quote_mint.key() == ctx.accounts.state_account.lpfi_mint {
            let lpfi_price = ctx.accounts.liquidity_pool.get_token_price(ctx.accounts.state_account.usdc_mint, ctx.accounts.state_account.lpfi_mint)?;
            let usdc_amount = lpfi_price * quote_amount as u128 / PRICE_MULTIPLIER;
            // In case of quote token is LpFi, pyth_quote_account is usdc so quote_price is for USDC price
            quote_total = quote_price * usdc_amount;

            msg!("Quote USDC Amount: !!{:?}!!", usdc_amount.to_string());
            let liquidity_pool = &mut ctx.accounts.liquidity_pool;     
            
            if liquidity_pool.tokena_mint == ctx.accounts.state_account.lpfi_mint {
                liquidity_pool.tokena_amount = liquidity_pool.tokena_amount + quote_amount;
                liquidity_pool.tokenb_amount = liquidity_pool.tokenb_amount - usdc_amount as u64;
            } else if liquidity_pool.tokenb_mint == ctx.accounts.state_account.lpfi_mint {
                liquidity_pool.tokenb_amount = liquidity_pool.tokenb_amount + quote_amount;
                liquidity_pool.tokena_amount = liquidity_pool.tokena_amount - usdc_amount as u64;
            }       
        }

        // destination token
        let pyth_price_info = &ctx.accounts.pyth_dest_account;
        let pyth_price_data = &pyth_price_info.try_borrow_data()?;
        let pyth_price = pyth_client::cast::<pyth_client::Price>(pyth_price_data);

        let dest_price = pyth_price.agg.price as u128;

        msg!("Quote Price: !!{:?}!!", quote_price.to_string());
        msg!("Dest Price: !!{:?}!!", dest_price.to_string());

        let mut transfer_amount = (quote_total/dest_price) as u64;

        // If dest_mint is LpFi DAO token
        if ctx.accounts.dest_mint.key() == ctx.accounts.state_account.lpfi_mint {
            let lpfi_price = ctx.accounts.liquidity_pool.get_token_price(ctx.accounts.state_account.usdc_mint, ctx.accounts.state_account.lpfi_mint)?;
            // In case of dest token is LpFi, pyth_dest_account is usdc so dest_price is for USDC price
            let usdc_amount = quote_total / dest_price;
            transfer_amount = (usdc_amount * PRICE_MULTIPLIER / lpfi_price) as u64;

            let liquidity_pool = &mut ctx.accounts.liquidity_pool;        

            if liquidity_pool.tokena_mint == ctx.accounts.state_account.lpfi_mint {
                liquidity_pool.tokena_amount = liquidity_pool.tokena_amount - transfer_amount;
                liquidity_pool.tokenb_amount = liquidity_pool.tokenb_amount + usdc_amount as u64;
            } else if liquidity_pool.tokenb_mint == ctx.accounts.state_account.lpfi_mint {
                liquidity_pool.tokenb_amount = liquidity_pool.tokenb_amount - transfer_amount;
                liquidity_pool.tokena_amount = liquidity_pool.tokena_amount + usdc_amount as u64;
            }
        }

        msg!("Transfer Amount: !!{:?}!!", transfer_amount.to_string());
        msg!("Quote Total: !!{:?}!!", quote_total.to_string());

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_quote.to_account_info(),
            to: ctx.accounts.quote_pool.to_account_info(),
            authority: ctx.accounts.user_authority.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, quote_amount)?;

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


        let cpi_accounts = Transfer {
            from: ctx.accounts.dest_pool.to_account_info(),
            to: ctx.accounts.user_dest.to_account_info(),
            authority: ctx.accounts.state_account.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, transfer_amount)?;

        Ok(())
    }

    pub fn liquidate_token(
        ctx: Context<LiquidateToken>,
        amount: u64
    ) -> Result<()> {
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
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.swap_pool.to_account_info(),
            to: ctx.accounts.auction_pool.to_account_info(),
            authority: ctx.accounts.state_account.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient Amount")]
    InsufficientAmount,

    #[msg("Invalid Quote Amount")]
    InvalidQuoteAmount,

    #[msg("Invalid Owner")]
    InvalidOwner
}
