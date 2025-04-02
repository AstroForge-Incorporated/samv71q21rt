#[doc = "Register `PDECF` reader"]
pub type R = crate::R<PdecfSpec>;
#[doc = "Register `PDECF` writer"]
pub type W = crate::W<PdecfSpec>;
#[doc = "Field `DEC_FACTOR` reader - Decimation Factor"]
pub type DecFactorR = crate::FieldReader;
#[doc = "Field `DEC_FACTOR` writer - Decimation Factor"]
pub type DecFactorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    pub fn dec_factor(&self) -> DecFactorR {
        DecFactorR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    pub fn dec_factor(&mut self) -> DecFactorW<PdecfSpec> {
        DecFactorW::new(self, 0)
    }
}
#[doc = "ISI Preview Decimation Factor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdecf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdecf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdecfSpec;
impl crate::RegisterSpec for PdecfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdecf::R`](R) reader structure"]
impl crate::Readable for PdecfSpec {}
#[doc = "`write(|w| ..)` method takes [`pdecf::W`](W) writer structure"]
impl crate::Writable for PdecfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDECF to value 0"]
impl crate::Resettable for PdecfSpec {}
