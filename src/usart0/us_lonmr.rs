#[doc = "Register `US_LONMR` reader"]
pub type R = crate::R<UsLonmrSpec>;
#[doc = "Register `US_LONMR` writer"]
pub type W = crate::W<UsLonmrSpec>;
#[doc = "Field `COMMT` reader - LON comm_type Parameter Value"]
pub type CommtR = crate::BitReader;
#[doc = "Field `COMMT` writer - LON comm_type Parameter Value"]
pub type CommtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLDET` reader - LON Collision Detection Feature"]
pub type ColdetR = crate::BitReader;
#[doc = "Field `COLDET` writer - LON Collision Detection Feature"]
pub type ColdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCOL` reader - Terminate Frame upon Collision Notification"]
pub type TcolR = crate::BitReader;
#[doc = "Field `TCOL` writer - Terminate Frame upon Collision Notification"]
pub type TcolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTAIL` reader - LON Collision Detection on Frame Tail"]
pub type CdtailR = crate::BitReader;
#[doc = "Field `CDTAIL` writer - LON Collision Detection on Frame Tail"]
pub type CdtailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAM` reader - LON DMA Mode"]
pub type DmamR = crate::BitReader;
#[doc = "Field `DMAM` writer - LON DMA Mode"]
pub type DmamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS` reader - LON Collision Detection Source"]
pub type LcdsR = crate::BitReader;
#[doc = "Field `LCDS` writer - LON Collision Detection Source"]
pub type LcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOFS` reader - End of Frame Condition Size"]
pub type EofsR = crate::FieldReader;
#[doc = "Field `EOFS` writer - End of Frame Condition Size"]
pub type EofsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - LON comm_type Parameter Value"]
    #[inline(always)]
    pub fn commt(&self) -> CommtR {
        CommtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LON Collision Detection Feature"]
    #[inline(always)]
    pub fn coldet(&self) -> ColdetR {
        ColdetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Terminate Frame upon Collision Notification"]
    #[inline(always)]
    pub fn tcol(&self) -> TcolR {
        TcolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LON Collision Detection on Frame Tail"]
    #[inline(always)]
    pub fn cdtail(&self) -> CdtailR {
        CdtailR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LON DMA Mode"]
    #[inline(always)]
    pub fn dmam(&self) -> DmamR {
        DmamR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LON Collision Detection Source"]
    #[inline(always)]
    pub fn lcds(&self) -> LcdsR {
        LcdsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:23 - End of Frame Condition Size"]
    #[inline(always)]
    pub fn eofs(&self) -> EofsR {
        EofsR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LON comm_type Parameter Value"]
    #[inline(always)]
    pub fn commt(&mut self) -> CommtW<UsLonmrSpec> {
        CommtW::new(self, 0)
    }
    #[doc = "Bit 1 - LON Collision Detection Feature"]
    #[inline(always)]
    pub fn coldet(&mut self) -> ColdetW<UsLonmrSpec> {
        ColdetW::new(self, 1)
    }
    #[doc = "Bit 2 - Terminate Frame upon Collision Notification"]
    #[inline(always)]
    pub fn tcol(&mut self) -> TcolW<UsLonmrSpec> {
        TcolW::new(self, 2)
    }
    #[doc = "Bit 3 - LON Collision Detection on Frame Tail"]
    #[inline(always)]
    pub fn cdtail(&mut self) -> CdtailW<UsLonmrSpec> {
        CdtailW::new(self, 3)
    }
    #[doc = "Bit 4 - LON DMA Mode"]
    #[inline(always)]
    pub fn dmam(&mut self) -> DmamW<UsLonmrSpec> {
        DmamW::new(self, 4)
    }
    #[doc = "Bit 5 - LON Collision Detection Source"]
    #[inline(always)]
    pub fn lcds(&mut self) -> LcdsW<UsLonmrSpec> {
        LcdsW::new(self, 5)
    }
    #[doc = "Bits 16:23 - End of Frame Condition Size"]
    #[inline(always)]
    pub fn eofs(&mut self) -> EofsW<UsLonmrSpec> {
        EofsW::new(self, 16)
    }
}
#[doc = "LON Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsLonmrSpec;
impl crate::RegisterSpec for UsLonmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonmr::R`](R) reader structure"]
impl crate::Readable for UsLonmrSpec {}
#[doc = "`write(|w| ..)` method takes [`us_lonmr::W`](W) writer structure"]
impl crate::Writable for UsLonmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_LONMR to value 0"]
impl crate::Resettable for UsLonmrSpec {}
