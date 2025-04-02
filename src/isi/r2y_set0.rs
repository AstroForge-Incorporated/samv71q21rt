#[doc = "Register `R2Y_SET0` reader"]
pub type R = crate::R<R2ySet0Spec>;
#[doc = "Register `R2Y_SET0` writer"]
pub type W = crate::W<R2ySet0Spec>;
#[doc = "Field `C0` reader - Color Space Conversion Matrix Coefficient C0"]
pub type C0R = crate::FieldReader;
#[doc = "Field `C0` writer - Color Space Conversion Matrix Coefficient C0"]
pub type C0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `C1` reader - Color Space Conversion Matrix Coefficient C1"]
pub type C1R = crate::FieldReader;
#[doc = "Field `C1` writer - Color Space Conversion Matrix Coefficient C1"]
pub type C1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `C2` reader - Color Space Conversion Matrix Coefficient C2"]
pub type C2R = crate::FieldReader;
#[doc = "Field `C2` writer - Color Space Conversion Matrix Coefficient C2"]
pub type C2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Roff` reader - Color Space Conversion Red Component Offset"]
pub type RoffR = crate::BitReader;
#[doc = "Field `Roff` writer - Color Space Conversion Red Component Offset"]
pub type RoffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    pub fn c0(&self) -> C0R {
        C0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1R {
        C1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2R {
        C2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Red Component Offset"]
    #[inline(always)]
    pub fn roff(&self) -> RoffR {
        RoffR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    pub fn c0(&mut self) -> C0W<R2ySet0Spec> {
        C0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1W<R2ySet0Spec> {
        C1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2W<R2ySet0Spec> {
        C2W::new(self, 16)
    }
    #[doc = "Bit 24 - Color Space Conversion Red Component Offset"]
    #[inline(always)]
    pub fn roff(&mut self) -> RoffW<R2ySet0Spec> {
        RoffW::new(self, 24)
    }
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r2y_set0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2y_set0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2ySet0Spec;
impl crate::RegisterSpec for R2ySet0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r2y_set0::R`](R) reader structure"]
impl crate::Readable for R2ySet0Spec {}
#[doc = "`write(|w| ..)` method takes [`r2y_set0::W`](W) writer structure"]
impl crate::Writable for R2ySet0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R2Y_SET0 to value 0"]
impl crate::Resettable for R2ySet0Spec {}
