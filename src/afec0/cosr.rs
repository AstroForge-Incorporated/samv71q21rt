#[doc = "Register `COSR` reader"]
pub type R = crate::R<CosrSpec>;
#[doc = "Register `COSR` writer"]
pub type W = crate::W<CosrSpec>;
#[doc = "Field `CSEL` reader - Sample & Hold unit Correction Select"]
pub type CselR = crate::BitReader;
#[doc = "Field `CSEL` writer - Sample & Hold unit Correction Select"]
pub type CselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sample & Hold unit Correction Select"]
    #[inline(always)]
    pub fn csel(&self) -> CselR {
        CselR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample & Hold unit Correction Select"]
    #[inline(always)]
    pub fn csel(&mut self) -> CselW<CosrSpec> {
        CselW::new(self, 0)
    }
}
#[doc = "AFEC Correction Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CosrSpec;
impl crate::RegisterSpec for CosrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cosr::R`](R) reader structure"]
impl crate::Readable for CosrSpec {}
#[doc = "`write(|w| ..)` method takes [`cosr::W`](W) writer structure"]
impl crate::Writable for CosrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COSR to value 0"]
impl crate::Resettable for CosrSpec {}
