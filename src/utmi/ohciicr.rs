#[doc = "Register `OHCIICR` reader"]
pub type R = crate::R<OhciicrSpec>;
#[doc = "Register `OHCIICR` writer"]
pub type W = crate::W<OhciicrSpec>;
#[doc = "Field `RES0` reader - USB PORTx Reset"]
pub type Res0R = crate::BitReader;
#[doc = "Field `RES0` writer - USB PORTx Reset"]
pub type Res0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARIE` reader - OHCI Asynchronous Resume Interrupt Enable"]
pub type ArieR = crate::BitReader;
#[doc = "Field `ARIE` writer - OHCI Asynchronous Resume Interrupt Enable"]
pub type ArieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPSTART` reader - "]
pub type AppstartR = crate::BitReader;
#[doc = "Field `APPSTART` writer - "]
pub type AppstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDPPUDIS` reader - USB Device Pull-up Disable"]
pub type UdppudisR = crate::BitReader;
#[doc = "Field `UDPPUDIS` writer - USB Device Pull-up Disable"]
pub type UdppudisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB PORTx Reset"]
    #[inline(always)]
    pub fn res0(&self) -> Res0R {
        Res0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - OHCI Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn arie(&self) -> ArieR {
        ArieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn appstart(&self) -> AppstartR {
        AppstartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 23 - USB Device Pull-up Disable"]
    #[inline(always)]
    pub fn udppudis(&self) -> UdppudisR {
        UdppudisR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PORTx Reset"]
    #[inline(always)]
    pub fn res0(&mut self) -> Res0W<OhciicrSpec> {
        Res0W::new(self, 0)
    }
    #[doc = "Bit 4 - OHCI Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn arie(&mut self) -> ArieW<OhciicrSpec> {
        ArieW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn appstart(&mut self) -> AppstartW<OhciicrSpec> {
        AppstartW::new(self, 5)
    }
    #[doc = "Bit 23 - USB Device Pull-up Disable"]
    #[inline(always)]
    pub fn udppudis(&mut self) -> UdppudisW<OhciicrSpec> {
        UdppudisW::new(self, 23)
    }
}
#[doc = "OHCI Interrupt Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ohciicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ohciicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OhciicrSpec;
impl crate::RegisterSpec for OhciicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ohciicr::R`](R) reader structure"]
impl crate::Readable for OhciicrSpec {}
#[doc = "`write(|w| ..)` method takes [`ohciicr::W`](W) writer structure"]
impl crate::Writable for OhciicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OHCIICR to value 0"]
impl crate::Resettable for OhciicrSpec {}
