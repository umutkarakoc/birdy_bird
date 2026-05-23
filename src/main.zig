const std = @import("std");
const rl = @import("raylib");
const assets = @import("assets.zig");

pub fn main() !void {
    rl.initWindow(1280, 720, "Birdy bird");
    defer rl.closeWindow();

    var sprites: assets.Sprites = try assets.Sprites.load();
    defer sprites.unload();

    var game: Game = .{ .sprites = sprites };

    var t = try rl.loadTexture("assets/obstacle.png");
    defer t.unload();

    while (!rl.windowShouldClose()) {
        game.dt = rl.getFrameTime();
        game.update();
        game.draw();
    }
}

const Game = struct {
    // Screen
    sw: f32 = 1280,
    sh: f32 = 720,
    dt: f32 = 0,

    // Bird
    gravity: f32 = 10.0,
    jump_force: f32 = 100.0,
    move_speed: f32 = 100.0,

    // Background
    bg_size: rl.Vector2 = .{ .x = 400, .y = 715 },
    bg_scroll_speed: f32 = -50.0,
    bg_scroll: f32 = 0.0,

    // Obstacle
    obs_size: rl.Vector2 = .{ .x = 105, .y = 338 },
    obs_gap_y: f32 = 60,

    // Game State
    is_started: bool = false,
    score: i32 = 0,

    //assets
    sprites: assets.Sprites,

    pub fn update(game: *Game) void {
        game.update_bg();
        if (!game.is_started) {
            return;
        }
    }

    pub fn update_bg(game: *Game) void {
        game.bg_scroll += game.bg_scroll_speed * game.dt;
        if (game.bg_scroll < -game.bg_size.x) {
            game.bg_scroll = 0;
        }
    }

    pub fn draw(game: Game) void {
        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(.ray_white);
        game.draw_bg();
        game.draw_obstacle();
    }

    pub fn draw_bg(game: Game) void {
        var x: f32 = 0.0;
        while (x <= game.sw + game.bg_size.x) : (x += game.bg_size.x) {
            rl.drawTextureV(
                game.sprites.bg,
                .{ .x = x + game.bg_scroll, .y = 0 },
                .ray_white,
            );
        }
    }

    pub fn draw_obstacle(game: Game) void {
        _ = game;
    }
};
