use std::{fmt::Debug, sync::Arc};

use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use vetis::{Str, VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    verbs: Vec<Str>,
}

impl MatcherConfigBuilder {
    /// Allow add a verb
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn verbs(mut self, verb: &str) -> Self {
        self.verbs
            .push(verb.to_compact_string());
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig {
            verbs: self
                .verbs
                .into_boxed_slice(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// A struct with request method matcher configuration.
///
/// # Field
///
/// * `verbs` - Target uri to regex to matchw with.
/// ```
pub struct MatcherConfig {
    verbs: Box<[Str]>,
}

impl ToString for MatcherConfig {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Debug for MatcherConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("method")
            .field(&self.verbs.join(","))
            .finish()
    }
}

#[typetag::serde(name = "method")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(MethodMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder() -> MatcherConfigBuilder {
        MatcherConfigBuilder { verbs: vec!["GET".to_compact_string()] }
    }

    /// Return verbs
    ///
    /// # Returns
    ///
    /// * `&Box<[Str]>` - Verbs.
    pub fn verbs(&self) -> &Box<[Str]> {
        &self.verbs
    }
}

/// Method matcher
pub struct MethodMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl MethodMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        MethodMatcher { config: config, program: None }
    }
}

impl Matcher for MethodMatcher {
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
