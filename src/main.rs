extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;

use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::object::ColliderDesc;
use nphysics3d::object::{BodyStatus, RigidBodyDesc};
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use na::{Isometry3, Matrix3, Point3, UnitQuaternion, Vector3};
use nphysics3d::math::{Inertia, Velocity};
use nphysics3d::object::{BodyPartHandle, BodySet};

use ncollide3d::shape::{Cuboid, ShapeHandle};

fn main() {
	//main physics stuff
	let mut mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, 0.0, 0.0));
	mechanical_world.gravity = Vector3::y() * -9.81;

	let mut geometrical_world = DefaultGeometricalWorld::new();

	let mut bodies = DefaultBodySet::new();
	let mut colliders = DefaultColliderSet::new();
	let mut joint_constraints = DefaultJointConstraintSet::new();
	let mut force_generators = DefaultForceGeneratorSet::new();

	//create rigid body
	let mut rb = RigidBodyDesc::new().gravity_enabled(true).build();

	//add rb to collection
	let handle_rb = bodies.insert(rb);
	//add collider to collection

	//create collider
	let shape = ShapeHandle::new(Cuboid::new(Vector3::new(1.0, 1.0, 1.0)));
	let boxCollider = ColliderDesc::new(shape).build(BodyPartHandle(handle_rb, 0));
	let collider_handler = colliders.insert(boxCollider);

	let mut window = Window::new("Kiss3d: cube");
	let mut c = window.add_cube(1.0, 1.0, 1.0);
	c.set_color(1.0, 0.0, 0.0);

	window.set_light(Light::StickToCamera);

	let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

	//main loop
	while !window.is_closed() {
		mechanical_world.step(
			&mut geometrical_world,
			&mut bodies,
			&mut colliders,
			&mut joint_constraints,
			&mut force_generators,
		);

		c.prepend_to_local_rotation(&rot);
	}
}
