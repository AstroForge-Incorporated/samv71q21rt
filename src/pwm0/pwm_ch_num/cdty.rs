#[doc = "Register `CDTY` reader"]
pub type R = crate::R<CdtySpec>;
#[doc = "Register `CDTY` writer"]
pub type W = crate::W<CdtySpec>;
#[doc = "Field `CDTY` reader - Channel Duty-Cycle"]
pub type CdtyR = crate::FieldReader<u32>;
#[doc = "Field `CDTY` writer - Channel Duty-Cycle"]
pub type CdtyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CdtyR {
        CdtyR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&mut self) -> CdtyW<CdtySpec> {
        CdtyW::new(self, 0)
    }
}
#[doc = "PWM Channel Duty Cycle Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdtySpec;
impl crate::RegisterSpec for CdtySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdty::R`](R) reader structure"]
impl crate::Readable for CdtySpec {}
#[doc = "`write(|w| ..)` method takes [`cdty::W`](W) writer structure"]
impl crate::Writable for CdtySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDTY to value 0"]
impl crate::Resettable for CdtySpec {}
