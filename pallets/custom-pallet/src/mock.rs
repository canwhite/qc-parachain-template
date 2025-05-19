///The following portion of code sets up a mock runtime (Test) to test the custom-pallet in an isolated environmen
///provide test runtime

use crate as custom_pallet;
use frame::{prelude::*, runtime::prelude::*, testing_prelude::*};

// Using frame_support macros, 
//it defines a minimal runtime configuration with traits such as RuntimeCall and RuntimeEvent
type Block = frame_system::mocking::MockBlock<Test>;


// Configure a mock runtime to test the pallet.
#[frame_construct_runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(// 获得这些运行时方法
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test; //定义Test

    // pallet_index用于为每个pallet分配一个唯一的索引号
    // 这个索引号在runtime中用于标识和区分不同的pallet
    // 例如：System pallet的索引为0，CustomPallet的索引为1

    // frame_system是Substrate框架中最基础的系统pallet
    // 它提供了区块链运行所需的核心功能，包括：
    // - 区块管理
    // - 账户管理
    // - 事件系统
    // - 存储管理
    // - 随机数生成
    // 每个runtime都必须包含frame_system pallet
    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type CustomPallet = custom_pallet;
}

// System pallet configuration
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test{
    // 在Substrate中，`type`关键字用于定义关联类型（Associated Type）
    // 关联类型是Rust trait中的一个重要概念，它允许在实现trait时指定具体的类型
    // 在Substrate框架中，`type`常用于：
    // 1. 定义pallet配置中的特定类型
    // 2. 为runtime中的抽象概念指定具体实现
    // 3. 连接不同pallet之间的类型依赖
    // 例如，在System pallet中，`type Block`用于指定当前runtime使用的区块类型
    // 这里我们使用之前定义的`Block`类型作为System pallet的区块类型
    type Block = Block;
}


// Custom pallet configuration
parameter_types! {
    pub const CounterMaxValue: u32 = 10;
}

impl custom_pallet::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type CounterMaxValue = CounterMaxValue;
    type WeightInfo = custom_pallet::weights::SubstrateWeight<Test>;
}

// Test externalities initialization
pub fn new_test_ext() -> TestExternalities {
    // new_test_ext() 函数用于创建一个新的测试环境
    // 它返回一个 TestExternalities 实例，这是 Substrate 框架中用于测试的核心结构
    // 该函数的主要作用是：
    // 1. 初始化一个干净的测试环境，模拟区块链的运行环境
    // 2. 为测试提供一个隔离的存储空间，确保测试之间不会相互影响
    // 3. 允许我们在测试中执行 runtime 调用，并验证其行为
    // 4. 通过 TestExternalities 可以访问和验证存储状态的变化
    // 5. 使用默认的 GenesisConfig 初始化系统状态
    // 这个函数是编写单元测试时的基础工具，通常在每个测试用例开始时调用
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}

