use ts_rs::TS;

use crate::data::params::CodecParams;
use crate::rational::Rational64;
use std::any::Any;
use std::sync::Arc;

/// Stream data.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(TS)]
#[ts(export)]
pub struct Stream {
    /// Format-specific track identifier.
    ///
    /// If negative, either the stream is not supported by the
    /// underlying format or the default progression should be used.
    ///
    /// Must be unique for each stream.
    pub id: isize,
    /// Stream position within the source file.
    pub index: usize,
    /// Codec parameters of the stream.
    pub params: CodecParams,
    /// Start position of the stream.
    ///
    /// If `None`, start position of the stream is not considered.
    pub start: Option<u64>,
    /// Stream duration.
    ///
    /// If `None`, stream duration is not considered.
    pub duration: Option<u64>,
    /// Timebase numerator/denominator.
    #[ts(type = "[number, number]")]
    pub timebase: Rational64,
    /// User private data.
    ///
    /// This data cannot be cloned.
    #[cfg_attr(feature = "serde", serde(skip))]
    #[ts(skip)]
    pub user_private: Option<Arc<dyn Any + Send + Sync>>,
}

impl Stream {
    /// Creates a new `Stream` instance from codec parameters.
    pub fn from_params(params: &CodecParams, timebase: Rational64) -> Self {
        Stream {
            id: -1,
            index: 0,
            params: params.clone(),
            start: None,
            duration: None,
            timebase,
            user_private: None,
        }
    }
    /// Returns extradata associated to the codec parameters of a stream.
    pub fn get_extradata(&self) -> Option<&[u8]> {
        self.params.extradata.as_deref()
    }
}

/// Group of streams.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct StreamGroup<'a> {
    /// Stream group ID.
    ///
    /// Must be unique for each group of stream.
    pub id: usize,
    /// Start position of the stream group.
    pub start: u64,
    /// End position of the stream group.
    pub end: u64,
    /// Streams of the group.
    pub streams: &'a [Stream],
}
