// Copyright 2025 New Vector Ltd.
//
// SPDX-License-Identifier: AGPL-3.0-only

use aide::{NoApi, OperationIo, transform::TransformOperation};
use axum::{Json, response::IntoResponse};
use hyper::StatusCode;
use mas_storage::BoxRng;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    admin::{
        call_context::CallContext,
        model::PolicyData,
        response::{ErrorResponse, SingleResponse},
    },
    impl_from_error_for_route,
};

#[derive(Debug, thiserror::Error, OperationIo)]
#[aide(output_with = "Json<ErrorResponse>")]
pub enum RouteError {
    #[error(transparent)]
    Internal(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl_from_error_for_route!(mas_storage::RepositoryError);

impl IntoResponse for RouteError {
    fn into_response(self) -> axum::response::Response {
        let error = ErrorResponse::from_error(&self);
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        (status, Json(error)).into_response()
    }
}

fn data_example() -> serde_json::Value {
    serde_json::json!({
        "hello": "world",
        "foo": 42,
        "bar": true
    })
}

/// # JSON payload for the `POST /api/admin/v1/policy-data`
#[derive(Deserialize, JsonSchema)]
#[serde(rename = "SetPolicyDataRequest")]
pub struct SetPolicyDataRequest {
    #[schemars(example = "data_example")]
    pub data: serde_json::Value,
}

pub fn doc(operation: TransformOperation) -> TransformOperation {
    operation
        .id("setPolicyData")
        .summary("Set the current policy data")
        .tag("policy-data")
        .response_with::<201, Json<SingleResponse<PolicyData>>, _>(|t| {
            let [sample, ..] = PolicyData::samples();
            let response = SingleResponse::new_canonical(sample);
            t.description("Policy data was successfully set")
                .example(response)
        })
}

#[tracing::instrument(name = "handler.admin.v1.policy_data.set", skip_all, err)]
pub async fn handler(
    CallContext {
        mut repo, clock, ..
    }: CallContext,
    NoApi(mut rng): NoApi<BoxRng>,
    Json(request): Json<SetPolicyDataRequest>,
) -> Result<(StatusCode, Json<SingleResponse<PolicyData>>), RouteError> {
    let policy_data = repo
        .policy_data()
        .set(&mut rng, &clock, request.data)
        .await?;

    repo.save().await?;

    Ok((
        StatusCode::CREATED,
        Json(SingleResponse::new_canonical(policy_data.into())),
    ))
}

#[cfg(test)]
mod tests {
    use hyper::{Request, StatusCode};
    use insta::assert_json_snapshot;
    use sqlx::PgPool;

    use crate::test_utils::{RequestBuilderExt, ResponseExt, TestState, setup};

    #[sqlx::test(migrator = "mas_storage_pg::MIGRATOR")]
    async fn test_create(pool: PgPool) {
        setup();
        let mut state = TestState::from_pool(pool).await.unwrap();
        let token = state.token_with_scope("urn:mas:admin").await;

        let request = Request::post("/api/admin/v1/policy-data")
            .bearer(&token)
            .json(serde_json::json!({
                "data": {
                    "hello": "world"
                }
            }));
        let response = state.request(request).await;
        response.assert_status(StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_json_snapshot!(body, @r###"
        {
          "data": {
            "type": "policy-data",
            "id": "01FSHN9AG0MZAA6S4AF7CTV32E",
            "attributes": {
              "created_at": "2022-01-16T14:40:00Z",
              "data": {
                "hello": "world"
              }
            },
            "links": {
              "self": "/api/admin/v1/policy-data/01FSHN9AG0MZAA6S4AF7CTV32E"
            }
          },
          "links": {
            "self": "/api/admin/v1/policy-data/01FSHN9AG0MZAA6S4AF7CTV32E"
          }
        }
        "###);
    }
}
