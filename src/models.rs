#[derive(juniper::GraphQLObject)]
pub struct Customer {
    id: String,
    name: String,
    age: i32,
    email: String,
    address: String,
}
