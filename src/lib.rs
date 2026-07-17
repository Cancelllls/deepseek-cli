pub struct Config {
    pub api_key: String,
    pub model: String,
}

impl Config {
    pub fn new(api_key: &str, model: &str) -> Result<Self, &'static str> {
        if api_key.is_empty() {
            return Err("API key cannot be empty");
        }
        if model.is_empty() {
            return("Model cannot be empty");
        }
        Ok(Config {
            api_key: api_key.to_string(),
            model: model.to_string(),
        })
    }
}

pub fn run_query(config: &Config,: &str) -> Result<String, &'static str> {
    if prompt.trim().is_empty() {
        return Err("Prompt cannot be empty");
    }
    // Placeholder for actual API call
    Ok(format!("Response to '{}' using model {} with key {}", prompt, config.model, config.api_key))
}
