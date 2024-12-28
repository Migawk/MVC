
pub mod user_service {
    use crate::{db::User, DB};

    pub struct UserService {
		db: DB
	}

    impl UserService {
		pub fn new() -> UserService {
			
			let db = DB::new().unwrap();
			UserService {
				db
			}
		}
        pub fn get_user(&self, name: &str) -> Result<User, ()> {
			self.db.get_user(name)
        }
    }
}
