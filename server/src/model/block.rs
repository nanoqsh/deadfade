#[derive(Clone, Copy, Default)]
pub(crate) struct Block(u8);

impl Block {
    const TAG_MASK: u8 = 0b111111;

    fn with_tag(self, Tag(t): Tag) -> Self {
        Self((self.0 & !Self::TAG_MASK) | t)
    }

    fn tag(self) -> Tag {
        Tag(self.0 & Self::TAG_MASK)
    }

    const GND_BIT: u8 = 1 << 7;

    fn with_gnd(self, f: bool) -> Self {
        if f {
            Self(self.0 | Self::GND_BIT)
        } else {
            Self(self.0 & !Self::GND_BIT)
        }
    }

    fn gnd(self) -> bool {
        self.0 & Self::GND_BIT == Self::GND_BIT
    }

    const AIR_BIT: u8 = 1 << 6;

    fn with_air(self, f: bool) -> Self {
        if f {
            Self(self.0 | Self::AIR_BIT)
        } else {
            Self(self.0 & !Self::AIR_BIT)
        }
    }

    fn air(self) -> bool {
        self.0 & Self::AIR_BIT == Self::AIR_BIT
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Tag(u8);

impl Tag {
    // Passable
    const EMPTY: Self = Self(0);
    const MOD_PASSABLE: Self = Self(1);

    // Mineable
    const ORE: Self = Self(2);
    const STONE: Self = Self(3);
    const WALL: Self = Self(4);

    // Other
    const VOID: Self = Self(5);
    const MOD_SOLID: Self = Self(6);

    fn is_passable(self) -> bool {
        self.0 < Self::ORE.0
    }

    fn is_mineable(self) -> bool {
        Self::MOD_PASSABLE.0 < self.0 && self.0 < Self::VOID.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag() {
        let bl = Block::default();
        assert!(matches!(bl.tag(), Tag::EMPTY));

        let bl = bl.with_tag(Tag::VOID);
        assert!(matches!(bl.tag(), Tag::VOID));

        let bl = bl.with_tag(Tag::EMPTY);
        assert!(matches!(bl.tag(), Tag::EMPTY));
    }

    #[test]
    fn gnd() {
        let bl = Block::default();
        assert!(!bl.gnd());

        let bl = bl.with_gnd(true);
        assert!(bl.gnd());

        let bl = bl.with_gnd(false);
        assert!(!bl.gnd());
    }

    #[test]
    fn air() {
        let bl = Block::default();
        assert!(!bl.air());

        let bl = bl.with_air(true);
        assert!(bl.air());

        let bl = bl.with_air(false);
        assert!(!bl.air());
    }

    #[test]
    fn joint() {
        let bl = Block::default();
        assert!(matches!(bl.tag(), Tag::EMPTY));
        assert!(!bl.gnd());
        assert!(!bl.air());

        let bl = bl.with_tag(Tag::VOID);
        assert!(matches!(bl.tag(), Tag::VOID));
        assert!(!bl.gnd());
        assert!(!bl.air());

        let bl = bl.with_gnd(true);
        assert!(matches!(bl.tag(), Tag::VOID));
        assert!(bl.gnd());
        assert!(!bl.air());

        let bl = bl.with_air(true);
        assert!(matches!(bl.tag(), Tag::VOID));
        assert!(bl.gnd());
        assert!(bl.air());

        let bl = bl.with_tag(Tag::ORE);
        assert!(matches!(bl.tag(), Tag::ORE));
        assert!(bl.gnd());
        assert!(bl.air());
    }

    #[test]
    fn is_passable() {
        const EXPECTED: [(Tag, bool); 7] = [
            (Tag::EMPTY, true),
            (Tag::MOD_PASSABLE, true),
            (Tag::ORE, false),
            (Tag::STONE, false),
            (Tag::WALL, false),
            (Tag::VOID, false),
            (Tag::MOD_SOLID, false),
        ];

        for (tag, pass) in EXPECTED {
            assert_eq!(tag.is_passable(), pass);
        }
    }

    #[test]
    fn is_mineable() {
        const EXPECTED: [(Tag, bool); 7] = [
            (Tag::EMPTY, false),
            (Tag::MOD_PASSABLE, false),
            (Tag::ORE, true),
            (Tag::STONE, true),
            (Tag::WALL, true),
            (Tag::VOID, false),
            (Tag::MOD_SOLID, false),
        ];

        for (tag, mine) in EXPECTED {
            assert_eq!(tag.is_mineable(), mine);
        }
    }
}
