#[doc = "Register `CNDA` reader"]
pub type R = crate::R<CndaSpec>;
#[doc = "Register `CNDA` writer"]
pub type W = crate::W<CndaSpec>;
#[doc = "Field `NDAIF` reader - Channel x Next Descriptor Interface"]
pub type NdaifR = crate::BitReader;
#[doc = "Field `NDAIF` writer - Channel x Next Descriptor Interface"]
pub type NdaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDA` reader - Channel x Next Descriptor Address"]
pub type NdaR = crate::FieldReader<u32>;
#[doc = "Field `NDA` writer - Channel x Next Descriptor Address"]
pub type NdaW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    pub fn ndaif(&self) -> NdaifR {
        NdaifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    pub fn nda(&self) -> NdaR {
        NdaR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    pub fn ndaif(&mut self) -> NdaifW<CndaSpec> {
        NdaifW::new(self, 0)
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    pub fn nda(&mut self) -> NdaW<CndaSpec> {
        NdaW::new(self, 2)
    }
}
#[doc = "Channel Next Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CndaSpec;
impl crate::RegisterSpec for CndaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnda::R`](R) reader structure"]
impl crate::Readable for CndaSpec {}
#[doc = "`write(|w| ..)` method takes [`cnda::W`](W) writer structure"]
impl crate::Writable for CndaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNDA to value 0"]
impl crate::Resettable for CndaSpec {}
