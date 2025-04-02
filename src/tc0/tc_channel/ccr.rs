#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `CLKEN` writer - Counter Clock Enable Command"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIS` writer - Counter Clock Disable Command"]
pub type ClkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRG` writer - Software Trigger Command"]
pub type SwtrgW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Counter Clock Enable Command"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<CcrSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter Clock Disable Command"]
    #[inline(always)]
    pub fn clkdis(&mut self) -> ClkdisW<CcrSpec> {
        ClkdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Trigger Command"]
    #[inline(always)]
    pub fn swtrg(&mut self) -> SwtrgW<CcrSpec> {
        SwtrgW::new(self, 2)
    }
}
#[doc = "Channel Control Register (channel = 0)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {}
