use crate::beacon_json::error::*;
use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::{FromRequest, RequestParts},
    http::{self, header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    BoxError, Error,
};
use serde::{de::DeserializeOwned, Serialize};
use std::ops::{Deref, DerefMut};

pub struct Json<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Json<T>
where
    T: DeserializeOwned,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = JsonRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // NOTE: implementation is forked from `axum::Json` for now...
        if json_content_type(req) {
            let bytes = Bytes::from_request(req)
                .await
                .map_err(|_| JsonRejection::BytesRejection)?;

            let value = match /*crate::beacon_json::serde::from_slice(&bytes) {*/
            serde_json::from_slice(&bytes) {
                Ok(value) => value,
                Err(err) => {
                    let rejection = match err.classify() {
                        serde_json::error::Category::Data => JsonDataError(Error::new(err)).into(),
                        serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
                            JsonSyntaxError(Error::new(err)).into()
                        }
                        serde_json::error::Category::Io => {
                            if cfg!(debug_assertions) {
                                // we don't use `serde_json::from_reader` and instead always buffer
                                // bodies first, so we shouldn't encounter any IO errors
                                unreachable!()
                            } else {
                                JsonSyntaxError(Error::new(err)).into()
                            }
                        }
                    };
                    return Err(rejection);
                }
            };

            Ok(Json(value))
        } else {
            Err(MissingJsonContentType.into())
        }
    }
}

fn json_content_type<B>(req: &RequestParts<B>) -> bool {
    let content_type = if let Some(content_type) = req.headers().get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"));

    is_json_content_type
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match crate::beacon_json::serde::to_vec(&self.0) {
            Ok(bytes) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                bytes,
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}
