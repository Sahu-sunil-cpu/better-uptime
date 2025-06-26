use std::sync::{Arc, Mutex};


use poem::{
     handler, web::{Data, Json, Path}
};
use crate::{request_input::CreateWebsiteOutput, request_output::GetWebsiteOutput};
use crate::request_output::CreateWebsiteInput;
use store::store::Store;





#[handler]
pub fn get_website(Path(id): Path<String>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput> {
   let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id).unwrap();
    let res = GetWebsiteOutput { url: website.url };
    Json(res)
}

#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteInput>, Data(s): Data<&Arc<Mutex<Store>>> ) -> Json<CreateWebsiteOutput> {
   let mut locked_s = s.lock().unwrap();
    let website = locked_s
        .create_website(
            String::from("83c524bc-0ffa-44cd-a1bc-2483246182a6"),
            data.url,
        )
        .unwrap();

    let res = CreateWebsiteOutput { id: website.id };
    Json(res)
}
