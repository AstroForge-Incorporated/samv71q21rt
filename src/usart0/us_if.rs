#[doc = "Register `US_IF` reader"]
pub type R = crate::R<UsIfSpec>;
#[doc = "Register `US_IF` writer"]
pub type W = crate::W<UsIfSpec>;
#[doc = "Field `IRDA_FILTER` reader - IrDA Filter"]
pub type IrdaFilterR = crate::FieldReader;
#[doc = "Field `IRDA_FILTER` writer - IrDA Filter"]
pub type IrdaFilterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    pub fn irda_filter(&self) -> IrdaFilterR {
        IrdaFilterR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    pub fn irda_filter(&mut self) -> IrdaFilterW<UsIfSpec> {
        IrdaFilterW::new(self, 0)
    }
}
#[doc = "IrDA Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsIfSpec;
impl crate::RegisterSpec for UsIfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_if::R`](R) reader structure"]
impl crate::Readable for UsIfSpec {}
#[doc = "`write(|w| ..)` method takes [`us_if::W`](W) writer structure"]
impl crate::Writable for UsIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_IF to value 0"]
impl crate::Resettable for UsIfSpec {}
