use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::collections::VecDeque;
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    StopMovingUp,
    StopMovingDown,
    StopMovingLeft,
    StopMovingRight,
}

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed_x: i32,
    speed_y: i32,
    facing: Facing,
    current_frame: i32,
    command_queue: VecDeque<Command>,
}

fn facing_spritesheet_row(facing: Facing) -> i32 {
    use self::Facing::*;
    match facing {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = player.sprite.size();
    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as i32 * facing_spritesheet_row(player.facing),
        frame_width,
        frame_height,
    );

    // Treats the center of the screen as the (0, 0) coordinate
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}

fn update_player(player: &mut Player) {
    match player.command_queue.pop_front() {
        Some(Command::MoveUp) => {
            player.speed_y -= PLAYER_MOVEMENT_SPEED;
            player.speed_x = 0;
        }
        Some(Command::MoveDown) => {
            player.speed_y += PLAYER_MOVEMENT_SPEED;
            player.speed_x = 0;
        }
        Some(Command::MoveLeft) => {
            player.speed_x -= PLAYER_MOVEMENT_SPEED;
            player.speed_y = 0;
        }
        Some(Command::MoveRight) => {
            player.speed_x += PLAYER_MOVEMENT_SPEED;
            player.speed_y = 0;
        }
        Some(Command::StopMovingUp) => {
            player.speed_y = 0;
        }
        Some(Command::StopMovingDown) => {
            player.speed_y = 0;
        }
        Some(Command::StopMovingLeft) => {
            player.speed_x = 0;
        }
        Some(Command::StopMovingRight) => {
            player.speed_x = 0;
        }

        None => {}
    }

    update_facing(player);

    if !((player.speed_x == 0) & (player.speed_y == 0)) {
        player.current_frame = (player.current_frame + 1) % 3;
    }

    player.position = player.position.offset(player.speed_x, player.speed_y);
}

fn update_facing(player: &mut Player) {
    if player.speed_y > 0 {
        player.facing = Facing::Down;
    } else if player.speed_y < 0 {
        player.facing = Facing::Up;
    } else if player.speed_x > 0 {
        player.facing = Facing::Right;
    } else if player.speed_x < 0 {
        player.facing = Facing::Left;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        speed_x: 0,
        speed_y: 0,
        facing: Facing::Down,
        current_frame: 0,
        command_queue: VecDeque::new(),
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.command_queue.push_back(Command::MoveLeft);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.command_queue.push_back(Command::MoveRight);
                }
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.command_queue.push_back(Command::MoveUp);
                }
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.command_queue.push_back(Command::MoveDown);
                }
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.command_queue.push_back(Command::StopMovingLeft);
                }
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.command_queue.push_back(Command::StopMovingRight);
                }
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.command_queue.push_back(Command::StopMovingUp);
                }
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.command_queue.push_back(Command::StopMovingDown);
                }
                _ => {}
            }
        }

        i = (i + 1) % 255;
        update_player(&mut player);

        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
