#[doc = "Register `US_IDTRX` reader"]
pub type R = crate::R<UsIdtrxSpec>;
#[doc = "Register `US_IDTRX` writer"]
pub type W = crate::W<UsIdtrxSpec>;
#[doc = "Field `IDTRX` reader - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
pub type IdtrxR = crate::FieldReader<u32>;
#[doc = "Field `IDTRX` writer - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
pub type IdtrxW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idtrx(&self) -> IdtrxR {
        IdtrxR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idtrx(&mut self) -> IdtrxW<UsIdtrxSpec> {
        IdtrxW::new(self, 0)
    }
}
#[doc = "LON IDT Rx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_idtrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idtrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsIdtrxSpec;
impl crate::RegisterSpec for UsIdtrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_idtrx::R`](R) reader structure"]
impl crate::Readable for UsIdtrxSpec {}
#[doc = "`write(|w| ..)` method takes [`us_idtrx::W`](W) writer structure"]
impl crate::Writable for UsIdtrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_IDTRX to value 0"]
impl crate::Resettable for UsIdtrxSpec {}
