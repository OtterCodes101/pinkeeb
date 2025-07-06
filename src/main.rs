use asteroids::{
	elements::{Grabbable, Model},
	ClientState, CustomElement, Migrate,
};
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use stardust_xr_fusion::{
	fields::{CylinderShape, Shape},
	project_local_resources,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
	asteroids::client::run::<State>(&[&project_local_resources!("res")]).await
}

#[derive(Debug, Serialize, Deserialize)] // Defining variables used in client
pub struct State {
	pos: Vec3,
	rot: Quat,
}

impl Default for State {
	// Defines the default value for the variables used in the client
	fn default() -> Self {
		Self {
			pos: Vec3::ZERO,
			rot: Quat::IDENTITY,
		}
	}
}
impl Migrate for State {
	type Old = Self;
}
impl ClientState for State {
	const APP_ID: &'static str = "lol.chromebooks.leah.pinkeeb";

	// Example: Cube with grab ring on the bottom (allows user to move it)
	// and a dial on the top that allows the user to change the size of the box
	fn reify(&self) -> asteroids::Element<Self> {
		Grabbable::new(
			Shape::Cylinder(CylinderShape {
				length: 0.03,
				radius: 0.01,
			}),
			self.pos,
			self.rot,
			|state: &mut Self, pos, rot| {
				state.pos = pos.into();
				state.rot = rot.into();
			},
		)
		.build()
		.child(Model::namespaced("pinkeeb", "pin").build())
	}
}
