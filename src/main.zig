const rl = @import("raylib");
const assets = @import("assets.zig");

const Game = struct {
    // Screen
    sw: i32 = 1920,
    sh: i32 = 1080,
    dt: f32 = 0,

    // Bird
    gravity: f32 = 10.0,
    jump_force: f32 = 100.0,
    move_speed: f32 = 100.0,

    // Background
    bg_size: rl.Vector2 = .{ .x = 400, .y = 715 },
    bg_scroll: f32 = 20.0,

    // Obstacle
    obs_size: rl.Vector2 = .{ .x = 105, .y = 338 },
    obs_gap_y: f32 = 60,

    // Game State
    is_started: bool = false,
    score: i32 = 0,

    //assets
    sprites: assets.Sprites,

    pub fn update(game: Game) void {
        if (!game.is_started) {
            return;
        }
    }

    pub fn draw() void {
        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(.ray_white);
        draw_obstacle();
    }
    pub fn draw_obstacle() void {}
};

pub fn main() !void {
    rl.initWindow(1920, 1080, "Birdy bird");
    defer rl.closeWindow();

    var sprites: assets.Sprites = assets.Sprites.load();
    defer sprites.unload();

    var game: Game = .{.assets};

    var t = try rl.loadTexture("assets/obstacle.png");
    defer t.unload();

    while (!rl.windowShouldClose()) {
        game.dt = rl.getFrameTime();
        game.update();
        game.draw();
    }
}
