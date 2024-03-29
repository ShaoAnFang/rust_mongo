#![allow(unused)]
use crate::models::location::Location;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{self, doc, extjson::de::Error, oid::ObjectId},
    options::{ClientOptions, FindOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

// use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<Location>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let database_url = std::env::var("MONGO_CLIENT").expect(".env MONGO_CLIENT 未設定");
        let client_options = ClientOptions::parse(database_url)
            .await
            .expect("error connecting to database");
        // client_options.app_name = Some("Rust Demo".to_string());
        let client = Client::with_options(client_options).unwrap();
        let database = client.database("travel");
        println!("DB connected successfully");
        let col = database.collection::<Location>("locations");
        MongoRepo { col }
    }

    pub async fn get_location(&self, region: &String) -> Result<Location, Error> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        // let filter = doc! {"_id": obj_id};
        let filter = doc! {"region" : region };
        let location = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting location detail");

        Ok(location.unwrap())
    }
    pub async fn get_locations(
        &self,
        region: String,
        count: String,
    ) -> Result<Vec<Location>, Error> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        // let filter = doc! {"_id": obj_id};
        let filter = doc! {"region" : region };
        let options = FindOptions::builder()
            .limit(count.parse::<i64>().unwrap())
            .build();
        let mut locations: Vec<Location> = Vec::new();
        let mut cursors = self
            .col
            .find(filter, options)
            .await
            .ok()
            .expect("Error getting list of location");

        while let Some(location) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            locations.push(location)
        }

        Ok(locations)
    }

    pub async fn get_random_locations(
        &self,
        region: String,
        count: String,
    ) -> Result<Vec<Location>, Error> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        // let filter = doc! {"_id": obj_id};
        // let filter = doc! {"region" : region };
        let options = FindOptions::builder()
            .limit(count.parse::<i64>().unwrap())
            .build();

        let pipeline = vec![
            // filter documents
            doc! {
                "$match": { "region": "桃園市"}
            },
            doc! {
                "$sample": { "size": count.parse::<i64>().unwrap()}
            },
        ];

        let mut cursors = self
            .col
            .aggregate(pipeline, None)
            .await
            .ok()
            .expect("Error getting list of location");
        let mut locations: Vec<Location> = Vec::new();
        while let Some(document) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            // println!("* {}", document);
            let location: Location = bson::from_document(document).unwrap();
            locations.push(location)
        }

        Ok(locations)
    }

    pub async fn post_location(&self, new_location: Location) -> Result<InsertOneResult, Error> {
        let new_doc = Location {
            name: new_location.name,
            add: new_location.add,
            description: new_location.description,
            region: new_location.region,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .expect("Error creating new location");
        Ok(user)
    }
}
