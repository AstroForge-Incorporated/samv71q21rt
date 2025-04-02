#[doc = "Register `Y2R_SET0` reader"]
pub type R = crate::R<Y2rSet0Spec>;
#[doc = "Register `Y2R_SET0` writer"]
pub type W = crate::W<Y2rSet0Spec>;
#[doc = "Field `C0` reader - Color Space Conversion Matrix Coefficient C0"]
pub type C0R = crate::FieldReader;
#[doc = "Field `C0` writer - Color Space Conversion Matrix Coefficient C0"]
pub type C0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `C1` reader - Color Space Conversion Matrix Coefficient C1"]
pub type C1R = crate::FieldReader;
#[doc = "Field `C1` writer - Color Space Conversion Matrix Coefficient C1"]
pub type C1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `C2` reader - Color Space Conversion Matrix Coefficient C2"]
pub type C2R = crate::FieldReader;
#[doc = "Field `C2` writer - Color Space Conversion Matrix Coefficient C2"]
pub type C2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `C3` reader - Color Space Conversion Matrix Coefficient C3"]
pub type C3R = crate::FieldReader;
#[doc = "Field `C3` writer - Color Space Conversion Matrix Coefficient C3"]
pub type C3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    pub fn c0(&self) -> C0R {
        C0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1R {
        C1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2R {
        C2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Color Space Conversion Matrix Coefficient C3"]
    #[inline(always)]
    pub fn c3(&self) -> C3R {
        C3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    pub fn c0(&mut self) -> C0W<Y2rSet0Spec> {
        C0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1W<Y2rSet0Spec> {
        C1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2W<Y2rSet0Spec> {
        C2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Color Space Conversion Matrix Coefficient C3"]
    #[inline(always)]
    pub fn c3(&mut self) -> C3W<Y2rSet0Spec> {
        C3W::new(self, 24)
    }
}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`y2r_set0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`y2r_set0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Y2rSet0Spec;
impl crate::RegisterSpec for Y2rSet0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`y2r_set0::R`](R) reader structure"]
impl crate::Readable for Y2rSet0Spec {}
#[doc = "`write(|w| ..)` method takes [`y2r_set0::W`](W) writer structure"]
impl crate::Writable for Y2rSet0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Y2R_SET0 to value 0"]
impl crate::Resettable for Y2rSet0Spec {}
