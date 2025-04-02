#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TXRDY0` reader - Transmit Ready Interrupt Flag of channel 0"]
pub type Txrdy0R = crate::BitReader;
#[doc = "Field `TXRDY1` reader - Transmit Ready Interrupt Flag of channel 1"]
pub type Txrdy1R = crate::BitReader;
#[doc = "Field `EOC0` reader - End of Conversion Interrupt Flag of channel 0"]
pub type Eoc0R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of Conversion Interrupt Flag of channel 1"]
pub type Eoc1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Flag of channel 0"]
    #[inline(always)]
    pub fn txrdy0(&self) -> Txrdy0R {
        Txrdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Flag of channel 1"]
    #[inline(always)]
    pub fn txrdy1(&self) -> Txrdy1R {
        Txrdy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Flag of channel 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> Eoc0R {
        Eoc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Flag of channel 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> Eoc1R {
        Eoc1R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
