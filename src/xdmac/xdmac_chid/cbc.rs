#[doc = "Register `CBC` reader"]
pub type R = crate::R<CbcSpec>;
#[doc = "Register `CBC` writer"]
pub type W = crate::W<CbcSpec>;
#[doc = "Field `BLEN` reader - Channel x Block Length"]
pub type BlenR = crate::FieldReader<u16>;
#[doc = "Field `BLEN` writer - Channel x Block Length"]
pub type BlenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    pub fn blen(&self) -> BlenR {
        BlenR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    pub fn blen(&mut self) -> BlenW<CbcSpec> {
        BlenW::new(self, 0)
    }
}
#[doc = "Channel Block Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CbcSpec;
impl crate::RegisterSpec for CbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbc::R`](R) reader structure"]
impl crate::Readable for CbcSpec {}
#[doc = "`write(|w| ..)` method takes [`cbc::W`](W) writer structure"]
impl crate::Writable for CbcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CBC to value 0"]
impl crate::Resettable for CbcSpec {}
