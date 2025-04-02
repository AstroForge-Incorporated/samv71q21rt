#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `RDERRE` reader - Remote Device Connection Error Interrupt Enable"]
pub type RderreR = crate::BitReader;
#[doc = "Field `RDERRE` writer - Remote Device Connection Error Interrupt Enable"]
pub type RderreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSHWC` reader - VBUS Hardware Control"]
pub type VbushwcR = crate::BitReader;
#[doc = "Field `VBUSHWC` writer - VBUS Hardware Control"]
pub type VbushwcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRZCLK` reader - Freeze USB Clock"]
pub type FrzclkR = crate::BitReader;
#[doc = "Field `FRZCLK` writer - Freeze USB Clock"]
pub type FrzclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBE` reader - USBHS Enable"]
pub type UsbeR = crate::BitReader;
#[doc = "Field `USBE` writer - USBHS Enable"]
pub type UsbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UID` reader - UID Pin Enable"]
pub type UidR = crate::BitReader;
#[doc = "Field `UID` writer - UID Pin Enable"]
pub type UidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USBHS Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uimodselect {
    #[doc = "0: The module is in USB Host mode."]
    Host = 0,
    #[doc = "1: The module is in USB Device mode."]
    Device = 1,
}
impl From<Uimodselect> for bool {
    #[inline(always)]
    fn from(variant: Uimodselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIMOD` reader - USBHS Mode"]
pub type UimodR = crate::BitReader<Uimodselect>;
impl UimodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uimodselect {
        match self.bits {
            false => Uimodselect::Host,
            true => Uimodselect::Device,
        }
    }
    #[doc = "The module is in USB Host mode."]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == Uimodselect::Host
    }
    #[doc = "The module is in USB Device mode."]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == Uimodselect::Device
    }
}
#[doc = "Field `UIMOD` writer - USBHS Mode"]
pub type UimodW<'a, REG> = crate::BitWriter<'a, REG, Uimodselect>;
impl<'a, REG> UimodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The module is in USB Host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(Uimodselect::Host)
    }
    #[doc = "The module is in USB Device mode."]
    #[inline(always)]
    pub fn device(self) -> &'a mut crate::W<REG> {
        self.variant(Uimodselect::Device)
    }
}
impl R {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn rderre(&self) -> RderreR {
        RderreR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&self) -> VbushwcR {
        VbushwcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FrzclkR {
        FrzclkR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> UsbeR {
        UsbeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - UID Pin Enable"]
    #[inline(always)]
    pub fn uid(&self) -> UidR {
        UidR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UimodR {
        UimodR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn rderre(&mut self) -> RderreW<CtrlSpec> {
        RderreW::new(self, 4)
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&mut self) -> VbushwcW<CtrlSpec> {
        VbushwcW::new(self, 8)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&mut self) -> FrzclkW<CtrlSpec> {
        FrzclkW::new(self, 14)
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline(always)]
    pub fn usbe(&mut self) -> UsbeW<CtrlSpec> {
        UsbeW::new(self, 15)
    }
    #[doc = "Bit 24 - UID Pin Enable"]
    #[inline(always)]
    pub fn uid(&mut self) -> UidW<CtrlSpec> {
        UidW::new(self, 24)
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline(always)]
    pub fn uimod(&mut self) -> UimodW<CtrlSpec> {
        UimodW::new(self, 25)
    }
}
#[doc = "General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
