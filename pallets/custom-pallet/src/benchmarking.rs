#![cfg(feature = "runtime-benchmarks")]

use super::{Pallet as CustomPallet, *};
use frame::deps::frame_support::assert_ok;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {

    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as CustomPallet;
    use frame_system::RawOrigin;

    
    #[benchmark]
    fn set_counter_value() {
        #[extrinsic_call]
        set_counter_value(RawOrigin::Root, 5);

        assert_eq!(CounterValue::<T>::get(), Some(5u32.into()));
    }

    #[benchmark]
    fn increment() {
        //setup
        let caller: T::AccountId = whitelisted_caller();

        assert_ok!(CustomPallet::<T>::set_counter_value(
            RawOrigin::Root.into(),
            5u32
        ));

        //call extrinsic
        #[extrinsic_call]
        increment(RawOrigin::Signed(caller.clone()), 1);

        //verification
        assert_eq!(CounterValue::<T>::get(), Some(6u32.into()));
        assert_eq!(UserInteractions::<T>::get(caller), 1u32.into());
    }

    #[benchmark]
    fn decrement() {
        let caller: T::AccountId = whitelisted_caller();

        assert_ok!(CustomPallet::<T>::set_counter_value(
            RawOrigin::Root.into(),
            5u32
        ));

        #[extrinsic_call]
        decrement(RawOrigin::Signed(caller.clone()), 1);

        assert_eq!(CounterValue::<T>::get(), Some(4u32.into()));
        assert_eq!(UserInteractions::<T>::get(caller), 1u32.into());
    }

    // impl_benchmark_test_suite! 是一个宏，用于为自定义pallet创建基准测试套件
    // 它接受三个参数：
    // 1. CustomPallet: 要测试的pallet类型
    // 2. crate::mock::new_test_ext(): 创建测试环境的函数
    // 3. crate::mock::Test: 测试运行时类型
    // 这个宏会自动生成测试用例来验证基准测试的正确性
    impl_benchmark_test_suite!(CustomPallet, crate::mock::new_test_ext(), crate::mock::Test);
}