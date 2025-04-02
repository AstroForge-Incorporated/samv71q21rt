#[doc = "Register `CMPM` reader"]
pub type R = crate::R<CmpmSpec>;
#[doc = "Register `CMPM` writer"]
pub type W = crate::W<CmpmSpec>;
#[doc = "Field `CEN` reader - Comparison x Enable"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Comparison x Enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR` reader - Comparison x Trigger"]
pub type CtrR = crate::FieldReader;
#[doc = "Field `CTR` writer - Comparison x Trigger"]
pub type CtrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPR` reader - Comparison x Period"]
pub type CprR = crate::FieldReader;
#[doc = "Field `CPR` writer - Comparison x Period"]
pub type CprW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPRCNT` reader - Comparison x Period Counter"]
pub type CprcntR = crate::FieldReader;
#[doc = "Field `CPRCNT` writer - Comparison x Period Counter"]
pub type CprcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CUPR` reader - Comparison x Update Period"]
pub type CuprR = crate::FieldReader;
#[doc = "Field `CUPR` writer - Comparison x Update Period"]
pub type CuprW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CUPRCNT` reader - Comparison x Update Period Counter"]
pub type CuprcntR = crate::FieldReader;
#[doc = "Field `CUPRCNT` writer - Comparison x Update Period Counter"]
pub type CuprcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    pub fn cpr(&self) -> CprR {
        CprR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    pub fn cprcnt(&self) -> CprcntR {
        CprcntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    pub fn cupr(&self) -> CuprR {
        CuprR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    pub fn cuprcnt(&self) -> CuprcntR {
        CuprcntR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<CmpmSpec> {
        CenW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CtrW<CmpmSpec> {
        CtrW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    pub fn cpr(&mut self) -> CprW<CmpmSpec> {
        CprW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    pub fn cprcnt(&mut self) -> CprcntW<CmpmSpec> {
        CprcntW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    pub fn cupr(&mut self) -> CuprW<CmpmSpec> {
        CuprW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    pub fn cuprcnt(&mut self) -> CuprcntW<CmpmSpec> {
        CuprcntW::new(self, 20)
    }
}
#[doc = "PWM Comparison 0 Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpmSpec;
impl crate::RegisterSpec for CmpmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpm::R`](R) reader structure"]
impl crate::Readable for CmpmSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpm::W`](W) writer structure"]
impl crate::Writable for CmpmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPM to value 0"]
impl crate::Resettable for CmpmSpec {}
