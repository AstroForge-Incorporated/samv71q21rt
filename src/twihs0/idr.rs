#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `TXCOMP` writer - Transmission Completed Interrupt Disable"]
pub type TxcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receive Holding Register Ready Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Holding Register Ready Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVACC` writer - Slave Access Interrupt Disable"]
pub type SvaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GACC` writer - General Call Access Interrupt Disable"]
pub type GaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Disable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Underrun Error Interrupt Disable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Not Acknowledge Interrupt Disable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLST` writer - Arbitration Lost Interrupt Disable"]
pub type ArblstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_WS` writer - Clock Wait State Interrupt Disable"]
pub type SclWsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSACC` writer - End Of Slave Access Interrupt Disable"]
pub type EosaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCACK` writer - Master Code Acknowledge Interrupt Disable"]
pub type McackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUT` writer - Timeout Error Interrupt Disable"]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERR` writer - PEC Error Interrupt Disable"]
pub type PecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDAM` writer - SMBus Default Address Match Interrupt Disable"]
pub type SmbdamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHHM` writer - SMBus Host Header Address Match Interrupt Disable"]
pub type SmbhhmW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission Completed Interrupt Disable"]
    #[inline(always)]
    pub fn txcomp(&mut self) -> TxcompW<IdrSpec> {
        TxcompW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<IdrSpec> {
        RxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<IdrSpec> {
        TxrdyW::new(self, 2)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Disable"]
    #[inline(always)]
    pub fn svacc(&mut self) -> SvaccW<IdrSpec> {
        SvaccW::new(self, 4)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Disable"]
    #[inline(always)]
    pub fn gacc(&mut self) -> GaccW<IdrSpec> {
        GaccW::new(self, 5)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<IdrSpec> {
        OvreW::new(self, 6)
    }
    #[doc = "Bit 7 - Underrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UnreW<IdrSpec> {
        UnreW::new(self, 7)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Disable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<IdrSpec> {
        NackW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Disable"]
    #[inline(always)]
    pub fn arblst(&mut self) -> ArblstW<IdrSpec> {
        ArblstW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Disable"]
    #[inline(always)]
    pub fn scl_ws(&mut self) -> SclWsW<IdrSpec> {
        SclWsW::new(self, 10)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Disable"]
    #[inline(always)]
    pub fn eosacc(&mut self) -> EosaccW<IdrSpec> {
        EosaccW::new(self, 11)
    }
    #[doc = "Bit 16 - Master Code Acknowledge Interrupt Disable"]
    #[inline(always)]
    pub fn mcack(&mut self) -> McackW<IdrSpec> {
        McackW::new(self, 16)
    }
    #[doc = "Bit 18 - Timeout Error Interrupt Disable"]
    #[inline(always)]
    pub fn tout(&mut self) -> ToutW<IdrSpec> {
        ToutW::new(self, 18)
    }
    #[doc = "Bit 19 - PEC Error Interrupt Disable"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PecerrW<IdrSpec> {
        PecerrW::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus Default Address Match Interrupt Disable"]
    #[inline(always)]
    pub fn smbdam(&mut self) -> SmbdamW<IdrSpec> {
        SmbdamW::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match Interrupt Disable"]
    #[inline(always)]
    pub fn smbhhm(&mut self) -> SmbhhmW<IdrSpec> {
        SmbhhmW::new(self, 21)
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
