// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::handler::error::ContactError;
use crate::handler::ContactForm;
use tracing::instrument;

#[instrument(
    name = "Write record to database"
    skip(form)
    fields(email = %form.email)
)]
pub async fn write(form: &ContactForm) -> Result<(), ContactError> {
    // Write the contact form data to dynamodb
    Ok(())
}
