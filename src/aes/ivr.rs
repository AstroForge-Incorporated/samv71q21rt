#[doc = "Register `IVR[%s]` writer"]
pub type W = crate::W<IvrSpec>;
#[doc = "Field `IV` writer - Initialization Vector"]
pub type IvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initialization Vector"]
    #[inline(always)]
    pub fn iv(&mut self) -> IvW<IvrSpec> {
        IvW::new(self, 0)
    }
}
#[doc = "Initialization Vector Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IvrSpec;
impl crate::RegisterSpec for IvrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ivr::W`](W) writer structure"]
impl crate::Writable for IvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IVR[%s] to value 0"]
impl crate::Resettable for IvrSpec {}
