#[doc = "Register `TIDM1` reader"]
pub type R = crate::R<Tidm1Spec>;
#[doc = "Register `TIDM1` writer"]
pub type W = crate::W<Tidm1Spec>;
#[doc = "Field `TID` reader - Type ID Match 1"]
pub type TidR = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 1"]
pub type TidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENID1` reader - Enable Copying of TID Matched Frames"]
pub type Enid1R = crate::BitReader;
#[doc = "Field `ENID1` writer - Enable Copying of TID Matched Frames"]
pub type Enid1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid1(&self) -> Enid1R {
        Enid1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    pub fn tid(&mut self) -> TidW<Tidm1Spec> {
        TidW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid1(&mut self) -> Enid1W<Tidm1Spec> {
        Enid1W::new(self, 31)
    }
}
#[doc = "Type ID Match 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tidm1Spec;
impl crate::RegisterSpec for Tidm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm1::R`](R) reader structure"]
impl crate::Readable for Tidm1Spec {}
#[doc = "`write(|w| ..)` method takes [`tidm1::W`](W) writer structure"]
impl crate::Writable for Tidm1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIDM1 to value 0"]
impl crate::Resettable for Tidm1Spec {}
