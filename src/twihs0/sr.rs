#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TXCOMP` reader - Transmission Completed (cleared by writing TWIHS_THR)"]
pub type TxcompR = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receive Holding Register Ready (cleared by reading TWIHS_RHR)"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Holding Register Ready (cleared by writing TWIHS_THR)"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `SVREAD` reader - Slave Read"]
pub type SvreadR = crate::BitReader;
#[doc = "Field `SVACC` reader - Slave Access"]
pub type SvaccR = crate::BitReader;
#[doc = "Field `GACC` reader - General Call Access (cleared on read)"]
pub type GaccR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error (cleared on read)"]
pub type OvreR = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun Error (cleared on read)"]
pub type UnreR = crate::BitReader;
#[doc = "Field `NACK` reader - Not Acknowledged (cleared on read)"]
pub type NackR = crate::BitReader;
#[doc = "Field `ARBLST` reader - Arbitration Lost (cleared on read)"]
pub type ArblstR = crate::BitReader;
#[doc = "Field `SCLWS` reader - Clock Wait State"]
pub type SclwsR = crate::BitReader;
#[doc = "Field `EOSACC` reader - End Of Slave Access (cleared on read)"]
pub type EosaccR = crate::BitReader;
#[doc = "Field `MCACK` reader - Master Code Acknowledge (cleared on read)"]
pub type McackR = crate::BitReader;
#[doc = "Field `TOUT` reader - Timeout Error (cleared on read)"]
pub type ToutR = crate::BitReader;
#[doc = "Field `PECERR` reader - PEC Error (cleared on read)"]
pub type PecerrR = crate::BitReader;
#[doc = "Field `SMBDAM` reader - SMBus Default Address Match (cleared on read)"]
pub type SmbdamR = crate::BitReader;
#[doc = "Field `SMBHHM` reader - SMBus Host Header Address Match (cleared on read)"]
pub type SmbhhmR = crate::BitReader;
#[doc = "Field `SCL` reader - SCL Line Value"]
pub type SclR = crate::BitReader;
#[doc = "Field `SDA` reader - SDA Line Value"]
pub type SdaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Completed (cleared by writing TWIHS_THR)"]
    #[inline(always)]
    pub fn txcomp(&self) -> TxcompR {
        TxcompR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready (cleared by reading TWIHS_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready (cleared by writing TWIHS_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Read"]
    #[inline(always)]
    pub fn svread(&self) -> SvreadR {
        SvreadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Access"]
    #[inline(always)]
    pub fn svacc(&self) -> SvaccR {
        SvaccR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call Access (cleared on read)"]
    #[inline(always)]
    pub fn gacc(&self) -> GaccR {
        GaccR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Error (cleared on read)"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Underrun Error (cleared on read)"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not Acknowledged (cleared on read)"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost (cleared on read)"]
    #[inline(always)]
    pub fn arblst(&self) -> ArblstR {
        ArblstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock Wait State"]
    #[inline(always)]
    pub fn sclws(&self) -> SclwsR {
        SclwsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End Of Slave Access (cleared on read)"]
    #[inline(always)]
    pub fn eosacc(&self) -> EosaccR {
        EosaccR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Code Acknowledge (cleared on read)"]
    #[inline(always)]
    pub fn mcack(&self) -> McackR {
        McackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Error (cleared on read)"]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PEC Error (cleared on read)"]
    #[inline(always)]
    pub fn pecerr(&self) -> PecerrR {
        PecerrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Default Address Match (cleared on read)"]
    #[inline(always)]
    pub fn smbdam(&self) -> SmbdamR {
        SmbdamR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match (cleared on read)"]
    #[inline(always)]
    pub fn smbhhm(&self) -> SmbhhmR {
        SmbhhmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - SCL Line Value"]
    #[inline(always)]
    pub fn scl(&self) -> SclR {
        SclR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SDA Line Value"]
    #[inline(always)]
    pub fn sda(&self) -> SdaR {
        SdaR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
