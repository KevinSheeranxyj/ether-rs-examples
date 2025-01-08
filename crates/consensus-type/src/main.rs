
use alloy::primitives::U256;
use eyre::Result;

fn main() -> Result<()> {

    let a = U256::from(1000_u32);
    let b = U256::from(1000_u32);

    assert!(a == b);


    Ok(())
}
