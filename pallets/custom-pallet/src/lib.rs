//条件编译属性，用于在没有标准库的情况下禁用标准库的使用。
//因为wasm环境通常不支持标准库
#![cfg_attr(not(feature = "std"), no_std)]

//pallet是mod，use的是方法
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//这是一个宏调用，用来标记接下来的模块是Substrate pallet
#[frame::pallet]
//定义一个名为pallet的公共模块
//定义了pallet的配置、存储、事件和错误
pub mod pallet {
    //从父函数中导入所有项
    use super::*;
    //pre之前lud玩，前奏
    use frame::prelude::*;

    //宏调用，用于标记Pallet结构体为pallet的主结构体
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Configuration trait for the pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        //defines the event type for the pallet
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // Defines the maximum value the counter can hold.
        type CounterMaxValue: Get<u32>;
    }

    //defines the events so that communicate with outside world 
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        //is emitted when the counter is explicitly set to a new value
        CounterValueSet {
            /// The new value set.
            counter_value: u32,
        },
        /// is emitted after a successful increment operation
        CounterIncremented {
            /// The new value set.
            counter_value: u32,
            /// The account who incremented the counter.
            who: T::AccountId,
            /// The amount by which the counter was incremented.
            incremented_amount: u32,
        },
        ///  is emitted after a successful decrement operation
        CounterDecremented {
            /// The new value set.
            counter_value: u32,
            /// The account who decremented the counter.
            who: T::AccountId,
            /// The amount by which the counter was decremented.
            decremented_amount: u32,
        },
    }


    /// Storage items are used to manage the pallet's state. 
    /// This pallet defines two items to handle the counter's state and user interactions:
    #[pallet::storage]
    pub type CounterValue<T> = StorageValue<_, u32>;

    /// Storage map to track the number of interactions performed by each account.
    #[pallet::storage]
    pub type UserInteractions<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u32>;

    
    #[pallet::error]
    // 这里的T来自于pallet的Config trait，它继承自frame_system::Config
    // 在Substrate中，T通常表示当前pallet的配置类型，它包含了运行时所需的各种类型和参数
    // 通过T我们可以访问到AccountId、BlockNumber等系统级类型
    pub enum Error<T> {
        /// The counter value exceeds the maximum allowed value.
        CounterValueExceedsMax,
        /// The counter value cannot be decremented below zero.
        CounterValueBelowZero,
        /// Overflow occurred in the counter.
        CounterOverflow,
        /// Overflow occurred in user interactions.
        UserInteractionOverflow,
    }

    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // call_index用于指定调用的索引号，从0开始顺序递增
        // 它帮助运行时系统快速定位和调用特定的函数
        // 每个pallet中的call_index必须是唯一的
        
        // weight表示执行该调用所需的计算资源
        // 0表示默认权重，实际使用时应根据操作复杂度设置适当值
        // weight用于防止资源滥用，确保区块链网络的安全和稳定
        // 复杂的操作需要更高的weight，用户需要支付更多费用
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn set_counter_value(origin: OriginFor<T>, new_value: u32) -> DispatchResult {
            ensure_root(origin);
            
            ensure!(
                new_value <= T::CounterMaxValue::get(),
                Error::<T>::CounterValueExceedsMax
            );

            CounterValue::<T>::put(new_value);
            Self::deposit_event(Event::<T>::CounterValueSet {
                counter_value: new_value,
            });

            Ok(())
        }


        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn increment(origin: OriginFor<T>, amount_to_increment: u32) -> DispatchResult {
            //拿到给谁
            let who = ensure_signed(origin)?;

            //拿到storage的value
            let current_value = CounterValue::<T>::get().unwrap_or(0);
        
            let new_value = current_value
                .checked_add(amount_to_increment)
                .ok_or(Error::<T>::CounterOverflow)?;
        
            ensure!(
                new_value <= T::CounterMaxValue::get(),
                Error::<T>::CounterValueExceedsMax
            );
        
            CounterValue::<T>::put(new_value);
        
            // 这里使用UserInteractions存储映射来更新用户交互次数
            // try_mutate方法用于安全地修改存储值，它会处理可能的溢出情况
            // &who是用户的AccountId，作为存储映射的key
            // interactions是当前用户的交互次数，如果不存在则默认为0
            // 每次调用increment函数时，用户的交互次数会加1
            // 如果发生溢出，会返回UserInteractionOverflow错误
            UserInteractions::<T>::try_mutate(&who, |interactions| -> Result<_, Error<T>> {
                let new_interactions = interactions
                    .unwrap_or(0)
                    .checked_add(1)
                    .ok_or(Error::<T>::UserInteractionOverflow)?;
                *interactions = Some(new_interactions); // Store the new value.
        
                Ok(())
            })?;
        
            Self::deposit_event(Event::<T>::CounterIncremented {
                counter_value: new_value,
                who,
                incremented_amount: amount_to_increment,
            });
        
            Ok(())

        }
    

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn decrement(origin: OriginFor<T>, amount_to_decrement: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let current_value = CounterValue::<T>::get().unwrap_or(0);
        
            let new_value = current_value
                .checked_sub(amount_to_decrement)
                .ok_or(Error::<T>::CounterValueBelowZero)?;
        
            CounterValue::<T>::put(new_value);
            
            
            UserInteractions::<T>::try_mutate(&who, |interactions| -> Result<_, Error<T>> {
                let new_interactions = interactions
                    .unwrap_or(0)
                    .checked_add(1)
                    .ok_or(Error::<T>::UserInteractionOverflow)?;
                *interactions = Some(new_interactions); // Store the new value.
        
                Ok(())
            })?;
        
            Self::deposit_event(Event::<T>::CounterDecremented {
                counter_value: new_value,
                who,
                decremented_amount: amount_to_decrement,
            });
        
            Ok(())
        }
    }


}