const std = @import("std");
const calc = @import("calc.zig");
const logger = calc.logger;
const tokenizer = calc.tokenizer;
const utils = calc.utils;
const TokenUtil = tokenizer.TokenUtil;
const File = std.fs.File;
const Allocator = std.mem.Allocator;

const LineBounds = struct {
    start: usize,
    end: usize,
};

const SurroundingBounds = struct {
    before: LineBounds,
    after: LineBounds,
};

pub const Logger = struct {
    const Self = @This();

    allocator: Allocator,
    tokens: *TokenUtil,
    code: []const u8,

    pub fn init(allocator: Allocator, tokens: *TokenUtil, code: []const u8) Self {
        return Self{
            .allocator = allocator,
            .tokens = tokens,
            .code = code,
        };
    }

    pub fn logError(self: *Self, err: anyerror) anyerror {
        var buffer: [utils.BUFFERED_WRITER_SIZE]u8 = undefined;
        var writer = std.fs.File.stdout().writer(&buffer);
        defer writer.end() catch {};
        var interface = writer.interface;

        const errStr = @errorName(err);
        const numSurroundingLines = 1;
        const contextBlock = findSurroundingLines(
            self.code,
            self.tokens.pos.currentLine,
            numSurroundingLines,
        );
        const beforeLines = self.code[contextBlock.before.start..contextBlock.before.end];
        const afterLines = self.code[contextBlock.after.start..contextBlock.after.end];

        const lineBounds = findLineBounds(self.code, self.tokens.pos.currentLine);
        const line = self.code[lineBounds.start..lineBounds.end];

        const startOffset = getStartOffset(
            self.tokens.tokens[self.tokens.pos.index].start,
            self.code,
        );

        try interface.writeAll("Error: ");
        try interface.writeAll(errStr);
        try interface.writeByte('\n');

        if (beforeLines.len > 0) {
            try interface.writeAll(beforeLines);
            try interface.writeByte('\n');
        }

        try interface.writeAll(line);
        try interface.writeByte('\n');

        var i: usize = 0;
        while (i < startOffset) : (i += 1) {
            try interface.writeByte(' ');
        }
        try interface.writeAll(&[_]u8{ '^', '\n' });

        if (afterLines.len > 0) {
            try interface.writeAll(afterLines);
            try interface.writeByte('\n');
        }

        return err;
    }
};

fn getStartOffset(loc: usize, code: []const u8) usize {
    var offset: usize = 0;

    for (code, 0..) |char, index| {
        if (index == loc) return offset;
        if (char == '\n') {
            offset = 0;
        } else {
            offset += 1;
        }
    }

    return offset;
}

fn findSurroundingLines(code: []const u8, line: usize, numSurroundingLines: usize) SurroundingBounds {
    var surroundingBefore = numSurroundingLines;

    if (line < numSurroundingLines) {
        surroundingBefore = line;
    }

    var currentLine: usize = 0;
    var beforeStart: ?usize = null;
    var beforeEnd: usize = 0;
    var afterStart: ?usize = null;
    var afterEnd: usize = 0;

    for (code, 0..) |char, index| {
        if (index == code.len - 1) {
            afterEnd = index;
            break;
        }

        if (currentLine + surroundingBefore + 1 == line and beforeStart == null) {
            beforeStart = index;
        } else if (currentLine + 1 == line and char == '\n') {
            beforeEnd = index;
        } else if (currentLine == line + 1 and afterStart == null) {
            afterStart = index;
        } else if (line + numSurroundingLines == currentLine and char == '\n') {
            afterEnd = index;
            break;
        }

        if (char == '\n') currentLine += 1;
    }

    if (beforeStart != null) {
        while (code[beforeStart.?] == '\n') beforeStart = beforeStart.? + 1;
    }

    if (afterStart) |start| {
        while (code[afterStart.?] == '\n') afterStart = afterStart.? + 1;

        if (start + 1 == afterEnd) {
            afterEnd = start;
        }
    }

    return .{
        .before = .{
            .start = if (beforeStart) |start| start else 0,
            .end = beforeEnd,
        },
        .after = .{
            .start = if (afterStart) |start| start else 0,
            .end = if (afterStart == null) 0 else afterEnd,
        },
    };
}

fn findLineBounds(code: []const u8, line: usize) LineBounds {
    var start: ?usize = null;
    var end: usize = 0;
    var currentLine: usize = 0;

    for (code, 0..) |char, index| {
        if (index == code.len - 1) {
            end = index;
            break;
        }

        if (currentLine == line) {
            if (start == null) {
                start = index;
            } else if (char == '\n') {
                end = index;
                break;
            }
        }

        if (char == '\n') currentLine += 1;
    }

    return .{
        .start = if (start) |s| s else 0,
        .end = end,
    };
}
