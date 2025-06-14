// Autogenerated by Thrift Compiler (0.19.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::vec_box, clippy::wrong_self_convention)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use thrift::OrderedFloat;
use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSerializable, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

use crate::types;

/// * Numeric codes indicating the type of error that occurred on the
/// * service.
/// * <dl>
/// *   <dt>UNKNOWN</dt>
/// *     <dd>No information available about the error</dd>
/// *   <dt>BAD_DATA_FORMAT</dt>
/// *     <dd>The format of the request data was incorrect</dd>
/// *   <dt>PERMISSION_DENIED</dt>
/// *     <dd>Not permitted to perform action</dd>
/// *   <dt>INTERNAL_ERROR</dt>
/// *     <dd>Unexpected problem with the service</dd>
/// *   <dt>DATA_REQUIRED</dt>
/// *     <dd>A required parameter/field was absent</dd>
/// *   <dt>LIMIT_REACHED</dt>
/// *     <dd>Operation denied due to data model limit</dd>
/// *   <dt>QUOTA_REACHED</dt>
/// *     <dd>Operation denied due to user storage limit</dd>
/// *   <dt>INVALID_AUTH</dt>
/// *     <dd>Username and/or password incorrect</dd>
/// *   <dt>AUTH_EXPIRED</dt>
/// *     <dd>Authentication token expired</dd>
/// *   <dt>DATA_CONFLICT</dt>
/// *     <dd>Change denied due to data model conflict</dd>
/// *   <dt>ENML_VALIDATION</dt>
/// *     <dd>Content of submitted note was malformed</dd>
/// *   <dt>SHARD_UNAVAILABLE</dt>
/// *     <dd>Service shard with account data is temporarily down</dd>
/// *   <dt>LEN_TOO_SHORT</dt>
/// *     <dd>Operation denied due to data model limit, where something such
/// *         as a string length was too short</dd>
/// *   <dt>LEN_TOO_LONG</dt>
/// *     <dd>Operation denied due to data model limit, where something such
/// *         as a string length was too long</dd>
/// *   <dt>TOO_FEW</dt>
/// *     <dd>Operation denied due to data model limit, where there were
/// *         too few of something.</dd>
/// *   <dt>TOO_MANY</dt>
/// *     <dd>Operation denied due to data model limit, where there were
/// *         too many of something.</dd>
/// *   <dt>UNSUPPORTED_OPERATION</dt>
/// *     <dd>Operation denied because it is currently unsupported.</dd>
/// *   <dt>TAKEN_DOWN</dt>
/// *     <dd>Operation denied because access to the corresponding object is
/// *         prohibited in response to a take-down notice.</dd>
/// *   <dt>RATE_LIMIT_REACHED</dt>
/// *     <dd>Operation denied because the calling application has reached
/// *         its hourly API call limit for this user.</dd>
/// *   <dt>BUSINESS_SECURITY_LOGIN_REQUIRED</dt>
/// *     <dd>Access to a business account has been denied because the user must complete
/// *        additional steps in order to comply with business security requirements.</dd>
/// *   <dt>DEVICE_LIMIT_REACHED</dt>
/// *     <dd>Operation denied because the user has exceeded their maximum allowed
/// *        number of devices.</dd>
/// *   <dt>OPENID_ALREADY_TAKEN</dt>
/// *     <dd>Operation failed because the Open ID is already associated with another user.</dd>
/// *   <dt>INVALID_OPENID_TOKEN</dt>
/// *     <dd>Operation denied because the Open ID token is invalid. Please re-issue a valid
/// *        token.</dd>
/// *	 <dt>USER_NOT_REGISTERED</dt>
/// *     <dd>There is no Evernote user associated with this OpenID account,
/// *     	   and no Evernote user with a matching email</dd>
/// *	 <dt>USER_NOT_ASSOCIATED</dt>
/// *     <dd>There is no Evernote user associated with this OpenID account,
/// *		   but Evernote user with matching email exists</dd>
/// *	 <dt>USER_ALREADY_ASSOCIATED</dt>
/// * 	   <dd>Evernote user is already associated with this provider
/// *		   using a different email address.</dd>
/// *	 <dt>ACCOUNT_CLEAR</dt>
/// *     <dd>The user's account has been disabled. Clients should deal with this errorCode
/// *       by logging the user out and purging all locally saved content, including local
/// *       edits not yet pushed to the server.</dd>
/// *	 <dt>SSO_AUTHENTICATION_REQUIRED</dt>
///  *     <dd>SSO authentication is the only tyoe of authentication allowed for the user's
///  *     account. This error is thrown when the user attempts to authenticate by another
///   *     method (password, OpenId, etc).</dd>
///  * </dl>
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EDAMErrorCode(pub i32);

impl EDAMErrorCode {
  pub const UNKNOWN: EDAMErrorCode = EDAMErrorCode(1);
  pub const BAD_DATA_FORMAT: EDAMErrorCode = EDAMErrorCode(2);
  pub const PERMISSION_DENIED: EDAMErrorCode = EDAMErrorCode(3);
  pub const INTERNAL_ERROR: EDAMErrorCode = EDAMErrorCode(4);
  pub const DATA_REQUIRED: EDAMErrorCode = EDAMErrorCode(5);
  pub const LIMIT_REACHED: EDAMErrorCode = EDAMErrorCode(6);
  pub const QUOTA_REACHED: EDAMErrorCode = EDAMErrorCode(7);
  pub const INVALID_AUTH: EDAMErrorCode = EDAMErrorCode(8);
  pub const AUTH_EXPIRED: EDAMErrorCode = EDAMErrorCode(9);
  pub const DATA_CONFLICT: EDAMErrorCode = EDAMErrorCode(10);
  pub const ENML_VALIDATION: EDAMErrorCode = EDAMErrorCode(11);
  pub const SHARD_UNAVAILABLE: EDAMErrorCode = EDAMErrorCode(12);
  pub const LEN_TOO_SHORT: EDAMErrorCode = EDAMErrorCode(13);
  pub const LEN_TOO_LONG: EDAMErrorCode = EDAMErrorCode(14);
  pub const TOO_FEW: EDAMErrorCode = EDAMErrorCode(15);
  pub const TOO_MANY: EDAMErrorCode = EDAMErrorCode(16);
  pub const UNSUPPORTED_OPERATION: EDAMErrorCode = EDAMErrorCode(17);
  pub const TAKEN_DOWN: EDAMErrorCode = EDAMErrorCode(18);
  pub const RATE_LIMIT_REACHED: EDAMErrorCode = EDAMErrorCode(19);
  pub const BUSINESS_SECURITY_LOGIN_REQUIRED: EDAMErrorCode = EDAMErrorCode(20);
  pub const DEVICE_LIMIT_REACHED: EDAMErrorCode = EDAMErrorCode(21);
  pub const OPENID_ALREADY_TAKEN: EDAMErrorCode = EDAMErrorCode(22);
  pub const INVALID_OPENID_TOKEN: EDAMErrorCode = EDAMErrorCode(23);
  pub const USER_NOT_ASSOCIATED: EDAMErrorCode = EDAMErrorCode(24);
  pub const USER_NOT_REGISTERED: EDAMErrorCode = EDAMErrorCode(25);
  pub const USER_ALREADY_ASSOCIATED: EDAMErrorCode = EDAMErrorCode(26);
  pub const ACCOUNT_CLEAR: EDAMErrorCode = EDAMErrorCode(27);
  pub const SSO_AUTHENTICATION_REQUIRED: EDAMErrorCode = EDAMErrorCode(28);
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::UNKNOWN,
    Self::BAD_DATA_FORMAT,
    Self::PERMISSION_DENIED,
    Self::INTERNAL_ERROR,
    Self::DATA_REQUIRED,
    Self::LIMIT_REACHED,
    Self::QUOTA_REACHED,
    Self::INVALID_AUTH,
    Self::AUTH_EXPIRED,
    Self::DATA_CONFLICT,
    Self::ENML_VALIDATION,
    Self::SHARD_UNAVAILABLE,
    Self::LEN_TOO_SHORT,
    Self::LEN_TOO_LONG,
    Self::TOO_FEW,
    Self::TOO_MANY,
    Self::UNSUPPORTED_OPERATION,
    Self::TAKEN_DOWN,
    Self::RATE_LIMIT_REACHED,
    Self::BUSINESS_SECURITY_LOGIN_REQUIRED,
    Self::DEVICE_LIMIT_REACHED,
    Self::OPENID_ALREADY_TAKEN,
    Self::INVALID_OPENID_TOKEN,
    Self::USER_NOT_ASSOCIATED,
    Self::USER_NOT_REGISTERED,
    Self::USER_ALREADY_ASSOCIATED,
    Self::ACCOUNT_CLEAR,
    Self::SSO_AUTHENTICATION_REQUIRED,
  ];
}

impl TSerializable for EDAMErrorCode {
  #[allow(clippy::trivially_copy_pass_by_ref)]
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    o_prot.write_i32(self.0)
  }
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<EDAMErrorCode> {
    let enum_value = i_prot.read_i32()?;
    Ok(EDAMErrorCode::from(enum_value))
  }
}

impl From<i32> for EDAMErrorCode {
  fn from(i: i32) -> Self {
    match i {
      1 => EDAMErrorCode::UNKNOWN,
      2 => EDAMErrorCode::BAD_DATA_FORMAT,
      3 => EDAMErrorCode::PERMISSION_DENIED,
      4 => EDAMErrorCode::INTERNAL_ERROR,
      5 => EDAMErrorCode::DATA_REQUIRED,
      6 => EDAMErrorCode::LIMIT_REACHED,
      7 => EDAMErrorCode::QUOTA_REACHED,
      8 => EDAMErrorCode::INVALID_AUTH,
      9 => EDAMErrorCode::AUTH_EXPIRED,
      10 => EDAMErrorCode::DATA_CONFLICT,
      11 => EDAMErrorCode::ENML_VALIDATION,
      12 => EDAMErrorCode::SHARD_UNAVAILABLE,
      13 => EDAMErrorCode::LEN_TOO_SHORT,
      14 => EDAMErrorCode::LEN_TOO_LONG,
      15 => EDAMErrorCode::TOO_FEW,
      16 => EDAMErrorCode::TOO_MANY,
      17 => EDAMErrorCode::UNSUPPORTED_OPERATION,
      18 => EDAMErrorCode::TAKEN_DOWN,
      19 => EDAMErrorCode::RATE_LIMIT_REACHED,
      20 => EDAMErrorCode::BUSINESS_SECURITY_LOGIN_REQUIRED,
      21 => EDAMErrorCode::DEVICE_LIMIT_REACHED,
      22 => EDAMErrorCode::OPENID_ALREADY_TAKEN,
      23 => EDAMErrorCode::INVALID_OPENID_TOKEN,
      24 => EDAMErrorCode::USER_NOT_ASSOCIATED,
      25 => EDAMErrorCode::USER_NOT_REGISTERED,
      26 => EDAMErrorCode::USER_ALREADY_ASSOCIATED,
      27 => EDAMErrorCode::ACCOUNT_CLEAR,
      28 => EDAMErrorCode::SSO_AUTHENTICATION_REQUIRED,
      _ => EDAMErrorCode(i)
    }
  }
}

impl From<&i32> for EDAMErrorCode {
  fn from(i: &i32) -> Self {
    EDAMErrorCode::from(*i)
  }
}

impl From<EDAMErrorCode> for i32 {
  fn from(e: EDAMErrorCode) -> i32 {
    e.0
  }
}

impl From<&EDAMErrorCode> for i32 {
  fn from(e: &EDAMErrorCode) -> i32 {
    e.0
  }
}

/// An enumeration that provides a reason for why a given contact was invalid, for example,
/// as thrown via an EDAMInvalidContactsException.
/// 
/// <dl>
///   <dt>BAD_ADDRESS</dt>
///     <dd>The contact information does not represent a valid address for a recipient.
///         Clients should be validating and normalizing contacts, so receiving this
///         error code commonly represents a client error.
///         </dd>
///   <dt>DUPLICATE_CONTACT</dt>
///     <dd>If the method throwing this exception accepts a list of contacts, this error
///         code indicates that the given contact is a duplicate of another contact in
///         the list.  Note that the server may clean up contacts, and that this cleanup
///         occurs before checking for duplication.  Receiving this error is commonly
///         an indication of a client issue, since client should be normalizing contacts
///         and removing duplicates. All instances that are duplicates are returned.  For
///         example, if a list of 5 contacts has the same e-mail address twice, the two
///         conflicting e-mail address contacts will be returned.
///         </dd>
///   <dt>NO_CONNECTION</dt>
///     <dd>Indicates that the given contact, an Evernote type contact, is not connected
///         to the user for which the call is being made. It is possible that clients are
///         out of sync with the server and should re-synchronize their identities and
///         business user state. See Identity.userConnected for more information on user
///         connections.
///         </dd>
/// </dl>
/// 
/// Note that if multiple reasons may apply, only one is returned. The precedence order
/// is BAD_ADDRESS, DUPLICATE_CONTACT, NO_CONNECTION, meaning that if a contact has a bad
/// address and is also duplicated, it will be returned as a BAD_ADDRESS.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EDAMInvalidContactReason(pub i32);

impl EDAMInvalidContactReason {
  pub const BAD_ADDRESS: EDAMInvalidContactReason = EDAMInvalidContactReason(0);
  pub const DUPLICATE_CONTACT: EDAMInvalidContactReason = EDAMInvalidContactReason(1);
  pub const NO_CONNECTION: EDAMInvalidContactReason = EDAMInvalidContactReason(2);
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::BAD_ADDRESS,
    Self::DUPLICATE_CONTACT,
    Self::NO_CONNECTION,
  ];
}

impl TSerializable for EDAMInvalidContactReason {
  #[allow(clippy::trivially_copy_pass_by_ref)]
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    o_prot.write_i32(self.0)
  }
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<EDAMInvalidContactReason> {
    let enum_value = i_prot.read_i32()?;
    Ok(EDAMInvalidContactReason::from(enum_value))
  }
}

impl From<i32> for EDAMInvalidContactReason {
  fn from(i: i32) -> Self {
    match i {
      0 => EDAMInvalidContactReason::BAD_ADDRESS,
      1 => EDAMInvalidContactReason::DUPLICATE_CONTACT,
      2 => EDAMInvalidContactReason::NO_CONNECTION,
      _ => EDAMInvalidContactReason(i)
    }
  }
}

impl From<&i32> for EDAMInvalidContactReason {
  fn from(i: &i32) -> Self {
    EDAMInvalidContactReason::from(*i)
  }
}

impl From<EDAMInvalidContactReason> for i32 {
  fn from(e: EDAMInvalidContactReason) -> i32 {
    e.0
  }
}

impl From<&EDAMInvalidContactReason> for i32 {
  fn from(e: &EDAMInvalidContactReason) -> i32 {
    e.0
  }
}

//
// EDAMUserException
//

/// This exception is thrown by EDAM procedures when a call fails as a result of
/// a problem that a caller may be able to resolve.  For example, if the user
/// attempts to add a note to their account which would exceed their storage
/// quota, this type of exception may be thrown to indicate the source of the
/// error so that they can choose an alternate action.
/// 
/// This exception would not be used for internal system errors that do not
/// reflect user actions, but rather reflect a problem within the service that
/// the user cannot resolve.
/// 
/// errorCode:  The numeric code indicating the type of error that occurred.
///   must be one of the values of EDAMErrorCode.
/// 
/// parameter:  If the error applied to a particular input parameter, this will
///   indicate which parameter. For some errors (USER_NOT_ASSOCIATED, USER_NOT_REGISTERED,
///   SSO_AUTHENTICATION_REQUIRED), this is the user's email.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EDAMUserException {
  pub error_code: EDAMErrorCode,
  pub parameter: Option<String>,
}

impl EDAMUserException {
  pub fn new<F2>(error_code: EDAMErrorCode, parameter: F2) -> EDAMUserException where F2: Into<Option<String>> {
    EDAMUserException {
      error_code,
      parameter: parameter.into(),
    }
  }
}

impl TSerializable for EDAMUserException {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<EDAMUserException> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<EDAMErrorCode> = None;
    let mut f_2: Option<String> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = EDAMErrorCode::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("EDAMUserException.error_code", &f_1)?;
    let ret = EDAMUserException {
      error_code: f_1.expect("auto-generated code should have checked for presence of required fields"),
      parameter: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("EDAMUserException");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("errorCode", TType::I32, 1))?;
    self.error_code.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.parameter {
      o_prot.write_field_begin(&TFieldIdentifier::new("parameter", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Error for EDAMUserException {}

impl From<EDAMUserException> for thrift::Error {
  fn from(e: EDAMUserException) -> Self {
    thrift::Error::User(Box::new(e))
  }
}

impl Display for EDAMUserException {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "remote service threw EDAMUserException")
  }
}

//
// EDAMSystemException
//

/// This exception is thrown by EDAM procedures when a call fails as a result of
/// a problem in the service that could not be changed through caller action.
/// 
/// errorCode:  The numeric code indicating the type of error that occurred.
///   must be one of the values of EDAMErrorCode.
/// 
/// message:  This may contain additional information about the error
/// 
/// rateLimitDuration:  Indicates the minimum number of seconds that an application should
///   expect subsequent API calls for this user to fail. The application should not retry
///   API requests for the user until at least this many seconds have passed. Present only
///   when errorCode is RATE_LIMIT_REACHED,
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EDAMSystemException {
  pub error_code: EDAMErrorCode,
  pub message: Option<String>,
  pub rate_limit_duration: Option<i32>,
}

impl EDAMSystemException {
  pub fn new<F2, F3>(error_code: EDAMErrorCode, message: F2, rate_limit_duration: F3) -> EDAMSystemException where F2: Into<Option<String>>, F3: Into<Option<i32>> {
    EDAMSystemException {
      error_code,
      message: message.into(),
      rate_limit_duration: rate_limit_duration.into(),
    }
  }
}

impl TSerializable for EDAMSystemException {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<EDAMSystemException> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<EDAMErrorCode> = None;
    let mut f_2: Option<String> = None;
    let mut f_3: Option<i32> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = EDAMErrorCode::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_i32()?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("EDAMSystemException.error_code", &f_1)?;
    let ret = EDAMSystemException {
      error_code: f_1.expect("auto-generated code should have checked for presence of required fields"),
      message: f_2,
      rate_limit_duration: f_3,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("EDAMSystemException");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("errorCode", TType::I32, 1))?;
    self.error_code.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.message {
      o_prot.write_field_begin(&TFieldIdentifier::new("message", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(fld_var) = self.rate_limit_duration {
      o_prot.write_field_begin(&TFieldIdentifier::new("rateLimitDuration", TType::I32, 3))?;
      o_prot.write_i32(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Error for EDAMSystemException {}

impl From<EDAMSystemException> for thrift::Error {
  fn from(e: EDAMSystemException) -> Self {
    thrift::Error::User(Box::new(e))
  }
}

impl Display for EDAMSystemException {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "remote service threw EDAMSystemException")
  }
}

//
// EDAMNotFoundException
//

/// This exception is thrown by EDAM procedures when a caller asks to perform
/// an operation on an object that does not exist.  This may be thrown based on an invalid
/// primary identifier (e.g. a bad GUID), or when the caller refers to an object
/// by another unique identifier (e.g. a User's email address).
/// 
/// identifier:  A description of the object that was not found on the server.
///   For example, "Note.notebookGuid" when a caller attempts to create a note in a
///   notebook that does not exist in the user's account.
/// 
/// key:  The value passed from the client in the identifier, which was not
///   found. For example, the GUID that was not found.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EDAMNotFoundException {
  pub identifier: Option<String>,
  pub key: Option<String>,
}

impl EDAMNotFoundException {
  pub fn new<F1, F2>(identifier: F1, key: F2) -> EDAMNotFoundException where F1: Into<Option<String>>, F2: Into<Option<String>> {
    EDAMNotFoundException {
      identifier: identifier.into(),
      key: key.into(),
    }
  }
}

impl TSerializable for EDAMNotFoundException {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<EDAMNotFoundException> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = None;
    let mut f_2: Option<String> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = EDAMNotFoundException {
      identifier: f_1,
      key: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("EDAMNotFoundException");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.identifier {
      o_prot.write_field_begin(&TFieldIdentifier::new("identifier", TType::String, 1))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.key {
      o_prot.write_field_begin(&TFieldIdentifier::new("key", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Error for EDAMNotFoundException {}

impl From<EDAMNotFoundException> for thrift::Error {
  fn from(e: EDAMNotFoundException) -> Self {
    thrift::Error::User(Box::new(e))
  }
}

impl Display for EDAMNotFoundException {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "remote service threw EDAMNotFoundException")
  }
}

//
// EDAMInvalidContactsException
//

/// An exception thrown when the provided Contacts fail validation. For instance,
/// email domains could be invalid, phone numbers might not be valid for SMS,
/// etc.
/// 
/// We will not provide individual reasons for each Contact's validation failure.
/// The presence of the Contact in this exception means that the user must figure
/// out how to take appropriate action to fix this Contact.
/// 
/// <dl>
///   <dt>contacts</dt>
///   <dd>The list of Contacts that are considered invalid by the service</dd>
/// 
///   <dt>parameter</dt>
///   <dd>If the error applied to a particular input parameter, this will
///   indicate which parameter.</dd>
/// 
///   <dt>reasons</dt>
///   <dd>If supplied, the list of reasons why the server considered a contact invalid,
///   matching, in order, the list returned in the contacts field.</dd>
/// </dl>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EDAMInvalidContactsException {
  pub contacts: Vec<types::Contact>,
  pub parameter: Option<String>,
  pub reasons: Option<Vec<EDAMInvalidContactReason>>,
}

impl EDAMInvalidContactsException {
  pub fn new<F2, F3>(contacts: Vec<types::Contact>, parameter: F2, reasons: F3) -> EDAMInvalidContactsException where F2: Into<Option<String>>, F3: Into<Option<Vec<EDAMInvalidContactReason>>> {
    EDAMInvalidContactsException {
      contacts,
      parameter: parameter.into(),
      reasons: reasons.into(),
    }
  }
}

impl TSerializable for EDAMInvalidContactsException {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<EDAMInvalidContactsException> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<types::Contact>> = None;
    let mut f_2: Option<String> = None;
    let mut f_3: Option<Vec<EDAMInvalidContactReason>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<types::Contact> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_0 = types::Contact::read_from_in_protocol(i_prot)?;
            val.push(list_elem_0);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        3 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<EDAMInvalidContactReason> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_1 = EDAMInvalidContactReason::read_from_in_protocol(i_prot)?;
            val.push(list_elem_1);
          }
          i_prot.read_list_end()?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("EDAMInvalidContactsException.contacts", &f_1)?;
    let ret = EDAMInvalidContactsException {
      contacts: f_1.expect("auto-generated code should have checked for presence of required fields"),
      parameter: f_2,
      reasons: f_3,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("EDAMInvalidContactsException");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("contacts", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::Struct, self.contacts.len() as i32))?;
    for e in &self.contacts {
      e.write_to_out_protocol(o_prot)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.parameter {
      o_prot.write_field_begin(&TFieldIdentifier::new("parameter", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.reasons {
      o_prot.write_field_begin(&TFieldIdentifier::new("reasons", TType::List, 3))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I32, fld_var.len() as i32))?;
      for e in fld_var {
        e.write_to_out_protocol(o_prot)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Error for EDAMInvalidContactsException {}

impl From<EDAMInvalidContactsException> for thrift::Error {
  fn from(e: EDAMInvalidContactsException) -> Self {
    thrift::Error::User(Box::new(e))
  }
}

impl Display for EDAMInvalidContactsException {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "remote service threw EDAMInvalidContactsException")
  }
}

