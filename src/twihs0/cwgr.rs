#[doc = "Register `CWGR` reader"]
pub type R = crate::R<CwgrSpec>;
#[doc = "Register `CWGR` writer"]
pub type W = crate::W<CwgrSpec>;
#[doc = "Field `CLDIV` reader - Clock Low Divider"]
pub type CldivR = crate::FieldReader;
#[doc = "Field `CLDIV` writer - Clock Low Divider"]
pub type CldivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHDIV` reader - Clock High Divider"]
pub type ChdivR = crate::FieldReader;
#[doc = "Field `CHDIV` writer - Clock High Divider"]
pub type ChdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKDIV` reader - Clock Divider"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - Clock Divider"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HOLD` reader - TWD Hold Time Versus TWCK Falling"]
pub type HoldR = crate::FieldReader;
#[doc = "Field `HOLD` writer - TWD Hold Time Versus TWCK Falling"]
pub type HoldW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    pub fn cldiv(&self) -> CldivR {
        CldivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    pub fn chdiv(&self) -> ChdivR {
        ChdivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:29 - TWD Hold Time Versus TWCK Falling"]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    pub fn cldiv(&mut self) -> CldivW<CwgrSpec> {
        CldivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    pub fn chdiv(&mut self) -> ChdivW<CwgrSpec> {
        ChdivW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<CwgrSpec> {
        CkdivW::new(self, 16)
    }
    #[doc = "Bits 24:29 - TWD Hold Time Versus TWCK Falling"]
    #[inline(always)]
    pub fn hold(&mut self) -> HoldW<CwgrSpec> {
        HoldW::new(self, 24)
    }
}
#[doc = "Clock Waveform Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwgrSpec;
impl crate::RegisterSpec for CwgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwgr::R`](R) reader structure"]
impl crate::Readable for CwgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cwgr::W`](W) writer structure"]
impl crate::Writable for CwgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CWGR to value 0"]
impl crate::Resettable for CwgrSpec {}
