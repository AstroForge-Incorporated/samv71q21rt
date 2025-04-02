#[doc = "Register `R2Y_SET2` reader"]
pub type R = crate::R<R2ySet2Spec>;
#[doc = "Register `R2Y_SET2` writer"]
pub type W = crate::W<R2ySet2Spec>;
#[doc = "Field `C6` reader - Color Space Conversion Matrix Coefficient C6"]
pub type C6R = crate::FieldReader;
#[doc = "Field `C6` writer - Color Space Conversion Matrix Coefficient C6"]
pub type C6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `C7` reader - Color Space Conversion Matrix Coefficient C7"]
pub type C7R = crate::FieldReader;
#[doc = "Field `C7` writer - Color Space Conversion Matrix Coefficient C7"]
pub type C7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `C8` reader - Color Space Conversion Matrix Coefficient C8"]
pub type C8R = crate::FieldReader;
#[doc = "Field `C8` writer - Color Space Conversion Matrix Coefficient C8"]
pub type C8W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Boff` reader - Color Space Conversion Blue Component Offset"]
pub type BoffR = crate::BitReader;
#[doc = "Field `Boff` writer - Color Space Conversion Blue Component Offset"]
pub type BoffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    pub fn c6(&self) -> C6R {
        C6R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    pub fn c7(&self) -> C7R {
        C7R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    pub fn c8(&self) -> C8R {
        C8R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    pub fn boff(&self) -> BoffR {
        BoffR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    pub fn c6(&mut self) -> C6W<R2ySet2Spec> {
        C6W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    pub fn c7(&mut self) -> C7W<R2ySet2Spec> {
        C7W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    pub fn c8(&mut self) -> C8W<R2ySet2Spec> {
        C8W::new(self, 16)
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    pub fn boff(&mut self) -> BoffW<R2ySet2Spec> {
        BoffW::new(self, 24)
    }
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r2y_set2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2y_set2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2ySet2Spec;
impl crate::RegisterSpec for R2ySet2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r2y_set2::R`](R) reader structure"]
impl crate::Readable for R2ySet2Spec {}
#[doc = "`write(|w| ..)` method takes [`r2y_set2::W`](W) writer structure"]
impl crate::Writable for R2ySet2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R2Y_SET2 to value 0"]
impl crate::Resettable for R2ySet2Spec {}
