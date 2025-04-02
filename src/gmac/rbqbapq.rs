#[doc = "Register `RBQBAPQ[%s]` reader"]
pub type R = crate::R<RbqbapqSpec>;
#[doc = "Register `RBQBAPQ[%s]` writer"]
pub type W = crate::W<RbqbapqSpec>;
#[doc = "Field `RXBQBA` reader - Receive Buffer Queue Base Address"]
pub type RxbqbaR = crate::FieldReader<u32>;
#[doc = "Field `RXBQBA` writer - Receive Buffer Queue Base Address"]
pub type RxbqbaW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn rxbqba(&self) -> RxbqbaR {
        RxbqbaR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn rxbqba(&mut self) -> RxbqbaW<RbqbapqSpec> {
        RxbqbaW::new(self, 2)
    }
}
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`rbqbapq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbqbapq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbqbapqSpec;
impl crate::RegisterSpec for RbqbapqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbqbapq::R`](R) reader structure"]
impl crate::Readable for RbqbapqSpec {}
#[doc = "`write(|w| ..)` method takes [`rbqbapq::W`](W) writer structure"]
impl crate::Writable for RbqbapqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RBQBAPQ[%s] to value 0"]
impl crate::Resettable for RbqbapqSpec {}
