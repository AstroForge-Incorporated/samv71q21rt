#[doc = "Register `WOL` reader"]
pub type R = crate::R<WolSpec>;
#[doc = "Register `WOL` writer"]
pub type W = crate::W<WolSpec>;
#[doc = "Field `IP` reader - ARP Request IP Address"]
pub type IpR = crate::FieldReader<u16>;
#[doc = "Field `IP` writer - ARP Request IP Address"]
pub type IpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MAG` reader - Magic Packet Event Enable"]
pub type MagR = crate::BitReader;
#[doc = "Field `MAG` writer - Magic Packet Event Enable"]
pub type MagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARP` reader - ARP Request IP Address"]
pub type ArpR = crate::BitReader;
#[doc = "Field `ARP` writer - ARP Request IP Address"]
pub type ArpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA1` reader - Specific Address Register 1 Event Enable"]
pub type Sa1R = crate::BitReader;
#[doc = "Field `SA1` writer - Specific Address Register 1 Event Enable"]
pub type Sa1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTI` reader - Multicast Hash Event Enable"]
pub type MtiR = crate::BitReader;
#[doc = "Field `MTI` writer - Multicast Hash Event Enable"]
pub type MtiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ARP Request IP Address"]
    #[inline(always)]
    pub fn ip(&self) -> IpR {
        IpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Magic Packet Event Enable"]
    #[inline(always)]
    pub fn mag(&self) -> MagR {
        MagR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ARP Request IP Address"]
    #[inline(always)]
    pub fn arp(&self) -> ArpR {
        ArpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Specific Address Register 1 Event Enable"]
    #[inline(always)]
    pub fn sa1(&self) -> Sa1R {
        Sa1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Multicast Hash Event Enable"]
    #[inline(always)]
    pub fn mti(&self) -> MtiR {
        MtiR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARP Request IP Address"]
    #[inline(always)]
    pub fn ip(&mut self) -> IpW<WolSpec> {
        IpW::new(self, 0)
    }
    #[doc = "Bit 16 - Magic Packet Event Enable"]
    #[inline(always)]
    pub fn mag(&mut self) -> MagW<WolSpec> {
        MagW::new(self, 16)
    }
    #[doc = "Bit 17 - ARP Request IP Address"]
    #[inline(always)]
    pub fn arp(&mut self) -> ArpW<WolSpec> {
        ArpW::new(self, 17)
    }
    #[doc = "Bit 18 - Specific Address Register 1 Event Enable"]
    #[inline(always)]
    pub fn sa1(&mut self) -> Sa1W<WolSpec> {
        Sa1W::new(self, 18)
    }
    #[doc = "Bit 19 - Multicast Hash Event Enable"]
    #[inline(always)]
    pub fn mti(&mut self) -> MtiW<WolSpec> {
        MtiW::new(self, 19)
    }
}
#[doc = "Wake on LAN Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WolSpec;
impl crate::RegisterSpec for WolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wol::R`](R) reader structure"]
impl crate::Readable for WolSpec {}
#[doc = "`write(|w| ..)` method takes [`wol::W`](W) writer structure"]
impl crate::Writable for WolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WOL to value 0"]
impl crate::Resettable for WolSpec {}
