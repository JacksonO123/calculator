const std = @import("std");
const calc = @import("calc.zig");
const tokenizer = calc.tokenizer;
const logger = calc.logger;
const utils = calc.utils;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const TokenUtil = tokenizer.TokenUtil;

const ParserError = error{
    UnexpectedToken,
};

const ExprNode = struct {
    const Self = @This();

    op: tokenizer.OperatorType,
    left: *Node,
    right: *Node,

    pub fn write(self: Self, writer: anytype) std.io.AnyWriter.Error!void {
        try writer.writeByte('(');
        try self.left.write(writer);
        try writer.writeByte(' ');
        try self.op.write(writer);
        try writer.writeByte(' ');
        try self.right.write(writer);
        try writer.writeByte(')');
    }
};

const NodeType = enum {
    Expr,
    Number,
};

pub const Node = union(NodeType) {
    const Self = @This();

    Expr: ExprNode,
    Number: tokenizer.Number,

    pub fn write(self: Self, writer: anytype) !void {
        switch (self) {
            .Expr => |expr| try expr.write(writer),
            .Number => |num| {
                if (!num.isPositive) {
                    try writer.writeByte('-');
                }

                try std.fmt.formatInt(num.data, 10, .lower, .{}, writer);
            },
        }
    }
};

pub fn parse(allocator: Allocator, tokenUtil: *TokenUtil) !*Node {
    const token = try tokenUtil.take();
    const expr = switch (token.tokType) {
        .LParen => a: {
            const tempExpr = try parse(allocator, tokenUtil);
            try tokenUtil.expectToken(.RParen);
            break :a tempExpr;
        },
        .Number => |num| try utils.createMut(Node, allocator, .{
            .Number = num,
        }),
        .RParen, .Operator => return tokenUtil.logger.logError(ParserError.UnexpectedToken),
        .NewLine => unreachable,
    };

    const opTok = tokenUtil.take() catch {
        return expr;
    };

    if (opTok.tokType != .Operator) {
        tokenUtil.returnToken();
        return expr;
    }

    const right = try parse(allocator, tokenUtil);

    const node = try utils.createMut(Node, allocator, .{
        .Expr = .{
            .op = opTok.tokType.Operator,
            .left = expr,
            .right = right,
        },
    });

    return node;
}
