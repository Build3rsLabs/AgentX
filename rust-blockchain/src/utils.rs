use crate::error::{AppError, AppResult};
use num_bigint::BigUint;
use std::str::FromStr;

// Convert EGLD amount to smallest denomination (10^18)
pub fn egld_to_denomination(amount: f64) -> AppResult<String> {
    // 1 EGLD = 10^18 smallest units
    let denomination = 1_000_000_000_000_000_000u128;
    
    // Convert to smallest units
    let amount_denomination = (amount * denomination as f64) as u128;
    
    Ok(amount_denomination.to_string())
}

// Convert smallest denomination to EGLD
pub fn denomination_to_egld(amount: &str) -> AppResult<f64> {
    // 1 EGLD = 10^18 smallest units
    let denomination = 1_000_000_000_000_000_000u128;
    
    // Parse the amount
    let amount_value = u128::from_str(amount).map_err(|e| {
        AppError::Validation(format!("Invalid amount format: {}", e))
    })?;
    
    // Convert to EGLD
    let egld_amount = amount_value as f64 / denomination as f64;
    
    Ok(egld_amount)
}

// Format a BigUint as a human-readable EGLD amount
pub fn format_egld_amount(amount: &BigUint) -> String {
    let denomination = BigUint::from(1_000_000_000_000_000_000u64);
    
    // Integer division to get the whole EGLD part
    let egld_whole = amount.clone() / denomination.clone();
    
    // Modulo to get the fractional part
    let egld_fraction = amount % denomination.clone();
    
    // Format with 4 decimal places
    let fraction_str = format!("{:018}", egld_fraction);
    let fraction_formatted = &fraction_str[0..4];
    
    format!("{}.{}", egld_whole, fraction_formatted)
}

// Generate a random nonce for authentication
pub fn generate_nonce() -> String {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let nonce: u64 = rng.gen();
    format!("{:016x}", nonce)
}