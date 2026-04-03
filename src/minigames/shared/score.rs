use bevy::{prelude::*};

#[derive(Component)]
pub struct Score(usize);

#[derive(Component)]
pub struct CleanupScore;

#[derive(Component)]
pub struct ScoreText;

pub fn setup_score(
    mut commands: Commands,
) {
    commands.spawn((
        Score(0),
        CleanupScore,
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            right: px(5),
            ..default()
        },
        CleanupScore,
    )).with_children(|parent| {
        parent.spawn((
            Text::default(),
            TextLayout::new_with_justify(Justify::Center),
            TextShadow::default(),
        )).with_children(|parent| {
            parent.spawn((
                TextSpan::new("Score: 0"),
                ScoreText,
            ));
        });
    });
}

pub fn increase_score(
    mut score: Single<&mut Score>,
) {
    score.0 += 1;
}

pub fn reset_score(
    mut score: Single<&mut Score>
) {
    score.0 = 0;
}

pub fn display_score(
    score: Single<&Score>,
    mut query: Query<&mut TextSpan, With<ScoreText>>,
) {
    let mut span = query.single_mut().unwrap();
    span.0 = format!("Score: {}", score.0);
}

pub fn cleanup_score(
    mut commands: Commands,
    score_cleanups: Query<(Entity, &CleanupScore)>
) {
    for score_cleanup in score_cleanups {
        commands.entity(score_cleanup.0).despawn();
    }
}