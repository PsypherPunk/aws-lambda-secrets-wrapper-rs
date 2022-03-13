# `aws-lambda-secrets-wrapper-rs`

This idea was inspired by AWS'
[documentation](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-modify.html#runtime-wrapper-example)
on extensions to the AWS Lambda runtime process and the inability to securely
pass secrets to AWS Lambda, much
[as you can](https://aws.amazon.com/premiumsupport/knowledge-center/ecs-data-security-container-task/)
with AWS ECS.

## Why?

To pass a secret to AWS Lambda, you can either:

- pass the ARN as an environment variable and have your Lambda retrieve the
  secret at runtime;
- pass the secret itself as an environment variable, encrypting the environment
  variables with KMS and restricting access to the key.

Both work, are valid and offer a secure way of handling secrets.

This is purely an attempt at an alternative wherein:

- you don't have to write code to retrieve those secrets: that should be
  managed by the runtime and passed to the Lambda, which can remain blissfully
  unaware;
- you don't have to be concerned about encrypting any secrets and managing the
  cost and concerns of yet another key.

## How?

If you want want to pass a secret, say an obviously-named environment variable
like `DATABASE_PASSWORD`, to an AWS Lambda, there are a few steps:

- add an *layer* to the Lambda containing a release from this repository;
- pass the ARN of the secret, securely stored in AWS Secrets Manager, to the
  AWS Lambda (the value will always be `/opt/secrets-wrapper`), suffixed with
  `_SECRET_ARN`;
- set the environment variable `AWS_LAMBDA_EXEC_WRAPPER`.

In Terraform, this might look like:

```hcl
resource "aws_lambda_layer_version" "this" {
  filename            = "aws-lambda-secrets-wrapper-0.1.0.zip"
  layer_name          = "wrapper"
}

resource "aws_lambda_function" "this" {
  layers = [
    aws_lambda_layer_version.wrapper.arn,
  ]

  environment {
    variables = {
      AWS_LAMBDA_EXEC_WRAPPER      = "/opt/secrets-wrapper"
      DATABASE_PASSWORD_SECRET_ARN = "arn:aws:secretsmanager:eu-west-1:…:secret:…"
    }
  }
}
```

When your Lambda runs, this will retrieve the secret from Secrets Manager via
the ARN and make the resulting value available to the Lambda, minus the
`_SECRET_ARN` suffix.
