// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;
#[derive(Error, Debug)]
pub enum ContactError {
    #[error("{0}")]
    Hcaptcha(#[from] hcaptcha::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}
