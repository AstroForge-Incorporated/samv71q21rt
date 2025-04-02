#[doc = "Register `CSELR` reader"]
pub type R = crate::R<CselrSpec>;
#[doc = "Register `CSELR` writer"]
pub type W = crate::W<CselrSpec>;
#[doc = "Field `CSEL` reader - Channel Selection"]
pub type CselR = crate::FieldReader;
#[doc = "Field `CSEL` writer - Channel Selection"]
pub type CselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn csel(&self) -> CselR {
        CselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn csel(&mut self) -> CselW<CselrSpec> {
        CselW::new(self, 0)
    }
}
#[doc = "AFEC Channel Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CselrSpec;
impl crate::RegisterSpec for CselrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cselr::R`](R) reader structure"]
impl crate::Readable for CselrSpec {}
#[doc = "`write(|w| ..)` method takes [`cselr::W`](W) writer structure"]
impl crate::Writable for CselrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSELR to value 0"]
impl crate::Resettable for CselrSpec {}
