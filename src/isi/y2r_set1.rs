#[doc = "Register `Y2R_SET1` reader"]
pub type R = crate::R<Y2rSet1Spec>;
#[doc = "Register `Y2R_SET1` writer"]
pub type W = crate::W<Y2rSet1Spec>;
#[doc = "Field `C4` reader - Color Space Conversion Matrix Coefficient C4"]
pub type C4R = crate::FieldReader<u16>;
#[doc = "Field `C4` writer - Color Space Conversion Matrix Coefficient C4"]
pub type C4W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `Yoff` reader - Color Space Conversion Luminance Default Offset"]
pub type YoffR = crate::BitReader;
#[doc = "Field `Yoff` writer - Color Space Conversion Luminance Default Offset"]
pub type YoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Croff` reader - Color Space Conversion Red Chrominance Default Offset"]
pub type CroffR = crate::BitReader;
#[doc = "Field `Croff` writer - Color Space Conversion Red Chrominance Default Offset"]
pub type CroffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Cboff` reader - Color Space Conversion Blue Chrominance Default Offset"]
pub type CboffR = crate::BitReader;
#[doc = "Field `Cboff` writer - Color Space Conversion Blue Chrominance Default Offset"]
pub type CboffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&self) -> C4R {
        C4R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - Color Space Conversion Luminance Default Offset"]
    #[inline(always)]
    pub fn yoff(&self) -> YoffR {
        YoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Color Space Conversion Red Chrominance Default Offset"]
    #[inline(always)]
    pub fn croff(&self) -> CroffR {
        CroffR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Color Space Conversion Blue Chrominance Default Offset"]
    #[inline(always)]
    pub fn cboff(&self) -> CboffR {
        CboffR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&mut self) -> C4W<Y2rSet1Spec> {
        C4W::new(self, 0)
    }
    #[doc = "Bit 12 - Color Space Conversion Luminance Default Offset"]
    #[inline(always)]
    pub fn yoff(&mut self) -> YoffW<Y2rSet1Spec> {
        YoffW::new(self, 12)
    }
    #[doc = "Bit 13 - Color Space Conversion Red Chrominance Default Offset"]
    #[inline(always)]
    pub fn croff(&mut self) -> CroffW<Y2rSet1Spec> {
        CroffW::new(self, 13)
    }
    #[doc = "Bit 14 - Color Space Conversion Blue Chrominance Default Offset"]
    #[inline(always)]
    pub fn cboff(&mut self) -> CboffW<Y2rSet1Spec> {
        CboffW::new(self, 14)
    }
}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`y2r_set1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`y2r_set1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Y2rSet1Spec;
impl crate::RegisterSpec for Y2rSet1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`y2r_set1::R`](R) reader structure"]
impl crate::Readable for Y2rSet1Spec {}
#[doc = "`write(|w| ..)` method takes [`y2r_set1::W`](W) writer structure"]
impl crate::Writable for Y2rSet1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Y2R_SET1 to value 0"]
impl crate::Resettable for Y2rSet1Spec {}
