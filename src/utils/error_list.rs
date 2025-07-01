#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorList {
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl ErrorList {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    pub fn new_with_error(error: &str) -> Self {
        let mut error_list = Self::new();
        error_list.add_error(error);
        error_list
    }
    pub fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }
    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
    pub fn view_errors(&self) -> Vec<String> {
        self.errors.clone()
    }
    pub fn view_warnings(&self) -> Vec<String> {
        self.warnings.clone()
    }
}

impl Default for ErrorList {
    fn default() -> Self {
        Self::new()
    }
}
