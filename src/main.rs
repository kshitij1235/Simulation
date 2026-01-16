use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

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
    for i in 0..world.positions.len() {
        let pos = &mut world.positions[i];
        let vel = &mut world.velocities[i];

        if pos.x <= 0.0 || pos.x >= (WIDTH - 1) as f32 {
            vel.dx = -vel.dx;
        }

        if pos.y <= 0.0 || pos.y >= (HEIGHT - 1) as f32 {
            vel.dy = -vel.dy;
        }
    }
}
fn render(world: &World, buffer: &mut [u32]) {
    // Clear screen (black)
    for pixel in buffer.iter_mut() {
        *pixel = 0x000000;
    }

    for pos in &world.positions {
        let x = pos.x as isize;
        let y = pos.y as isize;

        if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
            let idx = y as usize * WIDTH + x as usize;
            buffer[idx] = 0x00FFFFFF; // white dot
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
    let mut window = Window::new("dot simulation", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16_666)));

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut world = World {
        positions: Vec::new(),
        velocities: Vec::new(),
    };

    for i in 0..400 {
        world.positions.push(Position {
            x: (i * 4) as f32,
            y: (i * 3) as f32,
        });

        world.velocities.push(Velocity { dx: 1.2, dy: 0.8 });
    }

    //steps by each dot is getting moment
    let dt = 1.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        moment_system(&mut world, dt);
        boundary_system(&mut world);
        render(&world, &mut buffer);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
