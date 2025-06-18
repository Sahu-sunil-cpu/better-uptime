

use store::store::Store;




pub fn get_store() -> Store {
 let s = Store::default().unwrap();
return s;
}