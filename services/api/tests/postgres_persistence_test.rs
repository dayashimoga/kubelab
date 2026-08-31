use kubelab_api::db::Database;
use kubelab_api::db::users::UserRepository;
use kubelab_api::db::progress::ProgressRepository;
use kubelab_api::db::labs::LabRepository;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_postgres_persistence_and_migrations() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://kubelab:kubelab_secret_password@127.0.0.1:5432/kubelab".to_string());

    // Connect to PostgreSQL and automatically run migrations
    let db = match Database::connect(&database_url).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping live postgres test (database unreachable: {:?})", e);
            return;
        }
    };

    assert!(db.ping().await.is_ok(), "PostgreSQL ping must succeed");

    let pool = db.pool();

    // 1. Test User Repository
    let user_repo = UserRepository::new(pool);
    let unique_email = format!("learner_{}@kubelab.io", Uuid::new_v4());
    let user = user_repo
        .create_user(&unique_email, "Jane Doe", "$argon2id$mockhash", "learner")
        .await
        .expect("User creation must succeed");

    assert_eq!(user.email, unique_email);
    assert_eq!(user.role, "learner");

    let fetched_user = user_repo
        .find_by_email(&unique_email)
        .await
        .expect("Fetch user must succeed")
        .expect("User must exist in DB");
    assert_eq!(fetched_user.id, user.id);

    // 2. Test Progress Repository
    let progress_repo = ProgressRepository::new(pool);
    let initial_progress = progress_repo
        .get_progress(user.id)
        .await
        .expect("Fetch progress must succeed")
        .expect("Progress row must exist");
    assert_eq!(initial_progress.total_xp, 0);
    assert_eq!(initial_progress.level, 1);

    let updated_progress = progress_repo
        .add_xp(user.id, 600)
        .await
        .expect("Add XP must succeed");
    assert_eq!(updated_progress.total_xp, 600);
    assert_eq!(updated_progress.level, 2);

    let lab_progress = progress_repo
        .record_lab_completion(user.id, "k8s-pod-basics", 200)
        .await
        .expect("Record lab completion must succeed");
    assert_eq!(lab_progress.total_xp, 800);

    // 3. Test Lab Session Repository
    let lab_repo = LabRepository::new(pool);
    let expires = Utc::now() + chrono::Duration::hours(1);
    let session = lab_repo
        .create_session(user.id, "k8s-pod-basics", &format!("sb-{}", Uuid::new_v4()), expires)
        .await
        .expect("Create lab session must succeed");

    assert_eq!(session.status, "running");

    let completed_session = lab_repo
        .update_score(session.id, 100, "completed")
        .await
        .expect("Update session score must succeed");
    assert_eq!(completed_session.score, 100);
    assert_eq!(completed_session.status, "completed");
    assert!(completed_session.completed_at.is_some());
}
