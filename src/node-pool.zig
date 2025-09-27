const std = @import("std");
const calc = @import("calc.zig");
const parser = calc.parser;
const utils = calc.utils;
const Allocator = std.mem.Allocator;
const Stack = utils.Stack;
const ArrayList = std.ArrayList;

const NUM_NODES = 1024 * 64;

const NodePoolChunk = struct {
    nodes: []parser.Node,
    next: ?*NodePoolChunk,
};

pub const NodePool = struct {
    const Self = @This();

    allocator: Allocator,
    root: *NodePoolChunk,
    last: *NodePoolChunk,
    freeNodes: *ArrayList(*parser.Node),

    fn newChunk(allocator: Allocator) !*NodePoolChunk {
        const nodes = try allocator.alloc(parser.Node, NUM_NODES);
        const chunk = utils.createMut(NodePoolChunk, allocator, .{
            .nodes = nodes,
            .next = null,
        });
        return chunk;
    }

    pub fn init(allocator: Allocator) !Self {
        const chunk = try newChunk(allocator);
        const stack = try ArrayList(*parser.Node).initCapacity(allocator, NUM_NODES);
        var i: usize = 0;
        while (i < stack.items.len) : (i += 1) {
            stack.items[i] = &chunk.nodes[i];
        }
        const stackPtr = try utils.createMut(ArrayList(*parser.Node), allocator, stack);

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

        self.freeNodes.deinit(self.allocator);
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
        const chunk = try newChunk(self.allocator);
        self.last.next = chunk;
        self.last = chunk;

        var ptrs: [NUM_NODES]*parser.Node = undefined;
        var i: usize = 0;
        while (i < ptrs.len) : (i += 1) {
            ptrs[i] = &chunk.nodes[i];
        }

        try self.freeNodes.appendSlice(self.allocator, &ptrs);
    }
};
