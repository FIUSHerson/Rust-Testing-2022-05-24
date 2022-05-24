pub struct Package {
    sender_name: String,
    sender_address: String,
    sender_city: String,
    sender_state: String,
    sender_zip_code: i32,
    
    recipient_name: String,
    recipient_address: String,
    recipient_city: String,
    recipient_state: String,
    recipient_zip_code: i32,

    package_weight: f32
}


impl Package {
    pub fn new(
        sender_name: String,
        sender_address: String,
        sender_city: String,
        sender_state: String,
        sender_zip_code: i32,
        
        recipient_name: String,
        recipient_address: String,
        recipient_city: String,
        recipient_state: String,
        recipient_zip_code: i32,

        package_weight: f32
    ) -> Package {
        // This function creates and returns a new package.

        let user_package: Package = Package{};
    }


    /*
    Setters
    */


    fn set_sender_name(&self, name: &str) -> () {
        self.sender_name = name.to_string();
    }

    fn set_sender_address(&self, address: &str) -> () {
        self.sender_address = address.to_string();
    }

    fn set_sender_city(&self, city: &str) -> () {
        self.sender_city = city.to_string();
    }

    fn set_sender_state(&self, state: &str) -> () {
        self.sender_state = state.to_string();
    }

    fn set_sender_zip_code(&self, zip_code: &i32) -> () {
        self.sender_zip_code = *zip_code;
    }

    fn set_recipient_name(&self, name: &str) -> () {
        self.recipient_name = name.to_string();
    }

    fn set_recipient_address(&self, address: &str) -> () {
        self.recipient_address = address.to_string();
    }

    fn set_recipient_city(&self, city: &str) -> () {
        self.recipient_city = city.to_string();
    }

    fn set_recipient_state(&self, state: &str) -> () {
        self.recipient_state = state.to_string();
    }

    fn set_recipient_zip_code(&self, zip_code: &i32) -> () {
        self.recipient_zip_code = *zip_code;
    }

    fn set_package_weight(&self, weight: &f32) -> () {
        self.package_weight = *weight;
    }

    pub fn calculate_cost(&self, cost_per_ounce: &f32) -> f32 {
        self.package_weight * *cost_per_ounce
    }
}