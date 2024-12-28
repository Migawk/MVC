mod controller;
mod service;

pub mod user_provider {
	use crate::user::controller::user_controller::Controller;

    pub struct UserProvider {
		pub controller: Controller
	}

    impl UserProvider {
        pub fn new() -> UserProvider {
			let controller = Controller::new();
            UserProvider {
				controller
			}
        }
    }
}
