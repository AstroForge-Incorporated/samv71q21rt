#[doc = "Register `US_LONB1RX` reader"]
pub type R = crate::R<UsLonb1rxSpec>;
#[doc = "Register `US_LONB1RX` writer"]
pub type W = crate::W<UsLonb1rxSpec>;
#[doc = "Field `BETA1RX` reader - LON Beta1 Length after Reception"]
pub type Beta1rxR = crate::FieldReader<u32>;
#[doc = "Field `BETA1RX` writer - LON Beta1 Length after Reception"]
pub type Beta1rxW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Beta1 Length after Reception"]
    #[inline(always)]
    pub fn beta1rx(&self) -> Beta1rxR {
        Beta1rxR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Beta1 Length after Reception"]
    #[inline(always)]
    pub fn beta1rx(&mut self) -> Beta1rxW<UsLonb1rxSpec> {
        Beta1rxW::new(self, 0)
    }
}
#[doc = "LON Beta1 Rx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonb1rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonb1rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsLonb1rxSpec;
impl crate::RegisterSpec for UsLonb1rxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonb1rx::R`](R) reader structure"]
impl crate::Readable for UsLonb1rxSpec {}
#[doc = "`write(|w| ..)` method takes [`us_lonb1rx::W`](W) writer structure"]
impl crate::Writable for UsLonb1rxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_LONB1RX to value 0"]
impl crate::Resettable for UsLonb1rxSpec {}
