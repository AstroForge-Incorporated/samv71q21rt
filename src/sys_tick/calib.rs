#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CalibSpec>;
#[doc = "Field `TENMS` reader - Reload value to use for 10ms timing"]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "Indicates whether the TENMS value is exact\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skewselect {
    #[doc = "0: 10ms calibration value is exact"]
    Value0 = 0,
    #[doc = "1: 10ms calibration value is inexact, because of the clock frequency"]
    Value1 = 1,
}
impl From<Skewselect> for bool {
    #[inline(always)]
    fn from(variant: Skewselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKEW` reader - Indicates whether the TENMS value is exact"]
pub type SkewR = crate::BitReader<Skewselect>;
impl SkewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Skewselect {
        match self.bits {
            false => Skewselect::Value0,
            true => Skewselect::Value1,
        }
    }
    #[doc = "10ms calibration value is exact"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Skewselect::Value0
    }
    #[doc = "10ms calibration value is inexact, because of the clock frequency"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Skewselect::Value1
    }
}
#[doc = "Indicates whether the device provides a reference clock to the processor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Norefselect {
    #[doc = "0: The reference clock is provided"]
    Value0 = 0,
    #[doc = "1: The reference clock is not provided"]
    Value1 = 1,
}
impl From<Norefselect> for bool {
    #[inline(always)]
    fn from(variant: Norefselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOREF` reader - Indicates whether the device provides a reference clock to the processor"]
pub type NorefR = crate::BitReader<Norefselect>;
impl NorefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Norefselect {
        match self.bits {
            false => Norefselect::Value0,
            true => Norefselect::Value1,
        }
    }
    #[doc = "The reference clock is provided"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Norefselect::Value0
    }
    #[doc = "The reference clock is not provided"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Norefselect::Value1
    }
}
impl R {
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TenmsR {
        TenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - Indicates whether the TENMS value is exact"]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates whether the device provides a reference clock to the processor"]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calib::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalibSpec;
impl crate::RegisterSpec for CalibSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CalibSpec {}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CalibSpec {}
