#[doc = "Register `KEYWR[%s]` writer"]
pub type W = crate::W<KeywrSpec>;
#[doc = "Field `KEYW` writer - Key Word"]
pub type KeywW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key Word"]
    #[inline(always)]
    pub fn keyw(&mut self) -> KeywW<KeywrSpec> {
        KeywW::new(self, 0)
    }
}
#[doc = "Key Word Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keywr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeywrSpec;
impl crate::RegisterSpec for KeywrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keywr::W`](W) writer structure"]
impl crate::Writable for KeywrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYWR[%s] to value 0"]
impl crate::Resettable for KeywrSpec {}
