#[doc = "Register `CDA` reader"]
pub type R = crate::R<CdaSpec>;
#[doc = "Register `CDA` writer"]
pub type W = crate::W<CdaSpec>;
#[doc = "Field `DA` reader - Channel x Destination Address"]
pub type DaR = crate::FieldReader<u32>;
#[doc = "Field `DA` writer - Channel x Destination Address"]
pub type DaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn da(&mut self) -> DaW<CdaSpec> {
        DaW::new(self, 0)
    }
}
#[doc = "Channel Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdaSpec;
impl crate::RegisterSpec for CdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cda::R`](R) reader structure"]
impl crate::Readable for CdaSpec {}
#[doc = "`write(|w| ..)` method takes [`cda::W`](W) writer structure"]
impl crate::Writable for CdaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDA to value 0"]
impl crate::Resettable for CdaSpec {}
