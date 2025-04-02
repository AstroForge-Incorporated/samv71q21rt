#[doc = "Register `TOCV` reader"]
pub type R = crate::R<TocvSpec>;
#[doc = "Register `TOCV` writer"]
pub type W = crate::W<TocvSpec>;
#[doc = "Field `TOC` reader - Timeout Counter (cleared on write)"]
pub type TocR = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - Timeout Counter (cleared on write)"]
pub type TocW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout Counter (cleared on write)"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout Counter (cleared on write)"]
    #[inline(always)]
    pub fn toc(&mut self) -> TocW<TocvSpec> {
        TocW::new(self, 0)
    }
}
#[doc = "Timeout Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tocv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TocvSpec;
impl crate::RegisterSpec for TocvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocv::R`](R) reader structure"]
impl crate::Readable for TocvSpec {}
#[doc = "`write(|w| ..)` method takes [`tocv::W`](W) writer structure"]
impl crate::Writable for TocvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOCV to value 0"]
impl crate::Resettable for TocvSpec {}
