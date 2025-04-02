#[doc = "Register `US_IDTTX` reader"]
pub type R = crate::R<UsIdttxSpec>;
#[doc = "Register `US_IDTTX` writer"]
pub type W = crate::W<UsIdttxSpec>;
#[doc = "Field `IDTTX` reader - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
pub type IdttxR = crate::FieldReader<u32>;
#[doc = "Field `IDTTX` writer - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
pub type IdttxW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idttx(&self) -> IdttxR {
        IdttxR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idttx(&mut self) -> IdttxW<UsIdttxSpec> {
        IdttxW::new(self, 0)
    }
}
#[doc = "LON IDT Tx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_idttx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idttx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsIdttxSpec;
impl crate::RegisterSpec for UsIdttxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_idttx::R`](R) reader structure"]
impl crate::Readable for UsIdttxSpec {}
#[doc = "`write(|w| ..)` method takes [`us_idttx::W`](W) writer structure"]
impl crate::Writable for UsIdttxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_IDTTX to value 0"]
impl crate::Resettable for UsIdttxSpec {}
