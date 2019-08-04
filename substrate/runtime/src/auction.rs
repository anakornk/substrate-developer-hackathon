use support::{
    decl_module, decl_storage, StorageMap, StorageValue, ensure, 
    /*dispatch::Result,*/ Parameter, traits::Currency, decl_event
};
use runtime_primitives::traits::{SimpleArithmetic, Bounded, One, Member};
use parity_codec::{Encode, Decode};
use system::ensure_signed;
use rstd::result;
use crate::linkedlist::{LinkedItem, LinkedList};

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type ProductIndex: Parameter + Member + Default + Bounded + SimpleArithmetic + Copy;
    type Currency: Currency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

// #[derive(Debug, Encode, Decode)]
// enum ProductStatus {
//     Open,
//     Sold,
//     Off,
// }

// impl Default for ProductStatus {
//     fn default() -> Self { ProductStatus::Open }
// }

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Default, Encode, Decode)]
pub struct Product {
    name: u64,
    imageHash: u64,
    description: u64,
    // startPrice: Option<BalanceOf<T>>,
    // highestPrice: Option<BalanceOf<T>>,
    // winner: Option<<T as system::Trait>::AccountId>,
    // status: ProductStatus
}


type ProductLinkedItem<T> = LinkedItem<<T as Trait>::ProductIndex>;
type OwnedProductsList<T> = LinkedList<OwnedProducts<T>, <T as system::Trait>::AccountId, <T as Trait>::ProductIndex>;
type ProductsForAucList<T> = LinkedList<ProductsForAuc<T>, <T as system::Trait>::AccountId, <T as Trait>::ProductIndex>;

decl_storage! {
    trait Store for Module<T: Trait> as Products {
        /// store all the products
        pub Products get(product): map T::ProductIndex => Option<Product>;
        /// all products count
        pub ProductsCount get(products_count): T::ProductIndex;
        /// user owned products: user product index => global product index
        pub OwnedProducts get(owned_products): map (T::AccountId, Option<T::ProductIndex>) => Option<ProductLinkedItem<T>>;
        /// get global product index by user product index
        pub ProductsForAuc get(products_for_auc): map (T::AccountId, Option<T::ProductIndex>) => Option<ProductLinkedItem<T>>;
        /// product owner
        pub ProductOwner get(product_owner): map T::ProductIndex => Option<T::AccountId>;
        // product price now
        pub ProductPrices get(product_price): map T::ProductIndex => Option<BalanceOf<T>>;
        // bidder pay hightest price now
        pub Bidder get(bidder): map T::ProductIndex => Option<T::AccountId>;
    }
}

decl_event! {
    pub enum Event<T> where 
    <T as system::Trait>::AccountId, 
    <T as Trait>::ProductIndex,
    Balance = BalanceOf<T>,
    {
        /// a product is on acution
        OnAuction(AccountId, ProductIndex, Option<Balance>),
        /// add a new product
        NewProduct(AccountId, ProductIndex),
        /// higher price for bid
        NewHighBid(AccountId, ProductIndex, Option<Balance>),
        /// cancel auction
        AuctionCanceled(AccountId, ProductIndex),
        /// product sold: (owner, bidder, product_id, price)
        Sold(AccountId, AccountId, ProductIndex, Option<Balance>),
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;
        /// 开始一场拍卖
        pub fn display(origin, product_id: T::ProductIndex, start_price: Option<BalanceOf<T>>) {
            let sender = ensure_signed(origin)?;
            ensure!(<OwnedProducts<T>>::exists(&(sender.clone(), Some(product_id))), "Only owner can display this product!");
            
            // 是否设定了起拍价
            if let Some(start_price) = start_price {
                <ProductPrices<T>>::insert(product_id, start_price);
                // <ProductPrices<T>>::insert(product_id, (Some(start_price), sender));
            } else {
                <ProductPrices<T>>::remove(product_id);
            }
            // 商品是否已经在拍卖
            if let None = Self::products_for_auc(&(sender.clone(), Some(product_id))) {
                Self::insert_product_for_auction(&sender, product_id);
                Self::deposit_event(RawEvent::OnAuction(sender, product_id, start_price));
            } else {
                return Err("product is already on auction.");
            }
        }
        /// 添加新商品
        pub fn add_product(origin, product_name: u64, image_hash: u64, description: u64) {
            let sender = ensure_signed(origin)?;
            let new_product_id = Self::do_add_product(&sender, product_name, image_hash, description)?;

            Self::deposit_event(RawEvent::NewProduct(sender, new_product_id));
        }
        /// 竞价
        pub fn bid(origin, product_id: T::ProductIndex, price: BalanceOf<T>) {
            let sender = ensure_signed(origin)?;

            Self::do_bid(&sender, product_id, price)?;
        }
        /// 取消拍卖
        pub fn cancel(origin, product_id: T::ProductIndex) {
            let sender = ensure_signed(origin)?;
            // 发布者才可以取消拍卖
            ensure!(<OwnedProducts<T>>::exists(&(sender.clone(), Some(product_id))), "Only owner can cancel!");
            // 已经在拍卖的商品才可以取消
            ensure!(<ProductsForAuc<T>>::exists(&(sender.clone(), Some(product_id))), "Product is not on auction!");
            Self::do_cancel(&sender, product_id);
            Self::deposit_event(RawEvent::NewProduct(sender, product_id));
        }
        /// 拍卖结束
        pub fn stop(origin, product_id: T::ProductIndex) {
            let sender = ensure_signed(origin)?;
            // 发布者才可以结束拍卖
            ensure!(<OwnedProducts<T>>::exists(&(sender.clone(), Some(product_id))), "Only owner can cancel!");
            // 已经在拍卖的商品才可以结束拍卖
            ensure!(<ProductsForAuc<T>>::exists(&(sender.clone(), Some(product_id))), "Product is not on auction!");
            
            // 当前的出价
            let price = Self::product_price(product_id);
            ensure!(price.is_some(), "Invalid price");
            let price = price.unwrap();
            // 出价人
            let bidder = Self::bidder(product_id);
            ensure!(bidder.is_some(), "Invalid bidder");
            let bidder = bidder.unwrap();
            // 从拍卖商品列表移除
            Self::remove_from_auction(&sender, product_id)?;
            // 资金划拨
            T::Currency::transfer(&bidder, &sender, price)?;
            // 转移所有权
            Self::do_transfer(&sender, &bidder, product_id);

            Self::deposit_event(RawEvent::Sold(sender, bidder, product_id, Some(price)));
        }
    }
}

impl<T: Trait> Module<T> {
    fn insert_product_for_auction(owner: &T::AccountId, product_id: T::ProductIndex) {
        <ProductsForAucList<T>>::append(owner, product_id);
    }

    fn do_add_product(sender: &T::AccountId, product_name: u64, image_hash: u64, description: u64) ->
     result::Result<T::ProductIndex, &'static str> {

        let product = Product{
            name: product_name,
            imageHash: image_hash,
            // startPrice: start_price,
            description: description,
            // highestPrice: start_price,
            // winner: None,
            // status: ProductStatus::Off
        };

        let new_product_id = Self::next_product_id()?;
        Self::insert_product(sender, new_product_id, product);

        Ok(new_product_id)
    }

    fn insert_product(owner: &T::AccountId, product_id: T::ProductIndex, product: Product) {
        <Products<T>>::insert(product_id, product);
        <ProductsCount<T>>::put(product_id + One::one());
        <ProductOwner<T>>::insert(product_id, owner);
        <OwnedProductsList<T>>::append(owner, product_id);
    }

    fn next_product_id() -> result::Result<T::ProductIndex, &'static str> {
        let product_id = Self::products_count();
        if product_id == T::ProductIndex::max_value() {
            return Err("Products count overflow");
        }
        Ok(product_id)
    }

    fn do_bid(sender: &T::AccountId, product_id: T::ProductIndex, price: BalanceOf<T>) -> 
    result::Result<T::ProductIndex, &'static str> {
        let product = Self::product(product_id);
        ensure!(product.is_some(), "Invalid product id.");
        // 不应该参与自己发布的商品拍卖
        ensure!(Self::product_owner(&product_id).map(|owner| owner != *sender).unwrap_or(false),
        "should not bid owned product");
        // 查看竞价人账户余额是否足够
        let balance = T::Currency::free_balance(sender);
        ensure!(balance >= price, "Insufficient balance.");
        // 当前最高出价
        let last_price = Self::product_price(&product_id);
        ensure!(last_price.is_some(), "Not on auction.");
        let last_price = last_price.unwrap();
        // 只有出价高于当前报价才会更新
        if price > last_price {
            <ProductPrices<T>>::insert(product_id, price); // 新的出价
            <Bidder<T>>::insert(product_id, sender); // 记录出价人
            Self::deposit_event(RawEvent::NewHighBid((*sender).clone(), product_id, Some(price)));
        }
        Ok(product_id)
    }
    // 从拍卖商品列表删除
    fn remove_from_auction(sender: &T::AccountId, product_id: T::ProductIndex) ->
    result::Result<T::ProductIndex, &'static str> {
        <ProductsForAucList<T>>::remove(sender, product_id);
        Ok(product_id)
    }

    fn do_cancel(sender: &T::AccountId, product_id: T::ProductIndex) {
        Self::remove_from_auction(sender, product_id);
    }

    fn do_transfer(from: &T::AccountId, to: &T::AccountId, product_id: T::ProductIndex) {
        <OwnedProductsList<T>>::remove(from, product_id);
        <OwnedProductsList<T>>::append(to, product_id);
        <ProductOwner<T>>::insert(product_id, to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq, Debug)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}

    impl balances::Trait for Test {
        /// The type for recording an account's balance.
        type Balance = u32;
        /// What to do if an account's free balance gets zeroed.
        type OnFreeBalanceZero = ();
        /// What to do if a new account is created.
        type OnNewAccount = ();
        /// The uniquitous event type.
        type Event = ();

        type TransactionPayment = ();
        type DustRemoval = ();
        type TransferPayment = ();
    }

	impl Trait for Test {
		type ProductIndex = u32;
        type Currency = balances::Module<Test>;
        type Event = ();
	}
	type ProductModule = Module<Test>;

    type OwnedProductsTest = OwnedProducts<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

    #[test]
	fn owned_products_can_append_values() {
		with_externalities(&mut new_test_ext(), || {
			OwnedProductsList::<Test>::append(&0, 1);
            assert_eq!(OwnedProductsTest::get(&(0, None)), Some(ProductLinkedItem::<Test> {
                prev: Some(1),
                next: Some(1),
            }));
            assert_eq!(OwnedProductsTest::get(&(0, Some(1))), Some(ProductLinkedItem::<Test> {
                prev: None,
                next: None,
            }));

            OwnedProductsList::<Test>::append(&0, 2);
            assert_eq!(OwnedProductsTest::get(&(0, None)), Some(ProductLinkedItem::<Test> {
                prev: Some(2),
                next: Some(1),
            }));
            assert_eq!(OwnedProductsTest::get(&(0, Some(1))), Some(ProductLinkedItem::<Test> {
                prev: None,
                next: Some(2),
            }));
            assert_eq!(OwnedProductsTest::get(&(0, Some(2))), Some(ProductLinkedItem::<Test> {
                prev: Some(1),
                next: None,
            }));
		});
	}

    #[test]
	fn owned_products_can_remove_values() {
		with_externalities(&mut new_test_ext(), || {
            OwnedProductsList::<Test>::append(&0, 1);
            OwnedProductsList::<Test>::append(&0, 2);
            OwnedProductsList::<Test>::append(&0, 3);

            OwnedProductsList::<Test>::remove(&0, 2);

            assert_eq!(OwnedProductsTest::get(&(0, None)), Some(ProductLinkedItem::<Test> {
                prev: Some(3),
                next: Some(1),
            }));
            assert_eq!(OwnedProductsTest::get(&(0, Some(1))), Some(ProductLinkedItem::<Test> {
                prev: None,
                next: Some(3),
            }));
            assert_eq!(OwnedProductsTest::get(&(0, Some(2))), None);
            assert_eq!(OwnedProductsTest::get(&(0, Some(3))), Some(ProductLinkedItem::<Test> {
                prev: Some(1),
                next: None,
            }));

            OwnedProductsList::<Test>::remove(&0, 1);
            assert_eq!(OwnedProductsTest::get(&(0, None)), Some(ProductLinkedItem::<Test> {
                prev: Some(3),
                next: Some(3),
            }));
            assert_eq!(OwnedProductsTest::get(&(0, Some(1))), None);
            assert_eq!(OwnedProductsTest::get(&(0, Some(2))), None);
            assert_eq!(OwnedProductsTest::get(&(0, Some(3))), Some(ProductLinkedItem::<Test> {
                prev: None,
                next: None,
            }));

            OwnedProductsList::<Test>::remove(&0, 3);
            assert_eq!(OwnedProductsTest::get(&(0, None)), Some(ProductLinkedItem::<Test> {
                prev: None,
                next: None,
            }));
            assert_eq!(OwnedProductsTest::get(&(0, Some(1))), None);
            assert_eq!(OwnedProductsTest::get(&(0, Some(2))), None);
            assert_eq!(OwnedProductsTest::get(&(0, Some(3))), None);
        });
    }
}