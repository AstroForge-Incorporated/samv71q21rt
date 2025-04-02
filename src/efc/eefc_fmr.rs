#[doc = "Register `EEFC_FMR` reader"]
pub type R = crate::R<EefcFmrSpec>;
#[doc = "Register `EEFC_FMR` writer"]
pub type W = crate::W<EefcFmrSpec>;
#[doc = "Field `FRDY` reader - Flash Ready Interrupt Enable"]
pub type FrdyR = crate::BitReader;
#[doc = "Field `FRDY` writer - Flash Ready Interrupt Enable"]
pub type FrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWS` reader - Flash Wait State"]
pub type FwsR = crate::FieldReader;
#[doc = "Field `FWS` writer - Flash Wait State"]
pub type FwsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCOD` reader - Sequential Code Optimization Disable"]
pub type ScodR = crate::BitReader;
#[doc = "Field `SCOD` writer - Sequential Code Optimization Disable"]
pub type ScodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLOE` reader - Code Loop Optimization Enable"]
pub type CloeR = crate::BitReader;
#[doc = "Field `CLOE` writer - Code Loop Optimization Enable"]
pub type CloeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FwsR {
        FwsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&self) -> ScodR {
        ScodR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 26 - Code Loop Optimization Enable"]
    #[inline(always)]
    pub fn cloe(&self) -> CloeR {
        CloeR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&mut self) -> FrdyW<EefcFmrSpec> {
        FrdyW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&mut self) -> FwsW<EefcFmrSpec> {
        FwsW::new(self, 8)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&mut self) -> ScodW<EefcFmrSpec> {
        ScodW::new(self, 16)
    }
    #[doc = "Bit 26 - Code Loop Optimization Enable"]
    #[inline(always)]
    pub fn cloe(&mut self) -> CloeW<EefcFmrSpec> {
        CloeW::new(self, 26)
    }
}
#[doc = "EEFC Flash Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eefc_fmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefc_fmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EefcFmrSpec;
impl crate::RegisterSpec for EefcFmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefc_fmr::R`](R) reader structure"]
impl crate::Readable for EefcFmrSpec {}
#[doc = "`write(|w| ..)` method takes [`eefc_fmr::W`](W) writer structure"]
impl crate::Writable for EefcFmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEFC_FMR to value 0"]
impl crate::Resettable for EefcFmrSpec {}
