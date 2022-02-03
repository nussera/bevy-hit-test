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
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(hit_test_system)
        )
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    //Spawn some rectangles for reference.
    add_rectangle(&mut commands,25f32);
    add_rectangle(&mut commands,50f32);
    add_rectangle(&mut commands,100f32);
    add_rectangle(&mut commands,200f32);

    // We can create a doughnut via an SVG Path. This will generate hits.
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shapes::SvgPathShape{
            // When used in a browser, we need to flip the sweep flag.
            // M 0 50 a 50 50 0 1 >1< 0 1z M 5 50 a 45 45 0 1 >1< 0 1
            // Maybe a bug in lyon?
            svg_path_string:"M 0 50 a 50 50 0 1 0 0 1z M 5 50 a 45 45 0 1 0 0 1z".to_owned(),
            svg_doc_size_in_px: Vec2::new(100.,100.).to_owned()
        },
        DrawMode::Fill(FillMode{
            options: FillOptions::DEFAULT,
            color: Color::LIME_GREEN
        }),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ))
    .insert(Name::new("test circle"));

    // Or via the lyon PathBuilder. Unfortunately, this does not generate any hits.
    /*let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(45., 0.0));
    path_builder.arc(Vec2::new(0.0, 0.0), Vec2::new(45.0, 45.0), 6.2, 0.0);
    path_builder.close();
    path_builder.move_to(Vec2::new(50., 0.0));
    path_builder.arc(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0), 6.2, 0.0);
    path_builder.close();
    let path = path_builder.build();

    commands.spawn_bundle(ShapeBundle{
        mode: DrawMode::Fill(FillMode{
            options: FillOptions::DEFAULT,
            color: Color::LIME_GREEN
        }),
        path,
        ..Default::default()
    })
    .insert(Name::new("test circle"));*/
}

fn hit_test_system(query: Query<&Path, With<Name>>) {
    let path = query.get_single().unwrap();
    let pathslices = path.0.as_slice();
    println!("***********");
    //We expect hits at 45<=x<=50
    for x in 0..55 {
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
