use postgres::{Client, Error, NoTls};

mod action;

fn main() -> Result<(), Error>{
  let mut client = Client::connect("postgresql://test:test@localhost/test", NoTls)?;


  let username = String::from("Jesus");
  let user_id: i32 = 11;

  action::create::select(&mut client);
  action::create::insert(&mut client, username, user_id);


  Ok(())
}
