#[doc = "Register `AADLENR` reader"]
pub type R = crate::R<AadlenrSpec>;
#[doc = "Register `AADLENR` writer"]
pub type W = crate::W<AadlenrSpec>;
#[doc = "Field `AADLEN` reader - Additional Authenticated Data Length"]
pub type AadlenR = crate::FieldReader<u32>;
#[doc = "Field `AADLEN` writer - Additional Authenticated Data Length"]
pub type AadlenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    pub fn aadlen(&self) -> AadlenR {
        AadlenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    pub fn aadlen(&mut self) -> AadlenW<AadlenrSpec> {
        AadlenW::new(self, 0)
    }
}
#[doc = "Additional Authenticated Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aadlenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aadlenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AadlenrSpec;
impl crate::RegisterSpec for AadlenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aadlenr::R`](R) reader structure"]
impl crate::Readable for AadlenrSpec {}
#[doc = "`write(|w| ..)` method takes [`aadlenr::W`](W) writer structure"]
impl crate::Writable for AadlenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AADLENR to value 0"]
impl crate::Resettable for AadlenrSpec {}
