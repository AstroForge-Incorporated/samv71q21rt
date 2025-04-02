#[doc = "Register `SCSR` reader"]
pub type R = crate::R<ScsrSpec>;
#[doc = "Field `HCLKS` reader - HCLK Status"]
pub type HclksR = crate::BitReader;
#[doc = "Field `USBCLK` reader - USB FS Clock Status"]
pub type UsbclkR = crate::BitReader;
#[doc = "Field `PCK0` reader - Programmable Clock 0 Output Status"]
pub type Pck0R = crate::BitReader;
#[doc = "Field `PCK1` reader - Programmable Clock 1 Output Status"]
pub type Pck1R = crate::BitReader;
#[doc = "Field `PCK2` reader - Programmable Clock 2 Output Status"]
pub type Pck2R = crate::BitReader;
#[doc = "Field `PCK3` reader - Programmable Clock 3 Output Status"]
pub type Pck3R = crate::BitReader;
#[doc = "Field `PCK4` reader - Programmable Clock 4 Output Status"]
pub type Pck4R = crate::BitReader;
#[doc = "Field `PCK5` reader - Programmable Clock 5 Output Status"]
pub type Pck5R = crate::BitReader;
#[doc = "Field `PCK6` reader - Programmable Clock 6 Output Status"]
pub type Pck6R = crate::BitReader;
#[doc = "Field `PCK7` reader - Programmable Clock 7 Output Status"]
pub type Pck7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HCLK Status"]
    #[inline(always)]
    pub fn hclks(&self) -> HclksR {
        HclksR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - USB FS Clock Status"]
    #[inline(always)]
    pub fn usbclk(&self) -> UsbclkR {
        UsbclkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Status"]
    #[inline(always)]
    pub fn pck0(&self) -> Pck0R {
        Pck0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Status"]
    #[inline(always)]
    pub fn pck1(&self) -> Pck1R {
        Pck1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Status"]
    #[inline(always)]
    pub fn pck2(&self) -> Pck2R {
        Pck2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Status"]
    #[inline(always)]
    pub fn pck3(&self) -> Pck3R {
        Pck3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Programmable Clock 4 Output Status"]
    #[inline(always)]
    pub fn pck4(&self) -> Pck4R {
        Pck4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Programmable Clock 5 Output Status"]
    #[inline(always)]
    pub fn pck5(&self) -> Pck5R {
        Pck5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Programmable Clock 6 Output Status"]
    #[inline(always)]
    pub fn pck6(&self) -> Pck6R {
        Pck6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Programmable Clock 7 Output Status"]
    #[inline(always)]
    pub fn pck7(&self) -> Pck7R {
        Pck7R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "System Clock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScsrSpec;
impl crate::RegisterSpec for ScsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scsr::R`](R) reader structure"]
impl crate::Readable for ScsrSpec {}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for ScsrSpec {}
