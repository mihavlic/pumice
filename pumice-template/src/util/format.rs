#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormatAspectBits {
    pub color: u8,
    pub depth: u8,
    pub stencil: u8,
    pub unused: u8,
}

macro_rules! aspects {
    ($c:literal, $d:literal, $s:literal, $u:literal) => {
        (
            crate::vk10::ImageAspectFlags::empty(),
            FormatAspectBits {
                color: $c,
                depth: $d,
                stencil: $s,
                unused: $u,
            },
        )
    };
    ($c:literal, $d:literal, $s:literal, $u:literal $(, $aspect:ident)+) => {
        (
            $(
                crate::vk10::ImageAspectFlags::$aspect
            )|+,
            FormatAspectBits {
                color: $c,
                depth: $d,
                stencil: $s,
                unused: $u,
            },
        )
    };
}

impl FormatAspectBits {
    pub fn total_bits(self) -> u32 {
        self.color as u32 + self.depth as u32 + self.stencil as u32 + self.unused as u32
    }
    pub fn total_bytes(self) -> u32 {
        let bits = self.total_bits();
        assert!(bits % 8 == 0, "Bits aren't a multiple of 8");
        bits / 8
    }
}
