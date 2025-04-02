#[doc = "Register `HSTPIPICR_BLK_MODE[%s]` writer"]
pub type W = crate::W<HstpipicrBlkModeSpec>;
#[doc = "Field `RXINIC` writer - Received IN Data Interrupt Clear"]
pub type RxinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIC` writer - Transmitted OUT Data Interrupt Clear"]
pub type TxouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTPIC` writer - Transmitted SETUP Interrupt Clear"]
pub type TxstpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIC` writer - NAKed Interrupt Clear"]
pub type NakedicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OverficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDIC` writer - Received STALLed Interrupt Clear"]
pub type RxstalldicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIC` writer - Short Packet Interrupt Clear"]
pub type ShortpacketicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxinic(&mut self) -> RxinicW<HstpipicrBlkModeSpec> {
        RxinicW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn txoutic(&mut self) -> TxouticW<HstpipicrBlkModeSpec> {
        TxouticW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Clear"]
    #[inline(always)]
    pub fn txstpic(&mut self) -> TxstpicW<HstpipicrBlkModeSpec> {
        TxstpicW::new(self, 2)
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    pub fn nakedic(&mut self) -> NakedicW<HstpipicrBlkModeSpec> {
        NakedicW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OverficW<HstpipicrBlkModeSpec> {
        OverficW::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Clear"]
    #[inline(always)]
    pub fn rxstalldic(&mut self) -> RxstalldicW<HstpipicrBlkModeSpec> {
        RxstalldicW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketic(&mut self) -> ShortpacketicW<HstpipicrBlkModeSpec> {
        ShortpacketicW::new(self, 7)
    }
}
#[doc = "Host Pipe Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipicr_blk_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpipicrBlkModeSpec;
impl crate::RegisterSpec for HstpipicrBlkModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipicr_blk_mode::W`](W) writer structure"]
impl crate::Writable for HstpipicrBlkModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTPIPICR_BLK_MODE[%s] to value 0"]
impl crate::Resettable for HstpipicrBlkModeSpec {}
