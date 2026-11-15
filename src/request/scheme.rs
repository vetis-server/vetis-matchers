use std::{fmt::Debug, sync::Arc};

use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use vetis::{Str, VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    protos: Vec<Str>,
}

impl MatcherConfigBuilder {
    /// Allow add a proto
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_proto(mut self, proto: &str) -> Self {
        self.protos
            .push(proto.to_compact_string());
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig {
            protos: self
                .protos
                .into_boxed_slice(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// A struct with scheme matcher configuration.
///
/// # Field
///
/// * `protos` - Target uri to regex to matchw with.
/// ```
pub struct MatcherConfig {
    protos: Box<[Str]>,
}

impl ToString for MatcherConfig {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Debug for MatcherConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("scheme")
            .field(
                &self
                    .protos
                    .join(","),
            )
            .finish()
    }
}

#[typetag::serde(name = "scheme")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(SchemeMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder() -> MatcherConfigBuilder {
        MatcherConfigBuilder { protos: vec!["https".to_compact_string()] }
    }

    /// Return protos
    ///
    /// # Returns
    ///
    /// * `&Box<[Str]>` - Protocols versions.
    pub fn protos(&self) -> &Box<[Str]> {
        &self.protos
    }
}

/// Header matcher
pub struct SchemeMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl SchemeMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        SchemeMatcher { config, program: None }
    }
}

impl Matcher for SchemeMatcher {
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
