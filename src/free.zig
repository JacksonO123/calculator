const std = @import("std");
const calc = @import("calc.zig");
const parser = calc.parser;
const Allocator = std.mem.Allocator;
const Node = parser.Node;

pub fn freeNode(allocator: Allocator, node: *Node) void {
    switch (node.*) {
        .Expr => |expr| {
            freeNode(allocator, expr.left);
            freeNode(allocator, expr.right);
        },
        .Number => {},
    }

    allocator.destroy(node);
}
