#[doc = "Register `HCMR[%s]` reader"]
pub type R = crate::R<HcmrSpec>;
#[doc = "Register `HCMR[%s]` writer"]
pub type W = crate::W<HcmrSpec>;
#[doc = "Field `CHM` reader - Bitwise Channel Mask Bit \\[31:0\\]"]
pub type ChmR = crate::FieldReader<u32>;
#[doc = "Field `CHM` writer - Bitwise Channel Mask Bit \\[31:0\\]"]
pub type ChmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chm(&self) -> ChmR {
        ChmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chm(&mut self) -> ChmW<HcmrSpec> {
        ChmW::new(self, 0)
    }
}
#[doc = "HBI Channel Mask 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcmrSpec;
impl crate::RegisterSpec for HcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcmr::R`](R) reader structure"]
impl crate::Readable for HcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`hcmr::W`](W) writer structure"]
impl crate::Writable for HcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCMR[%s] to value 0"]
impl crate::Resettable for HcmrSpec {}
