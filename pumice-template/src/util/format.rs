#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormatAspectBits {
    pub color: u16,
    pub depth: u16,
    pub stencil: u16,
    pub unused: u16,
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

// CODEGEN START
