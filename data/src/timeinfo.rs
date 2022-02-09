use ts_rs::TS;

use crate::rational::Rational64;
use std::any::Any;
use std::sync::Arc;

/// Timestamp information for frames and packets.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(TS)]
#[ts(export)]
pub struct TimeInfo {
    /// Presentation timestamp.
    pub pts: Option<i64>,
    /// Decode timestamp.
    pub dts: Option<i64>,
    /// Duration (in timebase units).
    pub duration: Option<u64>,
    /// Timebase numerator/denominator (i.e 1/75th of a second).
    ///
    /// Its value does not vary among frames/packets, since it is
    /// computed and defined at stream level.

    #[ts(type = "[number, number] | null")]
    pub timebase: Option<Rational64>,
    /// Timebase user private data.

    #[cfg_attr(feature = "serde", serde(skip))]
    #[ts(skip)]
    pub user_private: Option<Arc<dyn Any + Send + Sync>>,
}
