#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Inter-IC Sound Controller Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modeselect {
    #[doc = "0: I2SC_CK and I2SC_WS pin inputs used as bit clock and word select/frame synchronization."]
    Slave = 0,
    #[doc = "1: Bit clock and word select/frame synchronization generated by I2SC from MCK and output to I2SC_CK and I2SC_WS pins. Peripheral clock or GCLK is output as master clock on I2SC_MCK if I2SC_MR.IMCKMODE is set."]
    Master = 1,
}
impl From<Modeselect> for bool {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Inter-IC Sound Controller Mode"]
pub type ModeR = crate::BitReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modeselect {
        match self.bits {
            false => Modeselect::Slave,
            true => Modeselect::Master,
        }
    }
    #[doc = "I2SC_CK and I2SC_WS pin inputs used as bit clock and word select/frame synchronization."]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Modeselect::Slave
    }
    #[doc = "Bit clock and word select/frame synchronization generated by I2SC from MCK and output to I2SC_CK and I2SC_WS pins. Peripheral clock or GCLK is output as master clock on I2SC_MCK if I2SC_MR.IMCKMODE is set."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Modeselect::Master
    }
}
#[doc = "Field `MODE` writer - Inter-IC Sound Controller Mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2SC_CK and I2SC_WS pin inputs used as bit clock and word select/frame synchronization."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Slave)
    }
    #[doc = "Bit clock and word select/frame synchronization generated by I2SC from MCK and output to I2SC_CK and I2SC_WS pins. Peripheral clock or GCLK is output as master clock on I2SC_MCK if I2SC_MR.IMCKMODE is set."]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Master)
    }
}
#[doc = "Data Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datalengthselect {
    #[doc = "0: Data length is set to 32 bits"]
    _32Bits = 0,
    #[doc = "1: Data length is set to 24 bits"]
    _24Bits = 1,
    #[doc = "2: Data length is set to 20 bits"]
    _20Bits = 2,
    #[doc = "3: Data length is set to 18 bits"]
    _18Bits = 3,
    #[doc = "4: Data length is set to 16 bits"]
    _16Bits = 4,
    #[doc = "5: Data length is set to 16-bit compact stereo. Left sample in bits 15:0 and right sample in bits 31:16 of same word."]
    _16BitsCompact = 5,
    #[doc = "6: Data length is set to 8 bits"]
    _8Bits = 6,
    #[doc = "7: Data length is set to 8-bit compact stereo. Left sample in bits 7:0 and right sample in bits 15:8 of the same word."]
    _8BitsCompact = 7,
}
impl From<Datalengthselect> for u8 {
    #[inline(always)]
    fn from(variant: Datalengthselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datalengthselect {
    type Ux = u8;
}
impl crate::IsEnum for Datalengthselect {}
#[doc = "Field `DATALENGTH` reader - Data Word Length"]
pub type DatalengthR = crate::FieldReader<Datalengthselect>;
impl DatalengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datalengthselect {
        match self.bits {
            0 => Datalengthselect::_32Bits,
            1 => Datalengthselect::_24Bits,
            2 => Datalengthselect::_20Bits,
            3 => Datalengthselect::_18Bits,
            4 => Datalengthselect::_16Bits,
            5 => Datalengthselect::_16BitsCompact,
            6 => Datalengthselect::_8Bits,
            7 => Datalengthselect::_8BitsCompact,
            _ => unreachable!(),
        }
    }
    #[doc = "Data length is set to 32 bits"]
    #[inline(always)]
    pub fn is_32_bits(&self) -> bool {
        *self == Datalengthselect::_32Bits
    }
    #[doc = "Data length is set to 24 bits"]
    #[inline(always)]
    pub fn is_24_bits(&self) -> bool {
        *self == Datalengthselect::_24Bits
    }
    #[doc = "Data length is set to 20 bits"]
    #[inline(always)]
    pub fn is_20_bits(&self) -> bool {
        *self == Datalengthselect::_20Bits
    }
    #[doc = "Data length is set to 18 bits"]
    #[inline(always)]
    pub fn is_18_bits(&self) -> bool {
        *self == Datalengthselect::_18Bits
    }
    #[doc = "Data length is set to 16 bits"]
    #[inline(always)]
    pub fn is_16_bits(&self) -> bool {
        *self == Datalengthselect::_16Bits
    }
    #[doc = "Data length is set to 16-bit compact stereo. Left sample in bits 15:0 and right sample in bits 31:16 of same word."]
    #[inline(always)]
    pub fn is_16_bits_compact(&self) -> bool {
        *self == Datalengthselect::_16BitsCompact
    }
    #[doc = "Data length is set to 8 bits"]
    #[inline(always)]
    pub fn is_8_bits(&self) -> bool {
        *self == Datalengthselect::_8Bits
    }
    #[doc = "Data length is set to 8-bit compact stereo. Left sample in bits 7:0 and right sample in bits 15:8 of the same word."]
    #[inline(always)]
    pub fn is_8_bits_compact(&self) -> bool {
        *self == Datalengthselect::_8BitsCompact
    }
}
#[doc = "Field `DATALENGTH` writer - Data Word Length"]
pub type DatalengthW<'a, REG> = crate::FieldWriter<'a, REG, 3, Datalengthselect, crate::Safe>;
impl<'a, REG> DatalengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data length is set to 32 bits"]
    #[inline(always)]
    pub fn _32_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalengthselect::_32Bits)
    }
    #[doc = "Data length is set to 24 bits"]
    #[inline(always)]
    pub fn _24_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalengthselect::_24Bits)
    }
    #[doc = "Data length is set to 20 bits"]
    #[inline(always)]
    pub fn _20_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalengthselect::_20Bits)
    }
    #[doc = "Data length is set to 18 bits"]
    #[inline(always)]
    pub fn _18_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalengthselect::_18Bits)
    }
    #[doc = "Data length is set to 16 bits"]
    #[inline(always)]
    pub fn _16_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalengthselect::_16Bits)
    }
    #[doc = "Data length is set to 16-bit compact stereo. Left sample in bits 15:0 and right sample in bits 31:16 of same word."]
    #[inline(always)]
    pub fn _16_bits_compact(self) -> &'a mut crate::W<REG> {
        self.variant(Datalengthselect::_16BitsCompact)
    }
    #[doc = "Data length is set to 8 bits"]
    #[inline(always)]
    pub fn _8_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datalengthselect::_8Bits)
    }
    #[doc = "Data length is set to 8-bit compact stereo. Left sample in bits 7:0 and right sample in bits 15:8 of the same word."]
    #[inline(always)]
    pub fn _8_bits_compact(self) -> &'a mut crate::W<REG> {
        self.variant(Datalengthselect::_8BitsCompact)
    }
}
#[doc = "Field `RXMONO` reader - Receive Mono"]
pub type RxmonoR = crate::BitReader;
#[doc = "Field `RXMONO` writer - Receive Mono"]
pub type RxmonoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMA` reader - Single or Multiple DMA Controller Channels for Receiver"]
pub type RxdmaR = crate::BitReader;
#[doc = "Field `RXDMA` writer - Single or Multiple DMA Controller Channels for Receiver"]
pub type RxdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLOOP` reader - Loopback Test Mode"]
pub type RxloopR = crate::BitReader;
#[doc = "Field `RXLOOP` writer - Loopback Test Mode"]
pub type RxloopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMONO` reader - Transmit Mono"]
pub type TxmonoR = crate::BitReader;
#[doc = "Field `TXMONO` writer - Transmit Mono"]
pub type TxmonoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMA` reader - Single or Multiple DMA Controller Channels for Transmitter"]
pub type TxdmaR = crate::BitReader;
#[doc = "Field `TXDMA` writer - Single or Multiple DMA Controller Channels for Transmitter"]
pub type TxdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSAME` reader - Transmit Data when Underrun"]
pub type TxsameR = crate::BitReader;
#[doc = "Field `TXSAME` writer - Transmit Data when Underrun"]
pub type TxsameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMCKDIV` reader - Selected Clock to I2SC Master Clock Ratio"]
pub type ImckdivR = crate::FieldReader;
#[doc = "Field `IMCKDIV` writer - Selected Clock to I2SC Master Clock Ratio"]
pub type ImckdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Master Clock to fs Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Imckfsselect {
    #[doc = "0: Sample frequency ratio set to 32"]
    M2sf32 = 0,
    #[doc = "1: Sample frequency ratio set to 64"]
    M2sf64 = 1,
    #[doc = "2: Sample frequency ratio set to 96"]
    M2sf96 = 2,
    #[doc = "3: Sample frequency ratio set to 128"]
    M2sf128 = 3,
    #[doc = "5: Sample frequency ratio set to 192"]
    M2sf192 = 5,
    #[doc = "7: Sample frequency ratio set to 256"]
    M2sf256 = 7,
    #[doc = "11: Sample frequency ratio set to 384"]
    M2sf384 = 11,
    #[doc = "15: Sample frequency ratio set to 512"]
    M2sf512 = 15,
    #[doc = "23: Sample frequency ratio set to 768"]
    M2sf768 = 23,
    #[doc = "31: Sample frequency ratio set to 1024"]
    M2sf1024 = 31,
    #[doc = "47: Sample frequency ratio set to 1536"]
    M2sf1536 = 47,
    #[doc = "63: Sample frequency ratio set to 2048"]
    M2sf2048 = 63,
}
impl From<Imckfsselect> for u8 {
    #[inline(always)]
    fn from(variant: Imckfsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Imckfsselect {
    type Ux = u8;
}
impl crate::IsEnum for Imckfsselect {}
#[doc = "Field `IMCKFS` reader - Master Clock to fs Ratio"]
pub type ImckfsR = crate::FieldReader<Imckfsselect>;
impl ImckfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Imckfsselect> {
        match self.bits {
            0 => Some(Imckfsselect::M2sf32),
            1 => Some(Imckfsselect::M2sf64),
            2 => Some(Imckfsselect::M2sf96),
            3 => Some(Imckfsselect::M2sf128),
            5 => Some(Imckfsselect::M2sf192),
            7 => Some(Imckfsselect::M2sf256),
            11 => Some(Imckfsselect::M2sf384),
            15 => Some(Imckfsselect::M2sf512),
            23 => Some(Imckfsselect::M2sf768),
            31 => Some(Imckfsselect::M2sf1024),
            47 => Some(Imckfsselect::M2sf1536),
            63 => Some(Imckfsselect::M2sf2048),
            _ => None,
        }
    }
    #[doc = "Sample frequency ratio set to 32"]
    #[inline(always)]
    pub fn is_m2sf32(&self) -> bool {
        *self == Imckfsselect::M2sf32
    }
    #[doc = "Sample frequency ratio set to 64"]
    #[inline(always)]
    pub fn is_m2sf64(&self) -> bool {
        *self == Imckfsselect::M2sf64
    }
    #[doc = "Sample frequency ratio set to 96"]
    #[inline(always)]
    pub fn is_m2sf96(&self) -> bool {
        *self == Imckfsselect::M2sf96
    }
    #[doc = "Sample frequency ratio set to 128"]
    #[inline(always)]
    pub fn is_m2sf128(&self) -> bool {
        *self == Imckfsselect::M2sf128
    }
    #[doc = "Sample frequency ratio set to 192"]
    #[inline(always)]
    pub fn is_m2sf192(&self) -> bool {
        *self == Imckfsselect::M2sf192
    }
    #[doc = "Sample frequency ratio set to 256"]
    #[inline(always)]
    pub fn is_m2sf256(&self) -> bool {
        *self == Imckfsselect::M2sf256
    }
    #[doc = "Sample frequency ratio set to 384"]
    #[inline(always)]
    pub fn is_m2sf384(&self) -> bool {
        *self == Imckfsselect::M2sf384
    }
    #[doc = "Sample frequency ratio set to 512"]
    #[inline(always)]
    pub fn is_m2sf512(&self) -> bool {
        *self == Imckfsselect::M2sf512
    }
    #[doc = "Sample frequency ratio set to 768"]
    #[inline(always)]
    pub fn is_m2sf768(&self) -> bool {
        *self == Imckfsselect::M2sf768
    }
    #[doc = "Sample frequency ratio set to 1024"]
    #[inline(always)]
    pub fn is_m2sf1024(&self) -> bool {
        *self == Imckfsselect::M2sf1024
    }
    #[doc = "Sample frequency ratio set to 1536"]
    #[inline(always)]
    pub fn is_m2sf1536(&self) -> bool {
        *self == Imckfsselect::M2sf1536
    }
    #[doc = "Sample frequency ratio set to 2048"]
    #[inline(always)]
    pub fn is_m2sf2048(&self) -> bool {
        *self == Imckfsselect::M2sf2048
    }
}
#[doc = "Field `IMCKFS` writer - Master Clock to fs Ratio"]
pub type ImckfsW<'a, REG> = crate::FieldWriter<'a, REG, 6, Imckfsselect>;
impl<'a, REG> ImckfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sample frequency ratio set to 32"]
    #[inline(always)]
    pub fn m2sf32(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf32)
    }
    #[doc = "Sample frequency ratio set to 64"]
    #[inline(always)]
    pub fn m2sf64(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf64)
    }
    #[doc = "Sample frequency ratio set to 96"]
    #[inline(always)]
    pub fn m2sf96(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf96)
    }
    #[doc = "Sample frequency ratio set to 128"]
    #[inline(always)]
    pub fn m2sf128(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf128)
    }
    #[doc = "Sample frequency ratio set to 192"]
    #[inline(always)]
    pub fn m2sf192(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf192)
    }
    #[doc = "Sample frequency ratio set to 256"]
    #[inline(always)]
    pub fn m2sf256(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf256)
    }
    #[doc = "Sample frequency ratio set to 384"]
    #[inline(always)]
    pub fn m2sf384(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf384)
    }
    #[doc = "Sample frequency ratio set to 512"]
    #[inline(always)]
    pub fn m2sf512(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf512)
    }
    #[doc = "Sample frequency ratio set to 768"]
    #[inline(always)]
    pub fn m2sf768(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf768)
    }
    #[doc = "Sample frequency ratio set to 1024"]
    #[inline(always)]
    pub fn m2sf1024(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf1024)
    }
    #[doc = "Sample frequency ratio set to 1536"]
    #[inline(always)]
    pub fn m2sf1536(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf1536)
    }
    #[doc = "Sample frequency ratio set to 2048"]
    #[inline(always)]
    pub fn m2sf2048(self) -> &'a mut crate::W<REG> {
        self.variant(Imckfsselect::M2sf2048)
    }
}
#[doc = "Field `IMCKMODE` reader - Master Clock Mode"]
pub type ImckmodeR = crate::BitReader;
#[doc = "Field `IMCKMODE` writer - Master Clock Mode"]
pub type ImckmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWS` reader - I2SC_WS Slot Width"]
pub type IwsR = crate::BitReader;
#[doc = "Field `IWS` writer - I2SC_WS Slot Width"]
pub type IwsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Inter-IC Sound Controller Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline(always)]
    pub fn datalength(&self) -> DatalengthR {
        DatalengthR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 8 - Receive Mono"]
    #[inline(always)]
    pub fn rxmono(&self) -> RxmonoR {
        RxmonoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Single or Multiple DMA Controller Channels for Receiver"]
    #[inline(always)]
    pub fn rxdma(&self) -> RxdmaR {
        RxdmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Loopback Test Mode"]
    #[inline(always)]
    pub fn rxloop(&self) -> RxloopR {
        RxloopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Mono"]
    #[inline(always)]
    pub fn txmono(&self) -> TxmonoR {
        TxmonoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single or Multiple DMA Controller Channels for Transmitter"]
    #[inline(always)]
    pub fn txdma(&self) -> TxdmaR {
        TxdmaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&self) -> TxsameR {
        TxsameR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Selected Clock to I2SC Master Clock Ratio"]
    #[inline(always)]
    pub fn imckdiv(&self) -> ImckdivR {
        ImckdivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline(always)]
    pub fn imckfs(&self) -> ImckfsR {
        ImckfsR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline(always)]
    pub fn imckmode(&self) -> ImckmodeR {
        ImckmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2SC_WS Slot Width"]
    #[inline(always)]
    pub fn iws(&self) -> IwsR {
        IwsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Inter-IC Sound Controller Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<MrSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline(always)]
    pub fn datalength(&mut self) -> DatalengthW<MrSpec> {
        DatalengthW::new(self, 2)
    }
    #[doc = "Bit 8 - Receive Mono"]
    #[inline(always)]
    pub fn rxmono(&mut self) -> RxmonoW<MrSpec> {
        RxmonoW::new(self, 8)
    }
    #[doc = "Bit 9 - Single or Multiple DMA Controller Channels for Receiver"]
    #[inline(always)]
    pub fn rxdma(&mut self) -> RxdmaW<MrSpec> {
        RxdmaW::new(self, 9)
    }
    #[doc = "Bit 10 - Loopback Test Mode"]
    #[inline(always)]
    pub fn rxloop(&mut self) -> RxloopW<MrSpec> {
        RxloopW::new(self, 10)
    }
    #[doc = "Bit 12 - Transmit Mono"]
    #[inline(always)]
    pub fn txmono(&mut self) -> TxmonoW<MrSpec> {
        TxmonoW::new(self, 12)
    }
    #[doc = "Bit 13 - Single or Multiple DMA Controller Channels for Transmitter"]
    #[inline(always)]
    pub fn txdma(&mut self) -> TxdmaW<MrSpec> {
        TxdmaW::new(self, 13)
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&mut self) -> TxsameW<MrSpec> {
        TxsameW::new(self, 14)
    }
    #[doc = "Bits 16:21 - Selected Clock to I2SC Master Clock Ratio"]
    #[inline(always)]
    pub fn imckdiv(&mut self) -> ImckdivW<MrSpec> {
        ImckdivW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline(always)]
    pub fn imckfs(&mut self) -> ImckfsW<MrSpec> {
        ImckfsW::new(self, 24)
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline(always)]
    pub fn imckmode(&mut self) -> ImckmodeW<MrSpec> {
        ImckmodeW::new(self, 30)
    }
    #[doc = "Bit 31 - I2SC_WS Slot Width"]
    #[inline(always)]
    pub fn iws(&mut self) -> IwsW<MrSpec> {
        IwsW::new(self, 31)
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
