use anchor_lang::prelude::*;
use pyth_client;
use anchor_spl::token::{self, Transfer };
use anchor_spl::token::{ Mint, TokenAccount, Token };
use anchor_lang::{solana_program, Result};
use solana_program::{
    program::{invoke, invoke_signed},
};

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
        state_account.usdc_mint = ctx.accounts.usdc_mint.key();

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
        tokenb_amount: u64,
        min_lp_amount: u64,
        fee: u8
    ) -> Result<()> {
        if tokena_amount == 0 || tokenb_amount == 0 {
            return Err(ErrorCode::InvalidQuoteAmount.into());

        }
        
        let creator: &Signer = &ctx.accounts.creator;
        let token_lp: &mut Account<Mint> = &mut ctx.accounts.token_lp;
        let token_acc_lp: &mut Account<TokenAccount> = &mut ctx.accounts.token_acc_lp;
        let token_program:&Program<Token> = &ctx.accounts.token_program;

        let token_a: &Account<Mint> = &ctx.accounts.tokena_mint;
        let token_b: &Account<Mint> = &ctx.accounts.tokenb_mint;
        let token_acc_a: &Account<TokenAccount> = &ctx.accounts.token_acc_a;
        let token_acc_b: &Account<TokenAccount> = &ctx.accounts.token_acc_b;

        let liquidity_pool = &mut ctx.accounts.liquidity_pool;

        liquidity_pool.title = "pool".to_string();
        liquidity_pool.state = 1;
        liquidity_pool.creator = *creator.key;
        liquidity_pool.token_lp = token_lp.key();
        liquidity_pool.token_acc_lp = token_acc_lp.key();
        liquidity_pool.total_lp_amount = 0;
        liquidity_pool.token_acc_a = token_acc_a.key();
        liquidity_pool.token_acc_b = token_acc_b.key();
        liquidity_pool.min_lp_amount = min_lp_amount;

        // Sort token mint and acc
        if token_a.key() >= token_b.key() {
            liquidity_pool.tokena_mint = token_a.key();
            liquidity_pool.tokenb_mint = token_b.key();
            liquidity_pool.token_acc_a = ctx.accounts.token_acc_a.key();
            liquidity_pool.token_acc_b = ctx.accounts.token_acc_b.key();

            liquidity_pool.tokena_amount = tokena_amount;
            liquidity_pool.tokenb_amount = tokenb_amount;
        } else {
            liquidity_pool.tokena_mint = token_b.key();
            liquidity_pool.tokenb_mint = token_a.key();
            liquidity_pool.token_acc_a = ctx.accounts.token_acc_b.key();
            liquidity_pool.token_acc_b = ctx.accounts.token_acc_a.key();

            liquidity_pool.tokena_amount = tokenb_amount;
            liquidity_pool.tokenb_amount = tokena_amount;
        }

        if fee < 5{
            liquidity_pool.fee = 5;
        }else if fee > 100 {
            liquidity_pool.fee = 100;
        }else{
            liquidity_pool.fee = fee;
        }

         //--------- LP Token Mint to Creator -----------------
         let lp_token_mint_ix = spl_token::instruction::mint_to(
            token_program.key,
            token_lp.to_account_info().key,
            token_acc_lp.to_account_info().key,
            &creator.to_account_info().key,
            &[&creator.to_account_info().key],
            0xFFFFFFFFFFFFFFFF
        )?;
        invoke(
            &lp_token_mint_ix,
            &[
                creator.to_account_info().clone(),
                token_program.to_account_info().clone(),
                token_lp.to_account_info().clone(),
                token_acc_lp.to_account_info().clone(),
            ],
        )?;

        //-------- PDA Generate --------------------------------
        let (pda, _nonce) = Pubkey::find_program_address(
            &[PREFIX.as_bytes()],
            ctx.program_id
        );        
        //-------- LP Token Owner Creator -> PDA ----------------
        let lp_owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            token_lp.to_account_info().key,
            Some(&pda),
            spl_token::instruction::AuthorityType::MintTokens,
            creator.to_account_info().key,
            &[&creator.to_account_info().key],
        )?;
        invoke(
            &lp_owner_change_ix,
            &[
                creator.to_account_info().clone(),
                token_program.to_account_info().clone(),
                token_lp.to_account_info().clone(),
            ],
        )?;
        //-------- LP Token Account Owner Creator -> PDA ----------------
        let lp_owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            token_acc_lp.to_account_info().key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            creator.to_account_info().key,
            &[&creator.to_account_info().key],
        )?;
        invoke(
            &lp_owner_change_ix,
            &[
                creator.to_account_info().clone(),
                token_program.to_account_info().clone(),
                token_acc_lp.to_account_info().clone(),
            ],
        )?;

        Ok(())
    }

    pub fn init_liquidity(ctx: Context<InitLiquidity>, 
        amount_a: u64, 
        amount_b: u64,
    ) -> Result<()> {
        let pool: &mut Account<PoolInfo> = &mut ctx.accounts.pool;
        let pool_pda = &ctx.accounts.pool_pda;
        let creator: &Signer = &ctx.accounts.creator;
        let creator_acc_a: &AccountInfo = &ctx.accounts.creator_acc_a;
        let creator_acc_b: &AccountInfo = &ctx.accounts.creator_acc_b;
        let token_acc_a: &AccountInfo = &ctx.accounts.token_acc_a;
        let token_acc_b: &AccountInfo = &ctx.accounts.token_acc_b;
        let ata_creator_lp: &mut Account<TokenAccount> =  &mut ctx.accounts.ata_creator_lp;
        let token_acc_lp:&mut Account<TokenAccount> = &mut ctx.accounts.token_acc_lp;
        let token_program:&Program<Token> = &ctx.accounts.token_program;

        if pool.state != 1 {
            return Err(ErrorCode::InitLiquidityStepError.into());
        }
        
        if (pool.tokena_amount != amount_a) || (pool.tokenb_amount != amount_b) {
            return Err(ErrorCode::InvalidQuoteAmount.into());
        } 

        if (pool.token_acc_a == token_acc_a.key() && pool.token_acc_b != token_acc_b.key()) 
            || (pool.token_acc_a != token_acc_a.key() && pool.token_acc_b == token_acc_b.key()) {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }

        pool.state = 2;

        
        //-------- Deposit Token A Creator -> POOL PDA -----------------------------
        let deposit_ix_a = spl_token::instruction::transfer(
            token_program.key,
            creator_acc_a.to_account_info().key,
            token_acc_a.to_account_info().key,
            creator.to_account_info().key,
            &[&creator.to_account_info().key],
            amount_a
        )?;
        invoke(
            &deposit_ix_a,
            &[
                creator.to_account_info().clone(),
                creator_acc_a.to_account_info().clone(),
                token_acc_a.to_account_info().clone(),
                token_program.to_account_info().clone(),
            ],
        )?;
        //-------- Deposit Token B Creator -> POOL PDA -----------------------------
        let deposit_ix_b = spl_token::instruction::transfer(
            token_program.key,
            creator_acc_b.to_account_info().key,
            token_acc_b.to_account_info().key,
            creator.to_account_info().key,
            &[&creator.to_account_info().key],
            amount_b
        )?;
        invoke(
            &deposit_ix_b,
            &[
                creator.to_account_info().clone(),
                creator_acc_b.to_account_info().clone(),
                token_acc_b.to_account_info().clone(),
                token_program.to_account_info().clone(),
            ],
        )?;

        let lp_rewards = pool.get_lptoken_amount(amount_a, amount_b)?;
        
        msg!("Lp reward token: {}", lp_rewards.to_string());
        pool.total_lp_amount += lp_rewards;

        //-------- PDA Generate --------------------------------
        let (pda, _nonce) = Pubkey::find_program_address(
            &[PREFIX.as_bytes()],
            ctx.program_id
        );        
        msg!("PDA: {}", pda.to_string());
        //---------- LP Token Rewards -----------------------------
        let lp_token_rewards_ix = spl_token::instruction::transfer(
            token_program.key,
            token_acc_lp.to_account_info().key,
            ata_creator_lp.to_account_info().key,
            &pda,
            &[&pda],
            lp_rewards
        )?;
        invoke_signed(
            &lp_token_rewards_ix,
            &[
                token_acc_lp.to_account_info().clone(),
                ata_creator_lp.to_account_info().clone(),
                token_program.to_account_info().clone(),
                pool_pda.to_account_info().clone()
            ],
            &[&[&PREFIX.as_bytes().as_ref(), &[_nonce]]],
        )?;
        Ok(())
    }

    // Delete Pool
    pub fn delete_pool(_ctx: Context<DeletePool>) -> Result<()> {
        Ok(())
    }

    // Create new token pair
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a: u64, // Desired amount
        amount_b: u64  // Desired amount
    ) -> Result<()> {
        
        if ctx.accounts.user_tokena.amount < amount_a || 
            ctx.accounts.user_tokenb.amount < amount_b
        {
            return Err(ErrorCode::InsufficientAmount.into());
        }

        let pool: &mut Account<PoolInfo> = &mut ctx.accounts.liquidity_pool;
        let authority: &Signer = &ctx.accounts.authority;
        let adder_acc_a = &ctx.accounts.user_tokena;
        let adder_acc_b = &ctx.accounts.user_tokenb;
        let token_acc_a = &ctx.accounts.tokena_pool;
        let token_acc_b = &ctx.accounts.tokenb_pool;
        let token_lp: &Account<Mint> = &ctx.accounts.token_lp;
        let ata_adder_lp: &Account<TokenAccount> = &ctx.accounts.ata_adder_lp;
        let token_acc_lp: &AccountInfo = &ctx.accounts.token_acc_lp;
        let pool_pda: &AccountInfo = &ctx.accounts.pool_pda;
        let token_program:&Program<Token> = &ctx.accounts.token_program;

        if pool.state != 2 {
            return Err(ErrorCode::AddLiquidityStepError.into());
        }

        if amount_a == 0 || amount_b == 0{
            return Err(ErrorCode::AmountZeroError.into());
        }

        if pool.token_lp != token_lp.key() {
            return Err(ErrorCode::LpTokenError.into());
        }

        if pool.token_acc_lp != token_acc_lp.key() {
            return Err(ErrorCode::LpTokenAccountError.into());
        }


        if (pool.token_acc_a == token_acc_a.key() && pool.token_acc_b != token_acc_b.key()) || 
           (pool.token_acc_a != token_acc_a.key() && pool.token_acc_b == token_acc_b.key()) {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }
        // Check if Input token's order is correct or not
        let is_right_order = pool.token_acc_a == token_acc_a.key();

        let reserve_a = if is_right_order { pool.tokena_amount } else { pool.tokenb_amount };
        let reserve_b = if is_right_order { pool.tokenb_amount } else { pool.tokena_amount };
        let reserve_a_f = reserve_a as f64;
        let reserve_b_f = reserve_b as f64;

        let amount_a_f = amount_a as f64;
        let amount_b_f = amount_a as f64;

        let optimal_b = (amount_a_f * reserve_b_f / reserve_a_f) as u64;
        let optimal_a = (amount_b_f * reserve_a_f / reserve_b_f) as u64;

        let senda_amount = if optimal_b <= amount_b { amount_a } else { optimal_a };
        let sendb_amount = if optimal_b <= amount_b { optimal_b } else { optimal_b };
        
        let lp_rewards = if is_right_order { pool.get_lptoken_amount(senda_amount, sendb_amount)? } else {pool.get_lptoken_amount(sendb_amount, senda_amount)?};

        msg!("Filtered amount (a, b, lp) {}, {}, {}", senda_amount, sendb_amount, lp_rewards);

        pool.total_lp_amount += lp_rewards;
        pool.tokena_amount += if is_right_order { senda_amount } else { sendb_amount };
        pool.tokenb_amount += if is_right_order { sendb_amount } else { senda_amount };

        //== Transfer token A User -> Pool
        {
            let cpi_accounts = Transfer {
                from: adder_acc_a.to_account_info(),
                to: token_acc_a.to_account_info(),
                authority: authority.to_account_info()
            };

            let cpi_program = token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, senda_amount)?;
        }

        //== Transfer token B User -> Pool
        {
            let cpi_accounts = Transfer {
                from: adder_acc_b.to_account_info(),
                to: token_acc_b.to_account_info(),
                authority: authority.to_account_info()
            };
    
            let cpi_program = token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, sendb_amount)?;
        }        

        //-------- PDA Generate --------------------------------
        let (pda, _nonce) = Pubkey::find_program_address(
            &[PREFIX.as_bytes()],
            ctx.program_id
        );        
        msg!("PDA: {}", pda.to_string());

        //---------- LP Token Rewards -----------------------------
        let lp_token_rewards_ix = spl_token::instruction::transfer(
            token_program.key,
            token_acc_lp.to_account_info().key,
            ata_adder_lp.to_account_info().key,
            &pda,
            &[&pda],
            lp_rewards
        )?;
        invoke_signed(
            &lp_token_rewards_ix,
            &[
                token_acc_lp.to_account_info().clone(),
                ata_adder_lp.to_account_info().clone(),
                token_program.to_account_info().clone(),
                pool_pda.to_account_info().clone()
            ],
            &[&[&PREFIX.as_bytes().as_ref(), &[_nonce]]],
        )?;
        Ok(())
    }

    // Remove Lp
    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, 
        amount_lp: u64,
    ) -> Result<()> {
        let pool: &mut Account<PoolInfo> = &mut ctx.accounts.pool;
        let remover: &Signer = &ctx.accounts.remover;
        let remover_acc_a: &AccountInfo = &ctx.accounts.remover_acc_a;
        let remover_acc_b: &AccountInfo = &ctx.accounts.remover_acc_b;
        let token_acc_a: &AccountInfo = &ctx.accounts.token_acc_a;
        let token_acc_b: &AccountInfo = &ctx.accounts.token_acc_b;
        let ata_remover_lp: &AccountInfo = &ctx.accounts.ata_remover_lp;
        let token_acc_lp: &AccountInfo = &ctx.accounts.token_acc_lp;
        let pool_pda: &AccountInfo = &ctx.accounts.pool_pda;
        let token_program:&Program<Token> = &ctx.accounts.token_program;

        if pool.state != 2 {
            return Err(ErrorCode::AddLiquidityStepError.into());
        }
        
        if amount_lp == 0 {
            return Err(ErrorCode::AmountZeroError.into());
        }

        if pool.token_acc_lp != token_acc_lp.key() {
            return Err(ErrorCode::LpTokenAccountError.into());
        }
        
        if (pool.token_acc_a == token_acc_a.key() && pool.token_acc_b != token_acc_b.key()) || 
           (pool.token_acc_a != token_acc_a.key() && pool.token_acc_b == token_acc_b.key()) {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }
        // Check if Input token's order is correct or not
        let is_right_order = pool.token_acc_a == token_acc_a.key();

        let amount_lp_f: f64 = amount_lp as f64;
        let amount_total_lp_f: f64 = pool.total_lp_amount as f64;
        let pool_amount_a_f: f64 = pool.tokena_amount as f64;
        let pool_amount_b_f: f64 = pool.tokenb_amount as f64;
        let amount_a: u64 = ((amount_lp_f / amount_total_lp_f) * pool_amount_a_f) as u64;
        let amount_b: u64 = ((amount_lp_f / amount_total_lp_f) * pool_amount_b_f) as u64;

        pool.tokena_amount -= amount_a;
        pool.tokenb_amount -= amount_b;
        pool.total_lp_amount -= amount_lp;        

        let senda_amount = if is_right_order { amount_a } else { amount_b };
        let sendb_amount = if is_right_order { amount_b } else { amount_a };
        //---------- LP Token Transfer Remover -> Pool PDA -----------------------------
        let lp_token_return_ix = spl_token::instruction::transfer(
            token_program.key,
            ata_remover_lp.to_account_info().key,
            token_acc_lp.to_account_info().key,
            &remover.to_account_info().key(),
            &[&remover.to_account_info().key()],
            amount_lp
        )?;
        invoke(
            &lp_token_return_ix,
            &[
                token_acc_lp.to_account_info().clone(),
                ata_remover_lp.to_account_info().clone(),
                token_program.to_account_info().clone(),
                remover.to_account_info().clone()
            ]
        )?;
        //-------- PDA Generate --------------------------------
        let (pda, _nonce) = Pubkey::find_program_address(
            &[PREFIX.as_bytes()],
            ctx.program_id
        );        
        msg!("PDA: {}", pda.to_string());
        //-------- Withdraw Token A POOL PDA -> Remover -----------------------------
        let withdraw_ix_a = spl_token::instruction::transfer(
            token_program.key,
            token_acc_a.to_account_info().key,
            remover_acc_a.to_account_info().key,
            &pda,
            &[&pda],
            senda_amount
        )?;
        invoke_signed(
            &withdraw_ix_a,
            &[
                pool_pda.to_account_info().clone(),
                remover_acc_a.to_account_info().clone(),
                token_acc_a.to_account_info().clone(),
                token_program.to_account_info().clone(),
            ],
            &[&[&PREFIX.as_bytes().as_ref(), &[_nonce]]],
        )?;
        //-------- Withdraw Token B POOL PDA -> Remover -----------------------------
        let withdraw_ix_b = spl_token::instruction::transfer(
            token_program.key,
            token_acc_b.to_account_info().key,
            remover_acc_b.to_account_info().key,
            &pda,
            &[&pda],
            sendb_amount
        )?;
        invoke_signed(
            &withdraw_ix_b,
            &[
                pool_pda.to_account_info().clone(),
                remover_acc_b.to_account_info().clone(),
                token_acc_b.to_account_info().clone(),
                token_program.to_account_info().clone(),
            ],
            &[&[&PREFIX.as_bytes().as_ref(), &[_nonce]]],
        )?;

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

        let mut transfer_amount = (quote_total/dest_price) as u64 * 955 / 1000; // fee 0.5%

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
    InvalidOwner,
    #[msg("Init Liquidity Step Error")]
    InitLiquidityStepError,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("error -> LpToken is wrong.")]
    LpTokenError,
    #[msg("error -> LpTokenAccount is wrong.")]
    LpTokenAccountError,
    #[msg("error -> Token is wrong.")]
    TokenError,
    #[msg("error -> TokenAccount is wrong&.")]
    TokenAccountError,
    #[msg("error -> Please finish to create pool.")]
    AddLiquidityStepError,
    #[msg("error -> Amount or Amp is zero.")]
    AmountZeroError,
}
