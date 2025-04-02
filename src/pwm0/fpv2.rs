#[doc = "Register `FPV2` reader"]
pub type R = crate::R<Fpv2Spec>;
#[doc = "Register `FPV2` writer"]
pub type W = crate::W<Fpv2Spec>;
#[doc = "Field `FPZH0` reader - Fault Protection to Hi-Z for PWMH output on channel 0"]
pub type Fpzh0R = crate::BitReader;
#[doc = "Field `FPZH0` writer - Fault Protection to Hi-Z for PWMH output on channel 0"]
pub type Fpzh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPZH1` reader - Fault Protection to Hi-Z for PWMH output on channel 1"]
pub type Fpzh1R = crate::BitReader;
#[doc = "Field `FPZH1` writer - Fault Protection to Hi-Z for PWMH output on channel 1"]
pub type Fpzh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPZH2` reader - Fault Protection to Hi-Z for PWMH output on channel 2"]
pub type Fpzh2R = crate::BitReader;
#[doc = "Field `FPZH2` writer - Fault Protection to Hi-Z for PWMH output on channel 2"]
pub type Fpzh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPZH3` reader - Fault Protection to Hi-Z for PWMH output on channel 3"]
pub type Fpzh3R = crate::BitReader;
#[doc = "Field `FPZH3` writer - Fault Protection to Hi-Z for PWMH output on channel 3"]
pub type Fpzh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPZL0` reader - Fault Protection to Hi-Z for PWML output on channel 0"]
pub type Fpzl0R = crate::BitReader;
#[doc = "Field `FPZL0` writer - Fault Protection to Hi-Z for PWML output on channel 0"]
pub type Fpzl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPZL1` reader - Fault Protection to Hi-Z for PWML output on channel 1"]
pub type Fpzl1R = crate::BitReader;
#[doc = "Field `FPZL1` writer - Fault Protection to Hi-Z for PWML output on channel 1"]
pub type Fpzl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPZL2` reader - Fault Protection to Hi-Z for PWML output on channel 2"]
pub type Fpzl2R = crate::BitReader;
#[doc = "Field `FPZL2` writer - Fault Protection to Hi-Z for PWML output on channel 2"]
pub type Fpzl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPZL3` reader - Fault Protection to Hi-Z for PWML output on channel 3"]
pub type Fpzl3R = crate::BitReader;
#[doc = "Field `FPZL3` writer - Fault Protection to Hi-Z for PWML output on channel 3"]
pub type Fpzl3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&self) -> Fpzh0R {
        Fpzh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&self) -> Fpzh1R {
        Fpzh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&self) -> Fpzh2R {
        Fpzh2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&self) -> Fpzh3R {
        Fpzh3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&self) -> Fpzl0R {
        Fpzl0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&self) -> Fpzl1R {
        Fpzl1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&self) -> Fpzl2R {
        Fpzl2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&self) -> Fpzl3R {
        Fpzl3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&mut self) -> Fpzh0W<Fpv2Spec> {
        Fpzh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&mut self) -> Fpzh1W<Fpv2Spec> {
        Fpzh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&mut self) -> Fpzh2W<Fpv2Spec> {
        Fpzh2W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&mut self) -> Fpzh3W<Fpv2Spec> {
        Fpzh3W::new(self, 3)
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&mut self) -> Fpzl0W<Fpv2Spec> {
        Fpzl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&mut self) -> Fpzl1W<Fpv2Spec> {
        Fpzl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&mut self) -> Fpzl2W<Fpv2Spec> {
        Fpzl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&mut self) -> Fpzl3W<Fpv2Spec> {
        Fpzl3W::new(self, 19)
    }
}
#[doc = "PWM Fault Protection Value 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fpv2Spec;
impl crate::RegisterSpec for Fpv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpv2::R`](R) reader structure"]
impl crate::Readable for Fpv2Spec {}
#[doc = "`write(|w| ..)` method takes [`fpv2::W`](W) writer structure"]
impl crate::Writable for Fpv2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPV2 to value 0"]
impl crate::Resettable for Fpv2Spec {}
