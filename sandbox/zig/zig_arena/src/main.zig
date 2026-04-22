const std = @import("std");

pub fn main(init: std.process.Init) !void {
    const gpa = init.gpa;
    const arena_ptr = init.arena;
    const arena_allocator = arena_ptr.allocator();

    std.debug.print("--- ArenaAllocator Example (Zig 0.16.0) ---\n", .{});

    const names = [_][]const u8{ "Zig", "Arena", "Allocator", "0.16.0", "Example" };

    var list: std.ArrayList([]u8) = .empty;
    defer list.deinit(arena_allocator);

    // reserves space to ensure that the internal buffer is allocated once
    try list.ensureTotalCapacity(arena_allocator, names.len);
    std.debug.print("Reserved capacity for {d} items.\n\n", .{names.len});

    for (names) |name| {
        const copy = try arena_allocator.dupe(u8, name); // Duplicate string into the arena
        try list.append(arena_allocator, copy); // Guaranteed not to reallocate the list itself
        std.debug.print("Allocated and added: {s}\n", .{copy});
    }

    std.debug.print("\nTotal items: {d}, Capacity: {d}\n", .{ list.items.len, list.capacity });

    var local_arena = std.heap.ArenaAllocator.init(gpa); // Creating a custom short-lived arena
    defer local_arena.deinit();
    const local_allocator = local_arena.allocator();

    const local_data = try local_allocator.alloc(u8, 1024);
    @memset(local_data, 0);
    std.debug.print("\nManual local arena also works perfectly.\n", .{});
}

test "simple test" {
    const gpa = std.testing.allocator;
    var list: std.ArrayList(i32) = .empty;
    defer list.deinit(gpa);
    try list.append(gpa, 42);
    try std.testing.expectEqual(@as(i32, 42), list.pop().?);
}
