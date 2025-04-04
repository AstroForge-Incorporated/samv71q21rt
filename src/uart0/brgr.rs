#[doc = "Register `BRGR` reader"]
pub type R = crate::R<BrgrSpec>;
#[doc = "Register `BRGR` writer"]
pub type W = crate::W<BrgrSpec>;
#[doc = "Field `CD` reader - Clock Divisor"]
pub type CdR = crate::FieldReader<u16>;
#[doc = "Field `CD` writer - Clock Divisor"]
pub type CdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&mut self) -> CdW<BrgrSpec> {
        CdW::new(self, 0)
    }
}
#[doc = "Baud Rate Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrgrSpec;
impl crate::RegisterSpec for BrgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brgr::R`](R) reader structure"]
impl crate::Readable for BrgrSpec {}
#[doc = "`write(|w| ..)` method takes [`brgr::W`](W) writer structure"]
impl crate::Writable for BrgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRGR to value 0"]
impl crate::Resettable for BrgrSpec {}
