pub const Planet = enum {
    earth,
    mercury,
    venus,
    mars,
    jupiter,
    saturn,
    uranus,
    neptune,

    pub fn age(self: Planet, seconds: usize) f64 {
        const secsInEarthYear: f64 = 31557600.0;
        const floatSeconds: f64 = @intToFloat(f64, seconds);
        switch (self) {
            Planet.earth => return floatSeconds / secsInEarthYear / 1.0,
            Planet.mercury => return floatSeconds / secsInEarthYear / 0.2408467,
            Planet.venus => return floatSeconds / secsInEarthYear / 0.61519726,
            Planet.mars => return floatSeconds / secsInEarthYear / 1.8808158,
            Planet.jupiter => return floatSeconds / secsInEarthYear / 11.862615,
            Planet.saturn => return floatSeconds / secsInEarthYear / 29.447498,
            Planet.uranus => return floatSeconds / secsInEarthYear / 84.016846,
            Planet.neptune => return floatSeconds / secsInEarthYear / 164.79132,
        }
    }
};
