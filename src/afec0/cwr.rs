#[doc = "Register `CWR` reader"]
pub type R = crate::R<CwrSpec>;
#[doc = "Register `CWR` writer"]
pub type W = crate::W<CwrSpec>;
#[doc = "Field `LOWTHRES` reader - Low Threshold"]
pub type LowthresR = crate::FieldReader<u16>;
#[doc = "Field `LOWTHRES` writer - Low Threshold"]
pub type LowthresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HIGHTHRES` reader - High Threshold"]
pub type HighthresR = crate::FieldReader<u16>;
#[doc = "Field `HIGHTHRES` writer - High Threshold"]
pub type HighthresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&self) -> LowthresR {
        LowthresR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&self) -> HighthresR {
        HighthresR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&mut self) -> LowthresW<CwrSpec> {
        LowthresW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&mut self) -> HighthresW<CwrSpec> {
        HighthresW::new(self, 16)
    }
}
#[doc = "AFEC Compare Window Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwrSpec;
impl crate::RegisterSpec for CwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwr::R`](R) reader structure"]
impl crate::Readable for CwrSpec {}
#[doc = "`write(|w| ..)` method takes [`cwr::W`](W) writer structure"]
impl crate::Writable for CwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CWR to value 0"]
impl crate::Resettable for CwrSpec {}
