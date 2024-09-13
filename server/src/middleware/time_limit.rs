use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use chrono::{Local, NaiveTime};

const LOWERLIMITTIME: [u32; 3] = [18, 0, 0];
const UPPERLIMITTIME: [u32; 3] = [23, 0, 0];

pub async fn time_limit_check(req: Request, next: Next) -> Result<Response, StatusCode> {
    if is_current_time_range() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

fn is_current_time_range() -> bool {
    let current_time = Local::now().time();

    let lowwer_limit_time =
        NaiveTime::from_hms_opt(LOWERLIMITTIME[0], LOWERLIMITTIME[1], LOWERLIMITTIME[2]).unwrap();  
    let upper_limit_time =
        NaiveTime::from_hms_opt(UPPERLIMITTIME[0], UPPERLIMITTIME[1], UPPERLIMITTIME[2]).unwrap();

    current_time > lowwer_limit_time && current_time < upper_limit_time
}
