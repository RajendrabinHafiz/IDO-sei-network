use crate::{
    msg::ContractStatus,
    state::{self, Config, Ido},
};
use cosmwasm_std::{
    Coin, DepsMut, StdError, StdResult, Deps,
};

pub fn assert_contract_active(deps: Deps) -> StdResult<()> {
    let config = Config::load(deps.storage)?;
    let active_status = ContractStatus::Active as u8;

    if config.status != active_status {
        return Err(StdError::generic_err("Contract is not active"));
    }

    Ok(())
}

pub fn assert_admin(
    deps: DepsMut,
    address: String,
) -> StdResult<()> {
    let canonical_admin = deps.api.addr_canonicalize(address.as_str())?;
    let config = Config::load(deps.storage)?;

    if config.admin != canonical_admin {
        return Err(StdError::unauthorized());
    }

    Ok(())
}

pub fn assert_ido_admin(
    deps: DepsMut,
    address: String,
    ido_id: u32,
) -> StdResult<()> {
    let canonical_admin = deps.api.addr_canonicalize(address.as_str())?;
    let ido = Ido::load(deps.storage, ido_id)?;

    if ido.admin != canonical_admin {
        return Err(StdError::unauthorized());
    }

    Ok(())
}

pub fn in_whitelist(
    deps: DepsMut,
    address: String,
    ido_id: u32,
) -> StdResult<bool> {
    let canonical_address = deps.api.addr_canonicalize(address.as_str())?;

    let ido_whitelist = state::ido_whitelist(ido_id);
    let whitelist_status = ido_whitelist.get(&deps.storage, &canonical_address);

    match whitelist_status {
        Some(value) => Ok(value),
        None => {
            let ido = Ido::load(&deps.storage, ido_id)?;
            Ok(ido.shared_whitelist)
        }
    }
}

pub fn sent_funds(coins: &[Coin]) -> StdResult<u128> {
    let mut amount: u128 = 0;

    for coin in coins {
        if coin.denom != "uscrt" {
            return Err(StdError::generic_err("Unsopported token"));
        }

        amount = amount.checked_add(coin.amount.u128()).unwrap();
    }

    Ok(amount)
}

#[cfg(test)]
mod tests {
    use crate::state::{self, Ido};
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn in_whitelist() {
        let mut deps = mock_dependencies();

        let mut ido = Ido::default();
        ido.shared_whitelist = false;
        ido.save(&mut deps.storage).unwrap();
        let ido_id = ido.id();

        let address = "address".to_string();
        let whitelisted = "whitelisted".to_string();
        let blacklisted = "blacklisted".to_string();
        let canonical_whitelisted = deps.api.addr_canonicalize(whitelisted.as_str())?;
        let canonical_blacklisted = deps.api.addr_canonicalize(blacklisted.as_str())?;

        let whitelist = state::ido_whitelist(ido_id);

        whitelist
            .insert(&mut deps.storage, &canonical_whitelisted, &true)
            .unwrap();
        whitelist
            .insert(&mut deps.storage, &canonical_blacklisted, &false)
            .unwrap();

        assert_eq!(super::in_whitelist(&deps, &address, ido_id), Ok(false));
        assert_eq!(super::in_whitelist(&deps, &whitelisted, ido_id), Ok(true));
        assert_eq!(super::in_whitelist(&deps, &blacklisted, ido_id), Ok(false));

        ido.shared_whitelist = true;
        ido.save(&mut deps.storage).unwrap();

        assert_eq!(super::in_whitelist(&deps, &address, ido_id), Ok(true));
        assert_eq!(super::in_whitelist(&deps, &whitelisted, ido_id), Ok(true));
        assert_eq!(super::in_whitelist(&deps, &blacklisted, ido_id), Ok(false));
    }
}
