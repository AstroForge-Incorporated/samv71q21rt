#[doc = "Register `US_BRGR` reader"]
pub type R = crate::R<UsBrgrSpec>;
#[doc = "Register `US_BRGR` writer"]
pub type W = crate::W<UsBrgrSpec>;
#[doc = "Field `CD` reader - Clock Divider"]
pub type CdR = crate::FieldReader<u16>;
#[doc = "Field `CD` writer - Clock Divider"]
pub type CdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FP` reader - Fractional Part"]
pub type FpR = crate::FieldReader;
#[doc = "Field `FP` writer - Fractional Part"]
pub type FpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FpR {
        FpR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    pub fn cd(&mut self) -> CdW<UsBrgrSpec> {
        CdW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&mut self) -> FpW<UsBrgrSpec> {
        FpW::new(self, 16)
    }
}
#[doc = "Baud Rate Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_brgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_brgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsBrgrSpec;
impl crate::RegisterSpec for UsBrgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_brgr::R`](R) reader structure"]
impl crate::Readable for UsBrgrSpec {}
#[doc = "`write(|w| ..)` method takes [`us_brgr::W`](W) writer structure"]
impl crate::Writable for UsBrgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_BRGR to value 0"]
impl crate::Resettable for UsBrgrSpec {}
