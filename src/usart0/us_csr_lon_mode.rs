#[doc = "Register `US_CSR_LON_MODE` reader"]
pub type R = crate::R<UsCsrLonModeSpec>;
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading US_RHR)"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready (cleared by writing US_THR)"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type OvreR = crate::BitReader;
#[doc = "Field `LSFE` reader - LON Short Frame Error"]
pub type LsfeR = crate::BitReader;
#[doc = "Field `LCRCE` reader - LON CRC Error"]
pub type LcrceR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty (cleared by writing US_THR)"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun Error"]
pub type UnreR = crate::BitReader;
#[doc = "Field `LTXD` reader - LON Transmission End Flag"]
pub type LtxdR = crate::BitReader;
#[doc = "Field `LCOL` reader - LON Collision Detected Flag"]
pub type LcolR = crate::BitReader;
#[doc = "Field `LFET` reader - LON Frame Early Termination"]
pub type LfetR = crate::BitReader;
#[doc = "Field `LRXD` reader - LON Reception End Flag"]
pub type LrxdR = crate::BitReader;
#[doc = "Field `LBLOVFE` reader - LON Backlog Overflow Error"]
pub type LblovfeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Ready (cleared by reading US_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LON Short Frame Error"]
    #[inline(always)]
    pub fn lsfe(&self) -> LsfeR {
        LsfeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LON CRC Error"]
    #[inline(always)]
    pub fn lcrce(&self) -> LcrceR {
        LcrceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 24 - LON Transmission End Flag"]
    #[inline(always)]
    pub fn ltxd(&self) -> LtxdR {
        LtxdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LON Collision Detected Flag"]
    #[inline(always)]
    pub fn lcol(&self) -> LcolR {
        LcolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LON Frame Early Termination"]
    #[inline(always)]
    pub fn lfet(&self) -> LfetR {
        LfetR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LON Reception End Flag"]
    #[inline(always)]
    pub fn lrxd(&self) -> LrxdR {
        LrxdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error"]
    #[inline(always)]
    pub fn lblovfe(&self) -> LblovfeR {
        LblovfeR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_csr_lon_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsCsrLonModeSpec;
impl crate::RegisterSpec for UsCsrLonModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_csr_lon_mode::R`](R) reader structure"]
impl crate::Readable for UsCsrLonModeSpec {}
#[doc = "`reset()` method sets US_CSR_LON_MODE to value 0"]
impl crate::Resettable for UsCsrLonModeSpec {}
