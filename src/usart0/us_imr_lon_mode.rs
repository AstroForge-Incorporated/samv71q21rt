#[doc = "Register `US_IMR_LON_MODE` reader"]
pub type R = crate::R<UsImrLonModeSpec>;
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OvreR = crate::BitReader;
#[doc = "Field `LSFE` reader - LON Short Frame Error Interrupt Mask"]
pub type LsfeR = crate::BitReader;
#[doc = "Field `LCRCE` reader - LON CRC Error Interrupt Mask"]
pub type LcrceR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `UNRE` reader - SPI Underrun Error Interrupt Mask"]
pub type UnreR = crate::BitReader;
#[doc = "Field `LTXD` reader - LON Transmission Done Interrupt Mask"]
pub type LtxdR = crate::BitReader;
#[doc = "Field `LCOL` reader - LON Collision Interrupt Mask"]
pub type LcolR = crate::BitReader;
#[doc = "Field `LFET` reader - LON Frame Early Termination Interrupt Mask"]
pub type LfetR = crate::BitReader;
#[doc = "Field `LRXD` reader - LON Reception Done Interrupt Mask"]
pub type LrxdR = crate::BitReader;
#[doc = "Field `LBLOVFE` reader - LON Backlog Overflow Error Interrupt Mask"]
pub type LblovfeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LON Short Frame Error Interrupt Mask"]
    #[inline(always)]
    pub fn lsfe(&self) -> LsfeR {
        LsfeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LON CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn lcrce(&self) -> LcrceR {
        LcrceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 24 - LON Transmission Done Interrupt Mask"]
    #[inline(always)]
    pub fn ltxd(&self) -> LtxdR {
        LtxdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LON Collision Interrupt Mask"]
    #[inline(always)]
    pub fn lcol(&self) -> LcolR {
        LcolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LON Frame Early Termination Interrupt Mask"]
    #[inline(always)]
    pub fn lfet(&self) -> LfetR {
        LfetR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LON Reception Done Interrupt Mask"]
    #[inline(always)]
    pub fn lrxd(&self) -> LrxdR {
        LrxdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error Interrupt Mask"]
    #[inline(always)]
    pub fn lblovfe(&self) -> LblovfeR {
        LblovfeR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_imr_lon_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsImrLonModeSpec;
impl crate::RegisterSpec for UsImrLonModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_imr_lon_mode::R`](R) reader structure"]
impl crate::Readable for UsImrLonModeSpec {}
#[doc = "`reset()` method sets US_IMR_LON_MODE to value 0"]
impl crate::Resettable for UsImrLonModeSpec {}
