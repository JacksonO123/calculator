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
