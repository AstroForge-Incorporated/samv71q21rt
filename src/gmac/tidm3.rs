#[doc = "Register `TIDM3` reader"]
pub type R = crate::R<Tidm3Spec>;
#[doc = "Register `TIDM3` writer"]
pub type W = crate::W<Tidm3Spec>;
#[doc = "Field `TID` reader - Type ID Match 3"]
pub type TidR = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 3"]
pub type TidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENID3` reader - Enable Copying of TID Matched Frames"]
pub type Enid3R = crate::BitReader;
#[doc = "Field `ENID3` writer - Enable Copying of TID Matched Frames"]
pub type Enid3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 3"]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid3(&self) -> Enid3R {
        Enid3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 3"]
    #[inline(always)]
    pub fn tid(&mut self) -> TidW<Tidm3Spec> {
        TidW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid3(&mut self) -> Enid3W<Tidm3Spec> {
        Enid3W::new(self, 31)
    }
}
#[doc = "Type ID Match 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tidm3Spec;
impl crate::RegisterSpec for Tidm3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm3::R`](R) reader structure"]
impl crate::Readable for Tidm3Spec {}
#[doc = "`write(|w| ..)` method takes [`tidm3::W`](W) writer structure"]
impl crate::Writable for Tidm3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIDM3 to value 0"]
impl crate::Resettable for Tidm3Spec {}
