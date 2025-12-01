use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
};
use sqlx::{Row, Executor};
use crate::models::*;
use crate::config::database::get_db_pool;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_timestamp() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}

pub async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "OK",
        "message": "API is running"
    })))
}

pub async fn test_connection() -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = get_db_pool().await;
    match sqlx::query("SELECT 1 as test")
        .fetch_one(pool)
        .await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "Database connected successfully"
        }))),
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_all_customers() -> Result<Json<Vec<CreateCustomer>>, StatusCode> {
    println!("🔥 get_all_customers function called");
    let pool = get_db_pool().await;
    
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let raw_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status::text
        FROM visa_db.h1bcustomer -- {}", timestamp);
    
    let rows = pool.fetch_all(raw_sql.as_str())
    .await
    .map_err(|e| {
        eprintln!("❌ Database error in get_all_customers: {}", e);
        eprintln!("❌ Error details: {:?}", e);
        eprintln!("❌ SQL Query: {}", raw_sql);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customers: Vec<CreateCustomer> = rows.into_iter().map(|row| CreateCustomer {
        customer_id: row.get("customer_id"),
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        dob: row.get("dob"),
        sex: row.get("sex"),
        marital_status: row.get("marital_status"),
        phone: row.get("phone"),
        emergency_contact_name: row.get("emergency_contact_name"),
        emergency_contact_phone: row.get("emergency_contact_phone"),
        employment_start_date: row.get("employment_start_date"),
        street_name: row.get("street_name"),
        city: row.get("city"),
        state: row.get("state"),
        zip: row.get("zip"),
        client_name: row.get("client_name"),
        client_street_name: row.get("client_street_name"),
        client_city: row.get("client_city"),
        client_state: row.get("client_state"),
        client_zip: row.get("client_zip"),
        lca_title: row.get("lca_title"),
        lca_salary: row.get("lca_salary"),
        lca_code: row.get("lca_code"),
        receipt_number: row.get("receipt_number"),
        h1b_start_date: row.get("h1b_start_date"),
        h1b_end_date: row.get("h1b_end_date"),
        h1b_status: row.get("h1b_status"),
    }).collect();

    Ok(Json(customers))
}

pub async fn create_visa_details(
    Json(payload): Json<CreateCompleteCustomerRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 create_visa_details function called");
    let pool = get_db_pool().await;
    let h1b_status = payload.h1b_status.as_deref().unwrap_or("Active");
    
    let _query_sql = "INSERT INTO visa_db.h1bcustomer (
            email, first_name, last_name, dob, sex, marital_status, phone,
            emergency_contact_name, emergency_contact_phone, employment_start_date,
            street_name, city, state, zip,
            client_name, client_street_name, client_city, client_state, client_zip,
            lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status
        ) VALUES (
            $1, $2, $3, $4, $5::visa_db.sex_enum, $6::visa_db.marital_status_enum, $7,
            $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26::visa_db.h1b_status_enum
        )";
    
    let raw_sql = format!("INSERT INTO global_visa_mgmt.h1bcustomer (
            email, first_name, last_name, dob, sex, marital_status, phone,
            emergency_contact_name, emergency_contact_phone, employment_start_date,
            street_name, city, state, zip,
            client_name, client_street_name, client_city, client_state, client_zip,
            lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, login_email, h1b_status
        ) VALUES (
            '{}', '{}', '{}', '{}', '{}'::global_visa_mgmt.sex_enum, '{}'::global_visa_mgmt.marital_status_enum, '{}',
            '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}', '{}', '{}', '{}', '{}'::global_visa_mgmt.h1b_status_enum
        )",
        payload.email.replace("'", "''"), payload.first_name.replace("'", "''"), payload.last_name.replace("'", "''"), 
        payload.dob, payload.sex, payload.marital_status, payload.phone.replace("'", "''"),
        payload.emergency_contact_name.replace("'", "''"), payload.emergency_contact_phone.replace("'", "''"), payload.employment_start_date,
        payload.street_name.replace("'", "''"), payload.city.replace("'", "''"), payload.state.replace("'", "''"), payload.zip.replace("'", "''"),
        payload.client_name.replace("'", "''"), payload.client_street_name.replace("'", "''"), payload.client_city.replace("'", "''"), 
        payload.client_state.replace("'", "''"), payload.client_zip.replace("'", "''"),
        payload.lca_title.replace("'", "''"), payload.lca_salary, payload.lca_code.replace("'", "''"), 
        payload.receipt_number.replace("'", "''"), payload.h1b_start_date, payload.h1b_end_date, payload.login_email.replace("'", "''"), h1b_status
    );
    
    match pool.execute(raw_sql.as_str()).await {
        Ok(result) => {
            Ok(Json(serde_json::json!({
                "message": "Visa details created successfully",
                "email": payload.email,
                "rows_affected": result.rows_affected()
            })))
        },
        Err(e) => {
            eprintln!("❌ Database error in create_visa_details: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ SQL Query: {}", raw_sql);
            eprintln!("❌ Email: {}", payload.email);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn soft_delete_customer(
    Path(email): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 soft_delete_customer function called for email: {}", email);
    let pool = get_db_pool().await;

    // Use the pool directly, SQLx will manage connections automatically
    match sqlx::query("UPDATE visa_db.h1bcustomer SET h1b_status = 'Inactive' WHERE email = $1")
        .bind(&email)
        .execute(&pool) // <-- pass the pool, not manually acquired conn
        .await 
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Ok(Json(serde_json::json!({
                    "status": 404,
                    "message": "Record not found in the database",
                    "email": email
                })))
            } else {
                Ok(Json(serde_json::json!({
                    "message": "Customer soft deleted successfully",
                    "email": email,
                    "rows_affected": result.rows_affected()
                })))
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in soft_delete_customer: {}", e);
            eprintln!("❌ Soft delete error details: {:?}", e);
            eprintln!("❌ Email: {}", email);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_customer_personal(
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = get_db_pool().await;
    
    let query_sql = "SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone 
        FROM visa_db.h1bcustomer WHERE customer_id = $1";
    
    match sqlx::query(query_sql)
        .bind(&id)
        .fetch_optional(&pool)
        .await {
        Ok(Some(row)) => {
            Ok(Json(serde_json::json!({
                "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
                "email": row.get::<String, _>("email"),
                "first_name": row.get::<String, _>("first_name"),
                "last_name": row.get::<String, _>("last_name"),
                "dob": row.get::<chrono::NaiveDate, _>("dob"),
                "sex": row.get::<String, _>("sex"),
                "marital_status": row.get::<String, _>("marital_status"),
                "phone": row.get::<String, _>("phone")
            })))
        },
        Ok(None) => {
            eprintln!("❌ Customer not found in get_customer_personal: {}", id);
            Err(StatusCode::NOT_FOUND)
        },
        Err(e) => {
            eprintln!("❌ Database error in get_customer_personal: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_customer_address(
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = get_db_pool().await;
    
    let query_sql = "SELECT customer_id, street_name, city, state, zip 
        FROM visa_db.h1bcustomer WHERE customer_id = $1";
    
    match sqlx::query(query_sql)
        .bind(&id)
        .fetch_optional(&pool)
        .await {
        Ok(Some(row)) => {
            Ok(Json(serde_json::json!({
                "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
                "street_name": row.get::<String, _>("street_name"),
                "city": row.get::<String, _>("city"),
                "state": row.get::<String, _>("state"),
                "zip": row.get::<String, _>("zip")
            })))
        },
        Ok(None) => {
            eprintln!("❌ Customer not found in get_customer_address: {}", id);
            Err(StatusCode::NOT_FOUND)
        },
        Err(e) => {
            eprintln!("❌ Database error in get_customer_address: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_customer_h1b(
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = get_db_pool();
    
    let query_sql = "SELECT customer_id, client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status::text 
        FROM visa_db.h1bcustomer WHERE customer_id = $1";
    
    match sqlx::query(query_sql)
        .bind(&id)
        .fetch_optional(&pool)
        .await {
        Ok(Some(row)) => {
            Ok(Json(serde_json::json!({
                "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
                "client_name": row.get::<String, _>("client_name"),
                "client_street_name": row.get::<String, _>("client_street_name"),
                "client_city": row.get::<String, _>("client_city"),
                "client_state": row.get::<String, _>("client_state"),
                "client_zip": row.get::<String, _>("client_zip"),
                "lca_title": row.get::<String, _>("lca_title"),
                "lca_salary": row.get::<rust_decimal::Decimal, _>("lca_salary"),
                "lca_code": row.get::<String, _>("lca_code"),
                "receipt_number": row.get::<String, _>("receipt_number"),
                "h1b_start_date": row.get::<chrono::NaiveDate, _>("h1b_start_date"),
                "h1b_end_date": row.get::<chrono::NaiveDate, _>("h1b_end_date"),
                "h1b_status": row.get::<String, _>("h1b_status")
            })))
        },
        Ok(None) => {
            eprintln!("❌ Customer not found in get_customer_h1b: {}", id);
            Err(StatusCode::NOT_FOUND)
        },
        Err(e) => {
            eprintln!("❌ Database error in get_customer_h1b: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_customer_address(
    Path(id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = get_db_pool();
    
    let query_sql = "UPDATE visa_db.h1bcustomer SET 
        street_name = $2, city = $3, state = $4, zip = $5 
        WHERE customer_id = $1";
    
    match sqlx::query(query_sql)
        .bind(&id)
        .bind(payload["street_name"].as_str().unwrap_or(""))
        .bind(payload["city"].as_str().unwrap_or(""))
        .bind(payload["state"].as_str().unwrap_or(""))
        .bind(payload["zip"].as_str().unwrap_or(""))
        .execute(&pool)
        .await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(Json(serde_json::json!({
                    "message": "Address updated successfully",
                    "customer_id": id
                })))
            } else {
                eprintln!("❌ Customer not found in update_customer_address: {}", id);
                Err(StatusCode::NOT_FOUND)
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in update_customer_address: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_customer_h1b(
    Path(id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = get_db_pool();
    
    let query_sql = "UPDATE visa_db.h1bcustomer SET 
        client_name = $2, client_street_name = $3, client_city = $4, client_state = $5, client_zip = $6,
        lca_title = $7, lca_salary = $8, lca_code = $9, receipt_number = $10, 
        h1b_start_date = $11, h1b_end_date = $12, h1b_status = $13::visa_db.h1b_status_enum 
        WHERE customer_id = $1";
    
    match sqlx::query(query_sql)
        .bind(&id)
        .bind(payload["client_name"].as_str().unwrap_or(""))
        .bind(payload["client_street_name"].as_str().unwrap_or(""))
        .bind(payload["client_city"].as_str().unwrap_or(""))
        .bind(payload["client_state"].as_str().unwrap_or(""))
        .bind(payload["client_zip"].as_str().unwrap_or(""))
        .bind(payload["lca_title"].as_str().unwrap_or(""))
        .bind(payload["lca_salary"].as_f64().unwrap_or(0.0))
        .bind(payload["lca_code"].as_str().unwrap_or(""))
        .bind(payload["receipt_number"].as_str().unwrap_or(""))
        .bind(payload["h1b_start_date"].as_str().unwrap_or(""))
        .bind(payload["h1b_end_date"].as_str().unwrap_or(""))
        .bind(payload["h1b_status"].as_str().unwrap_or("Active"))
        .execute(&pool)
        .await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(Json(serde_json::json!({
                    "message": "H1B details updated successfully",
                    "customer_id": id
                })))
            } else {
                eprintln!("❌ Customer not found in update_customer_h1b: {}", id);
                Err(StatusCode::NOT_FOUND)
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in update_customer_h1b: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_visa_details_by_id(
    Path(customer_id): Path<String>,
    Json(payload): Json<UpdateVisaDetailsRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 update_visa_details_by_id function called for customer_id: {}", customer_id);
    let pool = get_db_pool();
    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("❌ Failed to begin transaction in update_visa_details_by_id: {}", e);
        eprintln!("❌ Transaction error details: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let select_sql = "SELECT * FROM visa_db.h1bcustomer WHERE customer_id = $1";
    
    let current_row = sqlx::query(&select_sql)
        .bind(&customer_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("❌ Database error in update_visa_details_by_id select: {}", e);
            eprintln!("❌ Select error details: {:?}", e);
            eprintln!("❌ SQL Query: {}", select_sql);
            eprintln!("❌ Customer ID: {}", customer_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let current = match current_row {
        Some(row) => row,
        None => {
            return Ok(Json(serde_json::json!({
                "status": 404,
                "message": "Record not found in the database",
                "customer_id": customer_id
            })));
        }
    };
    
    let first_name = payload.first_name.unwrap_or_else(|| current.get("first_name"));
    let last_name = payload.last_name.unwrap_or_else(|| current.get("last_name"));
    let dob = payload.dob.unwrap_or_else(|| current.get("dob"));
    let sex = payload.sex.unwrap_or_else(|| current.get("sex"));
    let marital_status = payload.marital_status.unwrap_or_else(|| current.get("marital_status"));
    let phone = payload.phone.unwrap_or_else(|| current.get("phone"));
    let emergency_contact_name = payload.emergency_contact_name.unwrap_or_else(|| current.get("emergency_contact_name"));
    let emergency_contact_phone = payload.emergency_contact_phone.unwrap_or_else(|| current.get("emergency_contact_phone"));
    let employment_start_date = payload.employment_start_date.unwrap_or_else(|| current.get("employment_start_date"));
    let street_name = payload.street_name.unwrap_or_else(|| current.get("street_name"));
    let city = payload.city.unwrap_or_else(|| current.get("city"));
    let state = payload.state.unwrap_or_else(|| current.get("state"));
    let zip = payload.zip.unwrap_or_else(|| current.get("zip"));
    let client_name = payload.client_name.unwrap_or_else(|| current.get("client_name"));
    let client_street_name = payload.client_street_name.unwrap_or_else(|| current.get("client_street_name"));
    let client_city = payload.client_city.unwrap_or_else(|| current.get("client_city"));
    let client_state = payload.client_state.unwrap_or_else(|| current.get("client_state"));
    let client_zip = payload.client_zip.unwrap_or_else(|| current.get("client_zip"));
    let lca_title = payload.lca_title.unwrap_or_else(|| current.get("lca_title"));
    let lca_salary = payload.lca_salary.unwrap_or_else(|| current.get("lca_salary"));
    let lca_code = payload.lca_code.unwrap_or_else(|| current.get("lca_code"));
    let receipt_number = payload.receipt_number.unwrap_or_else(|| current.get("receipt_number"));
    let h1b_start_date = payload.h1b_start_date.unwrap_or_else(|| current.get("h1b_start_date"));
    let h1b_end_date = payload.h1b_end_date.unwrap_or_else(|| current.get("h1b_end_date"));
    let h1b_status = payload.h1b_status.unwrap_or_else(|| current.get("h1b_status"));
    
    let update_sql = "UPDATE visa_db.h1bcustomer SET
            first_name = $2, last_name = $3, dob = $4, sex = $5::visa_db.sex_enum,
            marital_status = $6::visa_db.marital_status_enum, phone = $7,
            emergency_contact_name = $8, emergency_contact_phone = $9, employment_start_date = $10,
            street_name = $11, city = $12, state = $13, zip = $14,
            client_name = $15, client_street_name = $16, client_city = $17, client_state = $18, client_zip = $19,
            lca_title = $20, lca_salary = $21, lca_code = $22, receipt_number = $23,
            h1b_start_date = $24, h1b_end_date = $25, h1b_status = $26::visa_db.h1b_status_enum
         WHERE customer_id = $1";
    
    let result = sqlx::query(&update_sql)
        .bind(&customer_id)
        .bind(&first_name)
        .bind(&last_name)
        .bind(&dob)
        .bind(&sex)
        .bind(&marital_status)
        .bind(&phone)
        .bind(&emergency_contact_name)
        .bind(&emergency_contact_phone)
        .bind(&employment_start_date)
        .bind(&street_name)
        .bind(&city)
        .bind(&state)
        .bind(&zip)
        .bind(&client_name)
        .bind(&client_street_name)
        .bind(&client_city)
        .bind(&client_state)
        .bind(&client_zip)
        .bind(&lca_title)
        .bind(&lca_salary)
        .bind(&lca_code)
        .bind(&receipt_number)
        .bind(&h1b_start_date)
        .bind(&h1b_end_date)
        .bind(&h1b_status)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("❌ Database error in update_visa_details_by_id: {}", e);
            eprintln!("❌ Update error details: {:?}", e);
            eprintln!("❌ SQL Query: {}", update_sql);
            eprintln!("❌ Customer ID: {}", customer_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    tx.commit().await.map_err(|e| {
        eprintln!("❌ Failed to commit transaction in update_visa_details_by_id: {}", e);
        eprintln!("❌ Commit error details: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::json!({
        "message": "Visa details updated successfully",
        "customer_id": customer_id,
        "rows_affected": result.rows_affected()
    })))
}

pub async fn get_customer_by_id(
    Path(customer_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = get_db_pool();
    
    let raw_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, login_email, h1b_status::text
        FROM global_visa_mgmt.h1bcustomer WHERE customer_id::text = '{}' AND h1b_status = 'Active'", customer_id.replace("'", "''"));
    
    match pool.fetch_optional(raw_sql.as_str())
        .await {
        Ok(Some(row)) => {
            let mut response = serde_json::Map::new();
            response.insert("customer_id".to_string(), serde_json::json!(row.get::<uuid::Uuid, _>("customer_id")));
            response.insert("email".to_string(), serde_json::json!(row.get::<String, _>("email")));
            response.insert("first_name".to_string(), serde_json::json!(row.get::<String, _>("first_name")));
            response.insert("last_name".to_string(), serde_json::json!(row.get::<String, _>("last_name")));
            response.insert("dob".to_string(), serde_json::json!(row.get::<chrono::NaiveDate, _>("dob")));
            response.insert("sex".to_string(), serde_json::json!(row.get::<String, _>("sex")));
            response.insert("marital_status".to_string(), serde_json::json!(row.get::<String, _>("marital_status")));
            response.insert("phone".to_string(), serde_json::json!(row.get::<String, _>("phone")));
            response.insert("emergency_contact_name".to_string(), serde_json::json!(row.get::<String, _>("emergency_contact_name")));
            response.insert("emergency_contact_phone".to_string(), serde_json::json!(row.get::<String, _>("emergency_contact_phone")));
            response.insert("employment_start_date".to_string(), serde_json::json!(row.get::<chrono::NaiveDate, _>("employment_start_date")));
            response.insert("street_name".to_string(), serde_json::json!(row.get::<String, _>("street_name")));
            response.insert("city".to_string(), serde_json::json!(row.get::<String, _>("city")));
            response.insert("state".to_string(), serde_json::json!(row.get::<String, _>("state")));
            response.insert("zip".to_string(), serde_json::json!(row.get::<String, _>("zip")));
            response.insert("client_name".to_string(), serde_json::json!(row.get::<String, _>("client_name")));
            response.insert("client_street_name".to_string(), serde_json::json!(row.get::<String, _>("client_street_name")));
            response.insert("client_city".to_string(), serde_json::json!(row.get::<String, _>("client_city")));
            response.insert("client_state".to_string(), serde_json::json!(row.get::<String, _>("client_state")));
            response.insert("client_zip".to_string(), serde_json::json!(row.get::<String, _>("client_zip")));
            response.insert("lca_title".to_string(), serde_json::json!(row.get::<String, _>("lca_title")));
            response.insert("lca_salary".to_string(), serde_json::json!(row.get::<rust_decimal::Decimal, _>("lca_salary")));
            response.insert("lca_code".to_string(), serde_json::json!(row.get::<String, _>("lca_code")));
            response.insert("receipt_number".to_string(), serde_json::json!(row.get::<String, _>("receipt_number")));
            response.insert("h1b_start_date".to_string(), serde_json::json!(row.get::<chrono::NaiveDate, _>("h1b_start_date")));
            response.insert("h1b_end_date".to_string(), serde_json::json!(row.get::<chrono::NaiveDate, _>("h1b_end_date")));
            response.insert("login_email".to_string(), serde_json::json!(row.get::<String, _>("login_email")));
            response.insert("h1b_status".to_string(), serde_json::json!(row.get::<String, _>("h1b_status")));
            Ok(Json(serde_json::Value::Object(response)))
        },
        Ok(None) => {
            Ok(Json(serde_json::json!({
                "message": "Data not found"
            })))
        },
        Err(e) => {
            eprintln!("❌ Database error in get_customer_by_id: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", customer_id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_customer_by_email(
    Path(email): Path<String>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let pool = get_db_pool();
    
    let raw_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, login_email, h1b_status::text
        FROM global_visa_mgmt.h1bcustomer WHERE (email = '{}' OR login_email = '{}')", email.replace("'", "''"), email.replace("'", "''"));
    
    match pool.fetch_all(raw_sql.as_str())
        .await {
        Ok(rows) => {
            if rows.is_empty() {
                Ok(Json(vec![serde_json::json!({
                    "message": "Data not found"
                })]))
            } else {
                let customers: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                    serde_json::json!({
                        "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
                        "email": row.get::<String, _>("email"),
                        "first_name": row.get::<String, _>("first_name"),
                        "last_name": row.get::<String, _>("last_name"),
                        "dob": row.get::<chrono::NaiveDate, _>("dob"),
                        "sex": row.get::<String, _>("sex"),
                        "marital_status": row.get::<String, _>("marital_status"),
                        "phone": row.get::<String, _>("phone"),
                        "emergency_contact_name": row.get::<String, _>("emergency_contact_name"),
                        "emergency_contact_phone": row.get::<String, _>("emergency_contact_phone"),
                        "employment_start_date": row.get::<chrono::NaiveDate, _>("employment_start_date"),
                        "street_name": row.get::<String, _>("street_name"),
                        "city": row.get::<String, _>("city"),
                        "state": row.get::<String, _>("state"),
                        "zip": row.get::<String, _>("zip"),
                        "client_name": row.get::<String, _>("client_name"),
                        "client_street_name": row.get::<String, _>("client_street_name"),
                        "client_city": row.get::<String, _>("client_city"),
                        "client_state": row.get::<String, _>("client_state"),
                        "client_zip": row.get::<String, _>("client_zip"),
                        "lca_title": row.get::<String, _>("lca_title"),
                        "lca_salary": row.get::<rust_decimal::Decimal, _>("lca_salary"),
                        "lca_code": row.get::<String, _>("lca_code"),
                        "receipt_number": row.get::<String, _>("receipt_number"),
                        "h1b_start_date": row.get::<chrono::NaiveDate, _>("h1b_start_date"),
                        "h1b_end_date": row.get::<chrono::NaiveDate, _>("h1b_end_date"),
                        "login_email": row.get::<String, _>("login_email"),
                        "h1b_status": row.get::<String, _>("h1b_status")
                    })
                }).collect();
                Ok(Json(customers))
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in get_customer_by_email: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Email: {}", email);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn soft_delete_customer_by_id(
    Path(customer_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 soft_delete_customer_by_id function called for customer_id: {}", customer_id);
    let pool = get_db_pool();

    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let check_sql = format!("SELECT h1b_status::text FROM global_visa_mgmt.h1bcustomer WHERE customer_id = '{}'::uuid -- {}", customer_id.replace("'", "''"), timestamp);
    
    match pool.fetch_optional(check_sql.as_str()).await {
        Ok(Some(row)) => {
            let current_status: String = row.get("h1b_status");
            if current_status == "Inactive" {
                return Ok(Json(serde_json::json!({
                    "message": "Data not found"
                })));
            }
        },
        Ok(None) => {
            return Ok(Json(serde_json::json!({
                "message": "Data not found"
            })));
        },
        Err(e) => {
            eprintln!("❌ Database error checking status: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let raw_sql = format!("UPDATE global_visa_mgmt.h1bcustomer SET h1b_status = 'Inactive' WHERE customer_id = '{}'::uuid -- {}", customer_id.replace("'", "''"), timestamp);

    match pool.execute(raw_sql.as_str()).await 
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Ok(Json(serde_json::json!({
                    "status": 404,
                    "message": "Record not found in the database",
                    "customer_id": customer_id
                })))
            } else {
                Ok(Json(serde_json::json!({
                    "message": "Customer soft deleted successfully",
                    "customer_id": customer_id,
                    "rows_affected": result.rows_affected()
                })))
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in soft_delete_customer_by_id: {}", e);
            eprintln!("❌ Soft delete error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", customer_id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
pub async fn update_customer_by_id(
    Path(customer_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 update_customer_by_id function called for customer_id: {}", customer_id);
    let pool = get_db_pool();

    let raw_sql = format!("UPDATE global_visa_mgmt.h1bcustomer SET 
        email = '{}', first_name = '{}', last_name = '{}', dob = '{}', 
        sex = '{}'::global_visa_mgmt.sex_enum, marital_status = '{}'::global_visa_mgmt.marital_status_enum, 
        phone = '{}', emergency_contact_name = '{}', emergency_contact_phone = '{}', 
        employment_start_date = '{}', street_name = '{}', city = '{}', state = '{}', 
        zip = '{}', client_name = '{}', client_street_name = '{}', client_city = '{}', 
        client_state = '{}', client_zip = '{}', lca_title = '{}', lca_salary = {}, 
        lca_code = '{}', receipt_number = '{}', h1b_start_date = '{}', h1b_end_date = '{}', login_email = '{}'
        WHERE customer_id = '{}'::uuid",
        payload["email"].as_str().unwrap_or("").replace("'", "''"),
        payload["first_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["last_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["dob"].as_str().unwrap_or(""),
        payload["sex"].as_str().unwrap_or(""),
        payload["marital_status"].as_str().unwrap_or(""),
        payload["phone"].as_str().unwrap_or("").replace("'", "''"),
        payload["emergency_contact_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["emergency_contact_phone"].as_str().unwrap_or("").replace("'", "''"),
        payload["employment_start_date"].as_str().unwrap_or(""),
        payload["street_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["city"].as_str().unwrap_or("").replace("'", "''"),
        payload["state"].as_str().unwrap_or("").replace("'", "''"),
        payload["zip"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_street_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_city"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_state"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_zip"].as_str().unwrap_or("").replace("'", "''"),
        payload["lca_title"].as_str().unwrap_or("").replace("'", "''"),
        payload["lca_salary"].as_str().unwrap_or("0"),
        payload["lca_code"].as_str().unwrap_or("").replace("'", "''"),
        payload["receipt_number"].as_str().unwrap_or("").replace("'", "''"),
        payload["h1b_start_date"].as_str().unwrap_or(""),
        payload["h1b_end_date"].as_str().unwrap_or(""),
        payload["login_email"].as_str().unwrap_or("").replace("'", "''"),
        customer_id.replace("'", "''")
    );

    match pool.execute(raw_sql.as_str()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(Json(serde_json::json!({
                    "message": "Customer updated successfully",
                    "customer_id": customer_id,
                    "rows_affected": result.rows_affected()
                })))
            } else {
                Ok(Json(serde_json::json!({
                    "message": "Customer not found",
                    "customer_id": customer_id
                })))
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in update_customer_by_id: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", customer_id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
pub async fn get_all_customers_with_status() -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    println!("🔥 get_all_customers_with_status function called");
    let pool = get_db_pool().await;
    
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let raw_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, login_email, h1b_status::text
        FROM global_visa_mgmt.h1bcustomer WHERE h1b_status = 'Active' -- {}", timestamp);
    
    let rows = pool.fetch_all(raw_sql.as_str())
    .await
    .map_err(|e| {
        eprintln!("❌ Database error in get_all_customers_with_status: {}", e);
        eprintln!("❌ Error details: {:?}", e);
        eprintln!("❌ SQL Query: {}", raw_sql);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customers: Vec<serde_json::Value> = rows.into_iter().map(|row| {
        serde_json::json!({
            "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
            "email": row.get::<String, _>("email"),
            "first_name": row.get::<String, _>("first_name"),
            "last_name": row.get::<String, _>("last_name"),
            "dob": row.get::<chrono::NaiveDate, _>("dob"),
            "sex": row.get::<String, _>("sex"),
            "marital_status": row.get::<String, _>("marital_status"),
            "phone": row.get::<String, _>("phone"),
            "emergency_contact_name": row.get::<String, _>("emergency_contact_name"),
            "emergency_contact_phone": row.get::<String, _>("emergency_contact_phone"),
            "employment_start_date": row.get::<chrono::NaiveDate, _>("employment_start_date"),
            "street_name": row.get::<String, _>("street_name"),
            "city": row.get::<String, _>("city"),
            "state": row.get::<String, _>("state"),
            "zip": row.get::<String, _>("zip"),
            "client_name": row.get::<String, _>("client_name"),
            "client_street_name": row.get::<String, _>("client_street_name"),
            "client_city": row.get::<String, _>("client_city"),
            "client_state": row.get::<String, _>("client_state"),
            "client_zip": row.get::<String, _>("client_zip"),
            "lca_title": row.get::<String, _>("lca_title"),
            "lca_salary": row.get::<rust_decimal::Decimal, _>("lca_salary"),
            "lca_code": row.get::<String, _>("lca_code"),
            "receipt_number": row.get::<String, _>("receipt_number"),
            "h1b_start_date": row.get::<chrono::NaiveDate, _>("h1b_start_date"),
            "h1b_end_date": row.get::<chrono::NaiveDate, _>("h1b_end_date"),
            "login_email": row.get::<String, _>("login_email"),
            "h1b_status": row.get::<String, _>("h1b_status")
        })
    }).collect();

    Ok(Json(customers))
}
pub async fn get_customer_by_login_email(
    Path(login_email): Path<String>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let pool = get_db_pool();
    
    let raw_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, login_email, h1b_status::text
        FROM global_visa_mgmt.h1bcustomer WHERE login_email = '{}' AND h1b_status = 'Active'", login_email.replace("'", "''"));
    
    match pool.fetch_all(raw_sql.as_str())
        .await {
        Ok(rows) => {
            if rows.is_empty() {
                Ok(Json(vec![serde_json::json!({
                    "message": "Data not found"
                })]))
            } else {
                let customers: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                    serde_json::json!({
                        "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
                        "email": row.get::<String, _>("email"),
                        "first_name": row.get::<String, _>("first_name"),
                        "last_name": row.get::<String, _>("last_name"),
                        "dob": row.get::<chrono::NaiveDate, _>("dob"),
                        "sex": row.get::<String, _>("sex"),
                        "marital_status": row.get::<String, _>("marital_status"),
                        "phone": row.get::<String, _>("phone"),
                        "emergency_contact_name": row.get::<String, _>("emergency_contact_name"),
                        "emergency_contact_phone": row.get::<String, _>("emergency_contact_phone"),
                        "employment_start_date": row.get::<chrono::NaiveDate, _>("employment_start_date"),
                        "street_name": row.get::<String, _>("street_name"),
                        "city": row.get::<String, _>("city"),
                        "state": row.get::<String, _>("state"),
                        "zip": row.get::<String, _>("zip"),
                        "client_name": row.get::<String, _>("client_name"),
                        "client_street_name": row.get::<String, _>("client_street_name"),
                        "client_city": row.get::<String, _>("client_city"),
                        "client_state": row.get::<String, _>("client_state"),
                        "client_zip": row.get::<String, _>("client_zip"),
                        "lca_title": row.get::<String, _>("lca_title"),
                        "lca_salary": row.get::<rust_decimal::Decimal, _>("lca_salary"),
                        "lca_code": row.get::<String, _>("lca_code"),
                        "receipt_number": row.get::<String, _>("receipt_number"),
                        "h1b_start_date": row.get::<chrono::NaiveDate, _>("h1b_start_date"),
                        "h1b_end_date": row.get::<chrono::NaiveDate, _>("h1b_end_date"),
                        "login_email": row.get::<String, _>("login_email"),
                        "h1b_status": row.get::<String, _>("h1b_status")
                    })
                }).collect();
                Ok(Json(customers))
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in get_customer_by_login_email: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Login Email: {}", login_email);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
pub async fn get_all_customers_no_filter() -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    println!("🔥 get_all_customers_no_filter function called");
    let pool = get_db_pool();
    
    let timestamp = get_timestamp();
    let raw_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, login_email, h1b_status::text
        FROM global_visa_mgmt.h1bcustomer -- {}", timestamp);
    
    let rows = pool.fetch_all(raw_sql.as_str())
    .await
    .map_err(|e| {
        eprintln!("❌ Database error in get_all_customers_no_filter: {}", e);
        eprintln!("❌ Error details: {:?}", e);
        eprintln!("❌ SQL Query: {}", raw_sql);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customers: Vec<serde_json::Value> = rows.into_iter().map(|row| {
        serde_json::json!({
            "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
            "email": row.get::<String, _>("email"),
            "first_name": row.get::<String, _>("first_name"),
            "last_name": row.get::<String, _>("last_name"),
            "dob": row.get::<chrono::NaiveDate, _>("dob"),
            "sex": row.get::<String, _>("sex"),
            "marital_status": row.get::<String, _>("marital_status"),
            "phone": row.get::<String, _>("phone"),
            "emergency_contact_name": row.get::<String, _>("emergency_contact_name"),
            "emergency_contact_phone": row.get::<String, _>("emergency_contact_phone"),
            "employment_start_date": row.get::<chrono::NaiveDate, _>("employment_start_date"),
            "street_name": row.get::<String, _>("street_name"),
            "city": row.get::<String, _>("city"),
            "state": row.get::<String, _>("state"),
            "zip": row.get::<String, _>("zip"),
            "client_name": row.get::<String, _>("client_name"),
            "client_street_name": row.get::<String, _>("client_street_name"),
            "client_city": row.get::<String, _>("client_city"),
            "client_state": row.get::<String, _>("client_state"),
            "client_zip": row.get::<String, _>("client_zip"),
            "lca_title": row.get::<String, _>("lca_title"),
            "lca_salary": row.get::<rust_decimal::Decimal, _>("lca_salary"),
            "lca_code": row.get::<String, _>("lca_code"),
            "receipt_number": row.get::<String, _>("receipt_number"),
            "h1b_start_date": row.get::<chrono::NaiveDate, _>("h1b_start_date"),
            "h1b_end_date": row.get::<chrono::NaiveDate, _>("h1b_end_date"),
            "login_email": row.get::<String, _>("login_email"),
            "h1b_status": row.get::<String, _>("h1b_status")
        })
    }).collect();

    Ok(Json(customers))
}

pub async fn activate_customer_by_id(
    Path(customer_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 activate_customer_by_id function called for customer_id: {}", customer_id);
    let pool = get_db_pool();

    let timestamp = get_timestamp();
    let check_sql = format!("SELECT h1b_status::text FROM global_visa_mgmt.h1bcustomer WHERE customer_id = '{}'::uuid -- {}", customer_id.replace("'", "''"), timestamp);
    
    match pool.fetch_optional(check_sql.as_str()).await {
        Ok(Some(row)) => {
            let current_status: String = row.get("h1b_status");
            if current_status == "Active" {
                return Ok(Json(serde_json::json!({
                    "message": "This customer is already active."
                })));
            }
        },
        Ok(None) => {
            return Ok(Json(serde_json::json!({
                "status": 404,
                "message": "Record not found in the database",
                "customer_id": customer_id
            })));
        },
        Err(e) => {
            eprintln!("❌ Database error checking status: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let raw_sql = format!("UPDATE global_visa_mgmt.h1bcustomer SET h1b_status = 'Active' WHERE customer_id = '{}'::uuid -- {}", customer_id.replace("'", "''"), timestamp);

    match pool.execute(raw_sql.as_str()).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Ok(Json(serde_json::json!({
                    "status": 404,
                    "message": "Record not found in the database",
                    "customer_id": customer_id
                })))
            } else {
                let select_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
                    emergency_contact_name, emergency_contact_phone, employment_start_date,
                    street_name, city, state, zip,
                    client_name, client_street_name, client_city, client_state, client_zip,
                    lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, login_email, h1b_status::text
                    FROM global_visa_mgmt.h1bcustomer WHERE customer_id = '{}'::uuid -- {}", customer_id.replace("'", "''"), timestamp);
                
                match pool.fetch_optional(select_sql.as_str()).await {
                    Ok(Some(row)) => {
                        Ok(Json(serde_json::json!({
                            "message": "Customer activated successfully",
                            "customer_id": customer_id,
                            "rows_affected": result.rows_affected(),
                            "updated_record": {
                                "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
                                "email": row.get::<String, _>("email"),
                                "first_name": row.get::<String, _>("first_name"),
                                "last_name": row.get::<String, _>("last_name"),
                                "dob": row.get::<chrono::NaiveDate, _>("dob"),
                                "sex": row.get::<String, _>("sex"),
                                "marital_status": row.get::<String, _>("marital_status"),
                                "phone": row.get::<String, _>("phone"),
                                "emergency_contact_name": row.get::<String, _>("emergency_contact_name"),
                                "emergency_contact_phone": row.get::<String, _>("emergency_contact_phone"),
                                "employment_start_date": row.get::<chrono::NaiveDate, _>("employment_start_date"),
                                "street_name": row.get::<String, _>("street_name"),
                                "city": row.get::<String, _>("city"),
                                "state": row.get::<String, _>("state"),
                                "zip": row.get::<String, _>("zip"),
                                "client_name": row.get::<String, _>("client_name"),
                                "client_street_name": row.get::<String, _>("client_street_name"),
                                "client_city": row.get::<String, _>("client_city"),
                                "client_state": row.get::<String, _>("client_state"),
                                "client_zip": row.get::<String, _>("client_zip"),
                                "lca_title": row.get::<String, _>("lca_title"),
                                "lca_salary": row.get::<rust_decimal::Decimal, _>("lca_salary"),
                                "lca_code": row.get::<String, _>("lca_code"),
                                "receipt_number": row.get::<String, _>("receipt_number"),
                                "h1b_start_date": row.get::<chrono::NaiveDate, _>("h1b_start_date"),
                                "h1b_end_date": row.get::<chrono::NaiveDate, _>("h1b_end_date"),
                                "login_email": row.get::<String, _>("login_email"),
                                "h1b_status": row.get::<String, _>("h1b_status")
                            }
                        })))
                    },
                    Ok(None) => {
                        Ok(Json(serde_json::json!({
                            "message": "Customer activated successfully",
                            "customer_id": customer_id,
                            "rows_affected": result.rows_affected()
                        })))
                    },
                    Err(e) => {
                        eprintln!("❌ Database error fetching updated record: {}", e);
                        Ok(Json(serde_json::json!({
                            "message": "Customer activated successfully",
                            "customer_id": customer_id,
                            "rows_affected": result.rows_affected()
                        })))
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in activate_customer_by_id: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", customer_id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}