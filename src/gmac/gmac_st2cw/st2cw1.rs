#[doc = "Register `ST2CW1` reader"]
pub type R = crate::R<St2cw1Spec>;
#[doc = "Register `ST2CW1` writer"]
pub type W = crate::W<St2cw1Spec>;
#[doc = "Field `OFFSVAL` reader - Offset Value in Bytes"]
pub type OffsvalR = crate::FieldReader;
#[doc = "Field `OFFSVAL` writer - Offset Value in Bytes"]
pub type OffsvalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Ethernet Frame Offset Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Offsstrtselect {
    #[doc = "0: Offset from the start of the frame"]
    Framestart = 0,
    #[doc = "1: Offset from the byte after the EtherType field"]
    Ethertype = 1,
    #[doc = "2: Offset from the byte after the IP header field"]
    Ip = 2,
    #[doc = "3: Offset from the byte after the TCP/UDP header field"]
    TcpUdp = 3,
}
impl From<Offsstrtselect> for u8 {
    #[inline(always)]
    fn from(variant: Offsstrtselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Offsstrtselect {
    type Ux = u8;
}
impl crate::IsEnum for Offsstrtselect {}
#[doc = "Field `OFFSSTRT` reader - Ethernet Frame Offset Start"]
pub type OffsstrtR = crate::FieldReader<Offsstrtselect>;
impl OffsstrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Offsstrtselect {
        match self.bits {
            0 => Offsstrtselect::Framestart,
            1 => Offsstrtselect::Ethertype,
            2 => Offsstrtselect::Ip,
            3 => Offsstrtselect::TcpUdp,
            _ => unreachable!(),
        }
    }
    #[doc = "Offset from the start of the frame"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        *self == Offsstrtselect::Framestart
    }
    #[doc = "Offset from the byte after the EtherType field"]
    #[inline(always)]
    pub fn is_ethertype(&self) -> bool {
        *self == Offsstrtselect::Ethertype
    }
    #[doc = "Offset from the byte after the IP header field"]
    #[inline(always)]
    pub fn is_ip(&self) -> bool {
        *self == Offsstrtselect::Ip
    }
    #[doc = "Offset from the byte after the TCP/UDP header field"]
    #[inline(always)]
    pub fn is_tcp_udp(&self) -> bool {
        *self == Offsstrtselect::TcpUdp
    }
}
#[doc = "Field `OFFSSTRT` writer - Ethernet Frame Offset Start"]
pub type OffsstrtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Offsstrtselect, crate::Safe>;
impl<'a, REG> OffsstrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Offset from the start of the frame"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut crate::W<REG> {
        self.variant(Offsstrtselect::Framestart)
    }
    #[doc = "Offset from the byte after the EtherType field"]
    #[inline(always)]
    pub fn ethertype(self) -> &'a mut crate::W<REG> {
        self.variant(Offsstrtselect::Ethertype)
    }
    #[doc = "Offset from the byte after the IP header field"]
    #[inline(always)]
    pub fn ip(self) -> &'a mut crate::W<REG> {
        self.variant(Offsstrtselect::Ip)
    }
    #[doc = "Offset from the byte after the TCP/UDP header field"]
    #[inline(always)]
    pub fn tcp_udp(self) -> &'a mut crate::W<REG> {
        self.variant(Offsstrtselect::TcpUdp)
    }
}
impl R {
    #[doc = "Bits 0:6 - Offset Value in Bytes"]
    #[inline(always)]
    pub fn offsval(&self) -> OffsvalR {
        OffsvalR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Ethernet Frame Offset Start"]
    #[inline(always)]
    pub fn offsstrt(&self) -> OffsstrtR {
        OffsstrtR::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Offset Value in Bytes"]
    #[inline(always)]
    pub fn offsval(&mut self) -> OffsvalW<St2cw1Spec> {
        OffsvalW::new(self, 0)
    }
    #[doc = "Bits 7:8 - Ethernet Frame Offset Start"]
    #[inline(always)]
    pub fn offsstrt(&mut self) -> OffsstrtW<St2cw1Spec> {
        OffsstrtW::new(self, 7)
    }
}
#[doc = "Screening Type 2 Compare Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`st2cw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2cw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St2cw1Spec;
impl crate::RegisterSpec for St2cw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2cw1::R`](R) reader structure"]
impl crate::Readable for St2cw1Spec {}
#[doc = "`write(|w| ..)` method takes [`st2cw1::W`](W) writer structure"]
impl crate::Writable for St2cw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST2CW1 to value 0"]
impl crate::Resettable for St2cw1Spec {}
