#[doc = "Register `MCTL` reader"]
pub type R = crate::R<MctlSpec>;
#[doc = "Register `MCTL` writer"]
pub type W = crate::W<MctlSpec>;
#[doc = "Field `XCMP` reader - Transfer Complete (Write 0 to Clear)"]
pub type XcmpR = crate::BitReader;
#[doc = "Field `XCMP` writer - Transfer Complete (Write 0 to Clear)"]
pub type XcmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Complete (Write 0 to Clear)"]
    #[inline(always)]
    pub fn xcmp(&self) -> XcmpR {
        XcmpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete (Write 0 to Clear)"]
    #[inline(always)]
    pub fn xcmp(&mut self) -> XcmpW<MctlSpec> {
        XcmpW::new(self, 0)
    }
}
#[doc = "MIF Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctlSpec;
impl crate::RegisterSpec for MctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctl::R`](R) reader structure"]
impl crate::Readable for MctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mctl::W`](W) writer structure"]
impl crate::Writable for MctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCTL to value 0"]
impl crate::Resettable for MctlSpec {}
