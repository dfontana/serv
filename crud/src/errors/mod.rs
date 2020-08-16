use actix_web::{
  dev::{Body, ResponseBody, ServiceResponse},
  error,
  http::{header, HeaderValue, StatusCode},
  middleware::errhandlers::ErrorHandlerResponse,
  HttpResponse, Result,
};
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
  #[error("An Internal error occured")]
  InternalError,
}

impl From<Box<dyn std::error::Error>> for UserError {
  fn from(_item: Box<dyn std::error::Error>) -> Self {
    UserError::InternalError
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
      UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}
