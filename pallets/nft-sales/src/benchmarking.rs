#![cfg(feature = "runtime-benchmarks")]
use crate::{self as pallet_nft_sales, *};
use common_types::CurrencyId;

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::assert_ok;
use frame_support::traits::tokens::nonfungibles::{Create, Inspect, Mutate};
use frame_system::RawOrigin;

// pub struct Pallet<T: Config>(pallet_nft_sales::Pallet<T>);
//
// pub trait Config:
// pallet_nft_sales::Config<ClassId = <Self as pallet_uniques::Config>::ClassId>
// // + pallet_balances::Config
// + pallet_uniques::Config
// {
// }

benchmarks! {
	where_clause {
		where
		T: Config + pallet_uniques::Config<ClassId = <T as Config>::ClassId>,
		<T as pallet_nft_sales::Config>::ClassId: From<u64>,
		// <T as pallet_balances::Config>::Balance: From<u128>,
		// <T as LoanConfig>::Rate: From<Rate>,
		// <T as LoanConfig>::Amount: From<Amount>,
		// <T as ORMLConfig>::Balance: From<u128>,
		// <T as ORMLConfig>::CurrencyId: From<CurrencyId>,
		// <T as TimestampConfig>::Moment: From<u64> + Into<u64>,
		// <T as pallet_nft_sales::Config>::Balance: From<u128>,
		<<T as pallet_nft_sales::Config>::Fungibles as fungibles::Inspect<AccountIdOf<T>>>::AssetId: From<CurrencyId>,
				<<T as pallet_nft_sales::Config>::Fungibles as fungibles::Inspect<AccountIdOf<T>>>::Balance: From<u128>,

		// <T as pallet_nft_sales::Config>::TrancheId: From<u8>,
		// <T as pallet_nft_sales::Config>::EpochId: From<u32>,
		// <T as pallet_nft_sales::Config>::PoolId: Into<u64> + IsType<PoolIdOf<T>>,
	}

	// Add an NFT for sale
	add {
		let seller_account = account::<T::AccountId>("seller", 0, 0);
		let seller_origin: RawOrigin<T::AccountId> = RawOrigin::Signed(seller_account.clone()).into();

		let class_id: <T as pallet_nft_sales::Config>::ClassId = 0u64.into();
		let instance_id: <T as pallet_nft_sales::Config>::InstanceId = 1.into();

		let clazz = create_nft_class::<T>(1, seller_account.clone());
		//
		// let class_id:  <T as pallet_nft_sales::Config>::ClassId = create_nft_class::<T>(0, seller_account.clone());
		// let instance_id: <T as pallet_nft_sales::Config>::InstanceId = 1.into();

		let price = Price { currency: CurrencyId::Usd.into(), amount: 10_000u128.into() };

		// We need the NFT to exist in pallet-uniques before we can put it for sale
		// Mint the nft in the uniques pallet
		// assert_ok!(pallet_uniques::mint(owner.clone(), class_id, instance_id, seller_account.clone()));

		// <pallet_uniques::Pallet<T> as Create<T::AccountId>>::create_class(
		// &class_id,
		// &seller_account,
		// &seller_account,
		// )
		// .expect("class creation should not fail");

	}: _(seller_origin, class_id, instance_id, price)
	verify {
		assert!(<Sales<T>>::contains_key(class_id, instance_id), "NFT should be for sale now");
	}

	// // Remove an NFT from sale
	// remove {
	// 	let seller_account = account::<T::AccountId>("seller", 0, 0);
	// 	let seller_origin = RawOrigin::Signed(seller_account.clone()).into();
	// 	let class_id = 0;
	// 	let instance_id = 1;
	// 	let price = Price { currency: CurrencyId::Usd, amount: 10_000 };
	//
	// 	// We need the nft in the storage beforehand to be able to remove it
	// 	<Sales<T>>::insert(class_id.clone(), instance_id.clone(), Sale { seller: seller_account, price});
	// }: remove(seller_origin, class_id, instance_id)
	// verify {
	// 	assert!(<Sales<T>>::get(class_id, instance_id).is_none(), "The NFT should have been removed from sale");
	// }
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);

pub(crate) fn nft_id<T>() -> <T as pallet_nft_sales::Config>::ClassId
where
	T: frame_system::Config
		// + pallet_nft_sales::Config<ClassId = <T as pallet_uniques::Config>::ClassId>
		+ pallet_nft_sales::Config,
	<T as pallet_nft_sales::Config>::ClassId: From<u64>,
{
	0u64.into()
}

pub(crate) fn create_nft_class<T>(
	class_id: u64,
	owner: T::AccountId,
) -> <T as pallet_nft_sales::Config>::ClassId
where
	T: frame_system::Config
		+ pallet_nft_sales::Config<ClassId = <T as pallet_uniques::Config>::ClassId>
		+ pallet_uniques::Config,
	<T as pallet_uniques::Config>::ClassId: From<u64>,
{
	// Create class. Shouldn't fail.
	let uniques_class_id: <T as pallet_uniques::Config>::ClassId = class_id.into();
	<pallet_uniques::Pallet<T> as Create<T::AccountId>>::create_class(
		&uniques_class_id,
		&owner,
		&owner,
	)
	.expect("class creation should not fail");
	uniques_class_id
}
