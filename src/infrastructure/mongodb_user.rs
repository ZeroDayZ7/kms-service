// src/infrastructure/mongodb_user.rs
use crate::domain::UserRepository;
use crate::domain::user::User;
use crate::domain::value_objects::user_id::UserId;
use crate::errors::{AppError, AppResult};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use async_trait::async_trait;

pub struct MongoUserRepository {
    db: Arc<Database>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MongoUserDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    email: String,
    password_hash: String,
}

impl MongoUserRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let collection = self.db.collection::<MongoUserDocument>("users");
        let doc = collection.find_one(doc! { "email": email }).await?;
        
        match doc {
            Some(d) => {
                let user_id = d.id.map(|oid| UserId::new(oid.to_hex()));
                let domain_email = crate::domain::value_objects::email::Email::new(d.email)?;
                Ok(Some(User {
                    id: user_id,
                    email: domain_email,
                    password_hash: d.password_hash,
                }))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, user: User) -> AppResult<()> {
        let collection = self.db.collection::<MongoUserDocument>("users");
        
        let oid = match user.id {
            Some(id) => Some(ObjectId::parse_str(id.as_inner()).map_err(|_| {
                AppError::ValidationError(format!("Invalid MongoDB ObjectId: {}", id))
            })?),
            None => None,
        };

        let doc = MongoUserDocument {
            id: oid,
            email: user.email.to_string(),
            password_hash: user.password_hash,
        };

        collection.insert_one(doc).await?;
        Ok(())
    }
}