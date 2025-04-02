#[doc = "Register `ACMR[%s]` reader"]
pub type R = crate::R<AcmrSpec>;
#[doc = "Register `ACMR[%s]` writer"]
pub type W = crate::W<AcmrSpec>;
#[doc = "Field `CHM` reader - Bitwise Channel Mask Bits 31 to 0"]
pub type ChmR = crate::FieldReader<u32>;
#[doc = "Field `CHM` writer - Bitwise Channel Mask Bits 31 to 0"]
pub type ChmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bits 31 to 0"]
    #[inline(always)]
    pub fn chm(&self) -> ChmR {
        ChmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bits 31 to 0"]
    #[inline(always)]
    pub fn chm(&mut self) -> ChmW<AcmrSpec> {
        ChmW::new(self, 0)
    }
}
#[doc = "AHB Channel Mask 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`acmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcmrSpec;
impl crate::RegisterSpec for AcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acmr::R`](R) reader structure"]
impl crate::Readable for AcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`acmr::W`](W) writer structure"]
impl crate::Writable for AcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACMR[%s] to value 0"]
impl crate::Resettable for AcmrSpec {}
