// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use async_trait::async_trait;
use crate::domain::UserRepository;
use crate::domain::ports::services::UserServicePort;
use crate::domain::user::User;
use crate::errors::{AppError, AppResult};
use std::sync::Arc;
use tracing::instrument;

pub struct UserService<R: UserRepository> {
    repo: Arc<R>,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> UserServicePort for UserService<R> {
    #[instrument(skip(self), fields(user_email = %email))]
    async fn get_user_by_email(&self, email: &str) -> AppResult<User> {
        self.repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Użytkownik {} nie istnieje", email)))
    }

    #[instrument(skip(self, user), fields(user_id = ?user.id))]
    async fn register_user(&self, user: User) -> AppResult<()> {
        self.repo.save(user).await
    }
}