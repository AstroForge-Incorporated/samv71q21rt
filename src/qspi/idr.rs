#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Disable"]
pub type RdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRE` writer - Transmit Data Register Empty Interrupt Disable"]
pub type TdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Disable"]
pub type OvresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSR` writer - Chip Select Rise Interrupt Disable"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSS` writer - Chip Select Status Interrupt Disable"]
pub type CssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTRE` writer - Instruction End Interrupt Disable"]
pub type InstreW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Disable"]
    #[inline(always)]
    pub fn rdrf(&mut self) -> RdrfW<IdrSpec> {
        RdrfW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Disable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TdreW<IdrSpec> {
        TdreW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmission Registers Empty Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<IdrSpec> {
        TxemptyW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovres(&mut self) -> OvresW<IdrSpec> {
        OvresW::new(self, 3)
    }
    #[doc = "Bit 8 - Chip Select Rise Interrupt Disable"]
    #[inline(always)]
    pub fn csr(&mut self) -> CsrW<IdrSpec> {
        CsrW::new(self, 8)
    }
    #[doc = "Bit 9 - Chip Select Status Interrupt Disable"]
    #[inline(always)]
    pub fn css(&mut self) -> CssW<IdrSpec> {
        CssW::new(self, 9)
    }
    #[doc = "Bit 10 - Instruction End Interrupt Disable"]
    #[inline(always)]
    pub fn instre(&mut self) -> InstreW<IdrSpec> {
        InstreW::new(self, 10)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
