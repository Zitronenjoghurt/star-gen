use crate::components::star_data::StarData;
use crate::events::star_spawn::StarSpawnEvent;
use crate::persistence::{delete_all_data, load_state, save_state};
use crate::physics::objects::star::Star;
use crate::plugins::star_events::StarEventsPlugin;
use crate::resources::settings::bloom::BloomSettings;
use crate::resources::settings::controls::ControlSettings;
use crate::resources::settings::graphics::GraphicsSettings;
use crate::resources::settings::wireframe::WireframeSettings;
use crate::resources::star_store::StarStore;
use bevy::prelude::{App, Assets, Events, Mesh, StandardMaterial, Vec3};
use bevy::MinimalPlugins;

fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    app.add_plugins(StarEventsPlugin);

    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Mesh>>();

    app.insert_resource(BloomSettings::default())
        .insert_resource(ControlSettings::default())
        .insert_resource(GraphicsSettings::default())
        .insert_resource(WireframeSettings::default())
        .insert_resource(StarStore::default());

    app
}

#[test]
fn test_save_load() {
    delete_all_data();

    // Create app, change some settings and spawn a star
    let mut app = build_app();

    let mut bloom_settings = app.world_mut().get_resource_mut::<BloomSettings>().unwrap();
    assert_eq!(bloom_settings.clone(), BloomSettings::default());
    bloom_settings.high_pass_frequency = 0.1;
    bloom_settings.active = false;

    let mut control_settings = app
        .world_mut()
        .get_resource_mut::<ControlSettings>()
        .unwrap();
    assert_eq!(control_settings.clone(), ControlSettings::default());
    control_settings.star_focus_auto_zoom_factor = 18.5;

    let mut graphics_settings = app
        .world_mut()
        .get_resource_mut::<GraphicsSettings>()
        .unwrap();
    assert_eq!(graphics_settings.clone(), GraphicsSettings::default());
    graphics_settings.vsync = false;
    graphics_settings.render_distance = 13.0;

    let mut wireframe_settings = app
        .world_mut()
        .get_resource_mut::<WireframeSettings>()
        .unwrap();
    assert_eq!(wireframe_settings.clone(), WireframeSettings::default());
    wireframe_settings.active = true;

    let initial_star = Star::new(100.0, Vec3::new(1.0, 2.0, 3.0));
    app.world_mut()
        .get_resource_mut::<Events<StarSpawnEvent>>()
        .unwrap()
        .send(StarSpawnEvent::new(initial_star.clone(), Some(10)));

    app.update();

    let star_store = app.world().get_resource::<StarStore>().unwrap();
    let stored_star = star_store.get_star(10).unwrap();
    let entity = star_store.get_entity(10).unwrap();
    assert_eq!(stored_star.clone(), initial_star.clone());

    let star_data = app.world().get::<StarData>(entity).unwrap();
    assert_eq!(star_data.get_star().clone(), initial_star.clone());

    save_state(app.world());

    // Create a new app, load state and validate data
    let mut new_app = build_app();
    load_state(new_app.world_mut());
    new_app.update();

    let bloom_settings = new_app.world().get_resource::<BloomSettings>().unwrap();
    assert_eq!(bloom_settings.high_pass_frequency, 0.1);
    assert!(!bloom_settings.active);

    let control_settings = new_app.world().get_resource::<ControlSettings>().unwrap();
    assert_eq!(control_settings.star_focus_auto_zoom_factor, 18.5);

    let graphics_settings = new_app.world().get_resource::<GraphicsSettings>().unwrap();
    assert!(!graphics_settings.vsync);
    assert_eq!(graphics_settings.render_distance, 13.0);

    let wireframe_settings = new_app.world().get_resource::<WireframeSettings>().unwrap();
    assert!(wireframe_settings.active);

    let star_store = new_app.world().get_resource::<StarStore>().unwrap();
    let stored_star = star_store.get_star(10).unwrap();
    let entity = star_store.get_entity(10).unwrap();
    assert_eq!(stored_star.clone(), initial_star.clone());

    let star_data = new_app.world().get::<StarData>(entity).unwrap();
    assert_eq!(star_data.get_star().clone(), initial_star.clone());

    delete_all_data();
}
