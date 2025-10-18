use crate::services::firebase_auth::verify_firebase_jwt;
use axum::{extract::Query, http::HeaderMap, http::StatusCode, response::Json};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Peak {
    pub id: String,
    pub image_url: String,
}

async fn fetch_peaks_from_db(email: Option<String>, page: usize, limit: usize) -> Vec<Peak> {
    let Some(email) = email else {
        return vec![];
    };

    let mut peaks = Vec::new();
    for i in 0..limit {
        peaks.push(Peak {
            id: format!("peak_{}", page * limit + i),
            image_url: format!(
                "https://storage.googleapis.com/{email}/peak_{}.jpg",
                page * limit + i
            ),
        });
    }
    peaks
}

pub async fn get_peaks(
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Peak>>, (StatusCode, String)> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing Authorization header".to_string(),
        ))?;

    let token = auth_header.strip_prefix("Bearer ").ok_or((
        StatusCode::UNAUTHORIZED,
        "Invalid Authorization header".to_string(),
    ))?;

    let claims = verify_firebase_jwt(token, "peak-bd11d")
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?;

    // claims.sub = Firebase UID
    println!("Authenticated user: {}", claims.sub);

    let page = params
        .get("page")
        .and_then(|p| p.parse::<usize>().ok())
        .unwrap_or(0);
    let limit = params
        .get("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(20);

    let peaks = fetch_peaks_from_db(claims.email, page, limit).await;

    Ok(Json(peaks))
}
