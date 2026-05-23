const rl = @import("raylib");

const sw = 1920;
const sh = 1080;
pub fn main() void {
    rl.initWindow(sw, sh, "Birdy bird");
    defer rl.closeWindow();

    while (!rl.windowShouldClose()) {
        draw();
    }
}

pub fn draw() void {
    rl.beginDrawing();
    defer rl.endDrawing();

    rl.clearBackground(.ray_white);
}
