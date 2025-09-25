const std = @import("std");
const builtin = @import("builtin");
const calc = @import("calc.zig");
const tokenizer = calc.tokenizer;
const parser = calc.parser;
const logger = calc.logger;
const free = calc.free;
const Allocator = std.mem.Allocator;
const Logger = logger.Logger;
const TokenUtil = tokenizer.TokenUtil;

pub fn main() !void {
    const dbg = builtin.mode == .Debug;
    var gp = std.heap.GeneralPurposeAllocator(.{ .safety = dbg }){};
    defer _ = gp.deinit();
    const allocator = gp.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    if (args.len < 2) {
        return error.NoInputFile;
    }

    const path = args[1];

    const code = try readRelativeFile(allocator, path);
    defer allocator.free(code);

    const tokens = try tokenizer.tokenize(allocator, code);
    defer allocator.free(tokens);

    for (tokens) |token| {
        std.debug.print("{any}\n", .{token});
    }

    var tokenUtil: TokenUtil = undefined;
    var loggerUtil = Logger.init(allocator, &tokenUtil, code);
    tokenUtil = TokenUtil.init(&loggerUtil, tokens);
    loggerUtil.tokens = &tokenUtil;

    const tree = try parser.parse(allocator, &tokenUtil);
    defer free.freeNode(allocator, tree);

    const stdout = std.io.getStdOut();
    const writer = stdout.writer();

    try tree.write(writer);
    try writer.writeByte('\n');
}

fn readRelativeFile(allocator: Allocator, path: []const u8) ![]const u8 {
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(allocator, std.math.maxInt(usize));
}
