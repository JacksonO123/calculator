const std = @import("std");
const calc = @import("calc.zig");
const parser = calc.parser;
const utils = calc.utils;
const Allocator = std.mem.Allocator;
const Stack = utils.Stack;

const NodePoolError = error{
    NoMoreNodes,
};

const NUM_NODES = 1024 * 64;

const FreeNodesStack = Stack(*parser.Node, NUM_NODES);

const NodePoolChunk = struct {
    nodes: []parser.Node,
    next: ?*NodePoolChunk,
};

pub const NodePool = struct {
    const Self = @This();

    allocator: Allocator,
    root: *NodePoolChunk,
    last: *NodePoolChunk,
    freeNodes: *FreeNodesStack,

    fn newChunk(allocator: Allocator) !*NodePoolChunk {
        const nodes = try allocator.alloc(parser.Node, NUM_NODES);
        const chunk = utils.createMut(NodePoolChunk, allocator, .{
            .nodes = nodes,
            .next = null,
        });
        return chunk;
    }

    pub fn init(allocator: Allocator) !Self {
        const chunk = try Self.newChunk(allocator);
        const stack = try FreeNodesStack.init(allocator);
        var i: usize = 0;
        while (i < stack.data.items.len) : (i += 1) {
            stack.data.items[i] = &chunk.nodes[i];
        }
        const stackPtr = try utils.createMut(FreeNodesStack, allocator, stack);

        return .{
            .allocator = allocator,
            .root = chunk,
            .last = chunk,
            .freeNodes = stackPtr,
        };
    }

    pub fn deinit(self: *Self) void {
        var current: ?*NodePoolChunk = self.root;
        while (current) |chunk| {
            self.allocator.free(chunk.nodes);
            self.allocator.destroy(chunk);
            current = chunk.next;
        }

        self.freeNodes.deinit();
        self.allocator.destroy(self.freeNodes);
    }

    pub fn newNode(self: *Self, val: parser.Node) !*parser.Node {
        const freePtr = self.freeNodes.pop();
        if (freePtr) |ptr| {
            ptr.* = val;
            return ptr;
        }

        try self.appendChunk();
        return try self.newNode(val);
    }

    fn appendChunk(self: *Self) !void {
        const chunk = try Self.newChunk(self.allocator);
        self.last.next = chunk;
        self.last = chunk;
        var slice = try self.freeNodes.appendChunk();
        var i: usize = 0;
        while (i < slice.len) : (i += 1) {
            slice[i] = &chunk.nodes[i];
        }
    }
};
