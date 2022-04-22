// use crate::helper::error::AppError;
use anyhow::Error;
use salvo::http::StatusCode;
use salvo::Response;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Res {
    pub status: u16,
    pub data: Value,
    pub time: u64,
    pub action: u64,
    pub msg: String,
}

impl Default for Res {
    fn default() -> Res {
        Res {
            status: StatusCode::OK.as_u16(),
            data: json!({}),
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            action: 0,
            msg: "".into(),
        }
    }
}

impl Res {
    pub fn data(mut self, value: Value) -> Res {
        self.data = value;
        self
    }

    pub fn time(mut self, time: u64) -> Res {
        self.time = time;
        self
    }

    pub fn code(mut self, code: u64) -> Res {
        self.action = code;
        self
    }

    // pub fn msg(mut self, err: AppError) -> Res {
    //     self.msg = err.to_string();
    //     self
    // }

    pub fn any_msg(mut self, err: Error) -> Res {
        self.msg = err.to_string();
        self
    }

    pub fn msg_string(mut self, err: String) -> Res {
        self.msg = err;
        self
    }

    pub fn ok(self, res: &mut Response) {
        res.set_status_code(StatusCode::OK);
        tracing::info!("{:?}", &self);
        res.render_json(&self);
    }

    pub fn fail(mut self, status_code: StatusCode, res: &mut Response) {
        res.set_status_code(status_code);
        self.status = status_code.as_u16();
        tracing::warn!("{:?}", &self);
        res.render_json(&self);
    }
}
