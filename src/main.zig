const std = @import("std");
const rl = @import("raylib");
const assets = @import("assets.zig");

pub fn main() !void {
    rl.initWindow(1280, 720, "Birdy bird");
    defer rl.closeWindow();

    var sprites: assets.Sprites = try assets.Sprites.load();
    defer sprites.unload();

    var game: Game = .{
        .prng = std.Random.DefaultPrng.init(@as(u64, @intFromFloat(rl.getTime() * 1000000.0))),
        .sprites = sprites,
    };

    while (!rl.windowShouldClose()) {
        game.dt = rl.getFrameTime();
        game.update();
        game.draw();
    }
}

const GameState = enum { Idle, Play, Dead };

const Game = struct {
    // Screen
    sw: f32 = 1280,
    sh: f32 = 720,
    dt: f32 = 0,

    // Bird
    bird_draw_offset: rl.Vector2 = .{ .x = -88, .y = -52 },
    bird_box: rl.Rectangle = .{ .width = 60, .height = 60, .x = 100, .y = 360 },
    bird_force: f32 = -400,
    jump_force: f32 = -350.0,
    gravity: f32 = 700.0,
    move_speed: f32 = 100.0,
    bird_anim_index: u16 = 0,
    bird_anim_time: f32 = 0,

    // Background
    bg_size: rl.Vector2 = .{ .x = 400, .y = 720 },
    bg_scroll_speed: f32 = -50.0,
    bg_scroll: f32 = 0.0,

    // Obstacle
    top_obstacles: [2]rl.Rectangle = .{
        .{ .width = 143, .height = 450, .x = 1280, .y = 0 },
        .{ .width = 143, .height = 450, .x = 1280 * 1.5, .y = 0 },
    },
    bottom_obstacles: [2]rl.Rectangle = .{
        .{ .width = 143, .height = 450, .x = 1280, .y = 0 },
        .{ .width = 143, .height = 450, .x = 1280 * 1.5, .y = 0 },
    },
    obs_gap_y: f32 = 200,
    obs_speed: f32 = -200,
    prng: std.Random.DefaultPrng = undefined,

    // Game State
    state: GameState = .Idle,
    score: i32 = 0,

    //assets
    sprites: assets.Sprites,

    pub fn update(game: *Game) void {
        game.updateBg();
        game.updateBird();
        if (game.state == .Play) {
            game.updateObstacle();
        } else if (game.state == .Idle) {
            if (rl.isKeyReleased(rl.KeyboardKey.space)) {
                game.startGame();
            }
        }
    }
    pub fn startGame(game: *Game) void {
        game.state = .Play;
        game.score = 0;
        for (&game.top_obstacles, 0..) |*top, i| {
            game.randomObstacleOffset(top, &game.bottom_obstacles[i]);
        }
    }

    pub fn updateBg(game: *Game) void {
        game.bg_scroll += game.bg_scroll_speed * game.dt;
        if (game.bg_scroll < -game.bg_size.x) {
            game.bg_scroll = 0;
        }
    }

    pub fn updateBird(game: *Game) void {
        game.bird_anim_time += game.dt;
        if (game.bird_anim_time >= 1.0) {
            game.bird_anim_time = 0.0;
        }
        game.bird_anim_index = @intFromFloat(game.bird_anim_time * 20.0);

        game.bird_force += game.gravity * game.dt;

        if (rl.isKeyPressed(rl.KeyboardKey.space) and game.state == .Play) {
            game.bird_force = game.jump_force;
        }
        game.bird_box.y += game.bird_force * game.dt;
    }

    pub fn updateObstacle(game: *Game) void {
        for (&game.top_obstacles, &game.bottom_obstacles) |*top, *bottom| {
            top.x += game.obs_speed * game.dt;
            bottom.x += game.obs_speed * game.dt;
            if (top.x < -top.width) {
                top.x = game.sw;
                bottom.x = game.sw;
            }
            if (checkCollision(&game.bird_box, top) or checkCollision(&game.bird_box, bottom)) {
                game.state = .Dead;
            }
        }
    }

    pub fn randomObstacleOffset(game: *Game, top: *rl.Rectangle, bottom: *rl.Rectangle) void {
        const padding = 100 - game.prng.random().float(f32) * 200;
        top.y = -100 - 90 + padding;
        bottom.y = 360 + 90 + padding;
    }

    pub fn draw(game: Game) void {
        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(.ray_white);
        game.drawBg();
        game.drawBird();
        game.drawObstacle();
    }

    pub fn drawBg(game: Game) void {
        var x: f32 = 0.0;
        while (x <= game.sw + game.bg_size.x) : (x += game.bg_size.x) {
            rl.drawTextureV(
                game.sprites.bg,
                .{ .x = x + game.bg_scroll, .y = 0 },
                .ray_white,
            );
        }
    }

    pub fn drawObstacle(game: Game) void {
        for (game.top_obstacles) |top| {
            rl.drawTextureV(
                game.sprites.obstacle_top,
                .{ .x = top.x, .y = top.y },
                .ray_white,
            );
        }
        for (game.bottom_obstacles) |bottom| {
            rl.drawTextureV(
                game.sprites.obstacle_bottom,
                .{ .x = bottom.x, .y = bottom.y },
                .ray_white,
            );
        }
    }

    pub fn drawBird(game: Game) void {
        const text = game.sprites.bird[game.bird_anim_index];
        rl.drawTextureV(
            text,
            .{
                .x = game.bird_box.x + game.bird_draw_offset.x,
                .y = game.bird_box.y + game.bird_draw_offset.y,
            },
            .ray_white,
        );
    }
};

fn checkCollision(a: *rl.Rectangle, b: *rl.Rectangle) bool {
    return a.x < b.x + b.width and
        a.x + a.width > b.x and
        a.y < b.y + b.height and
        a.y + a.height > b.y;
}
