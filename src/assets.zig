const std = @import("std");
const rl = @import("raylib");

pub const Sprites = struct {
    bird: [20]rl.Texture2D,
    dead: [20]rl.Texture2D,
    obstacle: rl.Texture2D,
    bg: rl.Texture2D,
    play: rl.Texture2D,

    pub fn load() !Sprites {
        var bird: [20]rl.Texture2D = undefined;

        for (&bird, 0..) |*tex, i| {
            var buf: [64]u8 = undefined;
            const path = std.fmt.bufPrintZ(
                &buf,
                "assets/bird/{d:0>2}.png",
                .{i},
            ) catch unreachable;
            tex.* = try rl.loadTexture(path);
        }

        var dead: [20]rl.Texture2D = undefined;
        for (&dead, 0..) |*tex, i| {
            var buf: [64]u8 = undefined;
            const path = std.fmt.bufPrintZ(
                &buf,
                "assets/dead/{d:0>2}.png",
                .{i},
            ) catch unreachable;
            tex.* = try rl.loadTexture(path);
        }

        return .{
            .bird = bird,
            .dead = dead,
            .bg = try rl.loadTexture("assets/bg.png"),
            .play = try rl.loadTexture("assets/play.png"),
            .obstacle = try rl.loadTexture("assets/obstacle.png"),
        };
    }

    pub fn unload(self: Sprites) void {
        for (self.bird) |tex| {
            rl.unloadTexture(tex);
        }
        for (self.dead) |tex| {
            rl.unloadTexture(tex);
        }
        rl.unloadTexture(self.bg);
        rl.unloadTexture(self.play);
        rl.unloadTexture(self.obstacle);
    }
};
