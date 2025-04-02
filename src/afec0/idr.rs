#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Disable 0"]
pub type Eoc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Disable 1"]
pub type Eoc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC2` writer - End of Conversion Interrupt Disable 2"]
pub type Eoc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC3` writer - End of Conversion Interrupt Disable 3"]
pub type Eoc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC4` writer - End of Conversion Interrupt Disable 4"]
pub type Eoc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC5` writer - End of Conversion Interrupt Disable 5"]
pub type Eoc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC6` writer - End of Conversion Interrupt Disable 6"]
pub type Eoc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC7` writer - End of Conversion Interrupt Disable 7"]
pub type Eoc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC8` writer - End of Conversion Interrupt Disable 8"]
pub type Eoc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC9` writer - End of Conversion Interrupt Disable 9"]
pub type Eoc9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC10` writer - End of Conversion Interrupt Disable 10"]
pub type Eoc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC11` writer - End of Conversion Interrupt Disable 11"]
pub type Eoc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Disable"]
pub type DrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Disable"]
pub type GovreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPE` writer - Comparison Event Interrupt Disable"]
pub type CompeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPCHG` writer - Temperature Change Interrupt Disable"]
pub type TempchgW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - End of Conversion Interrupt Disable 0"]
    #[inline(always)]
    pub fn eoc0(&mut self) -> Eoc0W<IdrSpec> {
        Eoc0W::new(self, 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Disable 1"]
    #[inline(always)]
    pub fn eoc1(&mut self) -> Eoc1W<IdrSpec> {
        Eoc1W::new(self, 1)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Disable 2"]
    #[inline(always)]
    pub fn eoc2(&mut self) -> Eoc2W<IdrSpec> {
        Eoc2W::new(self, 2)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Disable 3"]
    #[inline(always)]
    pub fn eoc3(&mut self) -> Eoc3W<IdrSpec> {
        Eoc3W::new(self, 3)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Disable 4"]
    #[inline(always)]
    pub fn eoc4(&mut self) -> Eoc4W<IdrSpec> {
        Eoc4W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Disable 5"]
    #[inline(always)]
    pub fn eoc5(&mut self) -> Eoc5W<IdrSpec> {
        Eoc5W::new(self, 5)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Disable 6"]
    #[inline(always)]
    pub fn eoc6(&mut self) -> Eoc6W<IdrSpec> {
        Eoc6W::new(self, 6)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Disable 7"]
    #[inline(always)]
    pub fn eoc7(&mut self) -> Eoc7W<IdrSpec> {
        Eoc7W::new(self, 7)
    }
    #[doc = "Bit 8 - End of Conversion Interrupt Disable 8"]
    #[inline(always)]
    pub fn eoc8(&mut self) -> Eoc8W<IdrSpec> {
        Eoc8W::new(self, 8)
    }
    #[doc = "Bit 9 - End of Conversion Interrupt Disable 9"]
    #[inline(always)]
    pub fn eoc9(&mut self) -> Eoc9W<IdrSpec> {
        Eoc9W::new(self, 9)
    }
    #[doc = "Bit 10 - End of Conversion Interrupt Disable 10"]
    #[inline(always)]
    pub fn eoc10(&mut self) -> Eoc10W<IdrSpec> {
        Eoc10W::new(self, 10)
    }
    #[doc = "Bit 11 - End of Conversion Interrupt Disable 11"]
    #[inline(always)]
    pub fn eoc11(&mut self) -> Eoc11W<IdrSpec> {
        Eoc11W::new(self, 11)
    }
    #[doc = "Bit 24 - Data Ready Interrupt Disable"]
    #[inline(always)]
    pub fn drdy(&mut self) -> DrdyW<IdrSpec> {
        DrdyW::new(self, 24)
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn govre(&mut self) -> GovreW<IdrSpec> {
        GovreW::new(self, 25)
    }
    #[doc = "Bit 26 - Comparison Event Interrupt Disable"]
    #[inline(always)]
    pub fn compe(&mut self) -> CompeW<IdrSpec> {
        CompeW::new(self, 26)
    }
    #[doc = "Bit 30 - Temperature Change Interrupt Disable"]
    #[inline(always)]
    pub fn tempchg(&mut self) -> TempchgW<IdrSpec> {
        TempchgW::new(self, 30)
    }
}
#[doc = "AFEC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
