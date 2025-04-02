#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ISI_EN` writer - ISI Module Enable Request"]
pub type IsiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISI_DIS` writer - ISI Module Disable Request"]
pub type IsiDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISI_SRST` writer - ISI Software Reset Request"]
pub type IsiSrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISI_CDC` writer - ISI Codec Request"]
pub type IsiCdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - ISI Module Enable Request"]
    #[inline(always)]
    pub fn isi_en(&mut self) -> IsiEnW<CrSpec> {
        IsiEnW::new(self, 0)
    }
    #[doc = "Bit 1 - ISI Module Disable Request"]
    #[inline(always)]
    pub fn isi_dis(&mut self) -> IsiDisW<CrSpec> {
        IsiDisW::new(self, 1)
    }
    #[doc = "Bit 2 - ISI Software Reset Request"]
    #[inline(always)]
    pub fn isi_srst(&mut self) -> IsiSrstW<CrSpec> {
        IsiSrstW::new(self, 2)
    }
    #[doc = "Bit 8 - ISI Codec Request"]
    #[inline(always)]
    pub fn isi_cdc(&mut self) -> IsiCdcW<CrSpec> {
        IsiCdcW::new(self, 8)
    }
}
#[doc = "ISI Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
