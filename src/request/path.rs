use cel::{Context, Env, Program, Value::Bool};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use vetis::{Str, VetisResult, errors::VetisError, matcher::Matcher};

/// Matcher config builder
pub struct MatcherConfigBuilder {
    paths: Vec<Str>,
}

impl MatcherConfigBuilder {
    /// Allow set a path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to add to matcher
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_path(mut self, path: &str) -> Self {
        self.paths
            .push(path.to_compact_string());
        self
    }

    /// Build the `MatcherConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<MatcherConfig>` - The `MatcherConfig` with the configured settings.
    pub fn build(self) -> VetisResult<MatcherConfig> {
        Ok(MatcherConfig {
            paths: self
                .paths
                .into_boxed_slice(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
/// A struct with path matcher configuration.
///
/// # Fields
///
/// * `paths` - Paths to match request with.
/// ```
pub struct MatcherConfig {
    paths: Box<[Str]>,
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
    /// * `&Box<[Str]> ` - List of paths.
    pub fn paths(&self) -> &Box<[Str]> {
        &self.paths
    }

    /// Return mutable paths
    ///
    /// # Returns
    ///
    /// * `&mut Box<[Str]>` - Mutable list of paths.
    pub fn paths_mut(&mut self) -> &mut Box<[Str]> {
        &mut self.paths
    }
}

/// Header matcher
pub struct PathMatcher {
    config: MatcherConfig,
    program: Option<Program>,
}

impl PathMatcher {
    /// Create a new path matcher
    ///
    /// # Arguments
    ///
    /// * `config` - Matcher configuration
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
