#[doc = "Register `CYCLE` reader"]
pub type R = crate::R<CycleSpec>;
#[doc = "Register `CYCLE` writer"]
pub type W = crate::W<CycleSpec>;
#[doc = "Field `NWE_CYCLE` reader - Total Write Cycle Length"]
pub type NweCycleR = crate::FieldReader<u16>;
#[doc = "Field `NWE_CYCLE` writer - Total Write Cycle Length"]
pub type NweCycleW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NRD_CYCLE` reader - Total Read Cycle Length"]
pub type NrdCycleR = crate::FieldReader<u16>;
#[doc = "Field `NRD_CYCLE` writer - Total Read Cycle Length"]
pub type NrdCycleW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&self) -> NweCycleR {
        NweCycleR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&self) -> NrdCycleR {
        NrdCycleR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&mut self) -> NweCycleW<CycleSpec> {
        NweCycleW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&mut self) -> NrdCycleW<CycleSpec> {
        NrdCycleW::new(self, 16)
    }
}
#[doc = "SMC Cycle Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cycle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cycle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CycleSpec;
impl crate::RegisterSpec for CycleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cycle::R`](R) reader structure"]
impl crate::Readable for CycleSpec {}
#[doc = "`write(|w| ..)` method takes [`cycle::W`](W) writer structure"]
impl crate::Writable for CycleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CYCLE to value 0"]
impl crate::Resettable for CycleSpec {}
