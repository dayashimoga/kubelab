use kubelab_learning::service::LearningService;

#[tokio::test]
async fn test_learning_service_tracks_and_lessons() {
    let service = LearningService::new();

    // 1. Verify tracks listing
    let tracks = service.list_tracks().await;
    assert!(!tracks.is_empty(), "Track catalog should not be empty");

    // 2. Query track by slug
    let first_track = &tracks[0];
    let found_track = service
        .get_track_by_slug(&first_track.slug)
        .await
        .expect("Track should be found by slug");
    assert_eq!(found_track.title, first_track.title);

    // 3. Query lessons for track
    let lessons = service.list_lessons_by_track(&first_track.slug).await;
    assert!(
        !lessons.is_empty(),
        "Lessons should exist for track {}",
        first_track.slug
    );

    // 4. Query individual lesson by ID
    let lesson_id = &lessons[0].id;
    let found_lesson = service
        .get_lesson_by_id(lesson_id)
        .await
        .expect("Lesson should be found by ID");
    assert_eq!(found_lesson.id, *lesson_id);
    assert!(!found_lesson.content_markdown.is_empty());

    // 5. Query non-existent track & lesson -> returns error
    assert!(service.get_track_by_slug("non-existent-track").await.is_err());
    assert!(service.get_lesson_by_id("non-existent-lesson").await.is_err());
}
