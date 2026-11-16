use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use vetis::{Str, VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder<'a> {
    pattern: &'a str,
}

impl<'a> MatcherConfigBuilder<'a> {
    /// Allow set a pattern
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn pattern(mut self, pattern: &'a str) -> Self {
        self.pattern = pattern;
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig {
            pattern: self
                .pattern
                .to_compact_string(),
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
    pattern: Str,
}

impl ToString for MatcherConfig {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Debug for MatcherConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("path_regex")
            .field(&self.pattern)
            .finish()
    }
}

#[typetag::serde(name = "path_regex")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(PathRegexMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder<'a>() -> MatcherConfigBuilder<'a> {
        MatcherConfigBuilder { pattern: "/api/,*" }
    }

    /// Return pattern
    ///
    /// # Returns
    ///
    /// * `&String` - Path pattern.
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

/// Header matcher
pub struct PathRegexMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl PathRegexMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        PathRegexMatcher { config, program: None }
    }
}

impl Matcher for PathRegexMatcher {
    fn compile(&mut self, engine: Arc<Env>) -> VetisResult<()> {
        self.program = Some(
            engine
                .compile(
                    &self
                        .config
                        .to_string(),
                )
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
