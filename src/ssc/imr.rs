#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `TXRDY` reader - Transmit Ready Interrupt Mask"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmit Empty Interrupt Mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receive Ready Interrupt Mask"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `OVRUN` reader - Receive Overrun Interrupt Mask"]
pub type OvrunR = crate::BitReader;
#[doc = "Field `CP0` reader - Compare 0 Interrupt Mask"]
pub type Cp0R = crate::BitReader;
#[doc = "Field `CP1` reader - Compare 1 Interrupt Mask"]
pub type Cp1R = crate::BitReader;
#[doc = "Field `TXSYN` reader - Tx Sync Interrupt Mask"]
pub type TxsynR = crate::BitReader;
#[doc = "Field `RXSYN` reader - Rx Sync Interrupt Mask"]
pub type RxsynR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ovrun(&self) -> OvrunR {
        OvrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Mask"]
    #[inline(always)]
    pub fn cp0(&self) -> Cp0R {
        Cp0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Mask"]
    #[inline(always)]
    pub fn cp1(&self) -> Cp1R {
        Cp1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Sync Interrupt Mask"]
    #[inline(always)]
    pub fn txsyn(&self) -> TxsynR {
        TxsynR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx Sync Interrupt Mask"]
    #[inline(always)]
    pub fn rxsyn(&self) -> RxsynR {
        RxsynR::new(((self.bits >> 11) & 1) != 0)
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
