#[doc = "Register `HSTDMASTATUS` reader"]
pub type R = crate::R<HstdmastatusSpec>;
#[doc = "Register `HSTDMASTATUS` writer"]
pub type W = crate::W<HstdmastatusSpec>;
#[doc = "Field `CHANN_ENB` reader - Channel Enable Status"]
pub type ChannEnbR = crate::BitReader;
#[doc = "Field `CHANN_ENB` writer - Channel Enable Status"]
pub type ChannEnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANN_ACT` reader - Channel Active Status"]
pub type ChannActR = crate::BitReader;
#[doc = "Field `CHANN_ACT` writer - Channel Active Status"]
pub type ChannActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_TR_ST` reader - End of Channel Transfer Status"]
pub type EndTrStR = crate::BitReader;
#[doc = "Field `END_TR_ST` writer - End of Channel Transfer Status"]
pub type EndTrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_BF_ST` reader - End of Channel Buffer Status"]
pub type EndBfStR = crate::BitReader;
#[doc = "Field `END_BF_ST` writer - End of Channel Buffer Status"]
pub type EndBfStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LDST` reader - Descriptor Loaded Status"]
pub type DescLdstR = crate::BitReader;
#[doc = "Field `DESC_LDST` writer - Descriptor Loaded Status"]
pub type DescLdstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_COUNT` reader - Buffer Byte Count"]
pub type BuffCountR = crate::FieldReader<u16>;
#[doc = "Field `BUFF_COUNT` writer - Buffer Byte Count"]
pub type BuffCountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    pub fn chann_enb(&self) -> ChannEnbR {
        ChannEnbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    pub fn chann_act(&self) -> ChannActR {
        ChannActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    pub fn end_tr_st(&self) -> EndTrStR {
        EndTrStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    pub fn end_bf_st(&self) -> EndBfStR {
        EndBfStR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    pub fn desc_ldst(&self) -> DescLdstR {
        DescLdstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    pub fn buff_count(&self) -> BuffCountR {
        BuffCountR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    pub fn chann_enb(&mut self) -> ChannEnbW<HstdmastatusSpec> {
        ChannEnbW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    pub fn chann_act(&mut self) -> ChannActW<HstdmastatusSpec> {
        ChannActW::new(self, 1)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    pub fn end_tr_st(&mut self) -> EndTrStW<HstdmastatusSpec> {
        EndTrStW::new(self, 4)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    pub fn end_bf_st(&mut self) -> EndBfStW<HstdmastatusSpec> {
        EndBfStW::new(self, 5)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    pub fn desc_ldst(&mut self) -> DescLdstW<HstdmastatusSpec> {
        DescLdstW::new(self, 6)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    pub fn buff_count(&mut self) -> BuffCountW<HstdmastatusSpec> {
        BuffCountW::new(self, 16)
    }
}
#[doc = "Host DMA Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmastatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmastatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstdmastatusSpec;
impl crate::RegisterSpec for HstdmastatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstdmastatus::R`](R) reader structure"]
impl crate::Readable for HstdmastatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hstdmastatus::W`](W) writer structure"]
impl crate::Writable for HstdmastatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTDMASTATUS to value 0"]
impl crate::Resettable for HstdmastatusSpec {}
