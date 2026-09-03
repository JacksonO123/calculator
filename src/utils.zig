const std = @import("std");
const utils = @import("utils.zig");
const Allocator = std.mem.Allocator;

pub const BUFFERED_WRITER_SIZE = 1024 * 32;

pub inline fn createMut(comptime T: type, allocator: Allocator, obj: T) Allocator.Error!*T {
    const ptr = try allocator.create(T);
    ptr.* = obj;
    return ptr;
}
