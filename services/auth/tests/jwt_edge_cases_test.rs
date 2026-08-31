use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use kubelab_auth::jwt::JwtService;
use kubelab_auth::models::{Claims, User, UserRole};
use uuid::Uuid;

#[test]
fn test_jwt_tampered_and_invalid_signatures() {
    let jwt_service = JwtService::new("secret-key-alpha-32-chars-long-123".to_string());
    let attacker_jwt = JwtService::new("attacker-forged-secret-key-9999".to_string());

    let user = User {
        id: Uuid::new_v4(),
        email: "student@kubelab.io".to_string(),
        name: "Student".to_string(),
        password_hash: "hash".to_string(),
        role: UserRole::Learner,
        avatar_url: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let (forged_token, _) = attacker_jwt.generate_token(&user).unwrap();

    // Verification against authentic secret MUST fail
    let res = jwt_service.verify_token(&forged_token);
    assert!(res.is_err(), "Forged token signature must be rejected");
}

#[test]
fn test_jwt_expired_token_rejection() {
    let jwt_service = JwtService::new("secret-key-alpha-32-chars-long-123".to_string());
    let past_time = Utc::now() - Duration::hours(2);

    let expired_claims = Claims {
        sub: Uuid::new_v4().to_string(),
        email: "expired@kubelab.io".to_string(),
        role: "learner".to_string(),
        exp: past_time.timestamp() as usize,
        iat: (past_time - Duration::hours(1)).timestamp() as usize,
        token_type: Some("access".to_string()),
    };

    let expired_token = encode(
        &Header::default(),
        &expired_claims,
        &EncodingKey::from_secret("secret-key-alpha-32-chars-long-123".as_bytes()),
    )
    .unwrap();

    let res = jwt_service.verify_token(&expired_token);
    assert!(res.is_err(), "Expired JWT must be rejected");
}

#[test]
fn test_jwt_token_type_mismatch_guards() {
    let jwt_service = JwtService::new("secret-key-alpha-32-chars-long-123".to_string());

    let user = User {
        id: Uuid::new_v4(),
        email: "admin@kubelab.io".to_string(),
        name: "Admin".to_string(),
        password_hash: "hash".to_string(),
        role: UserRole::Admin,
        avatar_url: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let (access_token, refresh_token, _) = jwt_service.generate_tokens(&user).unwrap();

    // Verify access token works on standard verification
    let access_claims = jwt_service.verify_token(&access_token).unwrap();
    assert_eq!(access_claims.token_type, Some("access".to_string()));

    // Verify refresh token works on refresh verification
    let refresh_claims = jwt_service.verify_refresh_token(&refresh_token).unwrap();
    assert_eq!(refresh_claims.token_type, Some("refresh".to_string()));

    // Verify access token is REJECTED if passed to refresh token validator
    let cross_type_check = jwt_service.verify_refresh_token(&access_token);
    assert!(cross_type_check.is_err(), "Access token must not be accepted as a refresh token");
}
