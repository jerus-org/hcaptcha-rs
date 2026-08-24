// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{collections::HashSet, fmt::Display};

use axum::Form;
use chrono::{TimeDelta, Utc};
use hcaptcha::Code;
// use rocket::{form::Form, post, serde::json::Json, FromForm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SuccessResponse {
    success: bool,
    credit: bool,
    hostname: String,
    challenge_ts: String,
}

impl SuccessResponse {
    #[allow(dead_code)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[allow(dead_code)]
    pub fn credit(&self) -> bool {
        self.credit
    }

    #[allow(dead_code)]
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    success: bool,
    #[serde(rename = "error-codes")]
    error_codes: Option<HashSet<Code>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestData {
    pub response: Option<String>,
    pub remoteip: Option<String>,
    pub sitekey: Option<String>,
    pub secret: Option<String>,
}

impl Display for RequestData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub async fn tc001(token: Form<RequestData>) -> Result<Form<SuccessResponse>, Form<ErrorResponse>> {
    let mut error_codes = HashSet::new();
    let mut early_exit = false;

    if token.response.is_none() {
        error_codes.insert(Code::MissingResponse);
        early_exit = true;
    };

    if token.secret.is_none() {
        error_codes.insert(Code::MissingSecret);
        early_exit = true;
    };

    if early_exit {
        return Err(Form(ErrorResponse {
            success: false,
            error_codes: Some(error_codes),
        }));
    }
    let timestamp = Utc::now()
        .checked_sub_signed(TimeDelta::try_minutes(10).unwrap())
        .unwrap()
        .to_rfc3339();

    let response = SuccessResponse {
        success: true,
        challenge_ts: timestamp,
        hostname: String::from("dummy-key-pass"),
        credit: false,
    };
    Ok(Form(response))
}
