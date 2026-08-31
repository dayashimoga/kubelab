use kubelab_auth::AuthService;

#[tokio::test]
async fn test_full_registration_and_login_flow() {
    let auth = AuthService::new("test-secret-key-32-characters-long!".to_string());

    // 1. Register new learner
    let reg_res = auth
        .register("cloud.engineer@kubelab.io", "Cloud Engineer", "SecureP@ss123!", None)
        .await
        .expect("Registration should succeed");

    assert_eq!(reg_res.user.email, "cloud.engineer@kubelab.io");
    assert!(!reg_res.tokens.access_token.is_empty());

    // 2. Duplicate registration fails
    let dup_err = auth
        .register("cloud.engineer@kubelab.io", "Another Name", "SecureP@ss123!", None)
        .await;
    assert!(dup_err.is_err());

    // 3. Login with valid credentials
    let login_res = auth
        .login("cloud.engineer@kubelab.io", "SecureP@ss123!")
        .await
        .expect("Login should succeed");
    assert_eq!(login_res.user.id, reg_res.user.id);

    // 4. Login with invalid password fails
    let invalid_res = auth.login("cloud.engineer@kubelab.io", "WrongPassword!").await;
    assert!(invalid_res.is_err());
}
