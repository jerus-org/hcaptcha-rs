# Custom Contact Lambda

This example provides a skeleton for a lambda to handle the submission of a contact form.

The contact form collects the Name, Phone and Email contacts and a message from the visitor.

The visitor is confirmed to be human by Hcaptcha and the token is sent with the form data to the lambda for processing.

## Custom implementation of response validation `hcaptcha_response`

The `response_valid` function validates the response extracted from the event body with the following steps:

1. Get the secret from the parameter store
2. Initialize a new client to connect to hcaptcha.com
3. Create the request using the secret and captcha response
4. Execute the response and return the result

## Processing

The processing steps are:

1. Extract the captcha field from the event body
2. Validate the Hcaptcha response token with custom response validation
3. Extract the contact form
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
