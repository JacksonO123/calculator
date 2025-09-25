const std = @import("std");
const calc = @import("calc.zig");
const parser = calc.parser;
const utils = calc.utils;
const Allocator = std.mem.Allocator;
const Stack = utils.Stack;

const NodePoolError = error{
    NoMoreNodes,
};

const NUM_NODES = 100000;

const FreeNodesStack = Stack(usize, NUM_NODES);

pub const NodePool = struct {
    const Self = @This();

    allocator: Allocator,
    nodes: []parser.Node,
    freeNodes: FreeNodesStack,

    pub fn init(allocator: Allocator) !Self {
        const slice = try allocator.alloc(parser.Node, NUM_NODES);

        return .{
            .allocator = allocator,
            .nodes = slice,
            .freeNodes = try FreeNodesStack.initWithIndices(allocator),
        };
    }

    pub fn deinit(self: *Self) void {
        self.allocator.free(self.nodes);
    }

    pub fn newNode(self: *Self, val: parser.Node) !*parser.Node {
        const freeIndex = self.freeNodes.pop();
        if (freeIndex) |index| {
            self.nodes[index] = val;
            return &self.nodes[index];
        }

        return NodePoolError.NoMoreNodes;
    }
};
