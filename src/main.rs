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
    // let shape = &shapes::Circle {
    //     radius: 30f32,
    //     ..Default::default()
    // };
    // let svg_doc_size = Vec2::new(512.,512.);
    // let shape = &shapes::SvgPathShape{
    //     svg_path_string:"m 210.49052,219.61666 c -54.97575,-3.12045 -153.83891,-43.5046 -181.900067,-79.34483 41.944976,3.29834 143.100787,1.42313 185.138697,1.61897 l 6e-5,-0.003 c 41.78023,-0.87477 200.563,-0.4537 261.24529,0 0.085,7.05106 0.79737,22.71244 1.07386,32.86306 -42.04814,8.31883 -101.90702,24.33338 -128.45794,63.97855 -10.53308,31.59203 39.6912,45.827 74.62215,55.19132 1.14898,12.80889 2.62233,32.62936 2.46309,44.71853 -75.4682,-0.86499 -141.64601,-1.07063 -209.86695,-1.35786 -10.81491,-1.77566 -6.66734,-23.1495 -4.31819,-32.38456 5.44628,-16.65332 38.03788,-18.20507 28.06768,-83.12367 -7.29786,-2.58188 -23.92259,-1.83114 -28.06768,-2.15756".to_owned(),
    //     svg_doc_size_in_px:svg_doc_size.to_owned()
    // };
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(10.0,-10.0));
    path_builder.line_to(Vec2::new(10.0,10.0));
    let path = path_builder.build();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.spawn_bundle(GeometryBuilder::build_as(
    //         shape,
    //         DrawMode::Stroke(StrokeMode::new(Color::LIME_GREEN, 2.0)),
    //         Transform {
    //             translation: Vec3::new(0.0, 0.0, 0.0),
    //             ..Default::default()
    //         },
    //     ))
    //     .insert(Name::new("test circle"));
    commands.spawn_bundle(ShapeBundle{
        mode: DrawMode::Stroke(StrokeMode::new(Color::LIME_GREEN, 2.0)),
        path: path,
        ..Default::default()
    })
        .insert(Name::new("test circle"));
}

fn hit_test_system(query: Query<&Path, With<Name>>) {
    let path = query.get_single().unwrap();
    let pathslices = path.0.as_slice();
    // let itr = pathslices.into_iter();
    for x in 0..20 {
        let point = point(x as f32, 0f32);
        let hit = hit_test::hit_test_path(&point, pathslices.iter(), FillRule::EvenOdd, 0.0001f32);
        println!("Hit at {x}: {hit}");
    }
}
