extern crate rand;

// NOTE: THIS DOES NOT WORK @rust1.47.0+

mod boid;
mod constants;
mod point;
mod vector;
mod world;

use amethyst::{
    assets::{DefaultLoader, Handle, Loader, LoaderBundle, ProcessingQueue},
    core::transform::{Transform, TransformBundle},
    ecs::World,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        Camera, RenderingBundle, SpriteRender, SpriteSheet,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    window::ScreenDimensions,
};
use world::World as BirdWorld;

const NUM_BOIDS: u32 = 250;
const SIZE: u32 = 600;

const BOID_BOD: &'static [[f64; 2]] = &[[5.0, 5.0], [10.0, 0.0], [5.0, 15.0], [0.0, 0.0]];

struct GameState {
    flock: BirdWorld,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData {
            world, resources, ..
        } = data;

        init_camera(world, resources);

        let sprite_sheet_handle = load_sprite_sheet(&resources);
        init_sprite(world, &sprite_sheet_handle);
    }

    // fn fixed_update(&mut self, _data: StateData<'_, GameData>) -> SimpleTrans {
    //     self.flock.step(1.1);
    //     let boids = self.flock.get_boids();
    //     // for i in 0..boids.len() {
    //     //     let boid = boids[i];
    //     //     let point = boid.get_point();
    //     //     let transform = context
    //     //         .transform
    //     //         .trans(point.get_x() as f64, point.get_y() as f64)
    //     //         .rot_rad(-boid.get_angle() as f64);

    //     //     // polygon(boid.color, BOID_BOD, transform, gfx);
    //     // }

    //     Trans::None
    // }
}

fn init_camera(world: &mut World, resources: &mut Resources) {
    let dimensions = resources.get::<ScreenDimensions>().unwrap();
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.0);

    world.push((
        Camera::standard_2d(dimensions.width(), dimensions.height()),
        transform,
    ));
}

fn load_sprite_sheet(resources: &Resources) -> Handle<SpriteSheet> {
    let loader = resources.get::<DefaultLoader>().unwrap();

    let texture = loader.load("sprites/bird.png");
    let sprites = loader.load("sprites/bird.ron");

    let sprite_sheet_store = resources.get::<ProcessingQueue<SpriteSheet>>().unwrap();
    loader.load_from_data(SpriteSheet { texture, sprites }, (), &sprite_sheet_store)
}

fn init_sprite(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) {
    let transform = Transform::default();
    let sprite = SpriteRender::new(sprite_sheet.clone(), 0);
    world.push((transform, sprite));
}

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?;
    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display_config.ron");

    let mut game_data = DispatcherBuilder::default();
    game_data
        .add_bundle(TransformBundle::default())
        .add_bundle(LoaderBundle)
        .add_bundle(UiBundle::<u32>::default())
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?.with_clear(ClearColor {
                        float32: [0.34, 0.36, 0.52, 1.0],
                    }),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        );

    let the_flock = BirdWorld::new(NUM_BOIDS, SIZE as f32);
    let game = Application::new(resources, GameState { flock: the_flock }, game_data)?;
    println!(
        "=== Flock of Seaboids with Amethyst ===\n {} [version {}]",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    );
    game.run();

    Ok(())
}
