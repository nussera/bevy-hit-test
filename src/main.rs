use bevy::{prelude::*, core::FixedTimestep};
use bevy_prototype_lyon::{prelude::*, entity::ShapeBundle, render::Shape};
use lyon_algorithms::{hit_test, math::point};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(30.0 / 60.0))
                .with_system(hit_test_system)
        )
        .run();
}

fn setup_system(mut commands: Commands) {
    let svg_doc_size = Vec2::new(100.,100.);
    let shape = &shapes::SvgPathShape{
        svg_path_string:"M 0 50 a 50 50 0 1 1 0 1z M 5 50 a 45 45 0 1 1 0 1z".to_owned(),
        svg_doc_size_in_px:svg_doc_size.to_owned()
    };
    // let mut path_builder = PathBuilder::new();
    // path_builder.move_to(Vec2::new(0.0, 30.0));
    // path_builder.arc(Vec2::new(10.0, 10.0), Vec2::new(20.0, 20.0), 0.0, 0.0);
    // path_builder.close();
    // let path = path_builder.build();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    add_rectangle(&mut commands, 25f32);
    add_rectangle(&mut commands,50f32);
    add_rectangle(&mut commands,100f32);
    add_rectangle(&mut commands,200f32);
    commands.spawn_bundle(GeometryBuilder::build_as(
            shape,
            DrawMode::Stroke(StrokeMode::new(Color::LIME_GREEN, 1.0)),
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
        ))
        .insert(Name::new("test circle"));
    // commands.spawn_bundle(ShapeBundle{
    //     mode: DrawMode::Stroke(StrokeMode::new(Color::LIME_GREEN, 1.0)),
    //     path: path,
    //     ..Default::default()
    // })
    //     .insert(Name::new("test circle"));
}

fn hit_test_system(query: Query<&Path, With<Name>>) {
    let path = query.get_single().unwrap();
    let pathslices = path.0.as_slice();
    // let itr = pathslices.into_iter();
    println!("***********");
    for x in 0..100 {
        let point = point(x as f32, 0f32);
        let hit = hit_test::hit_test_path(&point, pathslices.iter(), FillRule::EvenOdd, 0.0001f32);
        println!("Hit at {x}: {hit}");
    }
}

fn add_rectangle(commands: &mut Commands, size: f32) {
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shapes::Rectangle{ extents: Vec2::new(size, size), origin: RectangleOrigin::Center },
        DrawMode::Stroke(StrokeMode::new(Color::RED, 1.0)),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}
