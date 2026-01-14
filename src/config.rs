// window
pub const SCREEN_W: i32 = 640;
pub const SCREEN_H: i32 = 480;
pub const TARGET_FPS: u32 = 120;

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

// input tuning
pub const GAMEPAD_DEADZONE: f32 = 0.2;
pub const KEYBOARD_AXIS: f32 = 1.0;
pub const MOVE_AXIS_MIN: f32 = -1.0;
pub const MOVE_AXIS_MAX: f32 = 1.0;
