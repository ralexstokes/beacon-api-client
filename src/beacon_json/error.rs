use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::{FromRequest, RequestParts},
    http::{self, header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    BoxError, Error,
};

#[derive(Debug)]
pub struct JsonDataError(pub Error);

impl axum::response::IntoResponse for JsonDataError {
    fn into_response(self) -> axum::response::Response {
        (
            http::StatusCode::UNPROCESSABLE_ENTITY,
            format!(
                concat!(
                    "Failed to deserialize the JSON body into the target type",
                    ": {}"
                ),
                self.0
            ),
        )
            .into_response()
    }
}

impl std::fmt::Display for JsonDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to deserialize the JSON body into the target type"
        )
    }
}

impl std::error::Error for JsonDataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug)]
pub struct JsonSyntaxError(pub Error);

impl axum::response::IntoResponse for JsonSyntaxError {
    fn into_response(self) -> axum::response::Response {
        (
            http::StatusCode::BAD_REQUEST,
            format!(
                concat!("Failed to parse the request body as JSON", ": {}"),
                self.0
            ),
        )
            .into_response()
    }
}

impl std::fmt::Display for JsonSyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse the request body as JSON")
    }
}

impl std::error::Error for JsonSyntaxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug, Default)]
pub struct MissingJsonContentType;

impl axum::response::IntoResponse for MissingJsonContentType {
    fn into_response(self) -> axum::response::Response {
        (
            http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Expected request with `Content-Type: application/json`",
        )
            .into_response()
    }
}

impl std::fmt::Display for MissingJsonContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected request with `Content-Type: application/json`")
    }
}

impl std::error::Error for MissingJsonContentType {}

#[derive(Debug)]
pub enum JsonRejection {
    JsonDataError(JsonDataError),
    JsonSyntaxError(JsonSyntaxError),
    MissingJsonContentType(MissingJsonContentType),
    // TODO fill out
    BytesRejection,
}

impl axum::response::IntoResponse for JsonRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::JsonDataError(inner) => inner.into_response(),
            Self::JsonSyntaxError(inner) => inner.into_response(),
            Self::MissingJsonContentType(inner) => inner.into_response(),
            Self::BytesRejection => "failure".into_response(),
        }
    }
}

impl From<JsonDataError> for JsonRejection {
    fn from(inner: JsonDataError) -> Self {
        Self::JsonDataError(inner)
    }
}
impl From<JsonSyntaxError> for JsonRejection {
    fn from(inner: JsonSyntaxError) -> Self {
        Self::JsonSyntaxError(inner)
    }
}
impl From<MissingJsonContentType> for JsonRejection {
    fn from(inner: MissingJsonContentType) -> Self {
        Self::MissingJsonContentType(inner)
    }
}

impl std::fmt::Display for JsonRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonDataError(inner) => write!(f, "{}", inner),
            Self::JsonSyntaxError(inner) => write!(f, "{}", inner),
            Self::MissingJsonContentType(inner) => write!(f, "{}", inner),
            Self::BytesRejection => write!(f, "failure"),
        }
    }
}

impl std::error::Error for JsonRejection {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::JsonDataError(inner) => Some(inner),
            Self::JsonSyntaxError(inner) => Some(inner),
            Self::MissingJsonContentType(inner) => Some(inner),
            Self::BytesRejection => None,
        }
    }
}
