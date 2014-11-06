use std;

use std::num;

#[deriving(Clone,Zero)]
pub struct Vector2<T>(pub T,pub T);

impl<T> Vector2<T>{
	pub fn new(x: T,y: T) -> Vector2<T>{
		return Vector2(x,y);
	}
}

impl<T: FloatMath + Sub<T,T>> Vector2<T>{
	pub fn limit_magnitude(&mut self,magnitude: T){
		let current_magnitude = self.0*self.0 + self.1*self.1;
		if current_magnitude > magnitude*magnitude{
			let d = magnitude/current_magnitude.sqrt();
			self.0 = self.0*d;
			self.1 = self.1*d;
		}
	}

	pub fn magnitude(&self) -> T{
		return self.0.hypot(self.1);
	}

	pub fn unit(&self) -> Vector2<T>{
		*self / self.magnitude()
	}
}

impl<T: num::One> /*num::One for*/ Vector2<T>{
	pub fn one() -> Vector2<T>{
		Vector2(num::One::one(),num::One::one())
	}
}

impl<T: std::fmt::Show> std::fmt::Show for Vector2<T>{
	fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
		return write!(f,"vec({},{})",self.0,self.1);
	}
}

impl<T: Add<T,T>> Add<Vector2<T>,Vector2<T>> for Vector2<T>{
	fn add(&self, other: &Vector2<T>) -> Vector2<T>{
		return Vector2(self.0 + other.0,self.1 + other.1);
	}
}

impl<T: Sub<T,T>> Sub<Vector2<T>,Vector2<T>> for Vector2<T>{
	fn sub(&self, other: &Vector2<T>) -> Vector2<T>{
		return Vector2(self.0 - other.0,self.1 - other.1);
	}
}

impl<T: Mul<T,T>> Mul<T,Vector2<T>> for Vector2<T>{
	fn mul(&self, other: &T) -> Vector2<T>{
		return Vector2(self.0 * *other,self.1 * *other);
	}
}

impl<T: Mul<T,T> + Add<T,T>> Mul<Vector2<T>,T> for Vector2<T>{
	/// Dot product
	fn mul(&self, other: &Vector2<T>) -> T{
		self.dot_product(other)
	}
}

impl<T: Mul<T,T> + Add<T,T>> Vector2<T>{
	pub fn dot_product(&self,other: &Vector2<T>) -> T{
		self.0 * other.0 + self.1 * other.1
	}
}

impl<T: Div<T,T>> Div<T,Vector2<T>> for Vector2<T>{
	fn div(&self, other: &T) -> Vector2<T>{
		return Vector2(self.0 / *other,self.1 / *other);
	}
}

impl<T: Neg<T>> Neg<Vector2<T>> for Vector2<T>{
	fn neg(&self) -> Vector2<T>{
		return Vector2(-self.0,-self.1);
	}
}

impl<T: Signed> Vector2<T>{
	pub fn abs(&self) -> Vector2<T>{
		return Vector2(self.0.abs(),self.1.abs());
	}
}

impl<T: Mul<T,T> + Add<T,T> + FloatMath + Sub<T,T>> Vector2<T>{
	/// Projection of self on other
	pub fn project(&self,other: &Vector2<T>) -> Vector2<T>{
		other * (self.dot_product(other) / other.dot_product(other))
	}
}

#[test]
fn vector_limit1(){
	let mut v = Vector2::new(3.0 as f32,4.0);
	v.limit_magnitude(2.5);
	let m = v.magnitude();
	println!("{}.magnitude = {} = 2.5",v,m);
	assert!(m==2.5 || (2.5-m).abs() < 1.0e-06);
}

#[test]
fn vector_limit2(){
	let mut v = Vector2::new(100.0 as f32,100.0);
	v.limit_magnitude(10.0);
	let m = v.magnitude();
	println!("{}.magnitude = {} = 10.0",v,m);
	assert!(m==10.0 || (10.0-m).abs() < 1.0e-06);
}