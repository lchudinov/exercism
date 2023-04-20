const std = @import("std");
const EnumSet = std.EnumSet;

pub const Allergen = enum(u8) {
    eggs = 1,
    peanuts = 2,
    shellfish = 4,
    strawberries = 8,
    tomatoes = 16,
    chocolate = 32,
    pollen = 64,
    cats = 128,
};

pub fn isAllergicTo(score: u8, allergen: Allergen) bool {
    return score & @enumToInt(allergen) == @enumToInt(allergen);
}

pub fn initAllergenSet(score: usize) EnumSet(Allergen) {
    var set = EnumSet(Allergen){};
    const fields = std.meta.fields(Allergen);
    inline for (fields) |field| {
        const allergen = @intToEnum(Allergen, field.value);
        if (isAllergicTo(@truncate(u8, score), allergen)) {
            set.insert(allergen);
        }
    }
    return set;
}
