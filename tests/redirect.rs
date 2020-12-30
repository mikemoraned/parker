use reqwest::StatusCode;
use url::Url;

mod support;

#[actix_rt::test]
async fn redirects_root_url() {
    // Arrange
    let address = support::spawn_app(Url::parse("http://example.com/").unwrap());

    // Act
    let response = support::client()
        .get(&format!("{}/", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "http://example.com/"
    );
}

#[actix_rt::test]
async fn redirects_top_level_url() {
    // Arrange
    let address = support::spawn_app(Url::parse("http://example.com/").unwrap());

    // Act
    let response = support::client()
        .get(&format!("{}/some_url.html", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "http://example.com/"
    );
}

#[actix_rt::test]
async fn redirects_nested_url() {
    // Arrange
    let address = support::spawn_app(Url::parse("http://example.com/").unwrap());

    // Act
    let response = support::client()
        .get(&format!("{}/nested/to/some/depth.html", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "http://example.com/"
    );
}
