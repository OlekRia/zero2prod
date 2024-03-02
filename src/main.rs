use zero2prod::run;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    run().await
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::test::TestRequest;

//     #[tokio::test]
//     async fn health_check_succeeds() {
//         let request = TestRequest::default().to_http_request();
//         let response = health_check().await;
//         // This requires changing the return type of `health_check`
//         // from `impl Responder` to `HttpResponse` to compile
//         // You also need to import it with `use actix_web::HttpResponse`! assert!(response.status().is_success())
//         assert!(response.respond_to(&request).status().is_success());
//     }
// }
