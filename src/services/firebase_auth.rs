use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FirebaseClaims {
    pub sub: String,
    pub email: Option<String>,
    pub aud: String,
    pub exp: usize,
}

pub async fn verify_firebase_jwt(token: &str, project_id: &str) -> Result<FirebaseClaims, String> {
    // Get Firebase public keys
    let jwks_url =
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";
    let client = Client::new();
    let resp = client
        .get(jwks_url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

    // deserialize JSON from response
    let keys: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse JWKS: {}", e))?;

    // Get the kid from token header
    let header = decode_header(token).map_err(|e| format!("Invalid JWT header: {}", e))?;
    let kid = header.kid.ok_or("No 'kid' found in JWT header")?;

    // Get the key corresponding to this kid
    let key_pem = keys
        .get(&kid)
        .and_then(|v| v.as_str())
        .ok_or("Public key for kid not found")?;
    let decoding_key = DecodingKey::from_rsa_pem(key_pem.as_bytes())
        .map_err(|e| format!("Failed to create decoding key: {}", e))?;

    // Validate token
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[project_id]);
    validation.validate_exp = true;

    let token_data = decode::<FirebaseClaims>(token, &decoding_key, &validation)
        .map_err(|e| format!("JWT validation failed: {}", e))?;

    Ok(token_data.claims)
}
