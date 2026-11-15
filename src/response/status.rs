use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use vetis::{Str, VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    codes: Vec<Str>,
}

impl MatcherConfigBuilder {
    /// Allow set a response status code
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_code(mut self, code: &str) -> Self {
        self.codes
            .push(code.to_compact_string());
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig {
            codes: self
                .codes
                .into_boxed_slice(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// A struct with response status matcher configuration.
///
/// # Field
///
/// * `codes` - Target uri to regex to matchw with.
/// ```
pub struct MatcherConfig {
    codes: Box<[Str]>,
}

impl ToString for MatcherConfig {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Debug for MatcherConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("status")
            .field(&self.codes.join(","))
            .finish()
    }
}

#[typetag::serde(name = "status")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(StatusMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder() -> MatcherConfigBuilder {
        MatcherConfigBuilder { codes: vec![Str::const_new("200")] }
    }

    /// Return codes
    ///
    /// # Returns
    ///
    /// * `&Box<Str>` - Status code.
    pub fn codes(&self) -> &Box<[Str]> {
        &self.codes
    }
}

/// Header matcher
pub struct StatusMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl StatusMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        StatusMatcher { config, program: None }
    }
}

impl Matcher for StatusMatcher {
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
