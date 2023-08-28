// todo: does_it_fit
/* 
Instructions
Use the areas_volumes module provided to create two functions.

Create area_fit. It should return true if the geometric shape can fit inside the rectangular area as many times as is indicated by times.

The arguments of the function will be:

x and y: length and width of the rectangular area.
objects: the type of geometric shape.
times: the number of geometric shapes to fit inside the rectangular area.
a: size of dimension for:
side of a Square
base of a Triangle
radius of a Circle
side_a of a Rectangle
b: size of dimension for:
height of a Triangle
side_b of a Rectangle
Create volume_fit. It should return true if the geometric volume can fit inside the box as many times as is indicated by times.

The arguments of the function will be:

x, y and z: length, width and depth of the box.
objects: the type of geometric volume.
times: the number of geometric volumes to fit inside the box.
a: size of dimension for:
side of a Cube
radius of a Sphere
base_area of a triangular Pyramid
side_a of a Parallelepiped
base_radius of a Cone
b: size dimension for:
height of the triangular Pyramid
side_b of a Parallelepiped
height of the Cone
c: size dimension for:
side_c of a Parallelepiped
Expected Functions
pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {

}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {

}
areas_volumes.rs
pub enum GeometricalShapes {
	Square,
	Circle,
	Rectangle,
	Triangle,
}

pub enum GeometricalVolumes {
	Cube,
	Sphere,
	Cone,
	Pyramid,
	Parallelepiped,
}

pub fn square_area(side: usize) -> usize {
	side.pow(2)
}

pub fn triangle_area(base: usize, height: usize) -> f64 {
	(base as f64 * height as f64) / 2.0
}

pub fn circle_area(radius: usize) -> f64 {
	std::f64::consts::PI * (radius.pow(2) as f64)
}

pub fn rectangle_area(side_a: usize, side_b: usize) -> usize {
	side_a * side_b
}

pub fn cube_volume(side: usize) -> usize {
	side.pow(3)
}

pub fn sphere_volume(radius: usize) -> f64 {
	(4.0 / 3.0) * std::f64::consts::PI * (radius.pow(3) as f64)
}

pub fn triangular_pyramid_volume(base_area: f64, height: usize) -> f64 {
	(base_area * height as f64) / 3.0
}

pub fn parallelepiped_volume(side_a: usize, side_b: usize, side_c: usize) -> usize {
	side_a * side_b * side_c
}

pub fn cone_volume(base_radius: usize, height: usize) -> f64 {
	(1.0 / 3.0) * std::f64::consts::PI * base_radius.pow(2) as f64 * height as f64
}
 */
/* 
mod areas_volumes;
 pub use areas_volumes::{ GeometricalShapes, GeometricalVolumes};

 pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let object_area = match objects {
        GeometricalShapes::Square => areas_volumes::square_area(a),
        GeometricalShapes::Circle => areas_volumes::circle_area(a) as usize,
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b),
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b) as usize,
    };

    let total_area = x * y * times;
    object_area <= total_area as usize
}
 
 pub fn volume_fit(
     x: usize,
     y: usize,
     z: usize,
     objects: GeometricalVolumes,
     times: usize,
     a: usize,
     b: usize,
     c: usize,
 ) -> bool {
     let object_volume = match objects {
         GeometricalVolumes::Cube => areas_volumes::cube_volume(a),
         GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a) as usize,
         GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b) as usize,
         GeometricalVolumes::Pyramid => areas_volumes::triangular_pyramid_volume(a as f64, b) as usize,
         GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c),
     };
 
     let total_volume = x * y * z * times;
     object_volume <= total_volume as usize
 }
 */

mod areas_volumes;
pub use areas_volumes::{ GeometricalShapes, GeometricalVolumes};

/* Create area_fit. It should return true if the geometric shape can fit inside the rectangular area as many times as is indicated by times.

The arguments of the function will be:

x and y: length and width of the rectangular area.
objects: the type of geometric shape.
times: the number of geometric shapes to fit inside the rectangular area.
a: size of dimension for:
side of a Square
base of a Triangle
radius of a Circle
side_a of a Rectangle
b: size of dimension for:
height of a Triangle
side_b of a Rectangle */

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let object_area = match objects {
        GeometricalShapes::Square => areas_volumes::square_area(a),
        GeometricalShapes::Circle => areas_volumes::circle_area(a) as usize,
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b),
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b) as usize,
    };



    let total_area = x * y ;

    println!("object area: {} * {}", object_area, times);
    println!("total area: {}", total_area);
    object_area * times <= total_area
}


/* 
Create volume_fit. It should return true if the geometric volume can fit inside the box as many times as is indicated by times.

The arguments of the function will be:

x, y and z: length, width and depth of the box.
objects: the type of geometric volume.
times: the number of geometric volumes to fit inside the box.
a: size of dimension for:
side of a Cube
radius of a Sphere
base_area of a triangular Pyramid
side_a of a Parallelepiped
base_radius of a Cone
b: size dimension for:
height of the triangular Pyramid
side_b of a Parallelepiped
height of the Cone
c: size dimension for:
side_c of a Parallelepiped */
pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let object_volume = match objects {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a),
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a) as usize,
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b) as usize,
        GeometricalVolumes::Pyramid => areas_volumes::triangular_pyramid_volume(a as f64, b) as usize,
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c),
    };

    let total_volume = x * y * z ;
    println!("object area: {} * {}", object_volume, times);
    println!("total area: {}", total_volume);
    object_volume*times <= total_volume
}