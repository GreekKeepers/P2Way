use p2way::P2Way;

#[tokio::test]
pub async fn test_one_time_token() {
    let p2way = P2Way::new("".into(), "".into(), p2way::SANDBOX_URL);

    let token = p2way.one_time_token_generation().await;

    println!("One time token: {:?}", token);

    assert!(token.is_ok());
}
