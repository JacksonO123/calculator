const std = @import("std");
const calc = @import("calc.zig");
const utils = calc.utils;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const Writer = std.fs.File.Writer;

const INIT_TOK_CAPACITY = 1024 * 10;

pub const TokenizeError = error{
    NumberHasTwoPeriods,
    ExpectedCharacterFoundNothing,
    UnexpectedCharacter,
};

pub const TokenError = error{
    ExpectedTokenFoundNothing,
    UnexpectedToken,
};

fn tokenizeErrorToString(err: TokenizeError) []const u8 {
    return switch (err) {
        TokenizeError.NumberHasTwoPeriods => "number has two periods",
        TokenizeError.ExpectedCharacterFoundNothing => "expected character found nothing",
        TokenizeError.UnexpectedCharacter => "unexpected character",
    };
}

const TokenVariants = enum {
    NewLine,
    LParen,
    RParen,
    Operator,
    Number,
};

pub const OperatorType = enum {
    const Self = @This();

    Add,
    Sub,
    Mult,
    Div,

    pub fn toChar(self: Self) u8 {
        return switch (self) {
            .Add => '+',
            .Sub => '-',
            .Mult => '*',
            .Div => '/',
        };
    }

    pub fn write(self: Self, writer: *Writer) !void {
        try writer.interface.writeAll(&[_]u8{self.toChar()});
    }
};

pub const Number = struct {
    data: u64,
    isPositive: bool,
};

const TokenType = union(TokenVariants) {
    NewLine,
    LParen,
    RParen,
    Operator: OperatorType,
    Number: Number,
};

pub const Token = struct {
    const Self = @This();

    tokType: TokenType,
    start: usize,
    end: usize,

    pub fn init(tokType: TokenType, pos: usize) Self {
        return .{
            .tokType = tokType,
            .start = pos,
            .end = pos + 1,
        };
    }

    pub fn initRange(tokType: TokenType, start: usize, end: usize) Self {
        return .{
            .tokType = tokType,
            .start = start,
            .end = end,
        };
    }
};

pub fn tokenize(allocator: Allocator, code: []const u8) ![]Token {
    var charUtil = CharUtil.init(code);
    var tokens = try ArrayList(Token).initCapacity(allocator, INIT_TOK_CAPACITY);

    while (charUtil.hasNext()) {
        const token = parseNextToken(&charUtil) catch |e| {
            return charUtil.logError(e);
        };
        if (token) |t| {
            try tokens.append(allocator, t);
        }
    }

    return try tokens.toOwnedSlice(allocator);
}

fn parseNextToken(charUtil: *CharUtil) TokenizeError!?Token {
    const pos = charUtil.index;
    const char = try charUtil.take();
    return switch (char) {
        ' ' => null,
        '\n' => Token.init(.{ .NewLine = {} }, pos),
        '(' => Token.init(.{ .LParen = {} }, pos),
        ')' => Token.init(.{ .RParen = {} }, pos),
        '+' => Token.init(.{ .Operator = .Add }, pos),
        '*' => Token.init(.{ .Operator = .Mult }, pos),
        '/' => Token.init(.{ .Operator = .Div }, pos),
        '-' => {
            const next = try charUtil.peak();
            if (std.ascii.isDigit(next)) {
                const data = try parseNumber(charUtil);
                return Token.initRange(.{
                    .Number = .{
                        .data = data.value,
                        .isPositive = false,
                    },
                }, pos, pos + data.length - 1);
            }

            return Token.init(.{
                .Operator = .Sub,
            }, pos);
        },
        else => {
            if (std.ascii.isDigit(char)) {
                charUtil.returnChar();
                const data = try parseNumber(charUtil);
                return Token.initRange(.{
                    .Number = .{
                        .data = data.value,
                        .isPositive = true,
                    },
                }, pos, pos + data.length - 1);
            }

            return TokenizeError.UnexpectedCharacter;
        },
    };
}

const ParsedNumber = struct {
    value: u64,
    length: usize,
};

fn parseNumber(charUtil: *CharUtil) !ParsedNumber {
    var data: u64 = 0;
    var length: usize = 0;

    var char = try charUtil.take();
    var foundPeriod = false;
    while (std.ascii.isDigit(char) or char == '.') {
        if (char == '.') {
            if (foundPeriod) return TokenizeError.NumberHasTwoPeriods;
            foundPeriod = true;
        }

        const charData = char - '0';
        data *= 10;
        data += charData;
        length += 1;

        char = try charUtil.take();
    }

    charUtil.returnChar();

    return .{
        .value = data,
        .length = length,
    };
}

const CharUtil = struct {
    const Self = @This();

    index: usize,
    chars: []const u8,

    pub fn init(chars: []const u8) Self {
        return Self{
            .index = 0,
            .chars = chars,
        };
    }

    pub fn getSlice(self: Self) []const u8 {
        return self.chars[self.index..];
    }

    pub fn hasNext(self: Self) bool {
        return self.index < self.chars.len;
    }

    pub fn advance(self: *Self, amount: usize) !void {
        if (self.index + amount > self.chars.len) {
            return TokenizeError.ExpectedCharacterFoundNothing;
        }

        self.index += amount;
    }

    pub fn take(self: *Self) !u8 {
        if (self.index == self.chars.len) {
            return TokenizeError.ExpectedCharacterFoundNothing;
        }

        const char = self.chars[self.index];
        self.index += 1;
        return char;
    }

    pub fn peak(self: Self) !u8 {
        if (self.index >= self.chars.len) {
            return TokenizeError.ExpectedCharacterFoundNothing;
        }

        return self.chars[self.index];
    }

    pub fn returnChar(self: *Self) void {
        self.index -= 1;
    }

    pub fn getChars(self: Self) []const u8 {
        return self.chars;
    }

    pub fn logError(self: *Self, err: TokenizeError) TokenizeError {
        const stdout = std.fs.File.stdout();
        const errStr = tokenizeErrorToString(err);

        const index = self.index - 1;
        const bounds = getLineBounds(self.chars, index);
        const charIndex = index - bounds.start;
        const line = self.chars[bounds.start..bounds.end];

        stdout.writeAll("Error: ") catch {};
        stdout.writeAll(errStr) catch {};
        stdout.writeAll("\n") catch {};
        stdout.writeAll(line) catch {};
        stdout.writeAll("\n") catch {};

        var i: usize = 0;
        while (i < charIndex) : (i += 1) {
            stdout.writeAll(" ") catch {};
        }

        stdout.writeAll("^\n") catch {};

        return err;
    }
};

const LineBounds = struct {
    start: usize,
    end: usize,
};

fn getLineBounds(chars: []const u8, index: usize) LineBounds {
    var lineStart: usize = 0;
    var lineEnd: usize = 0;

    for (chars, 0..) |char, charIndex| {
        if (char == '\n') {
            if (charIndex < index) {
                lineStart = charIndex + 1;
            } else {
                lineEnd = charIndex;

                return .{
                    .start = lineStart,
                    .end = lineEnd,
                };
            }
        }
    }

    if (lineEnd < lineStart) lineEnd = chars.len;

    return .{
        .start = lineStart,
        .end = lineEnd,
    };
}

const TokenPosition = struct {
    index: usize,
    currentLine: usize,
};

pub const TokenUtil = struct {
    const Self = @This();

    pos: TokenPosition,
    tokens: []Token,

    pub fn init(tokens: []Token) Self {
        return Self{
            .pos = .{
                .index = 0,
                .currentLine = 0,
            },
            .tokens = tokens,
        };
    }

    pub fn reset(self: *Self) void {
        self.pos = .{
            .index = 0,
            .currentLine = 0,
        };
    }

    pub fn take(self: *Self) !Token {
        const res = try self.takeFixed();

        if (res.tokType == .NewLine) {
            return try self.take();
        }

        return res;
    }

    pub fn takeFixed(self: *Self) !Token {
        if (self.pos.index >= self.tokens.len) {
            self.pos.index = self.tokens.len - 1;
            return TokenError.ExpectedTokenFoundNothing;
        }

        const res = self.tokens[self.pos.index];
        self.pos.index += 1;

        if (res.tokType == .NewLine) {
            self.pos.currentLine += 1;
        }

        return res;
    }

    pub fn peakFixed(self: Self) !Token {
        if (self.pos.index >= self.tokens.len) {
            return TokenError.ExpectedTokenFoundNothing;
        }

        return self.tokens[self.pos.index];
    }

    pub fn peak(self: *Self) !Token {
        const res = try self.peakFixed();

        if (res.tokType == .NewLine) {
            self.pos.index += 1;
            self.pos.currentLine += 1;

            const newRes = self.peak();

            self.pos.index -= 1;
            self.pos.currentLine -= 1;

            return newRes;
        }

        return res;
    }

    pub fn returnToken(self: *Self) void {
        self.pos.index -= 1;
        while (self.tokens[self.pos.index].tokType == .NewLine) : (self.pos.index -= 1) {}
    }

    pub fn expectToken(self: *Self, tokenType: TokenType) !void {
        const token = try self.take();
        if (std.meta.activeTag(token.tokType) != std.meta.activeTag(tokenType)) {
            return TokenError.UnexpectedToken;
        }
    }

    pub fn hasNextFixed(self: Self) bool {
        if (self.pos.index < self.tokens.len) return true;
        return false;
    }

    pub fn hasNext(self: *Self) bool {
        _ = self.peak() catch {
            return false;
        };

        return true;
    }
};
