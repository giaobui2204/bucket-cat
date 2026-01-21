use raylib::prelude::Color;

// window
pub const SCREEN_W: i32 = 640;
pub const SCREEN_H: i32 = 480;
pub const TARGET_FPS: u32 = 120;

// colors
pub const COLOR_ACCENT: Color = Color::new(255, 179, 217, 255);
pub const COLOR_ACCENT_HOVER: Color = Color::new(255, 199, 230, 255);
pub const COLOR_ACCENT_TEXT: Color = Color::new(109, 43, 80, 255);
pub const COLOR_ACCENT_BORDER: Color = Color::new(233, 130, 180, 255);

pub const COLOR_LIGHT_BG: Color = Color::new(255, 224, 240, 255);
pub const COLOR_LIGHT_HOVER: Color = Color::new(255, 233, 245, 255);

pub const COLOR_MENU_BG_START: Color = Color::new(255, 240, 246, 255);
pub const COLOR_MENU_BG_END: Color = Color::new(255, 214, 232, 255);
pub const COLOR_MENU_CIRCLE: Color = Color::new(255, 228, 241, 120);

// scoring
pub const GAME_SCORE_LABEL: &str = "GAME_SCORE";
pub const SCORES_FILE: &str = "scores.txt";
pub const NORMAL_CAT_SCORE: i32 = 1;
pub const DEVIL_CAT_SCORE: i32 = 0;

// bucket
pub const BUCKET_W: f32 = 48.0;
pub const BUCKET_H: f32 = 150.0;
pub const BUCKET_ACCEL: f32 = 1800.0;
pub const BUCKET_MAX_SPEED: f32 = 420.0;
pub const BUCKET_FRICTION: f32 = 8.0;
pub const BUCKET_Y_OFFSET: f32 = 60.0;
pub const BUCKET_DRAW_SCALE: f32 = 2.0;
pub const BUCKET_FRAME_COUNT: usize = 2;
pub const BUCKET_FRAME_COLS: i32 = 2;
pub const BUCKET_FRAME_ROWS: i32 = 1;
pub const BUCKET_ANIM_FPS: f32 = 6.0;

// falling object
pub const OBJ_RADIUS: f32 = 8.0;
pub const OBJ_GRAVITY: f32 = 400.0;
pub const OBJ_BASE_SPEED: f32 = 20.0;
pub const OBJ_SPEED_SCALE: f32 = 0.01;
pub const OBJ_MAX_SPEED: f32 = 320.0;
pub const OBJ_FRAME_COUNT: usize = 12;
pub const OBJ_FRAME_COLS: i32 = 3;
pub const OBJ_FRAME_ROWS: i32 = 4;
pub const OBJ_ANIM_FPS: f32 = 10.0;
pub const OBJ_DRAW_SCALE: f32 = 1.2;

// spawn
pub const SPAWN_INTERVAL: f32 = 0.8;
pub const DEVIL_RATE_EARLY: f32 = 0.12;
pub const DEVIL_RATE_MID: f32 = 0.16;
pub const DEVIL_RATE_LATE: f32 = 0.20;
pub const DEVIL_RATE_EARLY_END: f32 = 30.0;
pub const DEVIL_RATE_MID_END: f32 = 90.0;
pub const ANGEL_RATE: f32 = 0.15;
pub const EXPLODE_RATE_IN_DEVIL: f32 = 0.05;
pub const DEVIL_COOLDOWN: f32 = 0.8;
pub const EXPLODE_PROTECT_SPAWNS: i32 = 18;

// devil cat effects
pub const DEVIL_INVERT_DURATION: f32 = 4.0;
pub const DEVIL_SIZE_EFFECT_DURATION: f32 = 6.0;
pub const DEVIL_SIZE_SMALL_SCALE: f32 = 0.7;
pub const DEVIL_SIZE_LARGE_SCALE: f32 = 1.3;
pub const DEVIL_SCORE_MULTIPLIER_DURATION: f32 = 8.0;


// angry cat mechanism
pub const ANGRY_BAR_MAX_WIDTH: f32 = 200.0;
pub const ANGRY_BAR_HEIGHT: f32 = 20.0;
pub const ANGRY_BAR_MIN_MAX: i32 = 15;
pub const ANGRY_BAR_MAX_MAX: i32 = 25;
pub const ANGRY_PENALTY_NORMAL: i32 = 1;
pub const ANGRY_PENALTY_ANGEL: i32 = 3;
pub const GIANT_CAT_SPEED: f32 = 210.0;
pub const CRYING_CAT_FRAME_W: f32 = 64.0;
pub const CRYING_CAT_FRAME_H: f32 = 52.0;
pub const CRYING_CAT_FRAMES: usize = 4;
pub const CRYING_CAT_FPS: f32 = 8.0; 
pub const CRYING_CAT_SCALE: f32 = 10.0;
pub const GIANT_CAT_LANDED_DELAY: f32 = 2.0;

pub const DEVIL_EXPLODE_ANIM_DURATION: f32 = 0.9;
pub const DEVIL_EFFECT_MESSAGE_DURATION: f32 = 2.4;

// input tuning
pub const GAMEPAD_DEADZONE: f32 = 0.2;
pub const KEYBOARD_AXIS: f32 = 1.0;
pub const MOVE_AXIS_MIN: f32 = -1.0;
pub const MOVE_AXIS_MAX: f32 = 1.0;
