#![allow(dead_code)]

use raytracer::tuple::*;

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    return Projectile {
        position: &proj.position + &proj.velocity,
        velocity: &proj.velocity + &(&env.gravity + &env.wind)
    };
}

fn main() {


    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0)
    };

    let mut p = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.0, 0.0).normalize(),
    };

    while p.position.1 > 0.0 {
      println!("At {:?}", p);
      p = tick(&env, &p)
    }

    println!("At {:?}", p);

    println!("Go speedracer!");
}
