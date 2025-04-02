#[doc = "Register `HSTIFR` writer"]
pub type W = crate::W<HstifrSpec>;
#[doc = "Field `DCONNIS` writer - Device Connection Interrupt Set"]
pub type DconnisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISCIS` writer - Device Disconnection Interrupt Set"]
pub type DdiscisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIS` writer - USB Reset Sent Interrupt Set"]
pub type RstisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMEDIS` writer - Downstream Resume Sent Interrupt Set"]
pub type RsmedisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSMIS` writer - Upstream Resume Received Interrupt Set"]
pub type RxrsmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSOFIS` writer - Host Start of Frame Interrupt Set"]
pub type HsofisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWUPIS` writer - Host Wake-Up Interrupt Set"]
pub type HwupisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_1` writer - DMA Channel 0 Interrupt Set"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_2` writer - DMA Channel 1 Interrupt Set"]
pub type Dma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_3` writer - DMA Channel 2 Interrupt Set"]
pub type Dma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_4` writer - DMA Channel 3 Interrupt Set"]
pub type Dma4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_5` writer - DMA Channel 4 Interrupt Set"]
pub type Dma5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_6` writer - DMA Channel 5 Interrupt Set"]
pub type Dma6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_7` writer - DMA Channel 6 Interrupt Set"]
pub type Dma7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Set"]
    #[inline(always)]
    pub fn dconnis(&mut self) -> DconnisW<HstifrSpec> {
        DconnisW::new(self, 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Set"]
    #[inline(always)]
    pub fn ddiscis(&mut self) -> DdiscisW<HstifrSpec> {
        DdiscisW::new(self, 1)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Set"]
    #[inline(always)]
    pub fn rstis(&mut self) -> RstisW<HstifrSpec> {
        RstisW::new(self, 2)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Set"]
    #[inline(always)]
    pub fn rsmedis(&mut self) -> RsmedisW<HstifrSpec> {
        RsmedisW::new(self, 3)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Set"]
    #[inline(always)]
    pub fn rxrsmis(&mut self) -> RxrsmisW<HstifrSpec> {
        RxrsmisW::new(self, 4)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Set"]
    #[inline(always)]
    pub fn hsofis(&mut self) -> HsofisW<HstifrSpec> {
        HsofisW::new(self, 5)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Set"]
    #[inline(always)]
    pub fn hwupis(&mut self) -> HwupisW<HstifrSpec> {
        HwupisW::new(self, 6)
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt Set"]
    #[inline(always)]
    pub fn dma_1(&mut self) -> Dma1W<HstifrSpec> {
        Dma1W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt Set"]
    #[inline(always)]
    pub fn dma_2(&mut self) -> Dma2W<HstifrSpec> {
        Dma2W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt Set"]
    #[inline(always)]
    pub fn dma_3(&mut self) -> Dma3W<HstifrSpec> {
        Dma3W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt Set"]
    #[inline(always)]
    pub fn dma_4(&mut self) -> Dma4W<HstifrSpec> {
        Dma4W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt Set"]
    #[inline(always)]
    pub fn dma_5(&mut self) -> Dma5W<HstifrSpec> {
        Dma5W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt Set"]
    #[inline(always)]
    pub fn dma_6(&mut self) -> Dma6W<HstifrSpec> {
        Dma6W::new(self, 30)
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt Set"]
    #[inline(always)]
    pub fn dma_7(&mut self) -> Dma7W<HstifrSpec> {
        Dma7W::new(self, 31)
    }
}
#[doc = "Host Global Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstifr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstifrSpec;
impl crate::RegisterSpec for HstifrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstifr::W`](W) writer structure"]
impl crate::Writable for HstifrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTIFR to value 0"]
impl crate::Resettable for HstifrSpec {}
