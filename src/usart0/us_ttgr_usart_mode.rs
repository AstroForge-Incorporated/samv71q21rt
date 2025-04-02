#[doc = "Register `US_TTGR_USART_MODE` reader"]
pub type R = crate::R<UsTtgrUsartModeSpec>;
#[doc = "Register `US_TTGR_USART_MODE` writer"]
pub type W = crate::W<UsTtgrUsartModeSpec>;
#[doc = "Field `TG` reader - Timeguard Value"]
pub type TgR = crate::FieldReader;
#[doc = "Field `TG` writer - Timeguard Value"]
pub type TgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&self) -> TgR {
        TgR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<UsTtgrUsartModeSpec> {
        TgW::new(self, 0)
    }
}
#[doc = "Transmitter Timeguard Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_ttgr_usart_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ttgr_usart_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsTtgrUsartModeSpec;
impl crate::RegisterSpec for UsTtgrUsartModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_ttgr_usart_mode::R`](R) reader structure"]
impl crate::Readable for UsTtgrUsartModeSpec {}
#[doc = "`write(|w| ..)` method takes [`us_ttgr_usart_mode::W`](W) writer structure"]
impl crate::Writable for UsTtgrUsartModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_TTGR_USART_MODE to value 0"]
impl crate::Resettable for UsTtgrUsartModeSpec {}
