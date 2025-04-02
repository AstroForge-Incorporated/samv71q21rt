#[doc = "Register `PSIZE` reader"]
pub type R = crate::R<PsizeSpec>;
#[doc = "Register `PSIZE` writer"]
pub type W = crate::W<PsizeSpec>;
#[doc = "Field `PREV_VSIZE` reader - Vertical Size for the Preview Path"]
pub type PrevVsizeR = crate::FieldReader<u16>;
#[doc = "Field `PREV_VSIZE` writer - Vertical Size for the Preview Path"]
pub type PrevVsizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PREV_HSIZE` reader - Horizontal Size for the Preview Path"]
pub type PrevHsizeR = crate::FieldReader<u16>;
#[doc = "Field `PREV_HSIZE` writer - Horizontal Size for the Preview Path"]
pub type PrevHsizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&self) -> PrevVsizeR {
        PrevVsizeR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&self) -> PrevHsizeR {
        PrevHsizeR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&mut self) -> PrevVsizeW<PsizeSpec> {
        PrevVsizeW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&mut self) -> PrevHsizeW<PsizeSpec> {
        PrevHsizeW::new(self, 16)
    }
}
#[doc = "ISI Preview Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsizeSpec;
impl crate::RegisterSpec for PsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psize::R`](R) reader structure"]
impl crate::Readable for PsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`psize::W`](W) writer structure"]
impl crate::Writable for PsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSIZE to value 0"]
impl crate::Resettable for PsizeSpec {}
