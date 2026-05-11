use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE};
use serde_json::Value;

pub async fn poll_xbl_presence(api_key: &str) -> Option<String> {
    if api_key.trim().is_empty() {
        return None;
    }

    let mut headers = HeaderMap::new();
    if let Ok(val) = HeaderValue::from_str(api_key) {
        headers.insert("X-Authorization", val);
    } else {
        return None;
    }
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));

    let client = reqwest::Client::new();
    let res = client.get("https://xbl.io/api/v2/presence")
        .headers(headers)
        .send()
        .await
        .ok()?;

    if !res.status().is_success() {
        return None;
    }

    let json: Value = res.json().await.ok()?;
    
    // Extract rich presence from the API response
    if let Some(devices) = json["content"]["devices"].as_array() {
        for device in devices {
            if let Some(titles) = device["titles"].as_array() {
                for title in titles {
                    if let Some(activity) = title.get("activity") {
                        if let Some(rp) = activity.get("richPresence") {
                            if let Some(rp_str) = rp.as_str() {
                                if !rp_str.is_empty() {
                                    return Some(rp_str.to_string());
                                }
                            }
                        }
                    }
                    
                    // Some games put state directly under `state` or `name`
                    // We only care about richPresence mostly, but we can fallback to name if it represents an activity
                    // (But usually name is just the game name, so we skip it to avoid redundancy)
                }
            }
        }
    }

    // If no rich presence found, just return a generic state if it's not empty
    if let Some(state) = json["content"]["state"].as_str() {
        if state != "Offline" {
            // We only really want to return something if we got a valid rich presence.
            // Returning "Online" is not useful.
        }
    }

    None
}
