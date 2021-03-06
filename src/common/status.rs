#[derive(Debug, Eq, PartialEq)]
pub enum StatusCode {
  OK = 0,
  OutOfMemory = 1,
  KeyError = 2,
  TypeError = 3,
  Invalid = 4,
  IOError = 5,

  UnknownError = 9,
  NotImplemented = 10,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ArrowError {
  code: StatusCode,
  posix_code: i16,
  message: String
}

impl ArrowError {
  pub fn out_of_memory(message: String) -> ArrowError {
    ArrowError::new(StatusCode::OutOfMemory, message, -1)
  }

  pub fn key_error(message: String) -> ArrowError {
    ArrowError::new(StatusCode::KeyError, message, -1)
  }

  pub fn type_error(message: String) -> ArrowError {
    ArrowError::new(StatusCode::TypeError, message, -1)
  }

  pub fn invalid(message: String) -> ArrowError {
    ArrowError::new(StatusCode::Invalid, message, -1)
  }

  pub fn io_error(message: String) -> ArrowError {
    ArrowError::new(StatusCode::IOError, message, -1)
  }

  pub fn unknown_error(message: String) -> ArrowError {
    ArrowError::new(StatusCode::UnknownError, message, -1)
  }

  pub fn not_implemented(message: String) -> ArrowError {
    ArrowError::new(StatusCode::NotImplemented, message, -1)
  }

  fn new(code: StatusCode, message: String, posix_code: i16) -> ArrowError {
    ArrowError {
      code: code,
      posix_code: posix_code,
      message: message
    }
  }

  pub fn code(&self) -> &StatusCode {
    &self.code
  }

  pub fn posix_code(&self) -> i16 {
    self.posix_code
  }

  pub fn message(&self) -> &String {
    &self.message
  }
}