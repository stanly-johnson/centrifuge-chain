// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use crate::utils::setup::*;
use codec::{Decode, Encode};
use common_traits::Permissions;
use common_types::PoolRole;
use frame_support::dispatch::DispatchResult;
use pallet_pools::{Error as PoolError, TrancheInput};
use runtime_common::{AccountId, Balance, PoolId, TrancheId, CFG as CURRENCY};
use sp_io::hashing::blake2_256;
use sp_runtime::{traits::One, FixedPointNumber, Perquintill};

pub fn invest_close_and_collect(
	pool_id: PoolId,
	investments: Vec<(Origin, TrancheId, Balance)>,
) -> DispatchResult {
	for (who, tranche_id, investment) in investments.clone() {
		Pools::update_invest_order(who, pool_id, tranche_id, investment)?;
	}

	Pools::close_epoch(Origin::signed(get_admin()), pool_id)?;

	let epoch = pallet_pools::Pool::<Runtime>::try_get(pool_id)
		.map_err(|_| PoolError::<Runtime>::NoSuchPool)?
		.last_epoch_closed;

	for (who, tranche_id, _) in investments {
		Pools::collect(who, pool_id, tranche_id, epoch as u32)?;
	}

	Ok(())
}

/// Creates a pool with the following spec
///
/// * Pool id as given
/// * Admin = get_admin() account
/// * 5 Tranches
///     * 0: 3% APR, 25% Risk buffer, Seniority 5
///     * 1: 5% APR, 10% Risk buffer, Seniority 4
///     * 2: 7% APR, 5% Risk buffer, Seniority 2
///     * 3: 10% APR, 5% Risk buffer, Seniority 1
///     * 4: Junior Tranche
/// * Whitelistings
/// 	* accounts with index 0 - 9 for tranche with id 0
///  	* accounts with index 10 - 19 for tranche with id 1
/// 	* accounts with index 20 - 29 for tranche with id 2
/// 	* accounts with index 30 - 39 for tranche with id 3
/// 	* accounts with index 40 - 49 for tranche with id 4
/// * Currency: CurrencyId::USD,
/// * MaxReserve: 1000
pub fn create_default_pool(id: PoolId) -> DispatchResult {
	let year_rate = Rate::saturating_from_integer(SECONDS_PER_YEAR);

	let rates = vec![3, 5, 7, 10];
	let mut interest_rates = rates
		.into_iter()
		.map(|rate| Some(Rate::saturating_from_rational(rate, 100) / year_rate + One::one()))
		.collect::<Vec<Option<_>>>();
	interest_rates.push(None);

	let risk_buffs: Vec<u64> = vec![25, 10, 5, 5];
	let mut risk_buffs = risk_buffs
		.into_iter()
		.map(|buffs| Some(Perquintill::from_percent(buffs)))
		.collect::<Vec<Option<_>>>();
	risk_buffs.push(None);

	let seniority: Vec<u32> = vec![5, 4, 2, 1];
	let mut seniority: Vec<Option<u32>> = seniority.into_iter().map(|sen| Some(sen)).collect();
	seniority.push(None);

	let tranches = interest_rates
		.into_iter()
		.zip(risk_buffs)
		.zip(seniority)
		.rev()
		.map(|((rate, buff), seniority)| TrancheInput {
			interest_per_sec: rate,
			min_risk_buffer: buff,
			seniority: seniority,
		})
		.collect();

	Pools::create(
		into_signed(get_admin()),
		id,
		tranches,
		CurrencyId::Usd,
		1000 * CURRENCY,
	)
}

pub fn account(name: &'static str, index: u32, seed: u32) -> AccountId {
	let entropy = (name, index, seed).using_encoded(blake2_256);
	AccountId::decode(&mut &entropy[..]).unwrap()
}

pub fn permission_for<P>(who: AccountId, pool: PoolId, role: PoolRole)
where
	P: Permissions<AccountId, Location = PoolId, Role = PoolRole>,
{
	P::add_permission(pool, who, role).unwrap();
}

pub fn get_account(idx: u32) -> AccountId {
	account("user", idx, 0)
}

pub fn get_signed(idx: u32) -> Origin {
	into_signed(get_account(idx))
}

pub fn into_signed(account: AccountId) -> Origin {
	Origin::signed(account)
}

pub fn get_root() -> Origin {
	Origin::root()
}

pub fn get_admin() -> AccountId {
	account("admin", 0, 0)
}
