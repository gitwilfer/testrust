use std::env;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    Testing,
    Staging,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Testing => "testing",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
    
    pub fn from_env() -> Self {
        let env_str = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        Self::from_str(&env_str).unwrap_or(Environment::Development)
    }
    
    pub fn is_dev(&self) -> bool {
        *self == Environment::Development
    }
    
    pub fn is_prod(&self) -> bool {
        *self == Environment::Production
    }
}

impl FromStr for Environment {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "testing" | "test" => Ok(Environment::Testing),
            "staging" => Ok(Environment::Staging),
            "production" | "prod" => Ok(Environment::Production),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}