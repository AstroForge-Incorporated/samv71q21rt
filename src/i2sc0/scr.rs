#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `RXOR` writer - Receive Overrun Status Clear"]
pub type RxorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUR` writer - Transmit Underrun Status Clear"]
pub type TxurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORCH` writer - Receive Overrun Per Channel Status Clear"]
pub type RxorchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXURCH` writer - Transmit Underrun Per Channel Status Clear"]
pub type TxurchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bit 2 - Receive Overrun Status Clear"]
    #[inline(always)]
    pub fn rxor(&mut self) -> RxorW<ScrSpec> {
        RxorW::new(self, 2)
    }
    #[doc = "Bit 6 - Transmit Underrun Status Clear"]
    #[inline(always)]
    pub fn txur(&mut self) -> TxurW<ScrSpec> {
        TxurW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Receive Overrun Per Channel Status Clear"]
    #[inline(always)]
    pub fn rxorch(&mut self) -> RxorchW<ScrSpec> {
        RxorchW::new(self, 8)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Per Channel Status Clear"]
    #[inline(always)]
    pub fn txurch(&mut self) -> TxurchW<ScrSpec> {
        TxurchW::new(self, 20)
    }
}
#[doc = "Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}
