const std = @import("std");
const Allocator = std.mem.Allocator;

const bufferedWriterSize = 1024 * 32;
pub const BufferedWriterType = std.io.BufferedWriter(bufferedWriterSize, std.fs.File.Writer);

pub fn getBufferedWriter() BufferedWriterType {
    const stdout = std.io.getStdOut();
    const stdoutWriter = stdout.writer();
    return std.io.BufferedWriter(bufferedWriterSize, @TypeOf(stdoutWriter)){
        .unbuffered_writer = stdoutWriter,
    };
}

const StackError = error{
    StackOverflow,
};

pub fn Stack(comptime T: type, size: comptime_int) type {
    return struct {
        const Self = @This();

        allocator: Allocator,
        data: []T,
        current: usize = size - 1,

        pub fn initPtrs(allocator: Allocator, start: T) !Self {
            const slice = try allocator.alloc(T, size);
            var res: Self = .{
                .allocator = allocator,
                .data = slice,
            };

            var i: usize = 0;
            while (i < size) : (i += 1) {
                res.data[i] = @ptrFromInt(@intFromPtr(start) + (i * @sizeOf(T) * 8));
            }

            return res;
        }

        pub fn deinit(self: *Self) void {
            self.allocator.free(self.data);
        }

        pub fn pop(self: *Self) ?T {
            if (self.current == 0) return null;

            const res = self.data[self.current];
            self.current -= 1;
            return res;
        }

        pub fn push(self: *Self, item: T) !void {
            if (self.current == size - 1) {
                return StackError.StackOverflow;
            }

            self.current += 1;
            self.data[self.current] = item;
        }
    };
}
