#[doc = "Register `MLBC1` reader"]
pub type R = crate::R<Mlbc1Spec>;
#[doc = "Register `MLBC1` writer"]
pub type W = crate::W<Mlbc1Spec>;
#[doc = "Field `LOCK` reader - MediaLB Lock Error Status (cleared by writing a 0)"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - MediaLB Lock Error Status (cleared by writing a 0)"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKM` reader - MediaLB Clock Missing Status (cleared by writing a 0)"]
pub type ClkmR = crate::BitReader;
#[doc = "Field `CLKM` writer - MediaLB Clock Missing Status (cleared by writing a 0)"]
pub type ClkmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDA` reader - Node Device Address"]
pub type NdaR = crate::FieldReader;
#[doc = "Field `NDA` writer - Node Device Address"]
pub type NdaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 6 - MediaLB Lock Error Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn clkm(&self) -> ClkmR {
        ClkmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Node Device Address"]
    #[inline(always)]
    pub fn nda(&self) -> NdaR {
        NdaR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - MediaLB Lock Error Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<Mlbc1Spec> {
        LockW::new(self, 6)
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn clkm(&mut self) -> ClkmW<Mlbc1Spec> {
        ClkmW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Node Device Address"]
    #[inline(always)]
    pub fn nda(&mut self) -> NdaW<Mlbc1Spec> {
        NdaW::new(self, 8)
    }
}
#[doc = "MediaLB Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mlbc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlbc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mlbc1Spec;
impl crate::RegisterSpec for Mlbc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mlbc1::R`](R) reader structure"]
impl crate::Readable for Mlbc1Spec {}
#[doc = "`write(|w| ..)` method takes [`mlbc1::W`](W) writer structure"]
impl crate::Writable for Mlbc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MLBC1 to value 0"]
impl crate::Resettable for Mlbc1Spec {}
