#[doc = "Register `CUBC` reader"]
pub type R = crate::R<CubcSpec>;
#[doc = "Register `CUBC` writer"]
pub type W = crate::W<CubcSpec>;
#[doc = "Field `UBLEN` reader - Channel x Microblock Length"]
pub type UblenR = crate::FieldReader<u32>;
#[doc = "Field `UBLEN` writer - Channel x Microblock Length"]
pub type UblenW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&self) -> UblenR {
        UblenR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&mut self) -> UblenW<CubcSpec> {
        UblenW::new(self, 0)
    }
}
#[doc = "Channel Microblock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cubc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cubc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CubcSpec;
impl crate::RegisterSpec for CubcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cubc::R`](R) reader structure"]
impl crate::Readable for CubcSpec {}
#[doc = "`write(|w| ..)` method takes [`cubc::W`](W) writer structure"]
impl crate::Writable for CubcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CUBC to value 0"]
impl crate::Resettable for CubcSpec {}
