const std = @import("std");

var rng = std.rand.DefaultPrng.init(0);

pub fn modifier(score: i8) i8 {
    return @divFloor(score - 10, 2);
}

pub fn ability() i8 {
    const scores = [4]i8{ rollDice(), rollDice(), rollDice(), rollDice() };
    const min = std.math.min(std.math.min3(scores[0], scores[1], scores[2]), scores[3]);
    var scoreExcluded = false;
    var sum: i8 = 0;
    for (scores) |score| {
        if (score == min and !scoreExcluded) {
            scoreExcluded = true;
            continue;
        }
        sum += score;
    }
    return sum;
}

fn rollDice() i8 {
    return @intCast(i8, rng.next() % 6 + 1);
}

pub const Character = struct {
    strength: i8,
    dexterity: i8,
    constitution: i8,
    intelligence: i8,
    wisdom: i8,
    charisma: i8,
    hitpoints: i8,

    pub fn init() Character {
        const strength: i8 = ability();
        const dexterity: i8 = ability();
        const constitution: i8 = ability();
        const intelligence: i8 = ability();
        const wisdom: i8 = ability();
        const charisma: i8 = ability();
        const hitpoints = 10 + modifier(constitution);
        return Character{ .strength = strength, .dexterity = dexterity, .constitution = constitution, .intelligence = intelligence, .wisdom = wisdom, .charisma = charisma, .hitpoints = hitpoints };
    }
};
