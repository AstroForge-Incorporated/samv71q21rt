#[doc = "Register `ST2RPQ[%s]` reader"]
pub type R = crate::R<St2rpqSpec>;
#[doc = "Register `ST2RPQ[%s]` writer"]
pub type W = crate::W<St2rpqSpec>;
#[doc = "Field `QNB` reader - Queue Number (0-5)"]
pub type QnbR = crate::FieldReader;
#[doc = "Field `QNB` writer - Queue Number (0-5)"]
pub type QnbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VLANP` reader - VLAN Priority"]
pub type VlanpR = crate::FieldReader;
#[doc = "Field `VLANP` writer - VLAN Priority"]
pub type VlanpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VLANE` reader - VLAN Enable"]
pub type VlaneR = crate::BitReader;
#[doc = "Field `VLANE` writer - VLAN Enable"]
pub type VlaneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2ETH` reader - Index of Screening Type 2 EtherType register x"]
pub type I2ethR = crate::FieldReader;
#[doc = "Field `I2ETH` writer - Index of Screening Type 2 EtherType register x"]
pub type I2ethW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETHE` reader - EtherType Enable"]
pub type EtheR = crate::BitReader;
#[doc = "Field `ETHE` writer - EtherType Enable"]
pub type EtheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPA` reader - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type CompaR = crate::FieldReader;
#[doc = "Field `COMPA` writer - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type CompaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COMPAE` reader - Compare A Enable"]
pub type CompaeR = crate::BitReader;
#[doc = "Field `COMPAE` writer - Compare A Enable"]
pub type CompaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPB` reader - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type CompbR = crate::FieldReader;
#[doc = "Field `COMPB` writer - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type CompbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COMPBE` reader - Compare B Enable"]
pub type CompbeR = crate::BitReader;
#[doc = "Field `COMPBE` writer - Compare B Enable"]
pub type CompbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPC` reader - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type CompcR = crate::FieldReader;
#[doc = "Field `COMPC` writer - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type CompcW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COMPCE` reader - Compare C Enable"]
pub type CompceR = crate::BitReader;
#[doc = "Field `COMPCE` writer - Compare C Enable"]
pub type CompceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&self) -> QnbR {
        QnbR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - VLAN Priority"]
    #[inline(always)]
    pub fn vlanp(&self) -> VlanpR {
        VlanpR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - VLAN Enable"]
    #[inline(always)]
    pub fn vlane(&self) -> VlaneR {
        VlaneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Index of Screening Type 2 EtherType register x"]
    #[inline(always)]
    pub fn i2eth(&self) -> I2ethR {
        I2ethR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - EtherType Enable"]
    #[inline(always)]
    pub fn ethe(&self) -> EtheR {
        EtheR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compa(&self) -> CompaR {
        CompaR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - Compare A Enable"]
    #[inline(always)]
    pub fn compae(&self) -> CompaeR {
        CompaeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compb(&self) -> CompbR {
        CompbR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Compare B Enable"]
    #[inline(always)]
    pub fn compbe(&self) -> CompbeR {
        CompbeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compc(&self) -> CompcR {
        CompcR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Compare C Enable"]
    #[inline(always)]
    pub fn compce(&self) -> CompceR {
        CompceR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&mut self) -> QnbW<St2rpqSpec> {
        QnbW::new(self, 0)
    }
    #[doc = "Bits 4:6 - VLAN Priority"]
    #[inline(always)]
    pub fn vlanp(&mut self) -> VlanpW<St2rpqSpec> {
        VlanpW::new(self, 4)
    }
    #[doc = "Bit 8 - VLAN Enable"]
    #[inline(always)]
    pub fn vlane(&mut self) -> VlaneW<St2rpqSpec> {
        VlaneW::new(self, 8)
    }
    #[doc = "Bits 9:11 - Index of Screening Type 2 EtherType register x"]
    #[inline(always)]
    pub fn i2eth(&mut self) -> I2ethW<St2rpqSpec> {
        I2ethW::new(self, 9)
    }
    #[doc = "Bit 12 - EtherType Enable"]
    #[inline(always)]
    pub fn ethe(&mut self) -> EtheW<St2rpqSpec> {
        EtheW::new(self, 12)
    }
    #[doc = "Bits 13:17 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compa(&mut self) -> CompaW<St2rpqSpec> {
        CompaW::new(self, 13)
    }
    #[doc = "Bit 18 - Compare A Enable"]
    #[inline(always)]
    pub fn compae(&mut self) -> CompaeW<St2rpqSpec> {
        CompaeW::new(self, 18)
    }
    #[doc = "Bits 19:23 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compb(&mut self) -> CompbW<St2rpqSpec> {
        CompbW::new(self, 19)
    }
    #[doc = "Bit 24 - Compare B Enable"]
    #[inline(always)]
    pub fn compbe(&mut self) -> CompbeW<St2rpqSpec> {
        CompbeW::new(self, 24)
    }
    #[doc = "Bits 25:29 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compc(&mut self) -> CompcW<St2rpqSpec> {
        CompcW::new(self, 25)
    }
    #[doc = "Bit 30 - Compare C Enable"]
    #[inline(always)]
    pub fn compce(&mut self) -> CompceW<St2rpqSpec> {
        CompceW::new(self, 30)
    }
}
#[doc = "Screening Type 2 Register Priority Queue\n\nYou can [`read`](crate::Reg::read) this register and get [`st2rpq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2rpq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St2rpqSpec;
impl crate::RegisterSpec for St2rpqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2rpq::R`](R) reader structure"]
impl crate::Readable for St2rpqSpec {}
#[doc = "`write(|w| ..)` method takes [`st2rpq::W`](W) writer structure"]
impl crate::Writable for St2rpqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST2RPQ[%s] to value 0"]
impl crate::Resettable for St2rpqSpec {}
