use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Forum {
    pub title: String,
    pub link: String,
    pub description: String,
}

impl Forum {
    pub fn new(title: String, link: String, description: String) -> Forum {
        Forum {
            title,
            link,
            description,
        }
    }
}
