output "lambda_invoke" {
  value = aws_lambda_function.lambda.invoke_arn
}

output "alias_invoke" {
  value = aws_lambda_alias.alias.invoke_arn
}

