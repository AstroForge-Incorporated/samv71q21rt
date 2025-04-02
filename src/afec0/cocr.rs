#[doc = "Register `COCR` reader"]
pub type R = crate::R<CocrSpec>;
#[doc = "Register `COCR` writer"]
pub type W = crate::W<CocrSpec>;
#[doc = "Field `AOFF` reader - Analog Offset"]
pub type AoffR = crate::FieldReader<u16>;
#[doc = "Field `AOFF` writer - Analog Offset"]
pub type AoffW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Analog Offset"]
    #[inline(always)]
    pub fn aoff(&self) -> AoffR {
        AoffR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Analog Offset"]
    #[inline(always)]
    pub fn aoff(&mut self) -> AoffW<CocrSpec> {
        AoffW::new(self, 0)
    }
}
#[doc = "AFEC Channel Offset Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CocrSpec;
impl crate::RegisterSpec for CocrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cocr::R`](R) reader structure"]
impl crate::Readable for CocrSpec {}
#[doc = "`write(|w| ..)` method takes [`cocr::W`](W) writer structure"]
impl crate::Writable for CocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COCR to value 0"]
impl crate::Resettable for CocrSpec {}
