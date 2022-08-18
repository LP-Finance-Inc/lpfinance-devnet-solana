use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };
use anchor_spl::associated_token::AssociatedToken;

use std::mem::size_of;

pub const PREFIX_ESCROW: &str = "swap-escrow";

#[derive(Accounts)]
pub struct CreateTokenATA<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    //--- LpUSD
    #[account(mut)]
    pub token_src: Box<Account<'info, Mint>>,
    #[account(        
        init_if_needed,
        payer = user,
        associated_token::mint = token_src,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpusd: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapStableswap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<SwapEscrow>(),
        payer = user
    )]
    pub swap_escrow: Account<'info, SwapEscrow>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,

    #[account(mut)]
    pub token_src: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_dest: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_src,
        associated_token::authority = user,
    )]    
    pub user_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_dest,
        associated_token::authority = user,
    )]    
    pub user_ata_dest: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_src,
        associated_token::authority = stable_swap_pool,
    )]
    pub pool_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_dest,
        associated_token::authority = stable_swap_pool,
    )]
    pub pool_ata_dest: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_src,
        associated_token::authority = swap_escrow,
    )]    
    pub escrow_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_dest,
        associated_token::authority = swap_escrow,
    )]    
    pub escrow_ata_dest: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapUniswap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<SwapEscrow>(),
        payer = user
    )]
    pub swap_escrow: Account<'info, SwapEscrow>,
    /// CHECK:
    #[account(mut)]
    pub uniswap_pool: AccountInfo<'info>,

    #[account(mut)]
    pub token_src: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_dest: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_src,
        associated_token::authority = user,
    )]    
    pub user_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_dest,
        associated_token::authority = user,
    )]    
    pub user_ata_dest: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_src,
        associated_token::authority = uniswap_pool,
    )]
    pub pool_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_dest,
        associated_token::authority = uniswap_pool,
    )]
    pub pool_ata_dest: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_src,
        associated_token::authority = swap_escrow,
    )]    
    pub escrow_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_dest,
        associated_token::authority = swap_escrow,
    )]    
    pub escrow_ata_dest: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub uniswap_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapPyth<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_src: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_dest: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_src: AccountInfo<'info>,
    /// CHECK:
    pub pyth_dest: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_src,
        associated_token::authority = user,
    )]    
    pub user_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_dest,
        associated_token::authority = user,
    )]    
    pub user_ata_dest: Box<Account<'info, TokenAccount>>,

    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpusdToLpfi<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub uniswap_pool: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpusd: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_lpfi: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = user,
    )]
    pub user_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpfi,
        associated_token::authority = user,
    )]    
    pub user_ata_lpfi: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub uniswap_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpfiToLpusd<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub uniswap_pool: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpusd: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_lpfi: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpusd,
        associated_token::authority = user,
    )]
    pub user_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = user,
    )]    
    pub user_ata_lpfi: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub uniswap_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct SwapLpusdToLpsolStep1<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    #[account(
        init_if_needed, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<SwapEscrow>(),
        payer = user
    )]
    pub swap_escrow: Box<Account<'info, SwapEscrow>>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpusd: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_usdc: AccountInfo<'info>,
    /// CHECK:
    pub pyth_wsol: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = user,
    )]
    pub user_ata_lpusd: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_wsol: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpusdToLpsolStep2<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    #[account(
        mut, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump
    )]
    pub swap_escrow: Box<Account<'info, SwapEscrow>>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpsol,
        associated_token::authority = user,
    )]
    pub user_ata_lpsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_wsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_wsol: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpsolToLpusdStep1<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    #[account(
        init_if_needed, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<SwapEscrow>(),
        payer = user
    )]
    pub swap_escrow: Box<Account<'info, SwapEscrow>>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_usdc: AccountInfo<'info>,
    /// CHECK:
    pub pyth_wsol: AccountInfo<'info>,
    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpsol,
        associated_token::authority = user,
    )]
    pub user_ata_lpsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_wsol: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_wsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_wsol: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpsolToLpusdStep2<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    #[account(
        init_if_needed, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<SwapEscrow>(),
        payer = user
    )]
    pub swap_escrow: Box<Account<'info, SwapEscrow>>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpusd: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpusd,
        associated_token::authority = user,
    )]
    pub user_ata_lpusd: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpusd,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpusdToNormal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpusd: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_dest: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_usdc: AccountInfo<'info>,
    /// CHECK:
    pub pyth_dest: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = user,
    )]
    pub user_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_dest,
        associated_token::authority = user,
    )]    
    pub user_ata_dest: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapNormalToLpusd<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_src: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_lpusd: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_src: AccountInfo<'info>,
    /// CHECK:
    pub pyth_usdc: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = token_src,
        associated_token::authority = user,
    )]    
    pub user_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpusd,
        associated_token::authority = user,
    )]
    pub user_ata_lpusd: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        associated_token::mint = token_lpusd,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpusd: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpsolToNormal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_dest: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_wsol: AccountInfo<'info>,
    /// CHECK:
    pub pyth_dest: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = user,
    )]
    pub user_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_dest,
        associated_token::authority = user,
    )]    
    pub user_ata_dest: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_wsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_wsol: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapNormalToLpsol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_src: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_lpsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_src: AccountInfo<'info>,
    /// CHECK:
    pub pyth_wsol: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = token_src,
        associated_token::authority = user,
    )]    
    pub user_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpsol,
        associated_token::authority = user,
    )]
    pub user_ata_lpsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_wsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_wsol: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpfiToNormal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub uniswap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpfi: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_dest: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_usdc: AccountInfo<'info>,
    /// CHECK:
    pub pyth_dest: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = user,
    )]
    pub user_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_dest,
        associated_token::authority = user,
    )]    
    pub user_ata_dest: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub uniswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapNormalToLpfi<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub uniswap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_src: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_lpfi: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_src: AccountInfo<'info>,
    /// CHECK:
    pub pyth_usdc: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = token_src,
        associated_token::authority = user,
    )]    
    pub user_ata_src: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpfi,
        associated_token::authority = user,
    )]
    pub user_ata_lpfi: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub uniswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpsolToLpfiStep1<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    #[account(
        init_if_needed, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<SwapEscrow>(),
        payer = user
    )]
    pub swap_escrow: Box<Account<'info, SwapEscrow>>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_usdc: AccountInfo<'info>,
    /// CHECK:
    pub pyth_wsol: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = user,
    )]
    pub user_ata_lpsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_wsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_wsol: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpsolToLpfiStep2<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    #[account(
        mut, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump
    )]
    pub swap_escrow: Box<Account<'info, SwapEscrow>>,
    /// CHECK:
    #[account(mut)]
    pub uniswap_pool: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpfi: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpfi,
        associated_token::authority = user,
    )]
    pub user_ata_lpfi: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub uniswap_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpfiToLpsolStep1<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    #[account(
        init_if_needed, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<SwapEscrow>(),
        payer = user
    )]
    pub swap_escrow: Box<Account<'info, SwapEscrow>>,
    /// CHECK:
    #[account(mut)]
    pub uniswap_pool: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_state_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpfi: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_usdc: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,

    /// CHECK:
    pub pyth_usdc: AccountInfo<'info>,
    /// CHECK:
    pub pyth_wsol: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = user,
    )]
    pub user_ata_lpfi: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = uniswap_pool,
    )]
    pub uniswap_pool_ata_usdc: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpfi,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpfi: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_usdc,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_usdc: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_wsol: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub uniswap_program: AccountInfo<'info>,
    /// CHECK:
    pub testtokens_program: AccountInfo<'info>,    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapLpfiToLpsolStep2<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut,seeds = [PREFIX_ESCROW.as_ref()], bump)]
    pub swap_pda: AccountInfo<'info>,
    #[account(
        init_if_needed, 
        seeds = [
            PREFIX_ESCROW.as_bytes(),
            user.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<SwapEscrow>(),
        payer = user
    )]
    pub swap_escrow: Box<Account<'info, SwapEscrow>>,
    /// CHECK:
    #[account(mut)]
    pub stable_swap_pool: AccountInfo<'info>,

    #[account(mut)]
    pub token_lpsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub token_wsol: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_lpsol,
        associated_token::authority = user,
    )]
    pub user_ata_lpsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = stable_swap_pool,
    )]
    pub stableswap_pool_ata_wsol: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_lpsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_lpsol: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_wsol,
        associated_token::authority = swap_pda,
    )]    
    pub escrow_ata_wsol: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:
    pub stableswap_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct SwapEscrow {
    pub amount_usdc: u64,
    pub amount_wsol: u64,
}
