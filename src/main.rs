use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use rand::Rng;

const WINDOW_WIDTH: u32 = 400;
const WINDOW_HEIGHT: u32 = 600;
const PLAYER_WIDTH: u32 = 40;
const PLAYER_HEIGHT: u32 = 60;
const PLATFORM_WIDTH: u32 = 70;
const PLATFORM_HEIGHT: u32 = 20;
const GRAVITY: f32 = 0.5;
const JUMP_FORCE: f32 = -12.0;

struct Game {
    player: Player,
    platforms: Vec<Platform>,
    score: i32,
}

struct Player {
    x: f32,
    y: f32,
    velocity_y: f32,
}

struct Platform {
    x: i32,
    y: i32,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Block Jump", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut game = Game {
        player: Player {
            x: WINDOW_WIDTH as f32 / 2.0,
            y: WINDOW_HEIGHT as f32 - 50.0,
            velocity_y: 0.0,
        },
        platforms: vec![],
        score: 0,
    };

    generate_initial_platforms(&mut game);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Update game state
        update(&mut game, &event_pump);

        // Render
        canvas.set_draw_color(Color::RGB(175, 175, 175));
        canvas.clear();
        render(&mut canvas, &game)?;
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn update(game: &mut Game, event_pump: &sdl2::EventPump) {
    // Update player position
    game.player.velocity_y += GRAVITY;
    game.player.y += game.player.velocity_y;

    // Handle horizontal movement
    let keys: Vec<Keycode> = event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

    if keys.contains(&Keycode::Left) {
        game.player.x -= 5.0;
    }
    if keys.contains(&Keycode::Right) {
        game.player.x += 5.0;
    }

    // Wrap player horizontally
    if game.player.x < 0.0 {
        game.player.x = WINDOW_WIDTH as f32;
    } else if game.player.x > WINDOW_WIDTH as f32 {
        game.player.x = 0.0;
    }

    // Check for collisions with platforms
    for platform in &game.platforms {
        if game.player.velocity_y > 0.0
            && (game.player.x + PLAYER_WIDTH as f32) > platform.x as f32
            && game.player.x < (platform.x + PLATFORM_WIDTH as i32) as f32
            && (game.player.y + PLAYER_HEIGHT as f32) > platform.y as f32
            //&& (game.player.y + PLAYER_HEIGHT as f32) < platform.y as f32 + PLATFORM_HEIGHT as f32
        {
            game.player.velocity_y = JUMP_FORCE;
        }
    }


    // Generate new platforms
    if game.platforms.last().unwrap().y > 0 {
        let mut rng = rand::thread_rng();
        let new_platform = Platform {
            x: rng.gen_range(0..WINDOW_WIDTH as i32 - PLATFORM_WIDTH as i32),
            y: game.platforms.last().unwrap().y - 100,
        };
        game.platforms.push(new_platform);
    }

    // Remove platforms that are off-screen
    game.platforms.retain(|p| p.y < WINDOW_HEIGHT as i32);

    // Update score
    game.score = game.score.max(-(game.player.y as i32 - WINDOW_HEIGHT as i32) / 10);

    // Game over condition
    if game.player.y > WINDOW_HEIGHT as f32 {
        game.player.y = WINDOW_HEIGHT as f32 - 50.0;
        game.player.velocity_y = 0.0;
        game.score = 0;
        game.platforms.clear();
        generate_initial_platforms(game);
    }
}

fn render(canvas: &mut Canvas<Window>, game: &Game) -> Result<(), String> {
    // Draw player
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.fill_rect(Rect::new(
        game.player.x as i32,
        game.player.y as i32,
        PLAYER_WIDTH,
        PLAYER_HEIGHT,
    ))?;

    // Draw platforms
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    for platform in &game.platforms {
        canvas.fill_rect(Rect::new(
            platform.x,
            platform.y,
            PLATFORM_WIDTH,
            PLATFORM_HEIGHT,
        ))?;
    }

    // Draw score
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("/usr/share/fonts/truetype/ubuntu/UbuntuMono-B.ttf", 24)?;
    let surface = font
        .render(&format!("Score: {}", game.score))
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let target = Rect::new(10, 10, surface.width(), surface.height());
    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}

fn generate_initial_platforms(game: &mut Game) {
    let mut rng = rand::thread_rng();
    for i in 0..5 {
        let platform = Platform {
            x: rng.gen_range(0..WINDOW_WIDTH as i32 - PLATFORM_WIDTH as i32),
            y: WINDOW_HEIGHT as i32 - 100 * (i + 1),
        };
        game.platforms.push(platform);
    }
}
