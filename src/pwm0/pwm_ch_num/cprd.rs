#[doc = "Register `CPRD` reader"]
pub type R = crate::R<CprdSpec>;
#[doc = "Register `CPRD` writer"]
pub type W = crate::W<CprdSpec>;
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CprdR = crate::FieldReader<u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CprdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CprdR {
        CprdR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&mut self) -> CprdW<CprdSpec> {
        CprdW::new(self, 0)
    }
}
#[doc = "PWM Channel Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cprd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cprd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprdSpec;
impl crate::RegisterSpec for CprdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cprd::R`](R) reader structure"]
impl crate::Readable for CprdSpec {}
#[doc = "`write(|w| ..)` method takes [`cprd::W`](W) writer structure"]
impl crate::Writable for CprdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPRD to value 0"]
impl crate::Resettable for CprdSpec {}
