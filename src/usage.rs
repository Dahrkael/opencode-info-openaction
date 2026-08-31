use serde::Deserialize;

const USAGE_URL: &str = "https://opencode.ai/zen/go/v1/usage";

#[derive(Deserialize, Debug, Clone)]
pub struct Window {
    #[allow(dead_code)]
    pub status: String,
    pub percent: u8,
    #[allow(dead_code)]
    #[serde(default)]
    #[serde(rename = "resetsAt")]
    pub resets_at: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UsageData {
    pub rolling: Option<Window>,
    pub weekly: Option<Window>,
    pub monthly: Option<Window>,
}

#[derive(Deserialize, Debug, Clone)]
struct Response {
    usage: UsageData,
}

pub async fn fetch_usage(api_key: &str) -> Result<UsageData, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| format!("failed to build client: {e}"))?;

    let resp = client
        .get(USAGE_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("request failed: {e}"))?;

    let status = resp.status();
    if !status.is_success() {
        let msg: &str = match status.as_u16() {
            401 => "invalid or missing API key",
            403 => "no Go subscription on this key",
            429 => "rate limited",
            _ => "unexpected response",
        };
        return Err(format!("HTTP {}: {}", status, msg));
    }

    let body: Response = resp
        .json()
        .await
        .map_err(|e| format!("failed to parse response: {e}"))?;

    Ok(body.usage)
}

pub fn percent_by_name(usage: &UsageData, name: &str) -> Option<u8> {
    let w = match name {
        "5h" => usage.rolling.as_ref(),
        "week" => usage.weekly.as_ref(),
        "month" => usage.monthly.as_ref(),
        _ => return None,
    };
    w.map(|w| w.percent)
}
