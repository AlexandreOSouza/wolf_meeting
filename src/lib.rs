use juniper::{EmptySubscription, RootNode};
use tokio_postgres::Client;

use models;

pub struct QueryRoot;
pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn customer(ctx: &Context, id: String) -> juniper::FieldResult<Customer> {
        let uuid = uuid::Uuid::parse_str(&id)?;
        let row = ctx
            .client
            .query_one(
                "SELECT name, age, email, address FROM customers WHERE id = $1",
                &[&uuid],
            )
            .await?;
        let customer = Customer {
            id,
            name: row.try_get(0)?,
            age: row.try_get(1)?,
            email: row.try_get(2)?,
            address: row.try_get(3)?,
        };
        Ok(customer)
    }
}

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn register_customer(
        ctx: &Context,
        name: String,
        age: i32,
        email: String,
        address: String,
    ) -> juniper::FieldResult<Customer> {
        let id = uuid::Uuid::new_v4();
        let email = email.to_lowercase();

        ctx.client
            .execute(
                "INSERT INTO customers(id, name, age, email, address) VALUES ($1, $2, $3, $4, $5)",
                &[&id, &name, &age, &email, &address],
            )
            .await?;
        Ok(Customer {
            id: "1".into(),
            name,
            age,
            email,
            address,
        })
    }
}

type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

struct Context {
    client: Client,
}

impl juniper::Context for Context {}
