#[doc = "Register `TEMPCWR` reader"]
pub type R = crate::R<TempcwrSpec>;
#[doc = "Register `TEMPCWR` writer"]
pub type W = crate::W<TempcwrSpec>;
#[doc = "Field `TLOWTHRES` reader - Temperature Low Threshold"]
pub type TlowthresR = crate::FieldReader<u16>;
#[doc = "Field `TLOWTHRES` writer - Temperature Low Threshold"]
pub type TlowthresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `THIGHTHRES` reader - Temperature High Threshold"]
pub type ThighthresR = crate::FieldReader<u16>;
#[doc = "Field `THIGHTHRES` writer - Temperature High Threshold"]
pub type ThighthresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&self) -> TlowthresR {
        TlowthresR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&self) -> ThighthresR {
        ThighthresR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&mut self) -> TlowthresW<TempcwrSpec> {
        TlowthresW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&mut self) -> ThighthresW<TempcwrSpec> {
        ThighthresW::new(self, 16)
    }
}
#[doc = "AFEC Temperature Compare Window Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tempcwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tempcwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempcwrSpec;
impl crate::RegisterSpec for TempcwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tempcwr::R`](R) reader structure"]
impl crate::Readable for TempcwrSpec {}
#[doc = "`write(|w| ..)` method takes [`tempcwr::W`](W) writer structure"]
impl crate::Writable for TempcwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEMPCWR to value 0"]
impl crate::Resettable for TempcwrSpec {}
