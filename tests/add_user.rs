mod common;

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let address = common::spawn_app();

    let client = reqwest::Client::new();
    let params= "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .get(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(params)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_400_for_missing_data() {
    let address = common::spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing all parameters"),
    ];

    for (params, error_message) in test_cases {
        let response = client
            .get(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(params)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(400, response.status().as_u16(),
        "The API did not fail with 400 Bad Request when the payload was {}",
        error_message);
    }
}

