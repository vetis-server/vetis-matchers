#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
/// Expression support
pub mod expr;
/// Grouped matched
pub(crate) mod grouped;
/// Request matchers
pub mod request;
/// Response matchers
pub mod response;

#[cfg(test)]
mod tests;

pub use grouped::{
    GroupedMatcher, MatcherConfig as GroupedMatcherConfig,
    MatcherConfigBuilder as GroupedMatcherConfigBuilder,
};
