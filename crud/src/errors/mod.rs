use actix_web::{
  dev::{Body, ResponseBody, ServiceResponse},
  error,
  http::{header, HeaderValue, StatusCode},
  middleware::errhandlers::ErrorHandlerResponse,
  HttpResponse, Result,
};
use mongodb::bson::{de, document::ValueAccessError, ser};
use thiserror::Error;

pub fn handle_500<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<Body>> {
  let mut new_res = res.map_body(|_head, _body| {
    ResponseBody::Body(Body::Message(Box::new("Internal Server Error".to_string())))
  });
  new_res
    .headers_mut()
    .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
  Ok(ErrorHandlerResponse::Response(new_res))
}

pub fn handle_404<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<Body>> {
  let mut new_res = res.map_body(|_head, _body| {
    ResponseBody::Body(Body::Message(Box::new("Resource Not Found".to_string())))
  });
  new_res
    .headers_mut()
    .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
  Ok(ErrorHandlerResponse::Response(new_res))
}

#[derive(Error, Debug)]
pub enum UserError {
  #[error("{field} is invalid: {reason}")]
  ValidationError { field: String, reason: String },
  #[error("Internal Error occured: {cause}")]
  InternalError { cause: String },
  #[error("An Unknown Internal error occured")]
  UnknownError,
}

impl From<Box<dyn std::error::Error>> for UserError {
  fn from(_item: Box<dyn std::error::Error>) -> Self {
    UserError::UnknownError
  }
}

impl From<mongodb::error::Error> for UserError {
  fn from(item: mongodb::error::Error) -> Self {
    UserError::InternalError {
      cause: item.kind.to_string(),
    }
  }
}

impl From<ser::Error> for UserError {
  fn from(item: ser::Error) -> Self {
    UserError::InternalError {
      cause: item.to_string(),
    }
  }
}

impl From<de::Error> for UserError {
  fn from(item: de::Error) -> Self {
    UserError::InternalError {
      cause: item.to_string(),
    }
  }
}

impl From<ValueAccessError> for UserError {
  fn from(_item: ValueAccessError) -> UserError {
    UserError::InternalError {
      cause: "Malformed document".into(),
    }
  }
}

impl From<String> for UserError {
  fn from(item: String) -> UserError {
    UserError::InternalError { cause: item }
  }
}

impl From<&str> for UserError {
  fn from(item: &str) -> UserError {
    UserError::InternalError { cause: item.into() }
  }
}

impl error::ResponseError for UserError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code())
      .header(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"))
      .body(self.to_string())
  }

  fn status_code(&self) -> StatusCode {
    match *self {
      UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
      UserError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
      UserError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}
