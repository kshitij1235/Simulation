#[derive(Debug, Copy, Clone)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    dx: f32,
    dy: f32,
}

struct World {
    positions: Vec<Position>,
    velocities: Vec<Velocity>,
}

fn boundary_system(world: &mut World) {
    let limit = 10.0;

    for i in 0..world.positions.len() {
        let position = world.positions[i];
        let velocity = &mut world.velocities[i];

        if position.x > limit || position.x < 0.0 {
            velocity.dx = -velocity.dx;
        }
    }
}

fn moment_system(world: &mut World, dt: f32) {
    for i in 0..world.positions.len() {
        let velocity = world.velocities[i];
        let position = &mut world.positions[i];

        position.x += velocity.dx * dt;
        position.y += velocity.dy * dt;
    }
}

fn main() {
    let mut world = World {
        positions: Vec::with_capacity(10),
        velocities: Vec::with_capacity(10),
    };

    // Initialize entities
    for i in 0..10 {
        world.positions.push(Position {
            x: i as f32,
            y: 0.0,
        });

        world.velocities.push(Velocity { dx: 1.0, dy: 0.0 });
    }

    let dt: f32 = 1.0 / 60.0;
    let steps: usize = 300;

    for step in 0..steps {
        moment_system(&mut world, dt);
        boundary_system(&mut world);
        if step % 60 == 0 {
            println!("After step {}:", step);
            println!("{:#?}", world.positions);
        }
    }
}
