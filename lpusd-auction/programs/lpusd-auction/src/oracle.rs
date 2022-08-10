use anchor_lang::prelude::*;
use pyth_sdk_solana::{load_price_feed_from_account_info, Price, PriceFeed };

use std::str::FromStr;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


// Dev-net
pub const PYTH_USDC_ADDRESS: &str = "5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7";
pub const PYTH_SOL_ADDRESS: &str = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";
pub const PYTH_RAY_ADDRESS: &str = "EhgAdTrgxi4ZoVZLQx1n93vULucPpiFi2BQtz9RJr1y6";
pub const PYTH_MSOL_ADDRESS: &str = "9a6RNx3tCu1TSs6TBSfV2XRXEPEZXQ6WB7jRojZRvyeZ";
pub const PYTH_STSOL_ADDRESS: &str = "2LwhbcswZekofMNRtDRMukZJNSRUiKYMFbqtBwqjDfke";
pub const PYTH_SCNSOL_ADDRESS: &str = "HoDAYYYhFvCNQNFPui51H8qvpcdz6KuVtq77ZGtHND2T";
pub const PYTH_SRM_ADDRESS: &str = "992moaMQKs32GKZ9dxi8keyM2bUmbrwBZpK4p2K6X5Vs";

// Mainnet-beta
// pub const PYTH_USDC_ADDRESS: &str = "Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD";
// pub const PYTH_SOL_ADDRESS: &str = "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG";
// pub const PYTH_RAY_ADDRESS: &str = "AnLf8tVYCM816gmBjiy8n53eXKKEDydT5piYjjQDPgTB";
// pub const PYTH_MSOL_ADDRESS: &str = "E4v1BBgoso9s64TQvmyownAVJbhbEPGyzA3qn4n46qj9";
// pub const PYTH_STSOL_ADDRESS: &str = "9mpaSy5ocwPvoaxWZc4S8MhUUeUKmmFqymBJTfY6CJ6o";
// pub const PYTH_SCNSOL_ADDRESS: &str = "25yGzWV5okF7aLivSCE4xnjVUPowQcThhhx5Q2fgFhkm";
// pub const PYTH_SRM_ADDRESS: &str = "3NBReDRTLKMQEKiLD5tGcx4kXbTf88b7f2xLS9UuGjym";


pub fn get_price(
    pyth_account: &AccountInfo
) -> Result<i64> {
    let price_feed: PriceFeed = load_price_feed_from_account_info( pyth_account ).unwrap();
    let current_price: Option<Price> = price_feed.get_current_price();
    if current_price != None {
        let price_data: Price = current_price.unwrap();
        Ok(price_data.price)
    } 
    // In case of None, need to use switchboard oracle, For now, here we use the risk method
    else {
        let mut _prev_price: Price;
        let mut _timestamp: i64;
        let (prev_price, timestamp) = price_feed.get_prev_price_unchecked();

        if timestamp == 0 {
            let unchecked_price: Price = price_feed.get_current_price_unchecked();

            Ok(unchecked_price.price)
        } else {
            Ok(prev_price.price)
        }
    }    
    
}

// Check only one
pub fn check_pyth_account (
    pyth_address: &str,
    pyth_account: &AccountInfo,
) -> Result<bool> {

    let pyth_address = Pubkey::from_str(pyth_address).unwrap();
    if pyth_account.key() != pyth_address {
        Ok(false)
    } else {
        Ok(true)
    }
}
pub fn check_pyth_accounts(    
    pyth_sol_account: &AccountInfo,
    pyth_ray_account: &AccountInfo,
    pyth_msol_account: &AccountInfo,
    pyth_stsol_account: &AccountInfo,
    pyth_scnsol_account: &AccountInfo,
    pyth_srm_account: &AccountInfo
) -> Result<bool> {

    let pyth_sol = Pubkey::from_str(PYTH_SOL_ADDRESS).unwrap();
    let pyth_ray = Pubkey::from_str(PYTH_RAY_ADDRESS).unwrap();
    let pyth_msol = Pubkey::from_str(PYTH_MSOL_ADDRESS).unwrap();
    let pyth_stsol = Pubkey::from_str(PYTH_STSOL_ADDRESS).unwrap();
    let pyth_scnsol = Pubkey::from_str(PYTH_SCNSOL_ADDRESS).unwrap();
    let pyth_srm = Pubkey::from_str(PYTH_SRM_ADDRESS).unwrap();

    let mut _result: bool = true;

    if pyth_sol_account.key() != pyth_sol {
        _result = false;
    }

    if pyth_ray_account.key() != pyth_ray {
        _result = false;
    }

    if pyth_msol_account.key() != pyth_msol {
        _result = false;
    }

    if pyth_stsol_account.key() != pyth_stsol {
        _result = false;
    }

    if pyth_scnsol_account.key() != pyth_scnsol {
        _result = false;
    }

    if pyth_srm_account.key() != pyth_srm {
        _result = false;
    }

    Ok(_result)
}
