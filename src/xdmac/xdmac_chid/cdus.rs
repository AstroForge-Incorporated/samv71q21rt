#[doc = "Register `CDUS` reader"]
pub type R = crate::R<CdusSpec>;
#[doc = "Register `CDUS` writer"]
pub type W = crate::W<CdusSpec>;
#[doc = "Field `DUBS` reader - Channel x Destination Microblock Stride"]
pub type DubsR = crate::FieldReader<u32>;
#[doc = "Field `DUBS` writer - Channel x Destination Microblock Stride"]
pub type DubsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    pub fn dubs(&self) -> DubsR {
        DubsR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    pub fn dubs(&mut self) -> DubsW<CdusSpec> {
        DubsW::new(self, 0)
    }
}
#[doc = "Channel Destination Microblock Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`cdus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdusSpec;
impl crate::RegisterSpec for CdusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdus::R`](R) reader structure"]
impl crate::Readable for CdusSpec {}
#[doc = "`write(|w| ..)` method takes [`cdus::W`](W) writer structure"]
impl crate::Writable for CdusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDUS to value 0"]
impl crate::Resettable for CdusSpec {}
