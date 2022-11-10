pub trait Status {
    fn get_status(&self) -> (bool, &Option<String>);
}

impl Status for crate::models::GetDsid200Response {
    fn get_status(&self) -> (bool, &Option<String>) {
        (self.ok, &self.message)
    }
}
