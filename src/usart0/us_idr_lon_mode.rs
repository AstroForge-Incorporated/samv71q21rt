#[doc = "Register `US_IDR_LON_MODE` writer"]
pub type W = crate::W<UsIdrLonModeSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSFE` writer - LON Short Frame Error Interrupt Disable"]
pub type LsfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCRCE` writer - LON CRC Error Interrupt Disable"]
pub type LcrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - SPI Underrun Error Interrupt Disable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTXD` writer - LON Transmission Done Interrupt Disable"]
pub type LtxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCOL` writer - LON Collision Interrupt Disable"]
pub type LcolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFET` writer - LON Frame Early Termination Interrupt Disable"]
pub type LfetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LRXD` writer - LON Reception Done Interrupt Disable"]
pub type LrxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBLOVFE` writer - LON Backlog Overflow Error Interrupt Disable"]
pub type LblovfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<UsIdrLonModeSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<UsIdrLonModeSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<UsIdrLonModeSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - LON Short Frame Error Interrupt Disable"]
    #[inline(always)]
    pub fn lsfe(&mut self) -> LsfeW<UsIdrLonModeSpec> {
        LsfeW::new(self, 6)
    }
    #[doc = "Bit 7 - LON CRC Error Interrupt Disable"]
    #[inline(always)]
    pub fn lcrce(&mut self) -> LcrceW<UsIdrLonModeSpec> {
        LcrceW::new(self, 7)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<UsIdrLonModeSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UnreW<UsIdrLonModeSpec> {
        UnreW::new(self, 10)
    }
    #[doc = "Bit 24 - LON Transmission Done Interrupt Disable"]
    #[inline(always)]
    pub fn ltxd(&mut self) -> LtxdW<UsIdrLonModeSpec> {
        LtxdW::new(self, 24)
    }
    #[doc = "Bit 25 - LON Collision Interrupt Disable"]
    #[inline(always)]
    pub fn lcol(&mut self) -> LcolW<UsIdrLonModeSpec> {
        LcolW::new(self, 25)
    }
    #[doc = "Bit 26 - LON Frame Early Termination Interrupt Disable"]
    #[inline(always)]
    pub fn lfet(&mut self) -> LfetW<UsIdrLonModeSpec> {
        LfetW::new(self, 26)
    }
    #[doc = "Bit 27 - LON Reception Done Interrupt Disable"]
    #[inline(always)]
    pub fn lrxd(&mut self) -> LrxdW<UsIdrLonModeSpec> {
        LrxdW::new(self, 27)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error Interrupt Disable"]
    #[inline(always)]
    pub fn lblovfe(&mut self) -> LblovfeW<UsIdrLonModeSpec> {
        LblovfeW::new(self, 28)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idr_lon_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsIdrLonModeSpec;
impl crate::RegisterSpec for UsIdrLonModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_idr_lon_mode::W`](W) writer structure"]
impl crate::Writable for UsIdrLonModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_IDR_LON_MODE to value 0"]
impl crate::Resettable for UsIdrLonModeSpec {}
