use axum::{
    middleware::Next,
    extract::Request,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;

pub async fn auth_middleware(request: Request, next: Next) -> impl IntoResponse {
    let headers = request.headers();
    
    let token = match headers.get("authorization") {
        Some(auth_header) => {
            match auth_header.to_str() {
                Ok(auth_str) => {
                    if auth_str.starts_with("Bearer ") {
                        &auth_str[7..]
                    } else {
                        return (StatusCode::UNAUTHORIZED, Json(json!({
                            "error": "wrong token",
                            "message": "you are not authorized"
                        }))).into_response();
                    }
                }
                Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({
                    "error": "wrong token",
                    "message": "you are not authorized"
                }))).into_response(),
            }
        }
        None => return (StatusCode::UNAUTHORIZED, Json(json!({
            "error": "wrong token",
            "message": "you are not authorized"
        }))).into_response(),
    };
    
    // Temporarily bypass Supabase verification for testing
    println!("Token received: {}", &token[..20]); // Log first 20 chars
    
    match verify_supabase_token(token).await {
        true => next.run(request).await,
        false => {
            println!("Token verification failed");
            (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "wrong token",
                "message": "you are not authorized"
            }))).into_response()
        }
    }
}

async fn verify_supabase_token(token: &str) -> bool {
    let supabase_url = match std::env::var("SUPABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("SUPABASE_URL not found in environment");
            return false;
        }
    };
    
    let supabase_api_key = match std::env::var("SUPABASE_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("SUPABASE_API_KEY not found in environment");
            return false;
        }
    };
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
        
    let url = format!("{}/auth/v1/user", supabase_url);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("apikey", supabase_api_key)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            println!("Supabase response status: {}", status);
            status.is_success()
        },
        Err(e) => {
            eprintln!("Error verifying token with Supabase: {}", e);
            false
        }
    }
}