#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

benchmarks! {
	// Add an NFT for sale
	add {
		let seller_account = account::<T::AccountId>("seller", 0, 0);
		let seller_origin = RawOrigin::Signed(seller_account.clone()).into();
		let class_id = 0;
		let instance_id = 1;
		let price = Price { currency: CurrencyId::Usd, amount: 10_000 };

	}: _(seller_origin, class_id, instance_id, price)
	verify {
		assert!(<Sales<T>>::contains_key(class_id, instance_id), "NFT should be for sale now");
	}

	// Remove an NFT from sale
	remove {
		let seller_account = account::<T::AccountId>("seller", 0, 0);
		let seller_origin = RawOrigin::Signed(seller_account.clone()).into();
		let class_id = 0;
		let instance_id = 1;
		let price = Price { currency: CurrencyId::Usd, amount: 10_000 };

		// We need the nft in the storage beforehand to be able to remove it
		<Sales<T>>::insert(class_id.clone(), instance_id.clone(), Sale { seller: seller::clone(), price});
	}: remove(seller_origin, class_id, instance_id)
	verify {
		assert!(<Sales<T>>::get(class_id, instance_id).is_none(), "The NFT should have been removed from sale");
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
