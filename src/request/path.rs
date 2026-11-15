use std::sync::Arc;

use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use vetis::{VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    paths: Vec<String>,
}

impl MatcherConfigBuilder {
    /// Allow set a path
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_path(mut self, path: &str) -> Self {
        self.paths
            .push(path.to_owned());
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig { paths: self.paths })
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
    paths: Vec<String>,
}

#[typetag::serde(name = "path")]
impl vetis::matcher::MatcherConfig for MatcherConfig {
    fn boxed_clone(&self) -> Box<dyn vetis::matcher::MatcherConfig> {
        Box::new(self.clone())
    }

    fn boxed_matcher(&self) -> Box<dyn Matcher> {
        Box::new(PathMatcher::new(self.clone()))
    }
}

impl MatcherConfig {
    /// Creates a new `MatcherConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `MatcherConfigBuilder` - The builder.
    pub fn builder() -> MatcherConfigBuilder {
        MatcherConfigBuilder { paths: vec![] }
    }

    /// Return paths
    ///
    /// # Returns
    ///
    /// * `&String` - Header field.
    pub fn paths(&self) -> &Vec<String> {
        &self.paths
    }

    /// Return mutable paths
    ///
    /// # Returns
    ///
    /// * `&Vec<String>` - List of paths.
    pub fn paths_mut(&mut self) -> &mut Vec<String> {
        &mut self.paths
    }
}

/// Header matcher
pub struct PathMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl PathMatcher {
    /// Create a new header matcher
    pub fn new(config: MatcherConfig) -> Self {
        PathMatcher { config, program: None }
    }
}

impl Matcher for PathMatcher {
    fn compile(&mut self, engine: Arc<Env>) -> VetisResult<()> {
        let paths = self
            .config
            .paths_mut();
        paths.sort_by(|a, b| b.cmp(a));
        self.program = Some(
            engine
                .compile(&format!(
                    "path({:?})",
                    &self
                        .config
                        .paths()
                        .join(",")
                ))
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
