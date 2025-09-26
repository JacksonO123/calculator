const std = @import("std");
const calc = @import("calc.zig");
const tokenizer = calc.tokenizer;
const logger = calc.logger;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const TokenUtil = tokenizer.TokenUtil;
const Context = calc.Context;
const Writer = std.fs.File.Writer;

const ParserError = error{
    UnexpectedToken,
};

const ExprNode = struct {
    const Self = @This();

    left: *Node,
    right: *Node,
    op: tokenizer.OperatorType,

    pub fn write(self: Self, writer: *Writer) std.io.AnyWriter.Error!void {
        try writer.interface.writeAll("(");
        try self.left.write(writer);
        try writer.interface.writeAll(" ");
        try self.op.write(writer);
        try writer.interface.writeAll(" ");
        try self.right.write(writer);
        try writer.interface.writeAll(")");
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

    pub fn write(self: Self, writer: *Writer) !void {
        switch (self) {
            .Expr => |expr| try expr.write(writer),
            .Number => |num| {
                if (!num.isPositive) {
                    try writer.interface.writeAll("-");
                }

                try writer.interface.print("{d}", .{num.data});
            },
        }
    }
};

pub fn parse(allocator: Allocator, context: *Context) !*Node {
    const token = try context.tokens.take();
    const expr = switch (token.tokType) {
        .LParen => a: {
            const tempExpr = try parse(allocator, context);
            try context.tokens.expectToken(.RParen);
            break :a tempExpr;
        },
        .Number => |num| try context.nodePool.newNode(.{
            .Number = num,
        }),
        .RParen, .Operator => return context.logger.logError(ParserError.UnexpectedToken),
        .NewLine => unreachable,
    };

    const opTok = context.tokens.take() catch {
        return expr;
    };

    if (opTok.tokType != .Operator) {
        context.tokens.returnToken();
        return expr;
    }

    const right = try parse(allocator, context);

    const node = try context.nodePool.newNode(.{
        .Expr = .{
            .op = opTok.tokType.Operator,
            .left = expr,
            .right = right,
        },
    });

    return node;
}
