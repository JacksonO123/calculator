const std = @import("std");
const calc = @import("calc.zig");
const utils = calc.utils;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub const BUFFERED_WRITER_SIZE = 1024 * 32;

pub inline fn createMut(comptime T: type, allocator: Allocator, obj: T) Allocator.Error!*T {
    const ptr = try allocator.create(T);
    ptr.* = obj;
    return ptr;
}

pub fn Stack(comptime T: type, size: comptime_int) type {
    return struct {
        const Self = @This();

        allocator: Allocator,
        data: *ArrayList(T),

        pub fn init(allocator: Allocator) !Self {
            const slice = try allocator.alloc(T, size);
            const list = ArrayList(T).fromOwnedSlice(slice);
            const listPtr = try utils.createMut(ArrayList(T), allocator, list);

            return .{
                .allocator = allocator,
                .data = listPtr,
            };
        }

        pub fn deinit(self: *Self) void {
            self.data.deinit(self.allocator);
            self.allocator.destroy(self.data);
        }

        pub fn pop(self: *Self) ?T {
            return self.data.pop();
        }

        pub fn push(self: *Self, item: T) !void {
            try self.data.append(self.allocator, item);
        }

        pub fn appendChunk(self: *Self) ![]T {
            const len = self.data.items.len;
            var ptrs: [size]T = undefined;
            try self.data.appendSlice(self.allocator, &ptrs);
            return self.data.items[len..];
        }
    };
}
