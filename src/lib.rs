use near_sdk::collections::UnorderedMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId, near_bindgen, PanicOnDefault, Promise};
use near_sdk::serde::{Serialize, Deserialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Marketplace {
    listed_products: UnorderedMap<String, Product>
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Product {
    id:String,
    name:String,
    description:String,
    image:String,
    location:String,
    price:String,
    owner: AccountId,
    sold:u32
}

impl Product {
    pub fn from_payload(payload:Payload) -> Self {
        Self { id: payload.id, name:payload.name, description: payload.description, image: payload.image, location: payload.location, price: payload.price, owner:env::signer_account_id(), sold: 0 }
    }

    pub fn increment_sold_amount(&mut self){
        self.sold = self.sold + 1;
    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    id:String,
    name:String,
    description:String,
    image:String,
    location:String,
    price:String
}

#[near_bindgen]
impl Marketplace{
    #[init]
    pub fn init() -> Self {
        Self {
            // key "l_pdt" = listed_products
            listed_products: UnorderedMap::new(b"l_pdt".to_vec())
        }
    }

    pub fn set_product(&mut self,  payload:Payload){
        let product = Product::from_payload(payload);
        self.listed_products.insert(&product.id, &product);
    }

    pub fn get_product(&self, id: &String) -> Option<Product>{
        self.listed_products.get(id)
    }

    pub fn get_products(&self) -> Vec<Product>{
        return self.listed_products.values_as_vector().to_vec();
    }

    #[payable]
    pub fn buy_product(&mut self, product_id: &String){
        match self.listed_products.get(product_id){
            Some(ref mut product) => {
                let price = product.price.parse().unwrap();
                assert_eq!(env::attached_deposit(),price , "attached deposit should be equal to the product's");
                let owner = &&product.owner.as_str();
                Promise::new (owner.parse().unwrap()).transfer(price);
                product.increment_sold_amount();
                self.listed_products.insert(&product.id, &product);
            }, _ => {
                env::panic_str("Product not found");
            }
        }
    }
}

   