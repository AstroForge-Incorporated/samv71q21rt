#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `EOC0` reader - End of Conversion Interrupt Mask 0"]
pub type Eoc0R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of Conversion Interrupt Mask 1"]
pub type Eoc1R = crate::BitReader;
#[doc = "Field `EOC2` reader - End of Conversion Interrupt Mask 2"]
pub type Eoc2R = crate::BitReader;
#[doc = "Field `EOC3` reader - End of Conversion Interrupt Mask 3"]
pub type Eoc3R = crate::BitReader;
#[doc = "Field `EOC4` reader - End of Conversion Interrupt Mask 4"]
pub type Eoc4R = crate::BitReader;
#[doc = "Field `EOC5` reader - End of Conversion Interrupt Mask 5"]
pub type Eoc5R = crate::BitReader;
#[doc = "Field `EOC6` reader - End of Conversion Interrupt Mask 6"]
pub type Eoc6R = crate::BitReader;
#[doc = "Field `EOC7` reader - End of Conversion Interrupt Mask 7"]
pub type Eoc7R = crate::BitReader;
#[doc = "Field `EOC8` reader - End of Conversion Interrupt Mask 8"]
pub type Eoc8R = crate::BitReader;
#[doc = "Field `EOC9` reader - End of Conversion Interrupt Mask 9"]
pub type Eoc9R = crate::BitReader;
#[doc = "Field `EOC10` reader - End of Conversion Interrupt Mask 10"]
pub type Eoc10R = crate::BitReader;
#[doc = "Field `EOC11` reader - End of Conversion Interrupt Mask 11"]
pub type Eoc11R = crate::BitReader;
#[doc = "Field `DRDY` reader - Data Ready Interrupt Mask"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `GOVRE` reader - General Overrun Error Interrupt Mask"]
pub type GovreR = crate::BitReader;
#[doc = "Field `COMPE` reader - Comparison Event Interrupt Mask"]
pub type CompeR = crate::BitReader;
#[doc = "Field `TEMPCHG` reader - Temperature Change Interrupt Mask"]
pub type TempchgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Conversion Interrupt Mask 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> Eoc0R {
        Eoc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Mask 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> Eoc1R {
        Eoc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Mask 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> Eoc2R {
        Eoc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Mask 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> Eoc3R {
        Eoc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Mask 4"]
    #[inline(always)]
    pub fn eoc4(&self) -> Eoc4R {
        Eoc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Mask 5"]
    #[inline(always)]
    pub fn eoc5(&self) -> Eoc5R {
        Eoc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Mask 6"]
    #[inline(always)]
    pub fn eoc6(&self) -> Eoc6R {
        Eoc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Mask 7"]
    #[inline(always)]
    pub fn eoc7(&self) -> Eoc7R {
        Eoc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of Conversion Interrupt Mask 8"]
    #[inline(always)]
    pub fn eoc8(&self) -> Eoc8R {
        Eoc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - End of Conversion Interrupt Mask 9"]
    #[inline(always)]
    pub fn eoc9(&self) -> Eoc9R {
        Eoc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - End of Conversion Interrupt Mask 10"]
    #[inline(always)]
    pub fn eoc10(&self) -> Eoc10R {
        Eoc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Conversion Interrupt Mask 11"]
    #[inline(always)]
    pub fn eoc11(&self) -> Eoc11R {
        Eoc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn govre(&self) -> GovreR {
        GovreR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Comparison Event Interrupt Mask"]
    #[inline(always)]
    pub fn compe(&self) -> CompeR {
        CompeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature Change Interrupt Mask"]
    #[inline(always)]
    pub fn tempchg(&self) -> TempchgR {
        TempchgR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "AFEC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
