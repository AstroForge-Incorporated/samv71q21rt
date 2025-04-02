#[doc = "Register `CIDR` reader"]
pub type R = crate::R<CidrSpec>;
#[doc = "Field `VERSION` reader - Version of the Device"]
pub type VersionR = crate::FieldReader;
#[doc = "Embedded Processor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eprocselect {
    #[doc = "0: Cortex-M7"]
    Samx7 = 0,
    #[doc = "1: ARM946ES"]
    Arm946es = 1,
    #[doc = "2: ARM7TDMI"]
    Arm7tdmi = 2,
    #[doc = "3: Cortex-M3"]
    Cm3 = 3,
    #[doc = "4: ARM920T"]
    Arm920t = 4,
    #[doc = "5: ARM926EJS"]
    Arm926ejs = 5,
    #[doc = "6: Cortex-A5"]
    Ca5 = 6,
    #[doc = "7: Cortex-M4"]
    Cm4 = 7,
}
impl From<Eprocselect> for u8 {
    #[inline(always)]
    fn from(variant: Eprocselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eprocselect {
    type Ux = u8;
}
impl crate::IsEnum for Eprocselect {}
#[doc = "Field `EPROC` reader - Embedded Processor"]
pub type EprocR = crate::FieldReader<Eprocselect>;
impl EprocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eprocselect {
        match self.bits {
            0 => Eprocselect::Samx7,
            1 => Eprocselect::Arm946es,
            2 => Eprocselect::Arm7tdmi,
            3 => Eprocselect::Cm3,
            4 => Eprocselect::Arm920t,
            5 => Eprocselect::Arm926ejs,
            6 => Eprocselect::Ca5,
            7 => Eprocselect::Cm4,
            _ => unreachable!(),
        }
    }
    #[doc = "Cortex-M7"]
    #[inline(always)]
    pub fn is_samx7(&self) -> bool {
        *self == Eprocselect::Samx7
    }
    #[doc = "ARM946ES"]
    #[inline(always)]
    pub fn is_arm946es(&self) -> bool {
        *self == Eprocselect::Arm946es
    }
    #[doc = "ARM7TDMI"]
    #[inline(always)]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == Eprocselect::Arm7tdmi
    }
    #[doc = "Cortex-M3"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        *self == Eprocselect::Cm3
    }
    #[doc = "ARM920T"]
    #[inline(always)]
    pub fn is_arm920t(&self) -> bool {
        *self == Eprocselect::Arm920t
    }
    #[doc = "ARM926EJS"]
    #[inline(always)]
    pub fn is_arm926ejs(&self) -> bool {
        *self == Eprocselect::Arm926ejs
    }
    #[doc = "Cortex-A5"]
    #[inline(always)]
    pub fn is_ca5(&self) -> bool {
        *self == Eprocselect::Ca5
    }
    #[doc = "Cortex-M4"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == Eprocselect::Cm4
    }
}
#[doc = "Nonvolatile Program Memory Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nvpsizselect {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: 8 Kbytes"]
    _8k = 1,
    #[doc = "2: 16 Kbytes"]
    _16k = 2,
    #[doc = "3: 32 Kbytes"]
    _32k = 3,
    #[doc = "5: 64 Kbytes"]
    _64k = 5,
    #[doc = "7: 128 Kbytes"]
    _128k = 7,
    #[doc = "8: 160 Kbytes"]
    _160k = 8,
    #[doc = "9: 256 Kbytes"]
    _256k = 9,
    #[doc = "10: 512 Kbytes"]
    _512k = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024k = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048k = 14,
}
impl From<Nvpsizselect> for u8 {
    #[inline(always)]
    fn from(variant: Nvpsizselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nvpsizselect {
    type Ux = u8;
}
impl crate::IsEnum for Nvpsizselect {}
#[doc = "Field `NVPSIZ` reader - Nonvolatile Program Memory Size"]
pub type NvpsizR = crate::FieldReader<Nvpsizselect>;
impl NvpsizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nvpsizselect> {
        match self.bits {
            0 => Some(Nvpsizselect::None),
            1 => Some(Nvpsizselect::_8k),
            2 => Some(Nvpsizselect::_16k),
            3 => Some(Nvpsizselect::_32k),
            5 => Some(Nvpsizselect::_64k),
            7 => Some(Nvpsizselect::_128k),
            8 => Some(Nvpsizselect::_160k),
            9 => Some(Nvpsizselect::_256k),
            10 => Some(Nvpsizselect::_512k),
            12 => Some(Nvpsizselect::_1024k),
            14 => Some(Nvpsizselect::_2048k),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Nvpsizselect::None
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Nvpsizselect::_8k
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Nvpsizselect::_16k
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Nvpsizselect::_32k
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == Nvpsizselect::_64k
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == Nvpsizselect::_128k
    }
    #[doc = "160 Kbytes"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == Nvpsizselect::_160k
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == Nvpsizselect::_256k
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == Nvpsizselect::_512k
    }
    #[doc = "1024 Kbytes"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == Nvpsizselect::_1024k
    }
    #[doc = "2048 Kbytes"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == Nvpsizselect::_2048k
    }
}
#[doc = "Second Nonvolatile Program Memory Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nvpsiz2select {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: 8 Kbytes"]
    _8k = 1,
    #[doc = "2: 16 Kbytes"]
    _16k = 2,
    #[doc = "3: 32 Kbytes"]
    _32k = 3,
    #[doc = "5: 64 Kbytes"]
    _64k = 5,
    #[doc = "7: 128 Kbytes"]
    _128k = 7,
    #[doc = "9: 256 Kbytes"]
    _256k = 9,
    #[doc = "10: 512 Kbytes"]
    _512k = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024k = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048k = 14,
}
impl From<Nvpsiz2select> for u8 {
    #[inline(always)]
    fn from(variant: Nvpsiz2select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nvpsiz2select {
    type Ux = u8;
}
impl crate::IsEnum for Nvpsiz2select {}
#[doc = "Field `NVPSIZ2` reader - Second Nonvolatile Program Memory Size"]
pub type Nvpsiz2R = crate::FieldReader<Nvpsiz2select>;
impl Nvpsiz2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nvpsiz2select> {
        match self.bits {
            0 => Some(Nvpsiz2select::None),
            1 => Some(Nvpsiz2select::_8k),
            2 => Some(Nvpsiz2select::_16k),
            3 => Some(Nvpsiz2select::_32k),
            5 => Some(Nvpsiz2select::_64k),
            7 => Some(Nvpsiz2select::_128k),
            9 => Some(Nvpsiz2select::_256k),
            10 => Some(Nvpsiz2select::_512k),
            12 => Some(Nvpsiz2select::_1024k),
            14 => Some(Nvpsiz2select::_2048k),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Nvpsiz2select::None
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Nvpsiz2select::_8k
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Nvpsiz2select::_16k
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Nvpsiz2select::_32k
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == Nvpsiz2select::_64k
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == Nvpsiz2select::_128k
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == Nvpsiz2select::_256k
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == Nvpsiz2select::_512k
    }
    #[doc = "1024 Kbytes"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == Nvpsiz2select::_1024k
    }
    #[doc = "2048 Kbytes"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == Nvpsiz2select::_2048k
    }
}
#[doc = "Internal SRAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sramsizselect {
    #[doc = "0: 48 Kbytes"]
    _48k = 0,
    #[doc = "1: 192 Kbytes"]
    _192k = 1,
    #[doc = "2: 384 Kbytes"]
    _384k = 2,
    #[doc = "3: 6 Kbytes"]
    _6k = 3,
    #[doc = "4: 24 Kbytes"]
    _24k = 4,
    #[doc = "5: 4 Kbytes"]
    _4k = 5,
    #[doc = "6: 80 Kbytes"]
    _80k = 6,
    #[doc = "7: 160 Kbytes"]
    _160k = 7,
    #[doc = "8: 8 Kbytes"]
    _8k = 8,
    #[doc = "9: 16 Kbytes"]
    _16k = 9,
    #[doc = "10: 32 Kbytes"]
    _32k = 10,
    #[doc = "11: 64 Kbytes"]
    _64k = 11,
    #[doc = "12: 128 Kbytes"]
    _128k = 12,
    #[doc = "13: 256 Kbytes"]
    _256k = 13,
    #[doc = "14: 96 Kbytes"]
    _96k = 14,
    #[doc = "15: 512 Kbytes"]
    _512k = 15,
}
impl From<Sramsizselect> for u8 {
    #[inline(always)]
    fn from(variant: Sramsizselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sramsizselect {
    type Ux = u8;
}
impl crate::IsEnum for Sramsizselect {}
#[doc = "Field `SRAMSIZ` reader - Internal SRAM Size"]
pub type SramsizR = crate::FieldReader<Sramsizselect>;
impl SramsizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramsizselect {
        match self.bits {
            0 => Sramsizselect::_48k,
            1 => Sramsizselect::_192k,
            2 => Sramsizselect::_384k,
            3 => Sramsizselect::_6k,
            4 => Sramsizselect::_24k,
            5 => Sramsizselect::_4k,
            6 => Sramsizselect::_80k,
            7 => Sramsizselect::_160k,
            8 => Sramsizselect::_8k,
            9 => Sramsizselect::_16k,
            10 => Sramsizselect::_32k,
            11 => Sramsizselect::_64k,
            12 => Sramsizselect::_128k,
            13 => Sramsizselect::_256k,
            14 => Sramsizselect::_96k,
            15 => Sramsizselect::_512k,
            _ => unreachable!(),
        }
    }
    #[doc = "48 Kbytes"]
    #[inline(always)]
    pub fn is_48k(&self) -> bool {
        *self == Sramsizselect::_48k
    }
    #[doc = "192 Kbytes"]
    #[inline(always)]
    pub fn is_192k(&self) -> bool {
        *self == Sramsizselect::_192k
    }
    #[doc = "384 Kbytes"]
    #[inline(always)]
    pub fn is_384k(&self) -> bool {
        *self == Sramsizselect::_384k
    }
    #[doc = "6 Kbytes"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        *self == Sramsizselect::_6k
    }
    #[doc = "24 Kbytes"]
    #[inline(always)]
    pub fn is_24k(&self) -> bool {
        *self == Sramsizselect::_24k
    }
    #[doc = "4 Kbytes"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == Sramsizselect::_4k
    }
    #[doc = "80 Kbytes"]
    #[inline(always)]
    pub fn is_80k(&self) -> bool {
        *self == Sramsizselect::_80k
    }
    #[doc = "160 Kbytes"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == Sramsizselect::_160k
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Sramsizselect::_8k
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Sramsizselect::_16k
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Sramsizselect::_32k
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == Sramsizselect::_64k
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == Sramsizselect::_128k
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == Sramsizselect::_256k
    }
    #[doc = "96 Kbytes"]
    #[inline(always)]
    pub fn is_96k(&self) -> bool {
        *self == Sramsizselect::_96k
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == Sramsizselect::_512k
    }
}
#[doc = "Architecture Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Archselect {
    #[doc = "16: SAM E70"]
    Same70 = 16,
    #[doc = "17: SAM S70"]
    Sams70 = 17,
    #[doc = "18: SAM V71"]
    Samv71 = 18,
    #[doc = "19: SAM V70"]
    Samv70 = 19,
}
impl From<Archselect> for u8 {
    #[inline(always)]
    fn from(variant: Archselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Archselect {
    type Ux = u8;
}
impl crate::IsEnum for Archselect {}
#[doc = "Field `ARCH` reader - Architecture Identifier"]
pub type ArchR = crate::FieldReader<Archselect>;
impl ArchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Archselect> {
        match self.bits {
            16 => Some(Archselect::Same70),
            17 => Some(Archselect::Sams70),
            18 => Some(Archselect::Samv71),
            19 => Some(Archselect::Samv70),
            _ => None,
        }
    }
    #[doc = "SAM E70"]
    #[inline(always)]
    pub fn is_same70(&self) -> bool {
        *self == Archselect::Same70
    }
    #[doc = "SAM S70"]
    #[inline(always)]
    pub fn is_sams70(&self) -> bool {
        *self == Archselect::Sams70
    }
    #[doc = "SAM V71"]
    #[inline(always)]
    pub fn is_samv71(&self) -> bool {
        *self == Archselect::Samv71
    }
    #[doc = "SAM V70"]
    #[inline(always)]
    pub fn is_samv70(&self) -> bool {
        *self == Archselect::Samv70
    }
}
#[doc = "Nonvolatile Program Memory Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nvptypselect {
    #[doc = "0: ROM"]
    Rom = 0,
    #[doc = "1: ROMless or on-chip Flash"]
    Romless = 1,
    #[doc = "2: Embedded Flash Memory"]
    Flash = 2,
    #[doc = "3: ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    RomFlash = 3,
    #[doc = "4: SRAM emulating ROM"]
    Sram = 4,
}
impl From<Nvptypselect> for u8 {
    #[inline(always)]
    fn from(variant: Nvptypselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nvptypselect {
    type Ux = u8;
}
impl crate::IsEnum for Nvptypselect {}
#[doc = "Field `NVPTYP` reader - Nonvolatile Program Memory Type"]
pub type NvptypR = crate::FieldReader<Nvptypselect>;
impl NvptypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nvptypselect> {
        match self.bits {
            0 => Some(Nvptypselect::Rom),
            1 => Some(Nvptypselect::Romless),
            2 => Some(Nvptypselect::Flash),
            3 => Some(Nvptypselect::RomFlash),
            4 => Some(Nvptypselect::Sram),
            _ => None,
        }
    }
    #[doc = "ROM"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == Nvptypselect::Rom
    }
    #[doc = "ROMless or on-chip Flash"]
    #[inline(always)]
    pub fn is_romless(&self) -> bool {
        *self == Nvptypselect::Romless
    }
    #[doc = "Embedded Flash Memory"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == Nvptypselect::Flash
    }
    #[doc = "ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    #[inline(always)]
    pub fn is_rom_flash(&self) -> bool {
        *self == Nvptypselect::RomFlash
    }
    #[doc = "SRAM emulating ROM"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == Nvptypselect::Sram
    }
}
#[doc = "Field `EXT` reader - Extension Flag"]
pub type ExtR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EprocR {
        EprocR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz(&self) -> NvpsizR {
        NvpsizR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz2(&self) -> Nvpsiz2R {
        Nvpsiz2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SramsizR {
        SramsizR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ArchR {
        ArchR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NvptypR {
        NvptypR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Chip ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CidrSpec;
impl crate::RegisterSpec for CidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr::R`](R) reader structure"]
impl crate::Readable for CidrSpec {}
#[doc = "`reset()` method sets CIDR to value 0"]
impl crate::Resettable for CidrSpec {}
