#[doc = "Register `RB` reader"]
pub type R = crate::R<RbSpec>;
#[doc = "Register `RB` writer"]
pub type W = crate::W<RbSpec>;
#[doc = "Field `RB` reader - Register B"]
pub type RbR = crate::FieldReader<u32>;
#[doc = "Field `RB` writer - Register B"]
pub type RbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RbR {
        RbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&mut self) -> RbW<RbSpec> {
        RbW::new(self, 0)
    }
}
#[doc = "Register B (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbSpec;
impl crate::RegisterSpec for RbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rb::R`](R) reader structure"]
impl crate::Readable for RbSpec {}
#[doc = "`write(|w| ..)` method takes [`rb::W`](W) writer structure"]
impl crate::Writable for RbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RB to value 0"]
impl crate::Resettable for RbSpec {}
