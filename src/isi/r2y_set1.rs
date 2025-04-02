#[doc = "Register `R2Y_SET1` reader"]
pub type R = crate::R<R2ySet1Spec>;
#[doc = "Register `R2Y_SET1` writer"]
pub type W = crate::W<R2ySet1Spec>;
#[doc = "Field `C3` reader - Color Space Conversion Matrix Coefficient C3"]
pub type C3R = crate::FieldReader;
#[doc = "Field `C3` writer - Color Space Conversion Matrix Coefficient C3"]
pub type C3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `C4` reader - Color Space Conversion Matrix Coefficient C4"]
pub type C4R = crate::FieldReader;
#[doc = "Field `C4` writer - Color Space Conversion Matrix Coefficient C4"]
pub type C4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `C5` reader - Color Space Conversion Matrix Coefficient C5"]
pub type C5R = crate::FieldReader;
#[doc = "Field `C5` writer - Color Space Conversion Matrix Coefficient C5"]
pub type C5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Goff` reader - Color Space Conversion Green Component Offset"]
pub type GoffR = crate::BitReader;
#[doc = "Field `Goff` writer - Color Space Conversion Green Component Offset"]
pub type GoffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C3"]
    #[inline(always)]
    pub fn c3(&self) -> C3R {
        C3R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&self) -> C4R {
        C4R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C5"]
    #[inline(always)]
    pub fn c5(&self) -> C5R {
        C5R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Green Component Offset"]
    #[inline(always)]
    pub fn goff(&self) -> GoffR {
        GoffR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C3"]
    #[inline(always)]
    pub fn c3(&mut self) -> C3W<R2ySet1Spec> {
        C3W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&mut self) -> C4W<R2ySet1Spec> {
        C4W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C5"]
    #[inline(always)]
    pub fn c5(&mut self) -> C5W<R2ySet1Spec> {
        C5W::new(self, 16)
    }
    #[doc = "Bit 24 - Color Space Conversion Green Component Offset"]
    #[inline(always)]
    pub fn goff(&mut self) -> GoffW<R2ySet1Spec> {
        GoffW::new(self, 24)
    }
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r2y_set1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2y_set1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2ySet1Spec;
impl crate::RegisterSpec for R2ySet1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r2y_set1::R`](R) reader structure"]
impl crate::Readable for R2ySet1Spec {}
#[doc = "`write(|w| ..)` method takes [`r2y_set1::W`](W) writer structure"]
impl crate::Writable for R2ySet1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R2Y_SET1 to value 0"]
impl crate::Resettable for R2ySet1Spec {}
