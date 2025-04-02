#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Enable"]
pub type RdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRE` writer - Transmit Data Register Empty Interrupt Enable"]
pub type TdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Enable"]
pub type OvresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSR` writer - Chip Select Rise Interrupt Enable"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSS` writer - Chip Select Status Interrupt Enable"]
pub type CssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTRE` writer - Instruction End Interrupt Enable"]
pub type InstreW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    pub fn rdrf(&mut self) -> RdrfW<IerSpec> {
        RdrfW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TdreW<IerSpec> {
        TdreW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmission Registers Empty Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<IerSpec> {
        TxemptyW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovres(&mut self) -> OvresW<IerSpec> {
        OvresW::new(self, 3)
    }
    #[doc = "Bit 8 - Chip Select Rise Interrupt Enable"]
    #[inline(always)]
    pub fn csr(&mut self) -> CsrW<IerSpec> {
        CsrW::new(self, 8)
    }
    #[doc = "Bit 9 - Chip Select Status Interrupt Enable"]
    #[inline(always)]
    pub fn css(&mut self) -> CssW<IerSpec> {
        CssW::new(self, 9)
    }
    #[doc = "Bit 10 - Instruction End Interrupt Enable"]
    #[inline(always)]
    pub fn instre(&mut self) -> InstreW<IerSpec> {
        InstreW::new(self, 10)
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
