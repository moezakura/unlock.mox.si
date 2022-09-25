use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;

use crate::domain::config;
use crate::domain::switch_bot;

#[derive(Deserialize)]
struct PostRequest {
    pub is_door_open: bool,
    pub is_interphone_open: bool,
}

#[derive(Serialize)]
struct OpenResult {
    pub door_open: String,
    pub interphone_open: String,
}

#[derive(Serialize)]
struct PostResponse {
    pub message: String,
    pub result: OpenResult,
}

#[post("/open")]
pub async fn post(req: web::Json<PostRequest>, data: web::Data<config::Config>) -> impl Responder {
    if !req.is_door_open && !req.is_interphone_open {
        let open_result = OpenResult {
            door_open: "not open".to_string(),
            interphone_open: "not open".to_string(),
        };
        let post_response = PostResponse {
            message: "invalid request. require \"is_door_open\" or \"is_interphone_open\" filed."
                .to_string(),
            result: open_result,
        };

        return HttpResponse::BadRequest().json(post_response);
    }

    let app_data = data.clone();
    let switch_bot_token = app_data.switch_bot_token.clone();
    let switch_bot_secret = app_data.switch_bot_secret.clone();

    let interphone_bot_id = app_data.interphone_bot_id.clone();
    let lock_bot_id = app_data.lock_bot_id.clone();

    let service = switch_bot::service::Service::new(switch_bot_token, switch_bot_secret);

    // インターホンを解錠する
    let interphone_open = if req.is_interphone_open {
        let interphone_open = open_interphone(interphone_bot_id, service.clone()).await;
        let interphone_open = match interphone_open {
            Ok(_) => "open".to_string(),
            Err(e) => e,
        };
        interphone_open
    } else {
        "not open".to_string()
    };

    // ドアを開ける
    let door_open = if req.is_door_open {
        let door_open = open_door(lock_bot_id, service.clone()).await;
        let door_open = match door_open {
            Ok(_) => "open".to_string(),
            Err(e) => e,
        };
        door_open
    } else {
        "not open".to_string()
    };

    let open_result = OpenResult {
        door_open: door_open,
        interphone_open: interphone_open,
    };
    let post_response = PostResponse {
        message: "ok".to_string(),
        result: open_result,
    };

    HttpResponse::Ok().json(post_response)
}

async fn open_interphone(
    interphone_bot_id: String,
    service: switch_bot::service::Service,
) -> Result<(), String> {
    let res = service.push_button(interphone_bot_id).await;
    if res.is_err() {
        let err = res.unwrap_err();
        return Err(err.to_string());
    }

    let res = res.unwrap();
    if res.status_code != 100 {
        return Err(res.message);
    }

    Ok(())
}

async fn open_door(
    lock_bot_id: String,
    service: switch_bot::service::Service,
) -> Result<(), String> {
    let res = service.open_lock(lock_bot_id).await;
    if res.is_err() {
        let err = res.unwrap_err();
        return Err(err.to_string());
    }

    let res = res.unwrap();
    if res.status_code != 100 {
        return Err(res.message);
    }

    Ok(())
}
