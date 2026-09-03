const std = @import("std");
const Writer = std.Io.Writer;
const Allocator = std.mem.Allocator;

const Context = @import("context.zig").Context;
const logger = @import("logger.zig");
const tokenizer = @import("tokenizer.zig");
const TokenUtil = tokenizer.TokenUtil;
const utils = @import("utils.zig");

const ParserError = error{
    UnexpectedToken,
};

const ExprNode = struct {
    const Self = @This();

    left: *Node,
    right: *Node,
    op: tokenizer.OperatorType,

    pub fn write(self: Self, writer: *Writer) std.Io.Writer.Error!void {
        try writer.writeAll("(");
        try self.left.write(writer);
        try writer.writeAll(" ");
        try self.op.write(writer);
        try writer.writeAll(" ");
        try self.right.write(writer);
        try writer.writeAll(")");
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

    pub fn allocate(self: Self, allocator: Allocator) !*Self {
        const ptr = try allocator.create(Self);
        ptr.* = self;
        return ptr;
    }

    pub fn write(self: Self, writer: *Writer) !void {
        switch (self) {
            .Expr => |expr| try expr.write(writer),
            .Number => |num| {
                if (!num.isPositive) {
                    try writer.writeAll("-");
                }

                try writer.print("{d}", .{num.data});
            },
        }
    }
};

pub fn parse(allocator: Allocator, context: *Context, writer: *Writer) !*Node {
    const expr = try parseImpl(allocator, context, writer);
    return rotatePrecedence(expr);
}

fn rotatePrecedence(node: *Node) *Node {
    const expr = if (node.* == .Expr) node.Expr else return node;
    const rightNode = if (expr.right.* == .Expr) expr.right else return node;
    const rightExpr = rightNode.Expr;

    if (@intFromEnum(expr.op) < @intFromEnum(rightExpr.op) or
        (expr.op == .Sub and rightExpr.op == .Sub))
    {
        const childLeft = rightExpr.left;
        node.Expr.right = childLeft;
        rightNode.Expr.left = rotatePrecedence(node);
        return rightNode;
    }

    return node;
}

pub fn parseImpl(allocator: Allocator, context: *Context, writer: *Writer) !*Node {
    const token = try context.tokens.take();
    const expr = switch (token.tokType) {
        .LParen => a: {
            const tempExpr = try parseImpl(allocator, context, writer);
            try context.tokens.expectToken(.RParen);
            break :a tempExpr;
        },
        .Number => |num| try (Node{
            .Number = num,
        }).allocate(allocator),
        .RParen, .Operator => return context.logger.logError(ParserError.UnexpectedToken, writer),
        .NewLine => unreachable,
    };

    const opTok = context.tokens.take() catch {
        return expr;
    };

    if (opTok.tokType != .Operator) {
        context.tokens.returnToken();
        return expr;
    }

    const right = try parseImpl(allocator, context, writer);

    const node = try (Node{
        .Expr = .{
            .op = opTok.tokType.Operator,
            .left = expr,
            .right = right,
        },
    }).allocate(allocator);

    return node;
}
