#[doc = "Register `CSUS` reader"]
pub type R = crate::R<CsusSpec>;
#[doc = "Register `CSUS` writer"]
pub type W = crate::W<CsusSpec>;
#[doc = "Field `SUBS` reader - Channel x Source Microblock Stride"]
pub type SubsR = crate::FieldReader<u32>;
#[doc = "Field `SUBS` writer - Channel x Source Microblock Stride"]
pub type SubsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel x Source Microblock Stride"]
    #[inline(always)]
    pub fn subs(&self) -> SubsR {
        SubsR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Source Microblock Stride"]
    #[inline(always)]
    pub fn subs(&mut self) -> SubsW<CsusSpec> {
        SubsW::new(self, 0)
    }
}
#[doc = "Channel Source Microblock Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`csus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsusSpec;
impl crate::RegisterSpec for CsusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csus::R`](R) reader structure"]
impl crate::Readable for CsusSpec {}
#[doc = "`write(|w| ..)` method takes [`csus::W`](W) writer structure"]
impl crate::Writable for CsusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSUS to value 0"]
impl crate::Resettable for CsusSpec {}
