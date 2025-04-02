#[doc = "Register `US_IER_LON_MODE` writer"]
pub type W = crate::W<UsIerLonModeSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSFE` writer - LON Short Frame Error Interrupt Enable"]
pub type LsfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCRCE` writer - LON CRC Error Interrupt Enable"]
pub type LcrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Underrun Error Interrupt Enable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTXD` writer - LON Transmission Done Interrupt Enable"]
pub type LtxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCOL` writer - LON Collision Interrupt Enable"]
pub type LcolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFET` writer - LON Frame Early Termination Interrupt Enable"]
pub type LfetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LRXD` writer - LON Reception Done Interrupt Enable"]
pub type LrxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBLOVFE` writer - LON Backlog Overflow Error Interrupt Enable"]
pub type LblovfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<UsIerLonModeSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<UsIerLonModeSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<UsIerLonModeSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - LON Short Frame Error Interrupt Enable"]
    #[inline(always)]
    pub fn lsfe(&mut self) -> LsfeW<UsIerLonModeSpec> {
        LsfeW::new(self, 6)
    }
    #[doc = "Bit 7 - LON CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn lcrce(&mut self) -> LcrceW<UsIerLonModeSpec> {
        LcrceW::new(self, 7)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<UsIerLonModeSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UnreW<UsIerLonModeSpec> {
        UnreW::new(self, 10)
    }
    #[doc = "Bit 24 - LON Transmission Done Interrupt Enable"]
    #[inline(always)]
    pub fn ltxd(&mut self) -> LtxdW<UsIerLonModeSpec> {
        LtxdW::new(self, 24)
    }
    #[doc = "Bit 25 - LON Collision Interrupt Enable"]
    #[inline(always)]
    pub fn lcol(&mut self) -> LcolW<UsIerLonModeSpec> {
        LcolW::new(self, 25)
    }
    #[doc = "Bit 26 - LON Frame Early Termination Interrupt Enable"]
    #[inline(always)]
    pub fn lfet(&mut self) -> LfetW<UsIerLonModeSpec> {
        LfetW::new(self, 26)
    }
    #[doc = "Bit 27 - LON Reception Done Interrupt Enable"]
    #[inline(always)]
    pub fn lrxd(&mut self) -> LrxdW<UsIerLonModeSpec> {
        LrxdW::new(self, 27)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error Interrupt Enable"]
    #[inline(always)]
    pub fn lblovfe(&mut self) -> LblovfeW<UsIerLonModeSpec> {
        LblovfeW::new(self, 28)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ier_lon_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsIerLonModeSpec;
impl crate::RegisterSpec for UsIerLonModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_ier_lon_mode::W`](W) writer structure"]
impl crate::Writable for UsIerLonModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_IER_LON_MODE to value 0"]
impl crate::Resettable for UsIerLonModeSpec {}
