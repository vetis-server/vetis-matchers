use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use vetis::{Str, VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder<'a> {
    field: &'a str,
    value: &'a str,
}

impl<'a> MatcherConfigBuilder<'a> {
    /// Allow set a field
    ///
    /// # Arguments
    ///
    /// * `&str` = Field name
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn field(mut self, field: &'a str) -> Self {
        self.field = field.into();
        self
    }

    /// Allow set a value
    ///
    /// # Arguments
    ///
    /// * `&str` = Field value
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = value.into();
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig {
            field: self
                .field
                .to_compact_string(),
            value: self
                .value
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
    field: Str,
    value: Str,
}

impl ToString for MatcherConfig {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Debug for MatcherConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("header")
            .field(&self.field)
            .field(&self.value)
            .finish()
    }
}

#[typetag::serde(name = "header_regex")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(HeaderRegexMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder<'a>() -> MatcherConfigBuilder<'a> {
        MatcherConfigBuilder { field: "HOST", value: "localhost" }
    }

    /// Return field
    ///
    /// # Returns
    ///
    /// * `&str` - Header field.
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Return value
    ///
    /// # Returns
    ///
    /// * `&str` - Header value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Header matcher
pub struct HeaderRegexMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl HeaderRegexMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        HeaderRegexMatcher { config, program: None }
    }
}

impl Matcher for HeaderRegexMatcher {
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
