const std = @import("std");
const mem = std.mem;

pub fn response(s: []const u8) []const u8 {
    const trimmed = mem.trim(u8, s, &std.ascii.whitespace);
    const isYelling = isUppercase(trimmed);
    const isQuestion = mem.endsWith(u8, trimmed, "?");
    const isSilence = trimmed.len == 0;
    if (isYelling and isQuestion) {
        return "Calm down, I know what I'm doing!";
    } else if (isSilence) {
        return "Fine. Be that way!";
    } else if (isQuestion) {
        return "Sure.";
    } else if (isYelling) {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}

fn isUppercase(s: []const u8) bool {
    var hasAlphabetic = false;
    for (s) |c| {
        if (std.ascii.isAlphabetic(c)) {
            hasAlphabetic = true;
        }
        if (std.ascii.isLower(c)) {
            return false;
        }
    }
    return hasAlphabetic;
}
