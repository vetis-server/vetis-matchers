use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use vetis::{Str, VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    specs: Vec<Str>,
}

impl MatcherConfigBuilder {
    /// Allow set a version spec
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_spec(mut self, spec: &str) -> Self {
        self.specs
            .push(spec.to_compact_string());
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig {
            specs: self
                .specs
                .into_boxed_slice(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// A struct with protocol version matcher configuration.
///
/// # Field
///
/// * `specs` - A list of comma separated protocol versions.
/// ```
pub struct MatcherConfig {
    specs: Box<[Str]>,
}

impl ToString for MatcherConfig {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Debug for MatcherConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("version")
            .field(&self.specs.join(","))
            .finish()
    }
}

#[typetag::serde(name = "version")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(VersionMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder() -> MatcherConfigBuilder {
        MatcherConfigBuilder { specs: vec!["HTTP/1.1".to_compact_string()] }
    }

    /// Return spec
    ///
    /// # Returns
    ///
    /// * `&Box[Str]>` - Header field.
    pub fn specs(&self) -> &Box<[Str]> {
        &self.specs
    }
}

/// Header matcher
pub struct VersionMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl VersionMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        VersionMatcher { config, program: None }
    }
}

impl Matcher for VersionMatcher {
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
