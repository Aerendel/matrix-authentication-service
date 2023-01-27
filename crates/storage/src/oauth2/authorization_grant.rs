// Copyright 2021-2023 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num::NonZeroU32;

use async_trait::async_trait;
use mas_data_model::{AuthorizationCode, AuthorizationGrant, Client, Session};
use oauth2_types::{requests::ResponseMode, scope::Scope};
use rand_core::RngCore;
use ulid::Ulid;
use url::Url;

use crate::{repository_impl, Clock};

/// An [`OAuth2AuthorizationGrantRepository`] helps interacting with
/// [`AuthorizationGrant`] saved in the storage backend
#[async_trait]
pub trait OAuth2AuthorizationGrantRepository: Send + Sync {
    /// The error type returned by the repository
    type Error;

    /// Create a new authorization grant
    ///
    /// Returns the newly created authorization grant
    ///
    /// # Parameters
    ///
    /// * `rng`: A random number generator
    /// * `clock`: The clock used to generate timestamps
    /// * `client`: The client that requested the authorization grant
    /// * `redirect_uri`: The redirect URI the client requested
    /// * `scope`: The scope the client requested
    /// * `code`: The authorization code used by this grant, if the `code`
    ///   `response_type` was requested
    /// * `state`: The state the client sent, if set
    /// * `nonce`: The nonce the client sent, if set
    /// * `max_age`: The maximum age since the user last authenticated, if asked
    ///   by the client
    /// * `response_mode`: The response mode the client requested
    /// * `response_type_id_token`: Whether the `id_token` `response_type` was
    ///   requested
    /// * `requires_consent`: Whether the client explicitly requested consent
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the underlying repository fails
    #[allow(clippy::too_many_arguments)]
    async fn add(
        &mut self,
        rng: &mut (dyn RngCore + Send),
        clock: &dyn Clock,
        client: &Client,
        redirect_uri: Url,
        scope: Scope,
        code: Option<AuthorizationCode>,
        state: Option<String>,
        nonce: Option<String>,
        max_age: Option<NonZeroU32>,
        response_mode: ResponseMode,
        response_type_id_token: bool,
        requires_consent: bool,
    ) -> Result<AuthorizationGrant, Self::Error>;

    /// Lookup an authorization grant by its ID
    ///
    /// Returns the authorization grant if found, `None` otherwise
    ///
    /// # Parameters
    ///
    /// * `id`: The ID of the authorization grant to lookup
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the underlying repository fails
    async fn lookup(&mut self, id: Ulid) -> Result<Option<AuthorizationGrant>, Self::Error>;

    /// Find an authorization grant by its code
    ///
    /// Returns the authorization grant if found, `None` otherwise
    ///
    /// # Parameters
    ///
    /// * `code`: The code of the authorization grant to lookup
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the underlying repository fails
    async fn find_by_code(&mut self, code: &str)
        -> Result<Option<AuthorizationGrant>, Self::Error>;

    /// Fulfill an authorization grant, by giving the [`Session`] that it
    /// created
    ///
    /// Returns the updated authorization grant
    ///
    /// # Parameters
    ///
    /// * `clock`: The clock used to generate timestamps
    /// * `session`: The session that was created using this authorization grant
    /// * `authorization_grant`: The authorization grant to fulfill
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the underlying repository fails
    async fn fulfill(
        &mut self,
        clock: &dyn Clock,
        session: &Session,
        authorization_grant: AuthorizationGrant,
    ) -> Result<AuthorizationGrant, Self::Error>;

    /// Mark an authorization grant as exchanged
    ///
    /// Returns the updated authorization grant
    ///
    /// # Parameters
    ///
    /// * `clock`: The clock used to generate timestamps
    /// * `authorization_grant`: The authorization grant to mark as exchanged
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the underlying repository fails
    async fn exchange(
        &mut self,
        clock: &dyn Clock,
        authorization_grant: AuthorizationGrant,
    ) -> Result<AuthorizationGrant, Self::Error>;

    /// Unset the `requires_consent` flag on an authorization grant
    ///
    /// Returns the updated authorization grant
    ///
    /// # Parameters
    ///
    /// * `authorization_grant`: The authorization grant to update
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the underlying repository fails
    async fn give_consent(
        &mut self,
        authorization_grant: AuthorizationGrant,
    ) -> Result<AuthorizationGrant, Self::Error>;
}

repository_impl!(OAuth2AuthorizationGrantRepository:
    async fn add(
        &mut self,
        rng: &mut (dyn RngCore + Send),
        clock: &dyn Clock,
        client: &Client,
        redirect_uri: Url,
        scope: Scope,
        code: Option<AuthorizationCode>,
        state: Option<String>,
        nonce: Option<String>,
        max_age: Option<NonZeroU32>,
        response_mode: ResponseMode,
        response_type_id_token: bool,
        requires_consent: bool,
    ) -> Result<AuthorizationGrant, Self::Error>;

    async fn lookup(&mut self, id: Ulid) -> Result<Option<AuthorizationGrant>, Self::Error>;

    async fn find_by_code(&mut self, code: &str)
        -> Result<Option<AuthorizationGrant>, Self::Error>;

    async fn fulfill(
        &mut self,
        clock: &dyn Clock,
        session: &Session,
        authorization_grant: AuthorizationGrant,
    ) -> Result<AuthorizationGrant, Self::Error>;

    async fn exchange(
        &mut self,
        clock: &dyn Clock,
        authorization_grant: AuthorizationGrant,
    ) -> Result<AuthorizationGrant, Self::Error>;

    async fn give_consent(
        &mut self,
        authorization_grant: AuthorizationGrant,
    ) -> Result<AuthorizationGrant, Self::Error>;
);
