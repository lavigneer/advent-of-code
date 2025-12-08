const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Add days manually as you complete them
    // To add a new day, duplicate this block and increment the day number
    const days = [_]u32{ 1 };

    inline for (days) |day| {
        const day_str = comptime std.fmt.comptimePrint("day{d}", .{day});

        const exe = b.addExecutable(.{
            .name = day_str,
            .root_module = b.createModule(.{
                .root_source_file = b.path("src/" ++ day_str ++ ".zig"),
                .target = target,
                .optimize = optimize,
            }),
        });

        b.installArtifact(exe);

        const run_step = b.step(day_str, std.fmt.comptimePrint("Run day {d}", .{day}));
        const run_cmd = b.addRunArtifact(exe);
        run_step.dependOn(&run_cmd.step);
        run_cmd.step.dependOn(b.getInstallStep());

        if (b.args) |args| {
            run_cmd.addArgs(args);
        }
    }
}
