#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `RDERRI` reader - Remote Device Connection Error Interrupt (Host mode only)"]
pub type RderriR = crate::BitReader;
#[doc = "Speed Status (Device mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speedselect {
    #[doc = "0: Full-Speed mode"]
    FullSpeed = 0,
    #[doc = "1: High-Speed mode"]
    HighSpeed = 1,
    #[doc = "2: Low-Speed mode"]
    LowSpeed = 2,
}
impl From<Speedselect> for u8 {
    #[inline(always)]
    fn from(variant: Speedselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speedselect {
    type Ux = u8;
}
impl crate::IsEnum for Speedselect {}
#[doc = "Field `SPEED` reader - Speed Status (Device mode only)"]
pub type SpeedR = crate::FieldReader<Speedselect>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speedselect> {
        match self.bits {
            0 => Some(Speedselect::FullSpeed),
            1 => Some(Speedselect::HighSpeed),
            2 => Some(Speedselect::LowSpeed),
            _ => None,
        }
    }
    #[doc = "Full-Speed mode"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        *self == Speedselect::FullSpeed
    }
    #[doc = "High-Speed mode"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == Speedselect::HighSpeed
    }
    #[doc = "Low-Speed mode"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == Speedselect::LowSpeed
    }
}
#[doc = "Field `CLKUSABLE` reader - UTMI Clock Usable"]
pub type ClkusableR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt (Host mode only)"]
    #[inline(always)]
    pub fn rderri(&self) -> RderriR {
        RderriR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status (Device mode only)"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> ClkusableR {
        ClkusableR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "General Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
