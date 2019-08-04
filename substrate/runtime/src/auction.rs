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

#[derive(Debug, Encode, Decode)]
enum ProductStatus {
    Open,
    Sold,
    Off,
}

impl Default for ProductStatus {
    fn default() -> Self { ProductStatus::Open }
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Default, Encode, Decode)]
pub struct Product<T> where T: Trait {
    name: u64,
    imageHash: u64,
    description: u64,
    startPrice: Option<BalanceOf<T>>,
    // highestPrice: Option<BalanceOf<T>>,
    // winner: Option<<T as system::Trait>::AccountId>,
    // status: ProductStatus
}

// #[derive(Encode, Decode)]
// pub struct BidInfo<T> where T: Trait {
//     price: Option<BalanceOf<T>>,
//     bidder: T::AccountId
// }

type ProductLinkedItem<T> = LinkedItem<<T as Trait>::ProductIndex>;
type OwnedProductsList<T> = LinkedList<OwnedProducts<T>, <T as system::Trait>::AccountId, <T as Trait>::ProductIndex>;
type ProductsForAucList<T> = LinkedList<ProductsForAuc<T>, <T as system::Trait>::AccountId, <T as Trait>::ProductIndex>;

decl_storage! {
    trait Store for Module<T: Trait> as Products {
        /// store all the products
        pub Products get(product): map T::ProductIndex => Option<Product<T>>;
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
        pub fn add_product(origin, product_name: u64, image_hash: u64, description: u64, 
        start_price: Option<BalanceOf<T>>) {
            let sender = ensure_signed(origin)?;
            let new_product_id = Self::do_add_product(&sender, product_name, image_hash, description, start_price)?;

            Self::deposit_event(RawEvent::NewProduct(sender, new_product_id));
        }
        /// 竞价
        pub fn bid(origin, product_id: T::ProductIndex, price: BalanceOf<T>) {
            let sender = ensure_signed(origin)?;

            Self::do_bid(&sender, product_id, price);
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
            Self::remove_from_auction(&sender, product_id);
            // 资金划拨
            T::Currency::transfer(&bidder, &sender, price);
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

    fn do_add_product(sender: &T::AccountId, product_name: u64, image_hash: u64, description: u64, 
    start_price: Option<BalanceOf<T>>) ->
     result::Result<T::ProductIndex, &'static str> {

        let product = Product{
            name: product_name,
            imageHash: image_hash,
            startPrice: start_price,
            description: description,
            // highestPrice: start_price,
            // winner: None,
            // status: ProductStatus::Off
        };

        let new_product_id = Self::next_product_id()?;
        Self::insert_product(sender, new_product_id, product);

        Ok(new_product_id)
    }

    fn insert_product(owner: &T::AccountId, product_id: T::ProductIndex, product: Product<T>) {
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