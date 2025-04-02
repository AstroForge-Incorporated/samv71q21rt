#[doc = "Register `TIDM4` reader"]
pub type R = crate::R<Tidm4Spec>;
#[doc = "Register `TIDM4` writer"]
pub type W = crate::W<Tidm4Spec>;
#[doc = "Field `TID` reader - Type ID Match 4"]
pub type TidR = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 4"]
pub type TidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENID4` reader - Enable Copying of TID Matched Frames"]
pub type Enid4R = crate::BitReader;
#[doc = "Field `ENID4` writer - Enable Copying of TID Matched Frames"]
pub type Enid4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 4"]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid4(&self) -> Enid4R {
        Enid4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 4"]
    #[inline(always)]
    pub fn tid(&mut self) -> TidW<Tidm4Spec> {
        TidW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid4(&mut self) -> Enid4W<Tidm4Spec> {
        Enid4W::new(self, 31)
    }
}
#[doc = "Type ID Match 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tidm4Spec;
impl crate::RegisterSpec for Tidm4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm4::R`](R) reader structure"]
impl crate::Readable for Tidm4Spec {}
#[doc = "`write(|w| ..)` method takes [`tidm4::W`](W) writer structure"]
impl crate::Writable for Tidm4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIDM4 to value 0"]
impl crate::Resettable for Tidm4Spec {}
