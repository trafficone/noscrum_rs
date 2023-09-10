# noscrum_rs
NoScrum is not scrum, there are no scrums, you are one person and you need to get things done.

## Technical Description
AWS Resource Stack
- SAM -- manages deployment of resources
- API Gateway -- handles HTTP requests
- Cognito -- manages auth of users
- DynamoDB -- handles data
- Lambda -- renders request results

Core dependencies
- lambda_http; axum -- convert between Lambda requests and HTTP requests
- axum; leptos_axum -- convert between Axum requests to Leptos routing
- dynomite -- Rust data types in DynamoDB
- leptos -- render reactive pages with co-located API handlers
