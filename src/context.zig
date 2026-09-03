pub const tokenizer = @import("tokenizer.zig");
pub const parser = @import("parser.zig");
pub const logMod = @import("logger.zig");

pub const Context = struct {
    const Self = @This();

    tokens: *tokenizer.TokenUtil,
    logger: *logMod.Logger,

    pub fn init(tokens: *tokenizer.TokenUtil, logger: *logMod.Logger) Self {
        return .{
            .tokens = tokens,
            .logger = logger,
        };
    }
};
