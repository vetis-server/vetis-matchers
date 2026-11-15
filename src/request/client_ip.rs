use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use vetis::{Str, VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    ranges: Vec<Str>,
}

impl MatcherConfigBuilder {
    /// Allow set a field
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn range(mut self, range: &str) -> Self {
        self.ranges
            .push(range.to_compact_string());
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig {
            ranges: self
                .ranges
                .into_boxed_slice(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// A struct with request client ip matcher configuration.
///
/// # Field
///
/// * `ranges` - Target uri to regex to matchw with.
/// ```
pub struct MatcherConfig {
    ranges: Box<[Str]>,
}

impl ToString for MatcherConfig {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Debug for MatcherConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("client_ip")
            .field(
                &self
                    .ranges
                    .join(","),
            )
            .finish()
    }
}

#[typetag::serde(name = "client_ip")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(ClientIpMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder() -> MatcherConfigBuilder {
        MatcherConfigBuilder { ranges: vec!["192.168.0.1/24".to_compact_string()] }
    }

    /// Return field
    ///
    /// # Returns
    ///
    /// * `&Box<[Str]>` - Header field.
    pub fn ranges(&self) -> &Box<[Str]> {
        &self.ranges
    }
}

/// Header matcher
pub struct ClientIpMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl ClientIpMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        ClientIpMatcher { config, program: None }
    }
}

impl Matcher for ClientIpMatcher {
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
