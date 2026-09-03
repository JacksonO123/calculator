const std = @import("std");
const builtin = @import("builtin");
const tokenizer = @import("tokenizer.zig");
const logMod = @import("logger.zig");
const parser = @import("parser.zig");
const utils = @import("utils.zig");
const Allocator = std.mem.Allocator;
const TokenUtil = tokenizer.TokenUtil;
const Context = @import("context.zig").Context;

pub fn main(init: std.process.Init) !void {
    const args = try init.minimal.args.toSlice(init.arena.allocator());
    if (args.len < 2) {
        return error.NoInputFile;
    }

    const path = args[1];

    var buffer: [utils.BUFFERED_WRITER_SIZE]u8 = undefined;
    var stdoutWriter = std.Io.File.stdout().writer(init.io, &buffer);
    const writer = &stdoutWriter.interface;
    defer stdoutWriter.end() catch {
        std.debug.print("problem\n", .{});
    };

    const code = try readRelativeFile(init.arena.allocator(), init.io, path);
    const tokens = try tokenizer.tokenize(init.arena.allocator(), code, writer);

    var tokenUtil = TokenUtil.init(tokens);
    var loggerUtil = logMod.Logger.init(&tokenUtil, code);

    var context = Context.init(&tokenUtil, &loggerUtil);

    const tree = try parser.parse(init.arena.allocator(), &context, writer);
    const simple = try tree.simplify(init.arena.allocator());

    try tree.write(writer);
    try writer.writeAll("\n");

    try simple.write(writer);
    try writer.writeAll("\n");
    try writer.flush();
}

fn readRelativeFile(allocator: Allocator, io: std.Io, path: []const u8) ![]const u8 {
    return try std.Io.Dir.readFileAlloc(std.Io.Dir.cwd(), io, path, allocator, .unlimited);
}
