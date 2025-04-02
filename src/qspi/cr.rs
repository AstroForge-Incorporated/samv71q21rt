#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `QSPIEN` writer - QSPI Enable"]
pub type QspienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPIDIS` writer - QSPI Disable"]
pub type QspidisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - QSPI Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LastxferW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QspienW<CrSpec> {
        QspienW::new(self, 0)
    }
    #[doc = "Bit 1 - QSPI Disable"]
    #[inline(always)]
    pub fn qspidis(&mut self) -> QspidisW<CrSpec> {
        QspidisW::new(self, 1)
    }
    #[doc = "Bit 7 - QSPI Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<CrSpec> {
        SwrstW::new(self, 7)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    pub fn lastxfer(&mut self) -> LastxferW<CrSpec> {
        LastxferW::new(self, 24)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
