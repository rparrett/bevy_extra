use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup.system())
        .run();

    Ok(())
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());

    let size = 80.;

    let spacing = 1.6;
    let screen_x = 570.;
    let screen_y = 150.;
    let mut x = -screen_x;
    let mut y = screen_y;

    for ease_function in &[
        bevy_easings::EaseFunction::QuadraticIn,
        bevy_easings::EaseFunction::QuadraticOut,
        bevy_easings::EaseFunction::QuadraticInOut,
        bevy_easings::EaseFunction::CubicIn,
        bevy_easings::EaseFunction::CubicOut,
        bevy_easings::EaseFunction::CubicInOut,
        bevy_easings::EaseFunction::QuarticIn,
        bevy_easings::EaseFunction::QuarticOut,
        bevy_easings::EaseFunction::QuarticInOut,
        bevy_easings::EaseFunction::QuinticIn,
        bevy_easings::EaseFunction::QuinticOut,
        bevy_easings::EaseFunction::QuinticInOut,
        bevy_easings::EaseFunction::SineIn,
        bevy_easings::EaseFunction::SineOut,
        bevy_easings::EaseFunction::SineInOut,
        bevy_easings::EaseFunction::CircularIn,
        bevy_easings::EaseFunction::CircularOut,
        bevy_easings::EaseFunction::CircularInOut,
        bevy_easings::EaseFunction::ExponentialIn,
        bevy_easings::EaseFunction::ExponentialOut,
        bevy_easings::EaseFunction::ExponentialInOut,
        bevy_easings::EaseFunction::ElasticIn,
        bevy_easings::EaseFunction::ElasticOut,
        bevy_easings::EaseFunction::ElasticInOut,
        bevy_easings::EaseFunction::BackIn,
        bevy_easings::EaseFunction::BackOut,
        bevy_easings::EaseFunction::BackInOut,
        bevy_easings::EaseFunction::BounceIn,
        bevy_easings::EaseFunction::BounceOut,
        bevy_easings::EaseFunction::BounceInOut,
    ] {
        commands
            .spawn((
                Transform::default().with_translate(Vec3::new(x, y, 0.)),
                GlobalTransform::default(),
            ))
            .with_children(|parent| {
                parent
                    .spawn(SpriteComponents {
                        material: materials.add(Color::RED.into()),
                        sprite: Sprite {
                            size: Vec2::new(size, size),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with(Transform::default().ease_to(
                        Transform::default().with_rotation(Quat::from_rotation_ypr(
                            0.,
                            0.,
                            3.1415 / 2.,
                        )),
                        *ease_function,
                        bevy_easings::AnimationType::PingPong {
                            duration: std::time::Duration::from_secs(1),
                            pause: std::time::Duration::from_millis(500),
                        },
                    ));
            });
        y -= size * spacing;
        if y < -screen_y {
            x += size * spacing;
            y = screen_y;
        }
    }
}