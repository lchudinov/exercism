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
    const set = initAllergenSet(score);
    return set.contains(allergen);
}

pub fn initAllergenSet(score: usize) EnumSet(Allergen) {
    var set = EnumSet(Allergen){};
    var s = score;
    while (s >= 256) {
        s -= 256;
    }
    while (s >= @enumToInt(Allergen.cats)) {
        s -= @enumToInt(Allergen.cats);
        set.insert(Allergen.cats);
    }
    while (s >= @enumToInt(Allergen.pollen)) {
        s -= @enumToInt(Allergen.pollen);
        set.insert(Allergen.pollen);
    }
    while (s >= @enumToInt(Allergen.chocolate)) {
        s -= @enumToInt(Allergen.chocolate);
        set.insert(Allergen.chocolate);
    }
    while (s >= @enumToInt(Allergen.tomatoes)) {
        s -= @enumToInt(Allergen.tomatoes);
        set.insert(Allergen.tomatoes);
    }
    while (s >= @enumToInt(Allergen.strawberries)) {
        s -= @enumToInt(Allergen.strawberries);
        set.insert(Allergen.strawberries);
    }
    while (s >= @enumToInt(Allergen.shellfish)) {
        s -= @enumToInt(Allergen.shellfish);
        set.insert(Allergen.shellfish);
    }
    while (s >= @enumToInt(Allergen.peanuts)) {
        s -= @enumToInt(Allergen.peanuts);
        set.insert(Allergen.peanuts);
    }
    while (s >= @enumToInt(Allergen.eggs)) {
        s -= @enumToInt(Allergen.eggs);
        set.insert(Allergen.eggs);
    }
    return set;
}
