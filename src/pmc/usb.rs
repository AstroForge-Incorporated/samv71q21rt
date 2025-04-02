#[doc = "Register `USB` reader"]
pub type R = crate::R<UsbSpec>;
#[doc = "Register `USB` writer"]
pub type W = crate::W<UsbSpec>;
#[doc = "Field `USBS` reader - USB Input Clock Selection"]
pub type UsbsR = crate::BitReader;
#[doc = "Field `USBS` writer - USB Input Clock Selection"]
pub type UsbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBDIV` reader - Divider for USB_48M"]
pub type UsbdivR = crate::FieldReader;
#[doc = "Field `USBDIV` writer - Divider for USB_48M"]
pub type UsbdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - USB Input Clock Selection"]
    #[inline(always)]
    pub fn usbs(&self) -> UsbsR {
        UsbsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Divider for USB_48M"]
    #[inline(always)]
    pub fn usbdiv(&self) -> UsbdivR {
        UsbdivR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB Input Clock Selection"]
    #[inline(always)]
    pub fn usbs(&mut self) -> UsbsW<UsbSpec> {
        UsbsW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Divider for USB_48M"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> UsbdivW<UsbSpec> {
        UsbdivW::new(self, 8)
    }
}
#[doc = "USB Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbSpec;
impl crate::RegisterSpec for UsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb::R`](R) reader structure"]
impl crate::Readable for UsbSpec {}
#[doc = "`write(|w| ..)` method takes [`usb::W`](W) writer structure"]
impl crate::Writable for UsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB to value 0"]
impl crate::Resettable for UsbSpec {}
