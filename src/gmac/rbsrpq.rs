#[doc = "Register `RBSRPQ[%s]` reader"]
pub type R = crate::R<RbsrpqSpec>;
#[doc = "Register `RBSRPQ[%s]` writer"]
pub type W = crate::W<RbsrpqSpec>;
#[doc = "Field `RBS` reader - Receive Buffer Size"]
pub type RbsR = crate::FieldReader<u16>;
#[doc = "Field `RBS` writer - Receive Buffer Size"]
pub type RbsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&self) -> RbsR {
        RbsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&mut self) -> RbsW<RbsrpqSpec> {
        RbsW::new(self, 0)
    }
}
#[doc = "Receive Buffer Size Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`rbsrpq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbsrpq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbsrpqSpec;
impl crate::RegisterSpec for RbsrpqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbsrpq::R`](R) reader structure"]
impl crate::Readable for RbsrpqSpec {}
#[doc = "`write(|w| ..)` method takes [`rbsrpq::W`](W) writer structure"]
impl crate::Writable for RbsrpqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RBSRPQ[%s] to value 0"]
impl crate::Resettable for RbsrpqSpec {}
