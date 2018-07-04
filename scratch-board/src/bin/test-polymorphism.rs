// define trait for common behavior

pub trait Draw {
    fn draw(&self);
}

pub struct Scene {
    pub items: Vec<Box<Draw>>,
}

impl Scene {
    pub fn render(&self) {
        for i in self.items.iter() {
            i.draw();
        }
    }
}

//

#[derive(Debug)]
pub struct Sphere {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub r: u32,
}

impl Draw for Sphere {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

//

#[derive(Debug)]
pub struct PointLight {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub c: u32,
}

impl Draw for PointLight {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

//

fn main() {
    let scene = Scene {
        items: vec![
            Box::new(Sphere {
                x: 10,
                y: 20,
                z: 10,
                r: 5,
            }),
            Box::new(PointLight {
                x: 10,
                y: 20,
                z: 10,
                c: 555,
            }),
        ],
    };

    scene.render();
}
