use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use vetis::{VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    field: String,
    value: String,
}

impl MatcherConfigBuilder {
    /// Allow set a field
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn field(mut self, field: &str) -> Self {
        self.field = field.into();
        self
    }

    /// Allow set a value
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn value(mut self, value: &str) -> Self {
        self.value = value.into();
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig { field: self.field, value: self.value })
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
    field: String,
    value: String,
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

#[typetag::serde(name = "url_pattern")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(UrlPatternMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder() -> MatcherConfigBuilder {
        MatcherConfigBuilder { field: "HOST".into(), value: "localhost".into() }
    }

    /// Return field
    ///
    /// # Returns
    ///
    /// * `&String` - Header field.
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Return value
    ///
    /// # Returns
    ///
    /// * `&String` - Header value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Header matcher
pub struct UrlPatternMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl UrlPatternMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        UrlPatternMatcher { config, program: None }
    }
}

impl Matcher for UrlPatternMatcher {
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
