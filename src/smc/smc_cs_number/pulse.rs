#[doc = "Register `PULSE` reader"]
pub type R = crate::R<PulseSpec>;
#[doc = "Register `PULSE` writer"]
pub type W = crate::W<PulseSpec>;
#[doc = "Field `NWE_PULSE` reader - NWE Pulse Length"]
pub type NwePulseR = crate::FieldReader;
#[doc = "Field `NWE_PULSE` writer - NWE Pulse Length"]
pub type NwePulseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NCS_WR_PULSE` reader - NCS Pulse Length in WRITE Access"]
pub type NcsWrPulseR = crate::FieldReader;
#[doc = "Field `NCS_WR_PULSE` writer - NCS Pulse Length in WRITE Access"]
pub type NcsWrPulseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NRD_PULSE` reader - NRD Pulse Length"]
pub type NrdPulseR = crate::FieldReader;
#[doc = "Field `NRD_PULSE` writer - NRD Pulse Length"]
pub type NrdPulseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NCS_RD_PULSE` reader - NCS Pulse Length in READ Access"]
pub type NcsRdPulseR = crate::FieldReader;
#[doc = "Field `NCS_RD_PULSE` writer - NCS Pulse Length in READ Access"]
pub type NcsRdPulseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    pub fn nwe_pulse(&self) -> NwePulseR {
        NwePulseR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_pulse(&self) -> NcsWrPulseR {
        NcsWrPulseR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    pub fn nrd_pulse(&self) -> NrdPulseR {
        NrdPulseR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_pulse(&self) -> NcsRdPulseR {
        NcsRdPulseR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    pub fn nwe_pulse(&mut self) -> NwePulseW<PulseSpec> {
        NwePulseW::new(self, 0)
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_pulse(&mut self) -> NcsWrPulseW<PulseSpec> {
        NcsWrPulseW::new(self, 8)
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    pub fn nrd_pulse(&mut self) -> NrdPulseW<PulseSpec> {
        NrdPulseW::new(self, 16)
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_pulse(&mut self) -> NcsRdPulseW<PulseSpec> {
        NcsRdPulseW::new(self, 24)
    }
}
#[doc = "SMC Pulse Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PulseSpec;
impl crate::RegisterSpec for PulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pulse::R`](R) reader structure"]
impl crate::Readable for PulseSpec {}
#[doc = "`write(|w| ..)` method takes [`pulse::W`](W) writer structure"]
impl crate::Writable for PulseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PULSE to value 0"]
impl crate::Resettable for PulseSpec {}
