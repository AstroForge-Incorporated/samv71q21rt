#[doc = "Register `HCTL` reader"]
pub type R = crate::R<HctlSpec>;
#[doc = "Register `HCTL` writer"]
pub type W = crate::W<HctlSpec>;
#[doc = "Field `RST0` reader - Address Generation Unit 0 Software Reset"]
pub type Rst0R = crate::BitReader;
#[doc = "Field `RST0` writer - Address Generation Unit 0 Software Reset"]
pub type Rst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST1` reader - Address Generation Unit 1 Software Reset"]
pub type Rst1R = crate::BitReader;
#[doc = "Field `RST1` writer - Address Generation Unit 1 Software Reset"]
pub type Rst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - HBI Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - HBI Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    pub fn rst0(&self) -> Rst0R {
        Rst0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    pub fn rst1(&self) -> Rst1R {
        Rst1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    pub fn rst0(&mut self) -> Rst0W<HctlSpec> {
        Rst0W::new(self, 0)
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    pub fn rst1(&mut self) -> Rst1W<HctlSpec> {
        Rst1W::new(self, 1)
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<HctlSpec> {
        EnW::new(self, 15)
    }
}
#[doc = "HBI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HctlSpec;
impl crate::RegisterSpec for HctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctl::R`](R) reader structure"]
impl crate::Readable for HctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hctl::W`](W) writer structure"]
impl crate::Writable for HctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCTL to value 0"]
impl crate::Resettable for HctlSpec {}
