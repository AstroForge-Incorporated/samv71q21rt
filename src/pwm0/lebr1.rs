#[doc = "Register `LEBR1` reader"]
pub type R = crate::R<Lebr1Spec>;
#[doc = "Register `LEBR1` writer"]
pub type W = crate::W<Lebr1Spec>;
#[doc = "Field `LEBDELAY` reader - Leading-Edge Blanking Delay for TRGINx"]
pub type LebdelayR = crate::FieldReader;
#[doc = "Field `LEBDELAY` writer - Leading-Edge Blanking Delay for TRGINx"]
pub type LebdelayW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PWMLFEN` reader - PWML Falling Edge Enable"]
pub type PwmlfenR = crate::BitReader;
#[doc = "Field `PWMLFEN` writer - PWML Falling Edge Enable"]
pub type PwmlfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMLREN` reader - PWML Rising Edge Enable"]
pub type PwmlrenR = crate::BitReader;
#[doc = "Field `PWMLREN` writer - PWML Rising Edge Enable"]
pub type PwmlrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMHFEN` reader - PWMH Falling Edge Enable"]
pub type PwmhfenR = crate::BitReader;
#[doc = "Field `PWMHFEN` writer - PWMH Falling Edge Enable"]
pub type PwmhfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMHREN` reader - PWMH Rising Edge Enable"]
pub type PwmhrenR = crate::BitReader;
#[doc = "Field `PWMHREN` writer - PWMH Rising Edge Enable"]
pub type PwmhrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Leading-Edge Blanking Delay for TRGINx"]
    #[inline(always)]
    pub fn lebdelay(&self) -> LebdelayR {
        LebdelayR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - PWML Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmlfen(&self) -> PwmlfenR {
        PwmlfenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWML Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmlren(&self) -> PwmlrenR {
        PwmlrenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PWMH Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmhfen(&self) -> PwmhfenR {
        PwmhfenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PWMH Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmhren(&self) -> PwmhrenR {
        PwmhrenR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Leading-Edge Blanking Delay for TRGINx"]
    #[inline(always)]
    pub fn lebdelay(&mut self) -> LebdelayW<Lebr1Spec> {
        LebdelayW::new(self, 0)
    }
    #[doc = "Bit 16 - PWML Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmlfen(&mut self) -> PwmlfenW<Lebr1Spec> {
        PwmlfenW::new(self, 16)
    }
    #[doc = "Bit 17 - PWML Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmlren(&mut self) -> PwmlrenW<Lebr1Spec> {
        PwmlrenW::new(self, 17)
    }
    #[doc = "Bit 18 - PWMH Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmhfen(&mut self) -> PwmhfenW<Lebr1Spec> {
        PwmhfenW::new(self, 18)
    }
    #[doc = "Bit 19 - PWMH Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmhren(&mut self) -> PwmhrenW<Lebr1Spec> {
        PwmhrenW::new(self, 19)
    }
}
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`lebr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lebr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lebr1Spec;
impl crate::RegisterSpec for Lebr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lebr1::R`](R) reader structure"]
impl crate::Readable for Lebr1Spec {}
#[doc = "`write(|w| ..)` method takes [`lebr1::W`](W) writer structure"]
impl crate::Writable for Lebr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEBR1 to value 0"]
impl crate::Resettable for Lebr1Spec {}
