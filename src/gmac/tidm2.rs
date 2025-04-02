#[doc = "Register `TIDM2` reader"]
pub type R = crate::R<Tidm2Spec>;
#[doc = "Register `TIDM2` writer"]
pub type W = crate::W<Tidm2Spec>;
#[doc = "Field `TID` reader - Type ID Match 2"]
pub type TidR = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 2"]
pub type TidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENID2` reader - Enable Copying of TID Matched Frames"]
pub type Enid2R = crate::BitReader;
#[doc = "Field `ENID2` writer - Enable Copying of TID Matched Frames"]
pub type Enid2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 2"]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid2(&self) -> Enid2R {
        Enid2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 2"]
    #[inline(always)]
    pub fn tid(&mut self) -> TidW<Tidm2Spec> {
        TidW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid2(&mut self) -> Enid2W<Tidm2Spec> {
        Enid2W::new(self, 31)
    }
}
#[doc = "Type ID Match 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tidm2Spec;
impl crate::RegisterSpec for Tidm2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm2::R`](R) reader structure"]
impl crate::Readable for Tidm2Spec {}
#[doc = "`write(|w| ..)` method takes [`tidm2::W`](W) writer structure"]
impl crate::Writable for Tidm2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIDM2 to value 0"]
impl crate::Resettable for Tidm2Spec {}
