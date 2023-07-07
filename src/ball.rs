use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
		app.register_type::<Ball>();
    }
}

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Ball {
    _velocity: f32,
}

#[derive(Bundle)]
pub struct BallBundle {
	pub ball: Ball,
	pub rigidbody: RigidBody,
	pub velocity: Velocity,
	pub collider: Collider,
}

impl Default for BallBundle {
	fn default() -> Self {
		Self {
			ball: Ball { _velocity: 100.0 },
			rigidbody: RigidBody::Dynamic,
			velocity: Velocity {
				linvel: Vec3::new(0., 0., 0.),
				angvel: Vec3::new(0., 0., 0.),
			},
			collider: Collider::ball(1.),
		}
	}
}

