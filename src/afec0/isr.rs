#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `EOC0` reader - End of Conversion 0 (cleared by reading AFEC_CDRx)"]
pub type Eoc0R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of Conversion 1 (cleared by reading AFEC_CDRx)"]
pub type Eoc1R = crate::BitReader;
#[doc = "Field `EOC2` reader - End of Conversion 2 (cleared by reading AFEC_CDRx)"]
pub type Eoc2R = crate::BitReader;
#[doc = "Field `EOC3` reader - End of Conversion 3 (cleared by reading AFEC_CDRx)"]
pub type Eoc3R = crate::BitReader;
#[doc = "Field `EOC4` reader - End of Conversion 4 (cleared by reading AFEC_CDRx)"]
pub type Eoc4R = crate::BitReader;
#[doc = "Field `EOC5` reader - End of Conversion 5 (cleared by reading AFEC_CDRx)"]
pub type Eoc5R = crate::BitReader;
#[doc = "Field `EOC6` reader - End of Conversion 6 (cleared by reading AFEC_CDRx)"]
pub type Eoc6R = crate::BitReader;
#[doc = "Field `EOC7` reader - End of Conversion 7 (cleared by reading AFEC_CDRx)"]
pub type Eoc7R = crate::BitReader;
#[doc = "Field `EOC8` reader - End of Conversion 8 (cleared by reading AFEC_CDRx)"]
pub type Eoc8R = crate::BitReader;
#[doc = "Field `EOC9` reader - End of Conversion 9 (cleared by reading AFEC_CDRx)"]
pub type Eoc9R = crate::BitReader;
#[doc = "Field `EOC10` reader - End of Conversion 10 (cleared by reading AFEC_CDRx)"]
pub type Eoc10R = crate::BitReader;
#[doc = "Field `EOC11` reader - End of Conversion 11 (cleared by reading AFEC_CDRx)"]
pub type Eoc11R = crate::BitReader;
#[doc = "Field `DRDY` reader - Data Ready (cleared by reading AFEC_LCDR)"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `GOVRE` reader - General Overrun Error (cleared by reading AFEC_ISR)"]
pub type GovreR = crate::BitReader;
#[doc = "Field `COMPE` reader - Comparison Error (cleared by reading AFEC_ISR)"]
pub type CompeR = crate::BitReader;
#[doc = "Field `TEMPCHG` reader - Temperature Change (cleared on read)"]
pub type TempchgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Conversion 0 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc0(&self) -> Eoc0R {
        Eoc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Conversion 1 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc1(&self) -> Eoc1R {
        Eoc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Conversion 2 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc2(&self) -> Eoc2R {
        Eoc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Conversion 3 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc3(&self) -> Eoc3R {
        Eoc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion 4 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc4(&self) -> Eoc4R {
        Eoc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion 5 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc5(&self) -> Eoc5R {
        Eoc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Conversion 6 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc6(&self) -> Eoc6R {
        Eoc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Conversion 7 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc7(&self) -> Eoc7R {
        Eoc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of Conversion 8 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc8(&self) -> Eoc8R {
        Eoc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - End of Conversion 9 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc9(&self) -> Eoc9R {
        Eoc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - End of Conversion 10 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc10(&self) -> Eoc10R {
        Eoc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Conversion 11 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc11(&self) -> Eoc11R {
        Eoc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - Data Ready (cleared by reading AFEC_LCDR)"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - General Overrun Error (cleared by reading AFEC_ISR)"]
    #[inline(always)]
    pub fn govre(&self) -> GovreR {
        GovreR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Comparison Error (cleared by reading AFEC_ISR)"]
    #[inline(always)]
    pub fn compe(&self) -> CompeR {
        CompeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature Change (cleared on read)"]
    #[inline(always)]
    pub fn tempchg(&self) -> TempchgR {
        TempchgR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "AFEC Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
