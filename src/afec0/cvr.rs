#[doc = "Register `CVR` reader"]
pub type R = crate::R<CvrSpec>;
#[doc = "Register `CVR` writer"]
pub type W = crate::W<CvrSpec>;
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub type OffsetcorrR = crate::FieldReader<u16>;
#[doc = "Field `OFFSETCORR` writer - Offset Correction"]
pub type OffsetcorrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub type GaincorrR = crate::FieldReader<u16>;
#[doc = "Field `GAINCORR` writer - Gain Correction"]
pub type GaincorrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OffsetcorrR {
        OffsetcorrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GaincorrR {
        GaincorrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&mut self) -> OffsetcorrW<CvrSpec> {
        OffsetcorrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&mut self) -> GaincorrW<CvrSpec> {
        GaincorrW::new(self, 16)
    }
}
#[doc = "AFEC Correction Values Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvrSpec;
impl crate::RegisterSpec for CvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvr::R`](R) reader structure"]
impl crate::Readable for CvrSpec {}
#[doc = "`write(|w| ..)` method takes [`cvr::W`](W) writer structure"]
impl crate::Writable for CvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVR to value 0"]
impl crate::Resettable for CvrSpec {}
