pub mod create {
    use postgres::types::Type;
    use postgres::{Client, Error};

    #[derive(Debug)]
    struct User {
        username: String,
        user_id: i32,
    }

    pub fn select(client: &mut Client) -> Result<(), Error> {
        for row in client.query("SELECT * FROM users", &[])? {
            let user = User {
                username: row.get(0),
                user_id: row.get(1),
            };
            println!("Record: {:?}", &user);
        }
        Ok(())
    }

    pub fn insert(client: &mut Client, username: String, user_id: i32) -> Result<(), Error> {
        let statement = client.prepare_typed(
            "INSERT INTO users (username, user_id) VALUES ($1, $2)",
            &[Type::VARCHAR, Type::INT4],
        )?;

        let res = client.execute(&statement, &[&username, &user_id])?;

        println!("Result while INSERT -> {}", &res);
        Ok(())
    }
}
