use bevy::{prelude::Transform, sprite::collide_aabb::Collision};

pub struct BuildingCollision(pub Collision, pub Transform);
