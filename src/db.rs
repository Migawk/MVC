use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub stars: i32,
}
pub struct DB {
    conn: Connection,
}
impl DB {
    pub fn new() -> Result<DB, ()> {
        let conn = Connection::open("db.sql");

        match conn {
            Ok(connect) => {
                let ent = DB { conn: connect };

                Ok(ent)
            }
            Err(_) => Err(()),
        }
    }

    pub fn init(&self) -> &DB {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS User(id INTEGER PRIMARY KEY, name TEXT, stars INTEGER DEFAULT 0);",
            (),
        ).expect("Err during init");
        self
    }

    pub fn create_user(&self, name: &str) -> &DB {
        self.conn
            .execute("INSERT INTO User(name) VALUES (?1)", &[name])
            .expect("Err during creating");

        self
    }

    pub fn get_user(&self, name: &str) -> Result<User, ()> {
        let mut users = self
            .conn
            .prepare("SELECT * FROM User")
            .expect("Err during getting");

        let iter = users
            .query_map([], |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    stars: row.get(2)?,
                })
            })
            .expect("Err during getting");

        for us in iter {
            let user = us.unwrap();
            if user.name == name {
                return Ok(user);
            };
        }
        Err(())
    }

    pub fn delete_user(&self, name: &str) -> Option<bool> {
        self.conn
            .execute("DELETE FROM User Where name=?1", &[name])
            .expect("Err during deleting");

        let res = self.get_user(name);
        match res {
            Ok(_) => Some(true),
            Err(()) => Some(false),
        }
    }

    pub fn update_user(&self, uld_name: &str, new_name: &str, stars: i32) -> Result<User, ()> {
        self.conn
            .execute(
                "UPDATE User set name=?2, stars=?3 WHERE name=?1",
                &[uld_name, new_name, &stars.to_string()],
            )
            .expect("Err during updating");

        self.get_user(new_name)
    }
}
