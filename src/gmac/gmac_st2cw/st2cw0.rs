#[doc = "Register `ST2CW0` reader"]
pub type R = crate::R<St2cw0Spec>;
#[doc = "Register `ST2CW0` writer"]
pub type W = crate::W<St2cw0Spec>;
#[doc = "Field `MASKVAL` reader - Mask Value"]
pub type MaskvalR = crate::FieldReader<u16>;
#[doc = "Field `MASKVAL` writer - Mask Value"]
pub type MaskvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COMPVAL` reader - Compare Value"]
pub type CompvalR = crate::FieldReader<u16>;
#[doc = "Field `COMPVAL` writer - Compare Value"]
pub type CompvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    pub fn maskval(&self) -> MaskvalR {
        MaskvalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> CompvalR {
        CompvalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    pub fn maskval(&mut self) -> MaskvalW<St2cw0Spec> {
        MaskvalW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    pub fn compval(&mut self) -> CompvalW<St2cw0Spec> {
        CompvalW::new(self, 16)
    }
}
#[doc = "Screening Type 2 Compare Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`st2cw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2cw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St2cw0Spec;
impl crate::RegisterSpec for St2cw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2cw0::R`](R) reader structure"]
impl crate::Readable for St2cw0Spec {}
#[doc = "`write(|w| ..)` method takes [`st2cw0::W`](W) writer structure"]
impl crate::Writable for St2cw0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST2CW0 to value 0"]
impl crate::Resettable for St2cw0Spec {}
