#[doc = "Register `DEVDMANXTDSC` reader"]
pub type R = crate::R<DevdmanxtdscSpec>;
#[doc = "Register `DEVDMANXTDSC` writer"]
pub type W = crate::W<DevdmanxtdscSpec>;
#[doc = "Field `NXT_DSC_ADD` reader - Next Descriptor Address"]
pub type NxtDscAddR = crate::FieldReader<u32>;
#[doc = "Field `NXT_DSC_ADD` writer - Next Descriptor Address"]
pub type NxtDscAddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&self) -> NxtDscAddR {
        NxtDscAddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&mut self) -> NxtDscAddW<DevdmanxtdscSpec> {
        NxtDscAddW::new(self, 0)
    }
}
#[doc = "Device DMA Channel Next Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmanxtdsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmanxtdsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevdmanxtdscSpec;
impl crate::RegisterSpec for DevdmanxtdscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmanxtdsc::R`](R) reader structure"]
impl crate::Readable for DevdmanxtdscSpec {}
#[doc = "`write(|w| ..)` method takes [`devdmanxtdsc::W`](W) writer structure"]
impl crate::Writable for DevdmanxtdscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVDMANXTDSC to value 0"]
impl crate::Resettable for DevdmanxtdscSpec {}
