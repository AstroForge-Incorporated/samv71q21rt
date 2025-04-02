#[doc = "Register `US_FIDI_USART_MODE` reader"]
pub type R = crate::R<UsFidiUsartModeSpec>;
#[doc = "Register `US_FIDI_USART_MODE` writer"]
pub type W = crate::W<UsFidiUsartModeSpec>;
#[doc = "Field `FI_DI_RATIO` reader - FI Over DI Ratio Value"]
pub type FiDiRatioR = crate::FieldReader<u16>;
#[doc = "Field `FI_DI_RATIO` writer - FI Over DI Ratio Value"]
pub type FiDiRatioW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&self) -> FiDiRatioR {
        FiDiRatioR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&mut self) -> FiDiRatioW<UsFidiUsartModeSpec> {
        FiDiRatioW::new(self, 0)
    }
}
#[doc = "FI DI Ratio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_fidi_usart_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_fidi_usart_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsFidiUsartModeSpec;
impl crate::RegisterSpec for UsFidiUsartModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_fidi_usart_mode::R`](R) reader structure"]
impl crate::Readable for UsFidiUsartModeSpec {}
#[doc = "`write(|w| ..)` method takes [`us_fidi_usart_mode::W`](W) writer structure"]
impl crate::Writable for UsFidiUsartModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_FIDI_USART_MODE to value 0"]
impl crate::Resettable for UsFidiUsartModeSpec {}
