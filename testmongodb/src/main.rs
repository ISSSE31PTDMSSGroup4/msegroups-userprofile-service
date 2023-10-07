
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, sync::Client};

fn main() -> mongodb::error::Result<()> {
  let mut client_options =
    ClientOptions::parse("mongodb+srv://admin:<Password>@cluster0.litlwrq.mongodb.net/?retryWrites=true&w=majority")?;

  // Set the server_api field of the client_options object to Stable API version 1
  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);

  // Get a handle to the cluster
  let client = Client::with_options(client_options)?;

  // Ping the server to see if you can connect to the cluster
  client
    .database("admin")
    .run_command(doc! {"ping": 1}, None)?;
  println!("Pinged your deployment. You successfully connected to MongoDB!");

  Ok(())
}