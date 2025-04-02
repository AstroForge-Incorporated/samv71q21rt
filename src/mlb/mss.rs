#[doc = "Register `MSS` reader"]
pub type R = crate::R<MssSpec>;
#[doc = "Register `MSS` writer"]
pub type W = crate::W<MssSpec>;
#[doc = "Field `RSTSYSCMD` reader - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type RstsyscmdR = crate::BitReader;
#[doc = "Field `RSTSYSCMD` writer - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type RstsyscmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKSYSCMD` reader - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type LksyscmdR = crate::BitReader;
#[doc = "Field `LKSYSCMD` writer - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type LksyscmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULKSYSCMD` reader - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type UlksyscmdR = crate::BitReader;
#[doc = "Field `ULKSYSCMD` writer - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type UlksyscmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSYSCMD` reader - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type CssyscmdR = crate::BitReader;
#[doc = "Field `CSSYSCMD` writer - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type CssyscmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWSYSCMD` reader - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type SwsyscmdR = crate::BitReader;
#[doc = "Field `SWSYSCMD` writer - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type SwsyscmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERVREQ` reader - Service Request Enabled"]
pub type ServreqR = crate::BitReader;
#[doc = "Field `SERVREQ` writer - Service Request Enabled"]
pub type ServreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn rstsyscmd(&self) -> RstsyscmdR {
        RstsyscmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lksyscmd(&self) -> LksyscmdR {
        LksyscmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn ulksyscmd(&self) -> UlksyscmdR {
        UlksyscmdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn cssyscmd(&self) -> CssyscmdR {
        CssyscmdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn swsyscmd(&self) -> SwsyscmdR {
        SwsyscmdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Service Request Enabled"]
    #[inline(always)]
    pub fn servreq(&self) -> ServreqR {
        ServreqR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn rstsyscmd(&mut self) -> RstsyscmdW<MssSpec> {
        RstsyscmdW::new(self, 0)
    }
    #[doc = "Bit 1 - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lksyscmd(&mut self) -> LksyscmdW<MssSpec> {
        LksyscmdW::new(self, 1)
    }
    #[doc = "Bit 2 - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn ulksyscmd(&mut self) -> UlksyscmdW<MssSpec> {
        UlksyscmdW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn cssyscmd(&mut self) -> CssyscmdW<MssSpec> {
        CssyscmdW::new(self, 3)
    }
    #[doc = "Bit 4 - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn swsyscmd(&mut self) -> SwsyscmdW<MssSpec> {
        SwsyscmdW::new(self, 4)
    }
    #[doc = "Bit 5 - Service Request Enabled"]
    #[inline(always)]
    pub fn servreq(&mut self) -> ServreqW<MssSpec> {
        ServreqW::new(self, 5)
    }
}
#[doc = "MediaLB System Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mss::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssSpec;
impl crate::RegisterSpec for MssSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss::R`](R) reader structure"]
impl crate::Readable for MssSpec {}
#[doc = "`write(|w| ..)` method takes [`mss::W`](W) writer structure"]
impl crate::Writable for MssSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSS to value 0"]
impl crate::Resettable for MssSpec {}
