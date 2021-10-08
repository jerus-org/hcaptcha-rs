# Derived Contact Lambda

This example provides a skeleton for a lambda to handle the submission of a contact form.

The contact form collects the Name, Phone and Email contacts and a message from the visitor.

The visitor is confirmed to be human by Hcaptcha and the token is sent with the form data to the lambda for processing.

## Struct and trait implementation

The Hcaptcha validation code is derived using the `"token"` field as identified by the `#[captcha]` attribute for the Hcaptcha Response Token.

## Processing

The processing steps are:

1. Extract the contact form
2. Retrieve the secret from the parameter store
3. Validate the Hcaptcha response token
4. Setup futures to:
   1. Notify contact information to the office
   2. Notify receipt for contact to the contact
   3. Write a record of the contact data
5. Execute the futures
6. Check for errors and log
   - Notification errors are fatal and return an error
   - Write to database just logs the error
7. Return success

## Notes

Tracing is enabled on the functions but does not record to full contact information minimize the personal information in the log files. Each log is tagged with request_id to distinguish between different instances of the lambda running at the same time. The name is added as the only information a visitor would have to link to the transactions.
