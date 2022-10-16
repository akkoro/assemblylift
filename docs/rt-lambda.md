AWS Lambda Runtime
------------------

The Lambda runtime is designed to work with the Lambda [Custom Runtime API](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html). 
It conforms with the Lambda [execution environment](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-custom.html), 
e.g. deployed as a binary named `bootstrap` and working with the custom runtime environment variables like `LAMBDA_TASK_ROOT`.  

WebAssembly modules are invoked in response to a new event, which is found by polling the "next event" API.

Requests are processed in order -- modules are not run in parallel.
