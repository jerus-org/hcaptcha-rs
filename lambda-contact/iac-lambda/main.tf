provider "aws" {
  region  = var.region
  profile = var.aws_profile_id
}

terraform {
  backend "s3" {}
}

data "aws_api_gateway_rest_api" "web_api" {
  name = var.apigw
}

data "aws_api_gateway_resource" "contact" {
  rest_api_id = data.aws_api_gateway_rest_api.web_api.id
  path        = "/${var.api_endpoint}"
}

# lambda

data "aws_iam_role" "role" {
  name = var.name
}

resource "aws_lambda_permission" "apigw_lambda" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "arn:aws:execute-api:${var.region}:${var.aws_account_id}:${data.aws_api_gateway_rest_api.web_api.id}/*/POST${data.aws_api_gateway_resource.contact.path}"

}

resource "aws_lambda_alias" "alias" {
  name             = var.release
  function_name    = aws_lambda_function.lambda.arn
  function_version = aws_lambda_function.lambda.version
}

resource "aws_lambda_function" "lambda" {
  function_name = var.lambda_name
  description   = var.lambda_description
  role          = data.aws_iam_role.role.arn
  handler       = "hello.handler"
  runtime       = "provided.al2"
  filename      = "rust.zip"
  publish       = true

  source_code_hash = filebase64sha256("rust.zip")

  environment {
    variables = {
      "RUST_LOG" = "info"
      # "RUST_LOG" = "bootstrap::handler=debug, hcaptcha=debug"
    }
  }

}
