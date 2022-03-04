mod db_common {
    pub trait CollectionPath {
        const DATABASE: &'static str = "mydb";
        const COLLECTION: &'static str;
    }
    
    pub trait KeyFrom {
        fn get_key(&self) -> Self;
    }
}

mod binding {
    use mongodb::bson::oid::ObjectId;
    use serde::{Deserialize, Serialize};

    pub const SCHEMA_VERSION: &str = "1.0.0";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Binding {
        #[serde(
            rename = "_id",
            skip_serializing_if = "Option::is_none",
            default = "Option::default"
        )]
        id: Option<ObjectId>,
        #[serde(skip_serializing_if = "String::is_empty")]
        thing_id: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        location: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        version: String,
    }

    impl super::db_common::CollectionPath for Binding {
        const COLLECTION: &'static str = "bindings";
    }

    impl super::db_common::KeyFrom for Binding {
        fn get_key(&self) -> Self {
            Self::filter_by_thing(&self.thing_id)
        }
    }

    impl Binding {
        // Standard initialization of a new binding.
        pub fn new(thing_id: &String, location: &String) -> Self {
            Self {
                id: None,
                thing_id: thing_id.to_owned(),
                location: location.to_owned(),
                version: SCHEMA_VERSION.to_string(),
            }
        }

        /// Return a Binding struct that can be used for filtering entries by
        /// a particular BotId.
        pub fn filter_by_thing(thing_id: &String) -> Self {
            Self {
                id: None,
                thing_id: thing_id.to_owned(),
                location: "".to_string(),
                version: "".to_string(),
            }
        }

        /// Return a Binding struct that can be used for filtering entries by
        /// a particular destination.
        pub fn filter_by_location(location: &String) -> Self {
            Self {
                id: None,
                thing_id: "".to_string(),
                location: location.to_owned(),
                version: "".to_string(),
            }
        }

        /// Get a key object
        pub fn get_key(other: &Self) -> Self {
            Self::filter_by_thing(&other.thing_id)
        }
    }

    impl PartialEq for Binding {
        /// Only compare equality if id, thing_id, or location are both present.
        fn eq(&self, other: &Self) -> bool {
            (self.id.is_none()
                || other.id.is_none()
                || (self.id.is_some() && other.id.is_some() && self.id.unwrap() == other.id.unwrap()))
                && (self.thing_id.is_empty()
                    || other.thing_id.is_empty()
                    || (!self.thing_id.is_empty()
                        && !other.thing_id.is_empty()
                        && self.thing_id == other.thing_id))
                && (self.location.is_empty()
                    || other.location.is_empty()
                    || (!self.location.is_empty()
                        && !other.location.is_empty()
                        && self.location == other.location))
        }
    }
}

mod mongo_if {
    use mongodb::bson::{to_bson, Document};
    use mongodb::error::Error as MongoDBError;
    use mongodb::options::UpdateOptions;
    use mongodb::Client;
    use serde::Serialize;
    use std::env;

    pub async fn get_mongo_client() -> Result<Client, MongoDBError> {
        let uri = env::var("MONGO_URI").unwrap();
        println!("Connecting to MongoDB at {}", &uri);
        Client::with_uri_str(uri).await
    }
    
    #[derive(Debug)]
    pub enum MongoDBResult {
        SUCCESS,
        ERROR(String),
    }
    
    pub async fn upsert<T>(client: &Client, item: &T) -> Result<(), MongoDBResult>
    where
        T: super::db_common::CollectionPath + super::db_common::KeyFrom + Serialize,
    {
        let d: Document = to_bson(&item).unwrap().as_document().unwrap().clone();
        let query: Document = to_bson(&item.get_key()).unwrap().as_document().unwrap().clone();
        let binding_collection = client.database(&T::DATABASE).collection(&T::COLLECTION);
        let options = UpdateOptions::builder()
            .upsert(true)
            .build();
        let upsert_result = binding_collection.update_one(query, d, None).await;
        match upsert_result {
            Ok(_) => Ok(()),
            Err(_) => Err(MongoDBResult::ERROR("Insert error".to_string())),
        }
    }
}

#[tokio::main]
async fn main() {
    let b = binding::Binding::new(&"carpet".to_string(), &"floor".to_string());
    let client = mongo_if::get_mongo_client().await.unwrap();
    mongo_if::upsert(&client, &b).await;
}
