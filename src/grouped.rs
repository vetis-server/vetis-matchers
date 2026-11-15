use std::sync::Arc;

use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use vetis::{VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    matchers: Vec<Box<dyn vetis::matcher::MatcherConfig>>,
}

impl MatcherConfigBuilder {
    /// Allow set a field
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_matcher<M>(mut self, matcher: M) -> Self
    where
        M: vetis::matcher::MatcherConfig + 'static,
    {
        self.matchers
            .push(Box::new(matcher));
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig {
            matchers: self
                .matchers
                .into_boxed_slice(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// A struct with header matcher configuration.
///
/// # Fields
///
/// * `pattern` - Target uri to regex to matchw with.
/// * `replacement` - The replacement uri.
/// ```
pub struct MatcherConfig {
    matchers: Box<[Box<dyn vetis::matcher::MatcherConfig>]>,
}

#[typetag::serde(name = "grouped")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(GroupedMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder() -> MatcherConfigBuilder {
        MatcherConfigBuilder { matchers: vec![] }
    }

    /// Return matchers
    ///
    /// # Returns
    ///
    /// * `&String` - Header field.
    pub fn matchers(&self) -> &Box<[Box<dyn vetis::matcher::MatcherConfig>]> {
        &self.matchers
    }
}

/// Header matcher
pub struct GroupedMatcher {
    _matchers: Box<[Box<dyn Matcher>]>,
    program: Option<Program>,
}

impl GroupedMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        GroupedMatcher {
            _matchers: config
                .matchers
                .iter()
                .map(|m| m.boxed_matcher())
                .collect(),
            program: None,
        }
    }
}

impl Matcher for GroupedMatcher {
    fn compile(&mut self, engine: Arc<Env>) -> VetisResult<()> {
        self.program = Some(
            engine
                .compile("")
                .map_err(|e| VetisError::matcher(e.to_compact_string()))?,
        );
        Ok(())
    }

    fn is_match(&self, context: &Context) -> bool {
        let Some(ref program) = self.program else {
            return false;
        };

        if let Ok(Bool(res)) = program.execute(&context) { res } else { false }
    }
}
