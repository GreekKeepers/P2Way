use p2way::P2Way;

#[tokio::test]
pub async fn test_one_time_token() {
    let p2way = P2Way::new("".into(), "".into(), p2way::PROD_URL);

    let token = p2way.one_time_token_generation().await;

    println!("One time token: {:?}", token);

    assert!(token.is_ok());
}

#[tokio::test]
pub async fn test_get_settings() {
    let p2way = P2Way::new("".into(), "".into(), p2way::SANDBOX_URL);

    let token = p2way.one_time_token_generation().await;

    println!("One time token: {:?}", token);

    assert!(token.is_ok());

    let settings = p2way.get_widget_settings(token.unwrap().token).await;

    println!("Settings: {:?}", settings);
    assert!(settings.is_ok());
}
