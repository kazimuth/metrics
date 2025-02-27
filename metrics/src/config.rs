use crate::Builder;
use std::time::Duration;

/// Holds the configuration for complex metric types.
#[derive(Clone, Debug)]
pub(crate) struct Configuration {
    pub histogram_window: Duration,
    pub histogram_granularity: Duration,
    pub upkeep_interval: Duration,
}

impl Configuration {
    pub fn from_builder(builder: &Builder) -> Self {
        Self {
            histogram_window: builder.histogram_window,
            histogram_granularity: builder.histogram_granularity,
            upkeep_interval: builder.upkeep_interval,
        }
    }
}
