#[doc = "Register `US_THR` writer"]
pub type W = crate::W<UsThrSpec>;
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub type TxchrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TXSYNH` writer - Sync Field to be Transmitted"]
pub type TxsynhW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr(&mut self) -> TxchrW<UsThrSpec> {
        TxchrW::new(self, 0)
    }
    #[doc = "Bit 15 - Sync Field to be Transmitted"]
    #[inline(always)]
    pub fn txsynh(&mut self) -> TxsynhW<UsThrSpec> {
        TxsynhW::new(self, 15)
    }
}
#[doc = "Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsThrSpec;
impl crate::RegisterSpec for UsThrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_thr::W`](W) writer structure"]
impl crate::Writable for UsThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_THR to value 0"]
impl crate::Resettable for UsThrSpec {}
