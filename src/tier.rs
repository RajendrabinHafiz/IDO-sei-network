#[cfg(test)]
pub mod manual {
    use crate::state::Config;
    use cosmwasm_std::{StdResult, DepsMut};
    use std::sync::Mutex;

    static TIER: Mutex<u8> = Mutex::new(0);
    static MIN_TIER: Mutex<u8> = Mutex::new(4);

    pub fn set_tier(tier: u8) {
        let mut tier_lock = TIER.lock().unwrap();
        *tier_lock = tier;
    }

    pub fn set_min_tier(tier: u8) {
        let mut tier_lock = MIN_TIER.lock().unwrap();
        *tier_lock = tier;
    }

    pub fn get_tier(
        _deps: DepsMut,
        _address: String,
        _viewing_key: Option<String>,
    ) -> StdResult<u8> {
        let tier_lock = TIER.lock().unwrap();
        Ok(*tier_lock)
    }

    pub fn get_min_tier(
        _deps: DepsMut,
        _config: &Config,
    ) -> StdResult<u8> {
        let tier_lock = MIN_TIER.lock().unwrap();
        Ok(*tier_lock)
    }

    pub fn get_tier_from_nft_contract(
        _deps: DepsMut,
        _address: String,
        _config: &Config,
        _viewing_key: String,
    ) -> StdResult<Option<u8>> {
        let tier_lock = TIER.lock().unwrap();
        Ok(Some(*tier_lock))
    }
}

#[cfg(test)]
mod tests {
    use super::manual::{get_tier, set_tier};
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn manual_tier() {
        let deps = mock_dependencies();
        let address = String::from("address");
        let tier = get_tier(&deps, address.clone(), None).unwrap();

        for i in 1..=4 {
            set_tier(i);
            assert_eq!(get_tier(&deps, address.clone(), None), Ok(i));
        }
        set_tier(tier);
    }
}