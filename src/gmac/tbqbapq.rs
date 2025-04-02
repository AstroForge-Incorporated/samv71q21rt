#[doc = "Register `TBQBAPQ[%s]` reader"]
pub type R = crate::R<TbqbapqSpec>;
#[doc = "Register `TBQBAPQ[%s]` writer"]
pub type W = crate::W<TbqbapqSpec>;
#[doc = "Field `TXBQBA` reader - Transmit Buffer Queue Base Address"]
pub type TxbqbaR = crate::FieldReader<u32>;
#[doc = "Field `TXBQBA` writer - Transmit Buffer Queue Base Address"]
pub type TxbqbaW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn txbqba(&self) -> TxbqbaR {
        TxbqbaR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn txbqba(&mut self) -> TxbqbaW<TbqbapqSpec> {
        TxbqbaW::new(self, 2)
    }
}
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`tbqbapq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbqbapq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbqbapqSpec;
impl crate::RegisterSpec for TbqbapqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbqbapq::R`](R) reader structure"]
impl crate::Readable for TbqbapqSpec {}
#[doc = "`write(|w| ..)` method takes [`tbqbapq::W`](W) writer structure"]
impl crate::Writable for TbqbapqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBQBAPQ[%s] to value 0"]
impl crate::Resettable for TbqbapqSpec {}
