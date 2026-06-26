use anyhow::Context;
use std::env;
use std::net::SocketAddr;

const DEFAULT_BIND_ADDR: &str = "0.0.0.0:3000";
pub struct Config {
    pub db_url: String,
    pub bind_addr: SocketAddr,
}

impl Config {
    pub fn from_getter(get: impl Fn(&str) -> Option<String>) -> anyhow::Result<Self> {
        let db_url = get("DATABASE_URL")
            .map(|url| url.trim().to_string())
            .filter(|v| !v.is_empty())
            .context("DATABASE_URL is empty")?;

        let bind_addr = get("BIND_ADDR")
            .map(|url| url.trim().to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_BIND_ADDR.to_string())
            .parse::<SocketAddr>()
            .context("BIND_ADDR must be a valid address, e.g. 0.0.0.0:3000")?;

        Ok(Config { db_url, bind_addr })
    }

    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_getter(|key| env::var(key).ok())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, DEFAULT_BIND_ADDR};
    use std::collections::HashMap;

    #[test]
    fn test_config_is_loaded_correctly() -> anyhow::Result<()> {
        let map: HashMap<&str, &str> = HashMap::from([
            ("DATABASE_URL", "postgresql://user:password@host:1234/test"),
            ("BIND_ADDR", "0.0.0.0:3000"),
        ]);

        let getter = |key: &str| map.get(key).map(|x| x.to_string());

        let config = Config::from_getter(getter)?;
        assert_eq!(config.db_url, "postgresql://user:password@host:1234/test");
        assert_eq!(config.bind_addr, "0.0.0.0:3000".parse().unwrap());
        Ok(())
    }

    #[test]
    fn test_config_is_loaded_correctly_with_trimming() -> anyhow::Result<()> {
        let map: HashMap<&str, &str> = HashMap::from([
            (
                "DATABASE_URL",
                "  postgresql://user:password@host:1234/test ",
            ),
            ("BIND_ADDR", "0.0.0.0:3000 "),
        ]);

        let getter = |key: &str| map.get(key).map(|x| x.to_string());

        let config = Config::from_getter(getter)?;
        assert_eq!(config.db_url, "postgresql://user:password@host:1234/test");
        assert_eq!(config.bind_addr, "0.0.0.0:3000".parse().unwrap());
        Ok(())
    }
    #[test]
    fn test_config_errors_out_on_empty_vars() -> anyhow::Result<()> {
        let map: HashMap<&str, &str> = HashMap::from([("DATABASE_URL", ""), ("BIND_ADDR", "")]);

        let getter = |key: &str| map.get(key).map(|x| x.to_string());

        assert!(Config::from_getter(getter).is_err());
        Ok(())
    }

    #[test]
    fn test_config_errors_out_on_undefined_vars() -> anyhow::Result<()> {
        let map: HashMap<&str, &str> = HashMap::new();
        let getter = |key: &str| map.get(key).map(|x| x.to_string());

        assert!(Config::from_getter(getter).is_err());
        Ok(())
    }
    #[test]
    fn test_config_defaults_to_a_sane_bind_addr() -> anyhow::Result<()> {
        let map: HashMap<&str, &str> =
            HashMap::from([("DATABASE_URL", "postgres"), ("BIND_ADDR", "")]);
        let getter = |key: &str| map.get(key).map(|x| x.to_string());

        assert_eq!(
            Config::from_getter(getter)?.bind_addr,
            DEFAULT_BIND_ADDR.parse().unwrap()
        );
        Ok(())
    }
    #[test]
    fn test_config_defaults_errors_out_on_invalid_addr() -> anyhow::Result<()> {
        let map: HashMap<&str, &str> = HashMap::from([
            ("DATABASE_URL", "postgres"),
            ("BIND_ADDR", "not-a-valid-case"),
        ]);
        let getter = |key: &str| map.get(key).map(|x| x.to_string());

        assert!(Config::from_getter(getter).is_err());
        Ok(())
    }
}
