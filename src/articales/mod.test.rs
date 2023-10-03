
// Controller function get_artical returns expected response
#[actix_rt::test]
async fn the_test_name() {
    // Arrange

    // Act

    // Assert
}

// GET request to "/artcale" returns expected response
#[actix_rt::test]
async fn the_test_name() {
    use actix_web::{test, App};
    use controller::get_artical;
    
    let mut app = test::init_service(App::new().service(articales_module)).await;
    
    let req = test::TestRequest::get().uri("/artcale").to_request();
    let resp = test::call_service(&mut app, req).await;
    
    assert!(resp.status().is_success());
}


// Request with missing JSON body returns 400 error
#[actix_rt::test]
async fn test_request_with_missing_json_body_returns_400_error() {
    let mut app = test::init_service(App::new().service(articales_module)).await;
    let req = test::TestRequest::post()
        .uri("/artcale")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}


