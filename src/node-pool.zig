const std = @import("std");
const calc = @import("calc.zig");
const parser = calc.parser;
const utils = calc.utils;
const Allocator = std.mem.Allocator;
const Stack = utils.Stack;

const NodePoolError = error{
    NoMoreNodes,
};

const NUM_NODES = 10;

const FreeNodesStack = Stack(*parser.Node, NUM_NODES);

pub const NodePool = struct {
    const Self = @This();

    allocator: Allocator,
    nodes: []parser.Node,
    freeNodes: FreeNodesStack,

    pub fn init(allocator: Allocator) !Self {
        const slice = try allocator.alloc(parser.Node, NUM_NODES);
        const stack = try FreeNodesStack.initPtrs(allocator, &slice[0]);

        return .{
            .allocator = allocator,
            .nodes = slice,
            .freeNodes = stack,
        };
    }

    pub fn deinit(self: *Self) void {
        self.allocator.free(self.nodes);
        self.freeNodes.deinit();
    }

    pub fn newNode(self: *Self, val: parser.Node) !*parser.Node {
        const freePtr = self.freeNodes.pop();
        if (freePtr) |ptr| {
            ptr.* = val;
            return ptr;
        }

        return NodePoolError.NoMoreNodes;
    }
};
