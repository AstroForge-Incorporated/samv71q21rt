#[doc = "Register `CSA` reader"]
pub type R = crate::R<CsaSpec>;
#[doc = "Register `CSA` writer"]
pub type W = crate::W<CsaSpec>;
#[doc = "Field `SA` reader - Channel x Source Address"]
pub type SaR = crate::FieldReader<u32>;
#[doc = "Field `SA` writer - Channel x Source Address"]
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn sa(&mut self) -> SaW<CsaSpec> {
        SaW::new(self, 0)
    }
}
#[doc = "Channel Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsaSpec;
impl crate::RegisterSpec for CsaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csa::R`](R) reader structure"]
impl crate::Readable for CsaSpec {}
#[doc = "`write(|w| ..)` method takes [`csa::W`](W) writer structure"]
impl crate::Writable for CsaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSA to value 0"]
impl crate::Resettable for CsaSpec {}
