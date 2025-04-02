#[doc = "Register `IERPQ[%s]` writer"]
pub type W = crate::W<IerpqSpec>;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUBR` writer - RX Used Bit Read"]
pub type RxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded or Late Collision"]
pub type RlexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFC` writer - Transmit Frame Corruption Due to AHB Error"]
pub type TfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type RovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HrespW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&mut self) -> RcompW<IerpqSpec> {
        RcompW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&mut self) -> RxubrW<IerpqSpec> {
        RxubrW::new(self, 2)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    pub fn rlex(&mut self) -> RlexW<IerpqSpec> {
        RlexW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&mut self) -> TfcW<IerpqSpec> {
        TfcW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&mut self) -> TcompW<IerpqSpec> {
        TcompW::new(self, 7)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&mut self) -> RovrW<IerpqSpec> {
        RovrW::new(self, 10)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&mut self) -> HrespW<IerpqSpec> {
        HrespW::new(self, 11)
    }
}
#[doc = "Interrupt Enable Register Priority Queue (1..5)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ierpq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerpqSpec;
impl crate::RegisterSpec for IerpqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ierpq::W`](W) writer structure"]
impl crate::Writable for IerpqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IERPQ[%s] to value 0"]
impl crate::Resettable for IerpqSpec {}
