#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXRDY0` writer - Transmit Ready Interrupt Enable of channel 0"]
pub type Txrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY1` writer - Transmit Ready Interrupt Enable of channel 1"]
pub type Txrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Enable of channel 0"]
pub type Eoc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Enable of channel 1"]
pub type Eoc1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Enable of channel 0"]
    #[inline(always)]
    pub fn txrdy0(&mut self) -> Txrdy0W<IerSpec> {
        Txrdy0W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Enable of channel 1"]
    #[inline(always)]
    pub fn txrdy1(&mut self) -> Txrdy1W<IerSpec> {
        Txrdy1W::new(self, 1)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Enable of channel 0"]
    #[inline(always)]
    pub fn eoc0(&mut self) -> Eoc0W<IerSpec> {
        Eoc0W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable of channel 1"]
    #[inline(always)]
    pub fn eoc1(&mut self) -> Eoc1W<IerSpec> {
        Eoc1W::new(self, 5)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
