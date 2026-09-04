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
    DivideByZero,
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
    Variable,
};

pub const Node = union(NodeType) {
    const Self = @This();

    Expr: ExprNode,
    Number: i64,
    Variable: []const u8,

    pub fn allocate(allocator: Allocator, data: Self) !*Self {
        const ptr = try allocator.create(Self);
        ptr.* = data;
        return ptr;
    }

    pub fn write(self: Self, writer: *Writer) !void {
        switch (self) {
            .Expr => |expr| try expr.write(writer),
            .Number => |num| try writer.print("{d}", .{num}),
            .Variable => |chars| try writer.print("{s}", .{chars}),
        }
    }

    pub fn simplify(self: *Self, allocator: Allocator) !*Self {
        switch (self.*) {
            .Number, .Variable => return self,
            .Expr => |expr| {
                switch (expr.op) {
                    .Add => {
                        const left = try expr.left.simplify(allocator);
                        const right = try expr.right.simplify(allocator);

                        if (left.* == .Number) {
                            if (right.* == .Number) {
                                return try Node.allocate(
                                    allocator,
                                    .{ .Number = left.Number + right.Number },
                                );
                            } else if (left.Number == 0) {
                                return right;
                            }
                        } else if (right.* == .Number and right.Number == 0) {
                            return left;
                        }

                        return try Node.allocate(allocator, .{
                            .Expr = .{
                                .left = left,
                                .right = right,
                                .op = .Add,
                            },
                        });
                    },
                    .Mult => {
                        const left = try expr.left.simplify(allocator);
                        const right = try expr.right.simplify(allocator);

                        if (left.* == .Number) {
                            if (right.* == .Number) {
                                return try Node.allocate(
                                    allocator,
                                    .{ .Number = left.Number * right.Number },
                                );
                            } else if (left.Number == 0) {
                                return try Node.allocate(allocator, .{ .Number = 0 });
                            } else if (left.Number == 1) {
                                return right;
                            }
                        } else if (right.* == .Number) {
                            if (right.Number == 0) {
                                return try Node.allocate(allocator, .{ .Number = 0 });
                            } else if (right.Number == 1) {
                                return left;
                            }
                        }

                        return try Node.allocate(allocator, .{
                            .Expr = .{
                                .left = left,
                                .right = right,
                                .op = .Mult,
                            },
                        });
                    },
                    .Sub => {
                        const left = try expr.left.simplify(allocator);
                        const right = try expr.right.simplify(allocator);

                        if (left.* == .Number and right.* == .Number) {
                            return try Node.allocate(
                                allocator,
                                .{ .Number = left.Number - right.Number },
                            );
                        } else if (right.* == .Number and right.Number == 0) {
                            return left;
                        }

                        return try Node.allocate(allocator, .{
                            .Expr = .{
                                .left = left,
                                .right = right,
                                .op = .Sub,
                            },
                        });
                    },
                    .Div => {
                        const left = try expr.left.simplify(allocator);
                        const right = try expr.right.simplify(allocator);

                        if (left.* == .Number and right.* == .Number) {
                            if (right.Number == 0) {
                                return ParserError.DivideByZero;
                            }
                            if (@mod(left.Number, right.Number) == 0) {
                                return try Node.allocate(
                                    allocator,
                                    .{ .Number = @divTrunc(left.Number, right.Number) },
                                );
                            }
                            const g = std.math.gcd(@abs(left.Number), @abs(right.Number));
                            if (g > 1) {
                                return try Node.allocate(allocator, .{
                                    .Expr = .{
                                        .left = try Node.allocate(
                                            allocator,
                                            .{ .Number = @divTrunc(left.Number, @as(i64, @intCast(g))) },
                                        ),
                                        .right = try Node.allocate(
                                            allocator,
                                            .{ .Number = @divTrunc(right.Number, @as(i64, @intCast(g))) },
                                        ),
                                        .op = .Div,
                                    },
                                });
                            }
                            return try Node.allocate(allocator, .{
                                .Expr = .{
                                    .left = left,
                                    .right = right,
                                    .op = .Div,
                                },
                            });
                        } else if (left.* == .Number) {
                            if (left.Number == 0) {
                                return try Node.allocate(allocator, .{ .Number = 0 });
                            } else if (left.Number == 1) {
                                return right;
                            }
                        } else if (right.* == .Number) {
                            if (right.Number == 1) {
                                return left;
                            }
                        }

                        return try Node.allocate(allocator, .{
                            .Expr = .{
                                .left = left,
                                .right = right,
                                .op = .Div,
                            },
                        });
                    },
                }
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
        .Number => |num| try Node.allocate(allocator, .{ .Number = num }),
        .RParen, .Operator => return context.logger.logError(ParserError.UnexpectedToken, writer),
        .Variable => |chars| try Node.allocate(allocator, .{ .Variable = chars }),
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

    const node = try Node.allocate(allocator, .{
        .Expr = .{
            .op = opTok.tokType.Operator,
            .left = expr,
            .right = right,
        },
    });

    return node;
}
