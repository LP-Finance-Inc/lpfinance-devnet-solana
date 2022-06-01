use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Burn, Token, TokenAccount }
};

declare_id!("3QTW9aZp4U2xoj9UfvTF6PEL3UZzfEHi8UtNruhw7GHL");

const PREFIX: &str = "test-tokens";
const TOKEN_DECIMALS: u8 = 9;

#[program]
pub mod test_tokens {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("INITIALIZE TOKEN PROGRAM");

        let state_account = &mut ctx.accounts.state_account;
        let config = &mut ctx.accounts.config;

        state_account.owner = ctx.accounts.authority.key();

        config.wsol_mint = ctx.accounts.wsol_mint.key();
        config.msol_mint = ctx.accounts.msol_mint.key();
        config.stsol_mint = ctx.accounts.stsol_mint.key();
        config.scnsol_mint = ctx.accounts.scnsol_mint.key();
        config.usdc_mint = ctx.accounts.usdc_mint.key();
        config.btc_mint = ctx.accounts.btc_mint.key();
        config.eth_mint = ctx.accounts.eth_mint.key();
        config.ray_mint = ctx.accounts.ray_mint.key();
        config.srm_mint = ctx.accounts.srm_mint.key();
        config.avax_mint = ctx.accounts.avax_mint.key();
        config.fida_mint = ctx.accounts.fida_mint.key();
        config.ftt_mint = ctx.accounts.ftt_mint.key();
        config.ftm_mint = ctx.accounts.ftm_mint.key();
        config.gmt_mint = ctx.accounts.gmt_mint.key();
        config.luna_mint = ctx.accounts.luna_mint.key();
        config.matic_mint = ctx.accounts.matic_mint.key();
        config.usdt_mint = ctx.accounts.usdt_mint.key();

        config.state_account = ctx.accounts.state_account.key();

        Ok(())
    }

    pub fn burn_token(
        ctx: Context<BurnToken>,
        amount: u64
    ) -> Result<()> {
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        let cpi_accounts = Burn {
            mint: ctx.accounts.token_mint.to_account_info(),
            from: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, amount)?;
        Ok(())
    }

    // Faucet MINT
    pub fn mint_token(
        ctx: Context<MintToken>,
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
pub struct Initialize<'info> {
    // Token program owner
    #[account(mut)]
    pub authority: Signer<'info>,
    // State Accounts
    #[account(init,
        seeds = [PREFIX.as_bytes()],
        bump,
        space = TokenStateAccount::LEN + 8,
        payer = authority
    )]
    pub state_account: Box<Account<'info, TokenStateAccount>>,

    // Config Accounts
    #[account(init,
        space = Config::LEN + 8,        
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

#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub state_account: Box<Account<'info, TokenStateAccount>>,
    #[account(
        mut,
        constraint = user_token.mint == token_mint.key(),
        constraint = user_token.owner == owner.key()
    )]
    pub user_token: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

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
impl TokenStateAccount {
    pub const LEN: usize = 2 * 32;
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

impl Config {
    pub const LEN: usize = 18 * 32;
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
