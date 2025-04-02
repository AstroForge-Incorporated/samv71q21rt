#[doc = "Register `CLENR` reader"]
pub type R = crate::R<ClenrSpec>;
#[doc = "Register `CLENR` writer"]
pub type W = crate::W<ClenrSpec>;
#[doc = "Field `CLEN` reader - Plaintext/Ciphertext Length"]
pub type ClenR = crate::FieldReader<u32>;
#[doc = "Field `CLEN` writer - Plaintext/Ciphertext Length"]
pub type ClenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    pub fn clen(&self) -> ClenR {
        ClenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    pub fn clen(&mut self) -> ClenW<ClenrSpec> {
        ClenW::new(self, 0)
    }
}
#[doc = "Plaintext/Ciphertext Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClenrSpec;
impl crate::RegisterSpec for ClenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clenr::R`](R) reader structure"]
impl crate::Readable for ClenrSpec {}
#[doc = "`write(|w| ..)` method takes [`clenr::W`](W) writer structure"]
impl crate::Writable for ClenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLENR to value 0"]
impl crate::Resettable for ClenrSpec {}
