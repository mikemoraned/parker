use actix_web::http::HeaderValue;
use url::Url;

mod support;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let address = support::spawn_app(Url::parse("http://example.com/").unwrap());

    // Act
    let response = support::client()
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(
        response.headers().get("Content-Length").unwrap(),
        HeaderValue::from(0)
    );
}
