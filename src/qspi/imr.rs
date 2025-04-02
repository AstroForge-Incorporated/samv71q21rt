#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `RDRF` reader - Receive Data Register Full Interrupt Mask"]
pub type RdrfR = crate::BitReader;
#[doc = "Field `TDRE` reader - Transmit Data Register Empty Interrupt Mask"]
pub type TdreR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty Mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `OVRES` reader - Overrun Error Interrupt Mask"]
pub type OvresR = crate::BitReader;
#[doc = "Field `CSR` reader - Chip Select Rise Interrupt Mask"]
pub type CsrR = crate::BitReader;
#[doc = "Field `CSS` reader - Chip Select Status Interrupt Mask"]
pub type CssR = crate::BitReader;
#[doc = "Field `INSTRE` reader - Instruction End Interrupt Mask"]
pub type InstreR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Mask"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Registers Empty Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovres(&self) -> OvresR {
        OvresR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Chip Select Rise Interrupt Mask"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Chip Select Status Interrupt Mask"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Instruction End Interrupt Mask"]
    #[inline(always)]
    pub fn instre(&self) -> InstreR {
        InstreR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
