#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Max Speed Mode for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maxs0select {
    #[doc = "0: External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TrigEvent = 0,
    #[doc = "1: Max speed mode enabled."]
    Maximum = 1,
}
impl From<Maxs0select> for bool {
    #[inline(always)]
    fn from(variant: Maxs0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXS0` reader - Max Speed Mode for Channel 0"]
pub type Maxs0R = crate::BitReader<Maxs0select>;
impl Maxs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maxs0select {
        match self.bits {
            false => Maxs0select::TrigEvent,
            true => Maxs0select::Maximum,
        }
    }
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        *self == Maxs0select::TrigEvent
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Maxs0select::Maximum
    }
}
#[doc = "Field `MAXS0` writer - Max Speed Mode for Channel 0"]
pub type Maxs0W<'a, REG> = crate::BitWriter<'a, REG, Maxs0select>;
impl<'a, REG> Maxs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut crate::W<REG> {
        self.variant(Maxs0select::TrigEvent)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Maxs0select::Maximum)
    }
}
#[doc = "Max Speed Mode for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maxs1select {
    #[doc = "0: External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TrigEvent = 0,
    #[doc = "1: Max speed mode enabled."]
    Maximum = 1,
}
impl From<Maxs1select> for bool {
    #[inline(always)]
    fn from(variant: Maxs1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXS1` reader - Max Speed Mode for Channel 1"]
pub type Maxs1R = crate::BitReader<Maxs1select>;
impl Maxs1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maxs1select {
        match self.bits {
            false => Maxs1select::TrigEvent,
            true => Maxs1select::Maximum,
        }
    }
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        *self == Maxs1select::TrigEvent
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Maxs1select::Maximum
    }
}
#[doc = "Field `MAXS1` writer - Max Speed Mode for Channel 1"]
pub type Maxs1W<'a, REG> = crate::BitWriter<'a, REG, Maxs1select>;
impl<'a, REG> Maxs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut crate::W<REG> {
        self.variant(Maxs1select::TrigEvent)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Maxs1select::Maximum)
    }
}
#[doc = "Word Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wordselect {
    #[doc = "0: One data to convert is written to the FIFO per access to DACC."]
    Disabled = 0,
    #[doc = "1: Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    Enabled = 1,
}
impl From<Wordselect> for bool {
    #[inline(always)]
    fn from(variant: Wordselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WORD` reader - Word Transfer Mode"]
pub type WordR = crate::BitReader<Wordselect>;
impl WordR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wordselect {
        match self.bits {
            false => Wordselect::Disabled,
            true => Wordselect::Enabled,
        }
    }
    #[doc = "One data to convert is written to the FIFO per access to DACC."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wordselect::Disabled
    }
    #[doc = "Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wordselect::Enabled
    }
}
#[doc = "Field `WORD` writer - Word Transfer Mode"]
pub type WordW<'a, REG> = crate::BitWriter<'a, REG, Wordselect>;
impl<'a, REG> WordW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One data to convert is written to the FIFO per access to DACC."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wordselect::Disabled)
    }
    #[doc = "Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wordselect::Enabled)
    }
}
#[doc = "Field `ZERO` reader - Must always be written to 0."]
pub type ZeroR = crate::BitReader;
#[doc = "Field `ZERO` writer - Must always be written to 0."]
pub type ZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Differential Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diffselect {
    #[doc = "0: DAC0 and DAC1 are single-ended outputs."]
    Disabled = 0,
    #[doc = "1: DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    Enabled = 1,
}
impl From<Diffselect> for bool {
    #[inline(always)]
    fn from(variant: Diffselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DiffR = crate::BitReader<Diffselect>;
impl DiffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diffselect {
        match self.bits {
            false => Diffselect::Disabled,
            true => Diffselect::Enabled,
        }
    }
    #[doc = "DAC0 and DAC1 are single-ended outputs."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Diffselect::Disabled
    }
    #[doc = "DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Diffselect::Enabled
    }
}
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG, Diffselect>;
impl<'a, REG> DiffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC0 and DAC1 are single-ended outputs."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Diffselect::Disabled)
    }
    #[doc = "DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Diffselect::Enabled)
    }
}
#[doc = "Field `PRESCALER` reader - Peripheral Clock to DAC Clock Ratio"]
pub type PrescalerR = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - Peripheral Clock to DAC Clock Ratio"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    pub fn maxs0(&self) -> Maxs0R {
        Maxs0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    pub fn maxs1(&self) -> Maxs1R {
        Maxs1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    pub fn word(&self) -> WordR {
        WordR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    pub fn maxs0(&mut self) -> Maxs0W<MrSpec> {
        Maxs0W::new(self, 0)
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    pub fn maxs1(&mut self) -> Maxs1W<MrSpec> {
        Maxs1W::new(self, 1)
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    pub fn word(&mut self) -> WordW<MrSpec> {
        WordW::new(self, 4)
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    pub fn zero(&mut self) -> ZeroW<MrSpec> {
        ZeroW::new(self, 5)
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DiffW<MrSpec> {
        DiffW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PrescalerW<MrSpec> {
        PrescalerW::new(self, 24)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {}
