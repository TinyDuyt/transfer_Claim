#![cfg_attr(not(feature = "std"),no_stds)] //编译标签 编译的时候使用std和no_std两种形式，如果不是std则一定是no_std

#[frame_support::pallet]

pub use pallet::*;
pub mod pallet{
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,//可调用函数的返回结果
        pallet_prelude::*

    };
    use frame_system::pallet_prelude::*;//系统模块需要的数据和数据类型信息
    use sp_std::vec::Vec;
    
    #[pallet::config] //配置接口
    pub trait config: frame_system::Config{
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage] //存储存证
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber) //AccountId表示用户id BlockNumber表示存入存证的区块
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId")]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>), //用户id和存证信息
        ClaimRevoked(T::AccountId,Vec<u8>),
    }

    #[pallet::error] //定义error
    pub enum Error<T>{
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks] 
    impl<T: Config>Hooks<BlockNumberFor<T>> for Pallet<T>{}

    #[pallet::call]
    impl<T: Config>Pallet<T>{
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo{
                let sender = ensure_signed(origin)?;

                ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

                Proofs::<T>::insert(
                    &claim,
                    (sender,clone(),frame_system::Pallet::<T>::block_number())
                );

                Self::deposit_event(Event::ClaimCreated(sender, claim));
                OK(().into())
            }
            pub fn transfer_claim(
                origin: OriginFor<T>,
                claim: Vec<u8>
            ) -> DispatchResultWithPostInfo{
                    let sender = ensure_signed(origin)?;

                    let(owner,_) = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;
    
                    ensure!(sender == owner, Error::<T>::NotProofOwner);
    
                    Proofs::<T>::insert(
                        &prrof,
                        (receiver,<frame_system::Module::<T>>::block_number()));
    
                    Self::deposit_event(Event::ClaimMutated(sender, proof,receiver));
                    
                }

        

            #[pallet::weight(0)]
            pub fn revoke_claim(
                origin: OriginFor<T>,
                claim: Vec<u8>
            ) -> DispatchResultWithPostInfo{
                let sender = ensure_signed(origin)?;

                let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

                ensurn!(owner == sender,Error::<T>::NotClaimOwner);

                Proofs::<T>::remove(&claim);

                Self::deposit_event(Event::ClaimRevoked(sender,claim));
                Ok(().into())
            }
        
    }
}