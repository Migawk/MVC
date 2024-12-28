
pub mod user_controller {
	use crate::user::service::user_service::UserService;
	use crate::{db::User, DB};

    pub struct Controller {
		service: UserService
	}

    impl Controller {
        pub fn new() -> Controller {
			let service = UserService::new();

            Controller {
				service
			}
        }
		pub fn get_user(&self, name: &str) -> Result<User, ()> {
			self.service.get_user(name)
		}
    }
}
