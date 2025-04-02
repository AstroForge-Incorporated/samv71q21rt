#[doc = "Register `FPV1` reader"]
pub type R = crate::R<Fpv1Spec>;
#[doc = "Register `FPV1` writer"]
pub type W = crate::W<Fpv1Spec>;
#[doc = "Field `FPVH0` reader - Fault Protection Value for PWMH output on channel 0"]
pub type Fpvh0R = crate::BitReader;
#[doc = "Field `FPVH0` writer - Fault Protection Value for PWMH output on channel 0"]
pub type Fpvh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVH1` reader - Fault Protection Value for PWMH output on channel 1"]
pub type Fpvh1R = crate::BitReader;
#[doc = "Field `FPVH1` writer - Fault Protection Value for PWMH output on channel 1"]
pub type Fpvh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVH2` reader - Fault Protection Value for PWMH output on channel 2"]
pub type Fpvh2R = crate::BitReader;
#[doc = "Field `FPVH2` writer - Fault Protection Value for PWMH output on channel 2"]
pub type Fpvh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVH3` reader - Fault Protection Value for PWMH output on channel 3"]
pub type Fpvh3R = crate::BitReader;
#[doc = "Field `FPVH3` writer - Fault Protection Value for PWMH output on channel 3"]
pub type Fpvh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL0` reader - Fault Protection Value for PWML output on channel 0"]
pub type Fpvl0R = crate::BitReader;
#[doc = "Field `FPVL0` writer - Fault Protection Value for PWML output on channel 0"]
pub type Fpvl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL1` reader - Fault Protection Value for PWML output on channel 1"]
pub type Fpvl1R = crate::BitReader;
#[doc = "Field `FPVL1` writer - Fault Protection Value for PWML output on channel 1"]
pub type Fpvl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL2` reader - Fault Protection Value for PWML output on channel 2"]
pub type Fpvl2R = crate::BitReader;
#[doc = "Field `FPVL2` writer - Fault Protection Value for PWML output on channel 2"]
pub type Fpvl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL3` reader - Fault Protection Value for PWML output on channel 3"]
pub type Fpvl3R = crate::BitReader;
#[doc = "Field `FPVL3` writer - Fault Protection Value for PWML output on channel 3"]
pub type Fpvl3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&self) -> Fpvh0R {
        Fpvh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&self) -> Fpvh1R {
        Fpvh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&self) -> Fpvh2R {
        Fpvh2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&self) -> Fpvh3R {
        Fpvh3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&self) -> Fpvl0R {
        Fpvl0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&self) -> Fpvl1R {
        Fpvl1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&self) -> Fpvl2R {
        Fpvl2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&self) -> Fpvl3R {
        Fpvl3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&mut self) -> Fpvh0W<Fpv1Spec> {
        Fpvh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&mut self) -> Fpvh1W<Fpv1Spec> {
        Fpvh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&mut self) -> Fpvh2W<Fpv1Spec> {
        Fpvh2W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&mut self) -> Fpvh3W<Fpv1Spec> {
        Fpvh3W::new(self, 3)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&mut self) -> Fpvl0W<Fpv1Spec> {
        Fpvl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&mut self) -> Fpvl1W<Fpv1Spec> {
        Fpvl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&mut self) -> Fpvl2W<Fpv1Spec> {
        Fpvl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&mut self) -> Fpvl3W<Fpv1Spec> {
        Fpvl3W::new(self, 19)
    }
}
#[doc = "PWM Fault Protection Value Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fpv1Spec;
impl crate::RegisterSpec for Fpv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpv1::R`](R) reader structure"]
impl crate::Readable for Fpv1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpv1::W`](W) writer structure"]
impl crate::Writable for Fpv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPV1 to value 0"]
impl crate::Resettable for Fpv1Spec {}
