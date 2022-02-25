
use mongodb::Client;
use mongodb::bson::doc;

use std::error::Error;
use bson::Document;

use tokio;

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {

    // MongoDB Connection String
    let client_uri = "mongodb://localhost:27017";

    // create mongodb Client with the connection String
    let client = Client::with_uri_str(&client_uri).await?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("{}", name);
    }

    // Get one collection of one Database on the mongoDB cluster
    let metadata_collection = client.database("SolanaTest").collection::<Document>("MetadataCollection");

    // Search for a specific file in the collection
    println!("{}",
             metadata_collection.find_one(
                 doc!
                 {"name" : "Cow #4674"}, None,
             ).await?.expect("Missing Cow")
    );

    // Create new Bson Document
    let new_doc = doc! {
        "name": "cow #4561",
        "seller_fee_basis_points": 500,
        "symbol":"",
        "uri": "https://solanavalley.com/api/token/faaf0bcc8664c8f2bbb1"
    };

    // Insert the new Document into the collection and get the insert result
    let insert_result = metadata_collection.insert_one(new_doc, None).await?;

    // Get the ID of the newly inserted Document
    println!("New document ID: {}", insert_result.inserted_id);

    // Needet for async await ? or maybe not ?
    Ok(())
}
