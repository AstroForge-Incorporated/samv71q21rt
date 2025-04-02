#[doc = "Register `SCER` writer"]
pub type W = crate::W<ScerSpec>;
#[doc = "Field `USBCLK` writer - Enable USB FS Clock"]
pub type UsbclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Enable"]
pub type Pck0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Enable"]
pub type Pck1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Enable"]
pub type Pck2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK3` writer - Programmable Clock 3 Output Enable"]
pub type Pck3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK4` writer - Programmable Clock 4 Output Enable"]
pub type Pck4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK5` writer - Programmable Clock 5 Output Enable"]
pub type Pck5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK6` writer - Programmable Clock 6 Output Enable"]
pub type Pck6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK7` writer - Programmable Clock 7 Output Enable"]
pub type Pck7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - Enable USB FS Clock"]
    #[inline(always)]
    pub fn usbclk(&mut self) -> UsbclkW<ScerSpec> {
        UsbclkW::new(self, 5)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Enable"]
    #[inline(always)]
    pub fn pck0(&mut self) -> Pck0W<ScerSpec> {
        Pck0W::new(self, 8)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Enable"]
    #[inline(always)]
    pub fn pck1(&mut self) -> Pck1W<ScerSpec> {
        Pck1W::new(self, 9)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Enable"]
    #[inline(always)]
    pub fn pck2(&mut self) -> Pck2W<ScerSpec> {
        Pck2W::new(self, 10)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Enable"]
    #[inline(always)]
    pub fn pck3(&mut self) -> Pck3W<ScerSpec> {
        Pck3W::new(self, 11)
    }
    #[doc = "Bit 12 - Programmable Clock 4 Output Enable"]
    #[inline(always)]
    pub fn pck4(&mut self) -> Pck4W<ScerSpec> {
        Pck4W::new(self, 12)
    }
    #[doc = "Bit 13 - Programmable Clock 5 Output Enable"]
    #[inline(always)]
    pub fn pck5(&mut self) -> Pck5W<ScerSpec> {
        Pck5W::new(self, 13)
    }
    #[doc = "Bit 14 - Programmable Clock 6 Output Enable"]
    #[inline(always)]
    pub fn pck6(&mut self) -> Pck6W<ScerSpec> {
        Pck6W::new(self, 14)
    }
    #[doc = "Bit 15 - Programmable Clock 7 Output Enable"]
    #[inline(always)]
    pub fn pck7(&mut self) -> Pck7W<ScerSpec> {
        Pck7W::new(self, 15)
    }
}
#[doc = "System Clock Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScerSpec;
impl crate::RegisterSpec for ScerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scer::W`](W) writer structure"]
impl crate::Writable for ScerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCER to value 0"]
impl crate::Resettable for ScerSpec {}
