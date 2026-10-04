use std::collections::HashMap;

use anyhow::{Result, bail};

/// Mock boundary for future asynchronous vault providers.
/// Never log the source, credentials, or returned values.
pub async fn fetch_secrets(source: &str) -> Result<HashMap<String, String>> {
    if source != "mock:demo" {
        bail!("unsupported secret source; this prototype only supports mock:demo");
    }
    Ok(HashMap::from([(
        "VEIL_ENV_DEMO_SECRET".to_owned(),
        "dummy-not-a-real-secret".to_owned(),
    )]))
}
