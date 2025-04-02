#[doc = "Register `RC` reader"]
pub type R = crate::R<RcSpec>;
#[doc = "Register `RC` writer"]
pub type W = crate::W<RcSpec>;
#[doc = "Field `RC` reader - Register C"]
pub type RcR = crate::FieldReader<u32>;
#[doc = "Field `RC` writer - Register C"]
pub type RcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&mut self) -> RcW<RcSpec> {
        RcW::new(self, 0)
    }
}
#[doc = "Register C (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcSpec;
impl crate::RegisterSpec for RcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc::R`](R) reader structure"]
impl crate::Readable for RcSpec {}
#[doc = "`write(|w| ..)` method takes [`rc::W`](W) writer structure"]
impl crate::Writable for RcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RC to value 0"]
impl crate::Resettable for RcSpec {}
