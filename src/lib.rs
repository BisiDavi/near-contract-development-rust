use near_sdk::collections::UnorderedMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Marketplace {
    products: UnorderedMap<String, String>
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
            // key "pdt" = product
            products: UnorderedMap::new(b"pdt".to_vec())
        }
    }

    pub fn set_product(&mut self, id:String, product_name:String){
        self.products.insert(&id, &product_name);
    }

    pub fn get_product(&self, id: &String) -> Option<String>{
        self.products.get(id)
    }
}

impl Product {
    pub fn from_payload(payload:Payload) -> Self {
        Self { id: payload.id, name:payload.name, description: payload.description, image: payload.image, location: payload.location, price: payload.price, owner:env::signerr_account_id(), sold: 0 }
    }
}