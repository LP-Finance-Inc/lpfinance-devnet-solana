use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount }
};
use std::mem::size_of;
use std::str::FromStr;

declare_id!("GdKm4s951GZDBSkfkJhFTbrjZGqNLFB55uhNgK82hK8U");

const PREFIX: &str = "test-tokens";
const TOKEN_DECIMALS: u8 = 9;

const MAX_USDC_FAUCET_LIMIT: u64 = 1000;

// PROTOCOL
pub const CBS_PDA: &str = "9SYSA3RPEakev2i9GNDBLD5NGNELnVYKchWkWRrK1J6B";
pub const AUCTION_PDA: &str = "6nqS5uUaXYUncgn81iEntgQafkQwMagSYZXUCQeqZHJd";
// SWAP
pub const SWAP_PDA1: &str = "6VBUBPA2Bev3dZTEJwfSVBJpCWv6sw9eoyywTS3cXmu3"; // LpSOL - wSOL pda
pub const SWAP_PDA2: &str = "BFteZ5EXKa4myspKtvKcD7DNkQaLFrwEVpvMDaGwbeTZ"; // LpUSD - USDC pda
pub const SWAP_PDA3: &str = "4Y2vLmpLtfo5gxvGhAK68RtKruQdm8vEvAWJVLicQhmf"; // SWAP-ROUTER


#[program]
pub mod test_tokens {
    use super::*;

    pub fn create_token1(ctx: Context<CreateToken1>) -> Result<()> {
        msg!("INITIALIZE TOKEN PROGRAM");

        let state_account = &mut ctx.accounts.state_account;
        let config = &mut ctx.accounts.config;

        state_account.owner = ctx.accounts.authority.key();

        config.wsol_mint = ctx.accounts.wsol_mint.key();
        config.msol_mint = ctx.accounts.msol_mint.key();
        config.stsol_mint = ctx.accounts.stsol_mint.key();
        config.scnsol_mint = ctx.accounts.scnsol_mint.key();
        config.usdc_mint = ctx.accounts.usdc_mint.key();

        config.state_account = ctx.accounts.state_account.key();

        Ok(())
    }

    pub fn create_token2(ctx: Context<CreateToken2>) -> Result<()> {
        msg!("CREATE TOKENs 2");

        let config = &mut ctx.accounts.config;

        config.btc_mint = ctx.accounts.btc_mint.key();
        config.eth_mint = ctx.accounts.eth_mint.key();
        config.ray_mint = ctx.accounts.ray_mint.key();
        config.srm_mint = ctx.accounts.srm_mint.key();
        config.avax_mint = ctx.accounts.avax_mint.key();
        config.fida_mint = ctx.accounts.fida_mint.key();

        config.state_account = ctx.accounts.state_account.key();

        Ok(())
    }
    pub fn create_token3(ctx: Context<CreateToken3>) -> Result<()> {
        msg!("CREATE TOKENs 3");
        let config = &mut ctx.accounts.config;

        config.ftt_mint = ctx.accounts.ftt_mint.key();
        config.ftm_mint = ctx.accounts.ftm_mint.key();
        config.gmt_mint = ctx.accounts.gmt_mint.key();
        config.luna_mint = ctx.accounts.luna_mint.key();
        config.matic_mint = ctx.accounts.matic_mint.key();
        config.usdt_mint = ctx.accounts.usdt_mint.key();
        Ok(())
    }
    // PROXY MINT
    pub fn mint_token(
        ctx: Context<MintToken>,
        amount: u64
    ) -> Result<()> {
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        let cbs_pubkey = Pubkey::from_str(CBS_PDA).unwrap();
        let auction_pubkey = Pubkey::from_str(AUCTION_PDA).unwrap();
        let swap1_pubkey = Pubkey::from_str(SWAP_PDA1).unwrap();
        let swap2_pubkey = Pubkey::from_str(SWAP_PDA2).unwrap();
        let swap3_pubkey = Pubkey::from_str(SWAP_PDA3).unwrap();

        let owner: Pubkey = ctx.accounts.owner.key();
        if cbs_pubkey != owner &&
            auction_pubkey != owner &&
            swap1_pubkey != owner &&
            swap2_pubkey != owner &&
            swap3_pubkey != owner
        {
            return Err(ErrorCode::InvalidOwner.into());
        }

        let (mint_token_authority, mint_token_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if mint_token_authority != ctx.accounts.state_account.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // Mint
        let seeds = &[
            PREFIX.as_bytes(),
            &[mint_token_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.state_account.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, amount)?;

        Ok(())
    }

    // Faucet MINT
    pub fn faucet_usdc(
        ctx: Context<FaucetUSDC>,
        amount: u64
    ) -> Result<()> {
        if amount == 0 || amount > MAX_USDC_FAUCET_LIMIT * 1000000000 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        let (mint_token_authority, mint_token_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);
    
        if mint_token_authority != ctx.accounts.state_account.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // Mint
        let seeds = &[
            PREFIX.as_bytes(),
            &[mint_token_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.state_account.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, amount)?;

        Ok(())
    }

    // Owner can mint token
    pub fn owner_mint_token(
        ctx: Context<OwnerMintToken>,
        amount: u64
    ) -> Result<()> {
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        let (mint_token_authority, mint_token_authority_bump) = 
        Pubkey::find_program_address(&[PREFIX.as_bytes()], ctx.program_id);

        if mint_token_authority != ctx.accounts.state_account.key() {
            return Err(ErrorCode::InvalidOwner.into());
        }

        // Mint
        let seeds = &[
            PREFIX.as_bytes(),
            &[mint_token_authority_bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.state_account.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn update_owner(
        ctx: Context<UpdateConfigAccount>,
        new_owner: Pubkey
    ) -> Result<()> {
        let state_account = &mut ctx.accounts.state_account;
        if state_account.owner != ctx.accounts.owner.key() || ctx.accounts.owner.key() == new_owner {
            return Err(ErrorCode::InvalidOwner.into());
        }

        state_account.owner = new_owner;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateToken1<'info> {
    // Token program owner
    #[account(mut)]
    pub authority: Signer<'info>,
    // State Accounts
    #[account(init,
        seeds = [PREFIX.as_bytes()],
        bump,
        space = size_of::<TokenStateAccount>() + 8,
        payer = authority
    )]
    pub state_account: Box<Account<'info, TokenStateAccount>>,

    // Config Accounts
    #[account(init,
        space = size_of::<Config>() + 8,        
        payer = authority
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"wsol_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub wsol_mint: Box<Account<'info, Mint>>,  

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"msol_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub msol_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"stsol_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub stsol_mint: Box<Account<'info, Mint>>,
    
    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"scnsol_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub scnsol_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"usdc_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct CreateToken2<'info> {
    // Token program owner
    #[account(mut)]
    pub authority: Signer<'info>,
    // State Accounts
    #[account(mut, constraint=state_account.owner == authority.key())]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    
    // Config Accounts
    #[account(mut, has_one=state_account)]
    pub config: Box<Account<'info, Config>>,
    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"btc_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub btc_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"eth_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub eth_mint: Box<Account<'info, Mint>>,


    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"ray_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub ray_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"srm_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub srm_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"avax_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub avax_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"fida_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub fida_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct CreateToken3<'info> {
    // Token program owner
    #[account(mut)]
    pub authority: Signer<'info>,
    // State Accounts
    #[account(mut, constraint=state_account.owner == authority.key())]
    pub state_account: Box<Account<'info, TokenStateAccount>>,    
    // Config Accounts
    #[account(mut, has_one=state_account)]
    pub config: Box<Account<'info, Config>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"ftt_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub ftt_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"ftm_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub ftm_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"gmt_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub gmt_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"luna_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub luna_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"matic_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub matic_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = state_account,
        seeds = [PREFIX.as_bytes(), b"usdt_mint".as_ref()],
        bump,
        payer = authority
    )]
    pub usdt_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

// Proxy mintable
#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = owner
    )]
    pub user_token: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}


#[derive(Accounts)]
pub struct FaucetUSDC<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = owner
    )]
    pub user_token: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint=config.usdc_mint == token_mint.key()
    )]
    pub token_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub config: Box<Account<'info, Config>>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

// Program deployer mintable
#[derive(Accounts)]
pub struct OwnerMintToken<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one=owner)]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = owner
    )]
    pub user_token: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct UpdateConfigAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner)]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    #[account(mut, has_one = state_account)]
    pub config: Box<Account<'info, Config>>
}

#[account]
#[derive(Default)]
pub struct TokenStateAccount {
    pub owner: Pubkey,              // 32
    pub second_owner: Pubkey        // 32
}

#[account]
#[derive(Default)]
pub struct Config {
    pub state_account: Pubkey,  // 32
    
    pub wsol_mint: Pubkey,      // 32
    pub msol_mint: Pubkey,      // 32
    pub stsol_mint: Pubkey,     // 32
    pub scnsol_mint: Pubkey,    // 32
    pub usdc_mint: Pubkey,      // 32
    pub btc_mint: Pubkey,       // 32
    pub eth_mint: Pubkey,       // 32
    pub ray_mint: Pubkey,       // 32
    pub srm_mint: Pubkey,       // 32
    pub avax_mint: Pubkey,      // 32
    pub fida_mint: Pubkey,      // 32
    pub ftt_mint: Pubkey,       // 32
    pub ftm_mint: Pubkey,       // 32
    pub gmt_mint: Pubkey,       // 32
    pub luna_mint: Pubkey,      // 32
    pub matic_mint: Pubkey,     // 32
    pub usdt_mint: Pubkey,      // 32
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Invalid Owner")]
    InvalidOwner,
    #[msg("Too often mint")]
    TooOftenMint
}
