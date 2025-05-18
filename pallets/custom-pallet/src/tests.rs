// 这种写法是Rust中常见的模块导入方式，具体解释如下：

// 1. `use crate::` 表示从当前crate（即当前项目）的根模块开始导入
// 2. `mock::*` 表示导入mock模块中的所有公共项
// 3. `Error, Event, UserInteractions` 表示从当前crate中导入特定的类型
// 4. `frame::deps::sp_runtime` 表示从frame库的deps模块中导入sp_runtime
// 5. `frame::testing_prelude::*` 表示从frame库的testing_prelude模块中导入所有公共项

// 这种写法的好处是：
// - 可以清晰地看到导入的来源
// - 避免命名冲突
// - 方便管理依赖
// - 提高代码可读性

use crate::{mock::*, Error, Event, UserInteractions};
use frame::deps::sp_runtime;
use frame::testing_prelude::*;

#[test]
fn it_works_for_increment() {
    //使用了匿名函数
    new_test_ext().execute_with(|| {
        //将系统区块高度设置为1
        System::set_block_number(1);
        //使用根权限来确保来确保成功设置计数值为0
        assert_ok!(CustomPallet::set_counter_value(RuntimeOrigin::root(), 0));

        // Increment the counter by 5
        // 使用签名账户1的权限来执行递增操作
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(1), 5));
        // Check that the event emitted matches the increment operation
        // 验证是否正确触发了CounterIncremented事件，并且事件中的数据与预期一致。
        System::assert_last_event(
            Event::CounterIncremented {
                counter_value: 5,
                who: 1,
                incremented_amount: 5,
            }
            .into(),//into()方法用于将Event::CounterIncremented事件转换为RuntimeEvent类型。
        );
    });
}


// Ensure non-root accounts cannot set counter value
#[test]
fn set_counter_value_fails_for_non_root() {
    new_test_ext().execute_with(|| {
        /*
        将系统区块高度设置为1？有什么用，为什么要把块高设置为1，块高是什么？
        区块高度（Block Number）是什么？
        在区块链中，区块高度是指当前区块在区块链中的位置。
        每个区块都有一个唯一的编号，从创世区块（第一个区块）开始，依次递增。
        例如，如果当前区块是第100个区块，那么它的区块高度就是100。
        设置区块高度为1的作用？
        在测试环境中，设置区块高度为1通常是为了模拟一个特定的区块高度，以便测试某些依赖于区块高度的功能或逻辑。
         */

        System::set_block_number(1);
        // assert_noop!宏：这是一个用于断言某个操作应该失败并返回特定错误的宏。
        assert_noop!(
            CustomPallet::set_counter_value(RuntimeOrigin::signed(1), 5), // non-root account
            //这是期望的错误类型。BadOrigin错误表示调用者没有足够的权限来执行该操作。
            sp_runtime::traits::BadOrigin // Expecting a BadOrigin error
        );
    });
}


#[test]
fn increment_handles_overflow() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // Set to max value
        assert_ok!(CustomPallet::set_counter_value(RuntimeOrigin::root(), 1));
        assert_noop!(
            CustomPallet::increment(RuntimeOrigin::signed(1), u32::MAX),
            Error::<Test>::CounterOverflow
        );
    });
}

// Check that user interactions are correctly tracked
#[test]
fn user_interactions_increment() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // Initialize counter value to 0
        assert_ok!(CustomPallet::set_counter_value(RuntimeOrigin::root(), 0));

        // Increment by 5 and decrement by 2
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(1), 5));
        assert_ok!(CustomPallet::decrement(RuntimeOrigin::signed(1), 2));

        //assert_eq!宏：这是一个断言宏，用于检查两个值是否相等。如果两个值不相等，则测试会失败并报告错误。
        //UserInteractions::<Test>::get(1)：这是从UserInteractions存储映射中获取账户ID为1的用户的交互次数。<Test>表示使用测试环境中的配置。
        //unwrap_or(0)：如果UserInteractions::<Test>::get(1)返回None（即没有找到该用户的交互次数），则使用unwrap_or(0)将其转换为0。这样可以确保即使没有记录，也不会导致程序崩溃。
        assert_eq!(UserInteractions::<Test>::get(1).unwrap_or(0), 2); // User should have 2 interactions
    });
}