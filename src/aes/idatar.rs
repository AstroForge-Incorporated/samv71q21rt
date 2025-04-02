#[doc = "Register `IDATAR[%s]` writer"]
pub type W = crate::W<IdatarSpec>;
#[doc = "Field `IDATA` writer - Input Data Word"]
pub type IdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data Word"]
    #[inline(always)]
    pub fn idata(&mut self) -> IdataW<IdatarSpec> {
        IdataW::new(self, 0)
    }
}
#[doc = "Input Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdatarSpec;
impl crate::RegisterSpec for IdatarSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idatar::W`](W) writer structure"]
impl crate::Writable for IdatarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDATAR[%s] to value 0"]
impl crate::Resettable for IdatarSpec {}
