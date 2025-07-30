mod constants;
mod decode;
mod error;
mod message;
mod outlook;
mod storage;
mod stream;

pub use self::message::Outlook;
pub use self::outlook::{Attachment, Outlook as OldOutlook, Person, TransportHeaders};
pub use error::{DataTypeError, Error};
