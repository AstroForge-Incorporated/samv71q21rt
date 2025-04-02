#[doc = "Register `CDS_MSP` reader"]
pub type R = crate::R<CdsMspSpec>;
#[doc = "Register `CDS_MSP` writer"]
pub type W = crate::W<CdsMspSpec>;
#[doc = "Field `SDS_MSP` reader - Channel x Source Data stride or Memory Set Pattern"]
pub type SdsMspR = crate::FieldReader<u16>;
#[doc = "Field `SDS_MSP` writer - Channel x Source Data stride or Memory Set Pattern"]
pub type SdsMspW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DDS_MSP` reader - Channel x Destination Data Stride or Memory Set Pattern"]
pub type DdsMspR = crate::FieldReader<u16>;
#[doc = "Field `DDS_MSP` writer - Channel x Destination Data Stride or Memory Set Pattern"]
pub type DdsMspW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel x Source Data stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn sds_msp(&self) -> SdsMspR {
        SdsMspR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Channel x Destination Data Stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn dds_msp(&self) -> DdsMspR {
        DdsMspR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel x Source Data stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn sds_msp(&mut self) -> SdsMspW<CdsMspSpec> {
        SdsMspW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Channel x Destination Data Stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn dds_msp(&mut self) -> DdsMspW<CdsMspSpec> {
        DdsMspW::new(self, 16)
    }
}
#[doc = "Channel Data Stride Memory Set Pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`cds_msp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cds_msp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdsMspSpec;
impl crate::RegisterSpec for CdsMspSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cds_msp::R`](R) reader structure"]
impl crate::Readable for CdsMspSpec {}
#[doc = "`write(|w| ..)` method takes [`cds_msp::W`](W) writer structure"]
impl crate::Writable for CdsMspSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDS_MSP to value 0"]
impl crate::Resettable for CdsMspSpec {}
