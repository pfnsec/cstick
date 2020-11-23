use bevy::prelude::*;
use ncollide3d::pipeline::CollisionObjectSlabHandle;

pub struct CameraEntity;
pub struct CameraPos {
    pub pos:  Vec3,
    pub dpos: Vec3,
    
}
impl Default for CameraPos {
	fn default() -> Self {
		Self {
            pos:  Vec3::new(-3.0, 5.0, 8.0),
            dpos: Vec3::new(0.0, 0.0, 0.0),
		}
	}
}


#[derive(Default)]
pub struct PlayerEntity {
    pub handle: Option<CollisionObjectSlabHandle>, 
}

pub struct PlayerPos {
    pub pos:  Vec3,
    pub dpos: Vec3,
    pub facing: Vec3,
}
impl Default for PlayerPos {
	fn default() -> Self {
		Self {
            pos:    Vec3::new(0.0, 1.0, 0.0),
            dpos:   Vec3::new(0.0, 0.0, 0.0),
            facing: Vec3::new(1.0, 0.0, 0.0),
		}
	}
}

#[derive(Default)]
pub struct StaticEntity {
    pub handle: Option<CollisionObjectSlabHandle>, 
}
