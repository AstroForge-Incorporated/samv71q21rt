#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Serial Memory Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smmselect {
    #[doc = "0: The QSPI is in SPI mode."]
    Spi = 0,
    #[doc = "1: The QSPI is in Serial Memory mode."]
    Memory = 1,
}
impl From<Smmselect> for bool {
    #[inline(always)]
    fn from(variant: Smmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMM` reader - Serial Memory Mode"]
pub type SmmR = crate::BitReader<Smmselect>;
impl SmmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smmselect {
        match self.bits {
            false => Smmselect::Spi,
            true => Smmselect::Memory,
        }
    }
    #[doc = "The QSPI is in SPI mode."]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Smmselect::Spi
    }
    #[doc = "The QSPI is in Serial Memory mode."]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == Smmselect::Memory
    }
}
#[doc = "Field `SMM` writer - Serial Memory Mode"]
pub type SmmW<'a, REG> = crate::BitWriter<'a, REG, Smmselect>;
impl<'a, REG> SmmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The QSPI is in SPI mode."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Smmselect::Spi)
    }
    #[doc = "The QSPI is in Serial Memory mode."]
    #[inline(always)]
    pub fn memory(self) -> &'a mut crate::W<REG> {
        self.variant(Smmselect::Memory)
    }
}
#[doc = "Local Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Llbselect {
    #[doc = "0: Local loopback path disabled."]
    Disabled = 0,
    #[doc = "1: Local loopback path enabled."]
    Enabled = 1,
}
impl From<Llbselect> for bool {
    #[inline(always)]
    fn from(variant: Llbselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub type LlbR = crate::BitReader<Llbselect>;
impl LlbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Llbselect {
        match self.bits {
            false => Llbselect::Disabled,
            true => Llbselect::Enabled,
        }
    }
    #[doc = "Local loopback path disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Llbselect::Disabled
    }
    #[doc = "Local loopback path enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Llbselect::Enabled
    }
}
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub type LlbW<'a, REG> = crate::BitWriter<'a, REG, Llbselect>;
impl<'a, REG> LlbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Local loopback path disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Llbselect::Disabled)
    }
    #[doc = "Local loopback path enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Llbselect::Enabled)
    }
}
#[doc = "Wait Data Read Before Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdrbtselect {
    #[doc = "0: No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    Disabled = 0,
    #[doc = "1: In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    Enabled = 1,
}
impl From<Wdrbtselect> for bool {
    #[inline(always)]
    fn from(variant: Wdrbtselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WdrbtR = crate::BitReader<Wdrbtselect>;
impl WdrbtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdrbtselect {
        match self.bits {
            false => Wdrbtselect::Disabled,
            true => Wdrbtselect::Enabled,
        }
    }
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wdrbtselect::Disabled
    }
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wdrbtselect::Enabled
    }
}
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WdrbtW<'a, REG> = crate::BitWriter<'a, REG, Wdrbtselect>;
impl<'a, REG> WdrbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdrbtselect::Disabled)
    }
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdrbtselect::Enabled)
    }
}
#[doc = "Chip Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csmodeselect {
    #[doc = "0: The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    NotReloaded = 0,
    #[doc = "1: The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    Lastxfer = 1,
    #[doc = "2: The chip select is deasserted systematically after each transfer."]
    Systematically = 2,
}
impl From<Csmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Csmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Csmodeselect {}
#[doc = "Field `CSMODE` reader - Chip Select Mode"]
pub type CsmodeR = crate::FieldReader<Csmodeselect>;
impl CsmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csmodeselect> {
        match self.bits {
            0 => Some(Csmodeselect::NotReloaded),
            1 => Some(Csmodeselect::Lastxfer),
            2 => Some(Csmodeselect::Systematically),
            _ => None,
        }
    }
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn is_not_reloaded(&self) -> bool {
        *self == Csmodeselect::NotReloaded
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    #[inline(always)]
    pub fn is_lastxfer(&self) -> bool {
        *self == Csmodeselect::Lastxfer
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn is_systematically(&self) -> bool {
        *self == Csmodeselect::Systematically
    }
}
#[doc = "Field `CSMODE` writer - Chip Select Mode"]
pub type CsmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csmodeselect>;
impl<'a, REG> CsmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn not_reloaded(self) -> &'a mut crate::W<REG> {
        self.variant(Csmodeselect::NotReloaded)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    #[inline(always)]
    pub fn lastxfer(self) -> &'a mut crate::W<REG> {
        self.variant(Csmodeselect::Lastxfer)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn systematically(self) -> &'a mut crate::W<REG> {
        self.variant(Csmodeselect::Systematically)
    }
}
#[doc = "Number Of Bits Per Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nbbitsselect {
    #[doc = "0: 8 bits for transfer"]
    _8Bit = 0,
    #[doc = "8: 16 bits for transfer"]
    _16Bit = 8,
}
impl From<Nbbitsselect> for u8 {
    #[inline(always)]
    fn from(variant: Nbbitsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nbbitsselect {
    type Ux = u8;
}
impl crate::IsEnum for Nbbitsselect {}
#[doc = "Field `NBBITS` reader - Number Of Bits Per Transfer"]
pub type NbbitsR = crate::FieldReader<Nbbitsselect>;
impl NbbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nbbitsselect> {
        match self.bits {
            0 => Some(Nbbitsselect::_8Bit),
            8 => Some(Nbbitsselect::_16Bit),
            _ => None,
        }
    }
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Nbbitsselect::_8Bit
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Nbbitsselect::_16Bit
    }
}
#[doc = "Field `NBBITS` writer - Number Of Bits Per Transfer"]
pub type NbbitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Nbbitsselect>;
impl<'a, REG> NbbitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Nbbitsselect::_8Bit)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Nbbitsselect::_16Bit)
    }
}
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DlybctR = crate::FieldReader;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DlybctW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLYCS` reader - Minimum Inactive QCS Delay"]
pub type DlycsR = crate::FieldReader;
#[doc = "Field `DLYCS` writer - Minimum Inactive QCS Delay"]
pub type DlycsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn smm(&self) -> SmmR {
        SmmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LlbR {
        LlbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WdrbtR {
        WdrbtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&self) -> CsmodeR {
        CsmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    pub fn nbbits(&self) -> NbbitsR {
        NbbitsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DlybctR {
        DlybctR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    pub fn dlycs(&self) -> DlycsR {
        DlycsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn smm(&mut self) -> SmmW<MrSpec> {
        SmmW::new(self, 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> LlbW<MrSpec> {
        LlbW::new(self, 1)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WdrbtW<MrSpec> {
        WdrbtW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&mut self) -> CsmodeW<MrSpec> {
        CsmodeW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    pub fn nbbits(&mut self) -> NbbitsW<MrSpec> {
        NbbitsW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&mut self) -> DlybctW<MrSpec> {
        DlybctW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    pub fn dlycs(&mut self) -> DlycsW<MrSpec> {
        DlycsW::new(self, 24)
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
