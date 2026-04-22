const std = @import("std");
const Io = std.Io;
const zig_async = @import("zig_async");

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    
    std.log.info("Starting Zig 0.16.0 Standard Project Async Demo", .{});

    // Demonstrate calling a function from our internal module (root.zig)
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_file_writer: Io.File.Writer = .init(.stdout(), io, &stdout_buffer);
    const stdout_writer = &stdout_file_writer.interface;
    try zig_async.printAnotherMessage(stdout_writer);
    try stdout_writer.flush();

    // Group demo: Manage multiple tasks with O(1) overhead.
    var group: Io.Group = .init;
    defer group.cancel(io); 

    std.log.info("Starting Group tasks...", .{});
    group.async(io, worker, .{ io, "Fast Worker", @as(i64, 50) });
    group.async(io, worker, .{ io, "Slow Worker", @as(i64, 150) });

    // await() on a group waits for all tasks to complete.
    try group.await(io);

    std.log.info("All tasks completed.", .{});
}

fn worker(io: Io, name: []const u8, delay_ms: i64) !void {
    var i: usize = 1;
    while (i <= 5) : (i += 1) {
        try io.sleep(.fromMilliseconds(delay_ms), .awake);
        std.log.info("{s} tick {d}", .{ name, i });
    }
}
