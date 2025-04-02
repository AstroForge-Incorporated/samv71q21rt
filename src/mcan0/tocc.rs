#[doc = "Register `TOCC` reader"]
pub type R = crate::R<ToccSpec>;
#[doc = "Register `TOCC` writer"]
pub type W = crate::W<ToccSpec>;
#[doc = "Enable Timeout Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etocselect {
    #[doc = "0: Timeout Counter disabled."]
    NoTimeout = 0,
    #[doc = "1: Timeout Counter enabled."]
    TosControlled = 1,
}
impl From<Etocselect> for bool {
    #[inline(always)]
    fn from(variant: Etocselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETOC` reader - Enable Timeout Counter"]
pub type EtocR = crate::BitReader<Etocselect>;
impl EtocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etocselect {
        match self.bits {
            false => Etocselect::NoTimeout,
            true => Etocselect::TosControlled,
        }
    }
    #[doc = "Timeout Counter disabled."]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == Etocselect::NoTimeout
    }
    #[doc = "Timeout Counter enabled."]
    #[inline(always)]
    pub fn is_tos_controlled(&self) -> bool {
        *self == Etocselect::TosControlled
    }
}
#[doc = "Field `ETOC` writer - Enable Timeout Counter"]
pub type EtocW<'a, REG> = crate::BitWriter<'a, REG, Etocselect>;
impl<'a, REG> EtocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout Counter disabled."]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Etocselect::NoTimeout)
    }
    #[doc = "Timeout Counter enabled."]
    #[inline(always)]
    pub fn tos_controlled(self) -> &'a mut crate::W<REG> {
        self.variant(Etocselect::TosControlled)
    }
}
#[doc = "Timeout Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tosselect {
    #[doc = "0: Continuous operation"]
    Continuous = 0,
    #[doc = "1: Timeout controlled by Tx Event FIFO"]
    TxEvTimeout = 1,
    #[doc = "2: Timeout controlled by Receive FIFO 0"]
    Rx0EvTimeout = 2,
    #[doc = "3: Timeout controlled by Receive FIFO 1"]
    Rx1EvTimeout = 3,
}
impl From<Tosselect> for u8 {
    #[inline(always)]
    fn from(variant: Tosselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tosselect {
    type Ux = u8;
}
impl crate::IsEnum for Tosselect {}
#[doc = "Field `TOS` reader - Timeout Select"]
pub type TosR = crate::FieldReader<Tosselect>;
impl TosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tosselect {
        match self.bits {
            0 => Tosselect::Continuous,
            1 => Tosselect::TxEvTimeout,
            2 => Tosselect::Rx0EvTimeout,
            3 => Tosselect::Rx1EvTimeout,
            _ => unreachable!(),
        }
    }
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Tosselect::Continuous
    }
    #[doc = "Timeout controlled by Tx Event FIFO"]
    #[inline(always)]
    pub fn is_tx_ev_timeout(&self) -> bool {
        *self == Tosselect::TxEvTimeout
    }
    #[doc = "Timeout controlled by Receive FIFO 0"]
    #[inline(always)]
    pub fn is_rx0_ev_timeout(&self) -> bool {
        *self == Tosselect::Rx0EvTimeout
    }
    #[doc = "Timeout controlled by Receive FIFO 1"]
    #[inline(always)]
    pub fn is_rx1_ev_timeout(&self) -> bool {
        *self == Tosselect::Rx1EvTimeout
    }
}
#[doc = "Field `TOS` writer - Timeout Select"]
pub type TosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tosselect, crate::Safe>;
impl<'a, REG> TosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Tosselect::Continuous)
    }
    #[doc = "Timeout controlled by Tx Event FIFO"]
    #[inline(always)]
    pub fn tx_ev_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Tosselect::TxEvTimeout)
    }
    #[doc = "Timeout controlled by Receive FIFO 0"]
    #[inline(always)]
    pub fn rx0_ev_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Tosselect::Rx0EvTimeout)
    }
    #[doc = "Timeout controlled by Receive FIFO 1"]
    #[inline(always)]
    pub fn rx1_ev_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Tosselect::Rx1EvTimeout)
    }
}
#[doc = "Field `TOP` reader - Timeout Period"]
pub type TopR = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout Period"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> EtocR {
        EtocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TosR {
        TosR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&mut self) -> EtocW<ToccSpec> {
        EtocW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&mut self) -> TosW<ToccSpec> {
        TosW::new(self, 1)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&mut self) -> TopW<ToccSpec> {
        TopW::new(self, 16)
    }
}
#[doc = "Timeout Counter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tocc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToccSpec;
impl crate::RegisterSpec for ToccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocc::R`](R) reader structure"]
impl crate::Readable for ToccSpec {}
#[doc = "`write(|w| ..)` method takes [`tocc::W`](W) writer structure"]
impl crate::Writable for ToccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOCC to value 0"]
impl crate::Resettable for ToccSpec {}
