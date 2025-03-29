const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "main",
        .target = target,
        .optimize = optimize,
    });

    const cppflags = [_][]const u8{ "-Wall", "-Wextra", "-std=c++20", "-O2" };

    exe.linkLibCpp();
    exe.addCSourceFile(.{
        .file = b.path("src/main.cpp"),
        .flags = &cppflags,
    });

    b.installArtifact(exe);

    // add run step
    const exe_run = b.addRunArtifact(exe);
    const exe_step = b.step("run", "run app");
    exe_step.dependOn(&exe_run.step);
}
