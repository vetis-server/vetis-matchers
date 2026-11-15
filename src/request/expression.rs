use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use vetis::{VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    expression: String,
}

impl MatcherConfigBuilder {
    /// Allow set a expression
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn expression(mut self, expression: &str) -> Self {
        self.expression = expression.into();
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig { expression: self.expression })
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// A struct with cel matcher configuration.
///
/// # Fields
///
/// * `expression` - CEL expression to evaluate.
/// ```
pub struct MatcherConfig {
    expression: String,
}

#[typetag::serde(name = "expression")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(ExpressionMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder() -> MatcherConfigBuilder {
        MatcherConfigBuilder { expression: "method == 'GET'".into() }
    }

    /// Return expression
    ///
    /// # Returns
    ///
    /// * `&String` - Rewrite mode.
    pub fn expression(&self) -> &str {
        &self.expression
    }
}

/// CEL expression matcher
pub struct ExpressionMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl ExpressionMatcher {
    /// Create a new CEL expression matcher
    pub fn new(config: MatcherConfig) -> Self {
        ExpressionMatcher { config, program: None }
    }
}

impl Matcher for ExpressionMatcher {
    fn compile(&mut self, engine: Arc<Env>) -> VetisResult<()> {
        self.program = Some(
            engine
                .compile(
                    &self
                        .config
                        .expression,
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
