
use ncollide3d::math::Isometry;
use ncollide3d::math::Vector;
use ncollide3d::shape::ShapeHandle;
use ncollide3d::shape::Cuboid;
use ncollide3d::pipeline::GeometricQueryType;
use ncollide3d::pipeline::CollisionGroups;
use ncollide3d::pipeline::CollisionObjectSlabHandle;
use ncollide3d::narrow_phase::ContactEvent;
use bevy::prelude::*;
use bevy_input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType};
use ncollide3d::world::CollisionWorld;
use ncollide3d;
mod sys;
use sys::entity::*;
use sys::gamepad::*;


fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .init_resource::<PlayerPos>()
        .init_resource::<CameraPos>()
        .init_resource::<GamepadState>()
        .init_resource::<CollisionMeta>()

        .add_startup_system(setup.system())

        .add_system(setup_player.system())
        .add_system(setup_static.system())

        .add_system(gamepad_update.system())

        .add_system(gamepad_player.system())
        .add_system(gamepad_camera.system())

        .add_system(playerpos_velocity.system())
        .add_system(camerapos_velocity.system())

        .run();
}


struct CollisionMeta {
    world: CollisionWorld<f32, CollisionObjectData>,
    staticgroup: CollisionGroups,
    playergroup: CollisionGroups,
    contacts_query: GeometricQueryType<f32>,
    static_done: bool,
    player_done: bool,
}

#[derive(Default)]
struct CollisionObjectData{
    id: u32
}

impl Default for CollisionMeta {
	fn default() -> Self {
        let mut staticgroup = CollisionGroups::new();
        let mut playergroup = CollisionGroups::new();

        playergroup.set_membership(&[1]);
        staticgroup.set_membership(&[2]);
        staticgroup.set_whitelist(&[1]);

        Self {
            world: CollisionWorld::new(0.02),
            staticgroup: staticgroup,
            playergroup: playergroup,
            contacts_query: GeometricQueryType::Contacts(0.0, 0.0),
            static_done: false,
            player_done: false,
        }
    }
}

impl CollisionMeta {
    fn add_player(&mut self, pos: Vec3) -> CollisionObjectSlabHandle {
        let mut position = Isometry::new(
            Vector::new(
               pos.x(),
               pos.y(),
               pos.z(),
            ),
            Vector::new(0., 0., 0.)
        );

        let (handle, _) = self.world.add(
            position,
            ShapeHandle::new(Cuboid::<f32>::new(Vector::new(1., 1., 1.))),
            self.playergroup,
            self.contacts_query,

            CollisionObjectData::default()
        );

        //let ball_object = self.world.collision_object(handle).unwrap();

        return handle;

    }

    fn add_static(&mut self, pos: Vec3) -> CollisionObjectSlabHandle {

        let mut position = Isometry::new(
            Vector::new(
               pos.x(),
               pos.y(),
               pos.z(),
            ),
            Vector::new(0., 0., 0.)
        );

        let (handle, _) = self.world.add(
            position,
            ShapeHandle::new(Cuboid::<f32>::new(Vector::new(1., 1., 1.))),
            self.staticgroup,
            self.contacts_query,

            CollisionObjectData::default()
        );

        return handle;
    }
    fn set_position(&mut self, handle: Option<CollisionObjectSlabHandle>, pos: Vec3) {
        let mut position = Isometry::new(
            Vector::new(
               pos.x(),
               pos.y(),
               pos.z(),
            ),
            Vector::new(0., 0., 0.)
        );

        match handle {
            Some(h) => {
                //h.set_position(position);
                let obj = self.world.collision_object(h);
                match obj {
                    Some(o) => {
                        //println!("{:?}", o.data().id)
                        //print!("R")
                    }
                    None => {}
                }

                self.world.set_position(h, position);
                self.world.update();
            }
            None => {}

        }

    }
}




/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world

    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })

        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.1, 0.2).into()),
            transform: Transform::from_translation(Vec3::new(3.0, 1.0, 5.0)),
            ..Default::default()
        }) 
        .with(StaticEntity::default())

        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.1, 0.2).into()),
            transform: Transform::from_translation(Vec3::new(4.0, 1.0, 5.0)),
            ..Default::default()
        }) 
        .with(StaticEntity::default())
        
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.1, 0.2).into()),
            transform: Transform::from_translation(Vec3::new(5.0, 1.0, 5.0)),
            ..Default::default()
        }) 
        .with(StaticEntity::default())

        // Player
        .spawn(PbrComponents {
            //mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            mesh: asset_server.load("models/monkey/Monkey.gltf#Mesh0/Primitive0"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(PlayerEntity::default())

        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })

        // camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(CameraEntity)
        ;
}


fn setup_player(
    mut collision_meta: ResMut<CollisionMeta>,
    //mut query: Query<(&mut PlayerEntity, &mut Transform)>
    mut query: Query<(&mut PlayerEntity, &mut Transform)>
) {
    if collision_meta.player_done {return}
    println!("setup_player");
    //let mut world = &collision_meta.world;
    //let mut playergroup = &collision_meta.playergroup;
    //let mut contacts_query = &collision_meta.contacts_query;

    for (mut player_entity, mut transform) in query.iter_mut() {
        print!("Setup...");

        let handle = collision_meta.add_player(
            transform.translation,
            //ShapeHandle::new(Cuboid::<f32>::new(Vector::new(1., 1., 1.))),

            //CollisionObjectData::default()
        );

        println!("{}", handle.uid());

        player_entity.handle = Some(handle);
    }
    collision_meta.player_done = true;

}

fn setup_static(
    mut collision_meta: ResMut<CollisionMeta>,
    mut query: Query<(&mut StaticEntity, &mut Transform)>
) {
    if collision_meta.static_done {return}
    println!("setup_static");
    for (mut static_entity, mut transform) in query.iter_mut() {

        let handle = collision_meta.add_static(
            transform.translation,
            //ShapeHandle::new(Cuboid::<f32>::new(Vector::new(1., 1., 1.))),

            //CollisionObjectData::default()
        );
        print!("Setup...");

        static_entity.handle = Some(handle);

    }

    collision_meta.static_done = true;
}


fn gamepad_update(
    mut event_reader: Local<EventReader<GamepadEvent>>,
    gamepad_event: Res<Events<GamepadEvent>>,
    mut gamepad_state: ResMut<GamepadState>,
) {
    for event in event_reader.iter(&gamepad_event) {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::ButtonChanged(GamepadButtonType::South, value)) => {
                if(*value == 1f32) {
                    gamepad_state.jump = true;
                } else {
                    gamepad_state.jump = false;
                }
            }

            GamepadEvent(gamepad, GamepadEventType::ButtonChanged(button_type, value)) => {
                println!("{:?} of {:?} is changed to {}", button_type, gamepad, value);
            }


            GamepadEvent(_, GamepadEventType::AxisChanged(GamepadAxisType::LeftStickX, value)) => {
                //playerpos.dpos.set_x(*value);
                gamepad_state.joy.set_x(*value);
            }

            GamepadEvent(_, GamepadEventType::AxisChanged(GamepadAxisType::LeftStickY, value)) => {
                gamepad_state.joy.set_y(*value);
            }
            GamepadEvent(_, GamepadEventType::AxisChanged(GamepadAxisType::RightStickX, value)) => {
                gamepad_state.cam.set_x(*value);
            }
            GamepadEvent(_, GamepadEventType::AxisChanged(GamepadAxisType::RightStickY, value)) => {
                gamepad_state.cam.set_y(*value);
            }
            _ => {}
        }
    }
}


fn gamepad_player(
    camerapos: Res<CameraPos>,
    mut playerpos: ResMut<PlayerPos>,
    gamepad_state: Res<GamepadState>,
) {

    let mut dpos = - gamepad_state.joy.x() * (camerapos.pos - playerpos.pos).cross(Vec3::unit_y());
    dpos += gamepad_state.joy.y() * (playerpos.pos - camerapos.pos);
    if(gamepad_state.jump) {
        dpos.set_y(1.0);
    } else {
        dpos.set_y(-0.1);
    }

    if dpos.length_squared() > 0.0 {
        playerpos.facing = playerpos.pos - 2. * dpos;
    }

    playerpos.dpos = dpos;
}


fn gamepad_camera(
    gamepad_state: Res<GamepadState>,
    playerpos: Res<PlayerPos>,
    mut camerapos: ResMut<CameraPos>,
) {
    let mut dpos = gamepad_state.cam.x() * (playerpos.pos - camerapos.pos).cross(Vec3::unit_y());
    dpos += gamepad_state.cam.y() * (playerpos.pos - camerapos.pos);

    camerapos.dpos = dpos;
}

fn camerapos_velocity(
    time: Res<Time>,
    playerpos: Res<PlayerPos>,
    mut camerapos: ResMut<CameraPos>,
    mut query: Query<(&CameraEntity, &mut Transform)>
) {
    for (_, mut transform) in query.iter_mut() {

        let dpos = time.delta_seconds * camerapos.dpos; 

        camerapos.pos += dpos;
    
        transform.translation = camerapos.pos;

        *transform = transform.looking_at(playerpos.pos, Vec3::unit_y());
        //transform.rotate(Quat::from_rotation_z(camerapos.yaw))
    }
}

fn playerpos_velocity(
    time: Res<Time>,
    mut collision_meta: ResMut<CollisionMeta>,
    mut playerpos: ResMut<PlayerPos>,
    mut query: Query<(&PlayerEntity, &mut Transform)>
) {

    for (player_entity, mut transform) in query.iter_mut() {

        for event in collision_meta.world.contact_events() {
            //println!("contact event!");
            //handle_contact_event(&world, event)
        }

        for event in collision_meta.world.proximity_events() {
            //println!("proximity event!");
            //handle_contact_event(&world, event)
        }


        let dpos = time.delta_seconds * playerpos.dpos; 


        let mut reaction = Vec3::zero();

        for (_, _, _, manifold) in collision_meta.world.contacts_with(player_entity.handle.unwrap(), true).unwrap() {

            let norm = manifold.deepest_contact().unwrap().contact.normal;

            reaction += Vec3::new(
                dpos.x().abs() * norm.x, 
                dpos.y().abs() * norm.y, 
                dpos.z().abs() * norm.z
            );

        }

        playerpos.pos += dpos - reaction;

        *transform = transform.looking_at(playerpos.facing, Vec3::unit_y());

        transform.translation = playerpos.pos;


        collision_meta.set_position(player_entity.handle, playerpos.pos)

    }
}
