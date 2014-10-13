use super::vector::Vector2 as Vector2;
use super::physics::Existence as Existence;
use super::physics::Mass as Mass;
use super::physics::WithPhysics as WithPhysics;
use super::gameloop::Updatable as Updatable;
use super::gameloop::Renderable as Renderable;
use lolirofle::gl::renderer::Renderer as Renderer;

#[deriving(Clone)]
pub struct Player{
	position: Vector2<f32>,
	velocity: Vector2<f32>
}
impl Player{
	pub fn new() -> Player{
		return Player{
			position: Vector2::new(0.0,0.0),
			velocity: Vector2::new(0.0,0.0),
		};
	}
}
impl Existence for Player{
	fn get_position(&self) -> Vector2<f32>{
		return self.position;
	}
}
impl Updatable for Player{
	fn update(&mut self,delta_time: f64){
		let acceleration = Vector2::new(0.0,9.82*16.0);

		self.velocity = self.velocity + acceleration  * (delta_time as f32);
		self.position = self.position + self.velocity * (delta_time as f32) / 2.0;
	}
}
impl Renderable for Player{
	fn render(&self,renderer: &Renderer){
		renderer.render_rectangle(
			self.get_position(),
			Vector2(16.0 as f32,16.0)
		);
	}
}
impl WithPhysics for Player{
	fn get_mass(&self) -> Mass{
		return 5.0;
	}
	fn get_velocity(&self) -> Vector2<f32>{
		return self.velocity;
	}
}
