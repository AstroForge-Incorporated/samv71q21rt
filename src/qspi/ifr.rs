#[doc = "Register `IFR` reader"]
pub type R = crate::R<IfrSpec>;
#[doc = "Register `IFR` writer"]
pub type W = crate::W<IfrSpec>;
#[doc = "Width of Instruction Code, Address, Option Code and Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Widthselect {
    #[doc = "0: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    SingleBitSpi = 0,
    #[doc = "1: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    DualOutput = 1,
    #[doc = "2: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    QuadOutput = 2,
    #[doc = "3: Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DualIo = 3,
    #[doc = "4: Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QuadIo = 4,
    #[doc = "5: Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DualCmd = 5,
    #[doc = "6: Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QuadCmd = 6,
}
impl From<Widthselect> for u8 {
    #[inline(always)]
    fn from(variant: Widthselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Widthselect {
    type Ux = u8;
}
impl crate::IsEnum for Widthselect {}
#[doc = "Field `WIDTH` reader - Width of Instruction Code, Address, Option Code and Data"]
pub type WidthR = crate::FieldReader<Widthselect>;
impl WidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Widthselect> {
        match self.bits {
            0 => Some(Widthselect::SingleBitSpi),
            1 => Some(Widthselect::DualOutput),
            2 => Some(Widthselect::QuadOutput),
            3 => Some(Widthselect::DualIo),
            4 => Some(Widthselect::QuadIo),
            5 => Some(Widthselect::DualCmd),
            6 => Some(Widthselect::QuadCmd),
            _ => None,
        }
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn is_single_bit_spi(&self) -> bool {
        *self == Widthselect::SingleBitSpi
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn is_dual_output(&self) -> bool {
        *self == Widthselect::DualOutput
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn is_quad_output(&self) -> bool {
        *self == Widthselect::QuadOutput
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == Widthselect::DualIo
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == Widthselect::QuadIo
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn is_dual_cmd(&self) -> bool {
        *self == Widthselect::DualCmd
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn is_quad_cmd(&self) -> bool {
        *self == Widthselect::QuadCmd
    }
}
#[doc = "Field `WIDTH` writer - Width of Instruction Code, Address, Option Code and Data"]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 3, Widthselect>;
impl<'a, REG> WidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn single_bit_spi(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::SingleBitSpi)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_output(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::DualOutput)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_output(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::QuadOutput)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::DualIo)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::QuadIo)
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::DualCmd)
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Widthselect::QuadCmd)
    }
}
#[doc = "Field `INSTEN` reader - Instruction Enable"]
pub type InstenR = crate::BitReader;
#[doc = "Field `INSTEN` writer - Instruction Enable"]
pub type InstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDREN` reader - Address Enable"]
pub type AddrenR = crate::BitReader;
#[doc = "Field `ADDREN` writer - Address Enable"]
pub type AddrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTEN` reader - Option Enable"]
pub type OptenR = crate::BitReader;
#[doc = "Field `OPTEN` writer - Option Enable"]
pub type OptenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAEN` reader - Data Enable"]
pub type DataenR = crate::BitReader;
#[doc = "Field `DATAEN` writer - Data Enable"]
pub type DataenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Option Code Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Optlselect {
    #[doc = "0: The option code is 1 bit long."]
    Option1bit = 0,
    #[doc = "1: The option code is 2 bits long."]
    Option2bit = 1,
    #[doc = "2: The option code is 4 bits long."]
    Option4bit = 2,
    #[doc = "3: The option code is 8 bits long."]
    Option8bit = 3,
}
impl From<Optlselect> for u8 {
    #[inline(always)]
    fn from(variant: Optlselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Optlselect {
    type Ux = u8;
}
impl crate::IsEnum for Optlselect {}
#[doc = "Field `OPTL` reader - Option Code Length"]
pub type OptlR = crate::FieldReader<Optlselect>;
impl OptlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Optlselect {
        match self.bits {
            0 => Optlselect::Option1bit,
            1 => Optlselect::Option2bit,
            2 => Optlselect::Option4bit,
            3 => Optlselect::Option8bit,
            _ => unreachable!(),
        }
    }
    #[doc = "The option code is 1 bit long."]
    #[inline(always)]
    pub fn is_option_1bit(&self) -> bool {
        *self == Optlselect::Option1bit
    }
    #[doc = "The option code is 2 bits long."]
    #[inline(always)]
    pub fn is_option_2bit(&self) -> bool {
        *self == Optlselect::Option2bit
    }
    #[doc = "The option code is 4 bits long."]
    #[inline(always)]
    pub fn is_option_4bit(&self) -> bool {
        *self == Optlselect::Option4bit
    }
    #[doc = "The option code is 8 bits long."]
    #[inline(always)]
    pub fn is_option_8bit(&self) -> bool {
        *self == Optlselect::Option8bit
    }
}
#[doc = "Field `OPTL` writer - Option Code Length"]
pub type OptlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Optlselect, crate::Safe>;
impl<'a, REG> OptlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The option code is 1 bit long."]
    #[inline(always)]
    pub fn option_1bit(self) -> &'a mut crate::W<REG> {
        self.variant(Optlselect::Option1bit)
    }
    #[doc = "The option code is 2 bits long."]
    #[inline(always)]
    pub fn option_2bit(self) -> &'a mut crate::W<REG> {
        self.variant(Optlselect::Option2bit)
    }
    #[doc = "The option code is 4 bits long."]
    #[inline(always)]
    pub fn option_4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Optlselect::Option4bit)
    }
    #[doc = "The option code is 8 bits long."]
    #[inline(always)]
    pub fn option_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Optlselect::Option8bit)
    }
}
#[doc = "Address Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrlselect {
    #[doc = "0: The address is 24 bits long."]
    _24Bit = 0,
    #[doc = "1: The address is 32 bits long."]
    _32Bit = 1,
}
impl From<Addrlselect> for bool {
    #[inline(always)]
    fn from(variant: Addrlselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRL` reader - Address Length"]
pub type AddrlR = crate::BitReader<Addrlselect>;
impl AddrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrlselect {
        match self.bits {
            false => Addrlselect::_24Bit,
            true => Addrlselect::_32Bit,
        }
    }
    #[doc = "The address is 24 bits long."]
    #[inline(always)]
    pub fn is_24_bit(&self) -> bool {
        *self == Addrlselect::_24Bit
    }
    #[doc = "The address is 32 bits long."]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == Addrlselect::_32Bit
    }
}
#[doc = "Field `ADDRL` writer - Address Length"]
pub type AddrlW<'a, REG> = crate::BitWriter<'a, REG, Addrlselect>;
impl<'a, REG> AddrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address is 24 bits long."]
    #[inline(always)]
    pub fn _24_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Addrlselect::_24Bit)
    }
    #[doc = "The address is 32 bits long."]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Addrlselect::_32Bit)
    }
}
#[doc = "Data Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tfrtypselect {
    #[doc = "0: Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    TrsfrRead = 0,
    #[doc = "1: Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    TrsfrReadMemory = 1,
    #[doc = "2: Write transfer into the serial memory.Scrambling is not performed."]
    TrsfrWrite = 2,
    #[doc = "3: Write data transfer into the serial memory.If enabled, scrambling is performed."]
    TrsfrWriteMemory = 3,
}
impl From<Tfrtypselect> for u8 {
    #[inline(always)]
    fn from(variant: Tfrtypselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tfrtypselect {
    type Ux = u8;
}
impl crate::IsEnum for Tfrtypselect {}
#[doc = "Field `TFRTYP` reader - Data Transfer Type"]
pub type TfrtypR = crate::FieldReader<Tfrtypselect>;
impl TfrtypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfrtypselect {
        match self.bits {
            0 => Tfrtypselect::TrsfrRead,
            1 => Tfrtypselect::TrsfrReadMemory,
            2 => Tfrtypselect::TrsfrWrite,
            3 => Tfrtypselect::TrsfrWriteMemory,
            _ => unreachable!(),
        }
    }
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    #[inline(always)]
    pub fn is_trsfr_read(&self) -> bool {
        *self == Tfrtypselect::TrsfrRead
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    #[inline(always)]
    pub fn is_trsfr_read_memory(&self) -> bool {
        *self == Tfrtypselect::TrsfrReadMemory
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn is_trsfr_write(&self) -> bool {
        *self == Tfrtypselect::TrsfrWrite
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn is_trsfr_write_memory(&self) -> bool {
        *self == Tfrtypselect::TrsfrWriteMemory
    }
}
#[doc = "Field `TFRTYP` writer - Data Transfer Type"]
pub type TfrtypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tfrtypselect, crate::Safe>;
impl<'a, REG> TfrtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    #[inline(always)]
    pub fn trsfr_read(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrtypselect::TrsfrRead)
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    #[inline(always)]
    pub fn trsfr_read_memory(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrtypselect::TrsfrReadMemory)
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn trsfr_write(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrtypselect::TrsfrWrite)
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn trsfr_write_memory(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrtypselect::TrsfrWriteMemory)
    }
}
#[doc = "Continuous Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crmselect {
    #[doc = "0: The Continuous Read mode is disabled."]
    Disabled = 0,
    #[doc = "1: The Continuous Read mode is enabled."]
    Enabled = 1,
}
impl From<Crmselect> for bool {
    #[inline(always)]
    fn from(variant: Crmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRM` reader - Continuous Read Mode"]
pub type CrmR = crate::BitReader<Crmselect>;
impl CrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crmselect {
        match self.bits {
            false => Crmselect::Disabled,
            true => Crmselect::Enabled,
        }
    }
    #[doc = "The Continuous Read mode is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Crmselect::Disabled
    }
    #[doc = "The Continuous Read mode is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Crmselect::Enabled
    }
}
#[doc = "Field `CRM` writer - Continuous Read Mode"]
pub type CrmW<'a, REG> = crate::BitWriter<'a, REG, Crmselect>;
impl<'a, REG> CrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Continuous Read mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Crmselect::Disabled)
    }
    #[doc = "The Continuous Read mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Crmselect::Enabled)
    }
}
#[doc = "Field `NBDUM` reader - Number Of Dummy Cycles"]
pub type NbdumR = crate::FieldReader;
#[doc = "Field `NBDUM` writer - Number Of Dummy Cycles"]
pub type NbdumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn insten(&self) -> InstenR {
        InstenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&self) -> AddrenR {
        AddrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn opten(&self) -> OptenR {
        OptenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&self) -> DataenR {
        DataenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optl(&self) -> OptlR {
        OptlR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrl(&self) -> AddrlR {
        AddrlR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtyp(&self) -> TfrtypR {
        TfrtypR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crm(&self) -> CrmR {
        CrmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline(always)]
    pub fn nbdum(&self) -> NbdumR {
        NbdumR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<IfrSpec> {
        WidthW::new(self, 0)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn insten(&mut self) -> InstenW<IfrSpec> {
        InstenW::new(self, 4)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&mut self) -> AddrenW<IfrSpec> {
        AddrenW::new(self, 5)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn opten(&mut self) -> OptenW<IfrSpec> {
        OptenW::new(self, 6)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&mut self) -> DataenW<IfrSpec> {
        DataenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optl(&mut self) -> OptlW<IfrSpec> {
        OptlW::new(self, 8)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrl(&mut self) -> AddrlW<IfrSpec> {
        AddrlW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtyp(&mut self) -> TfrtypW<IfrSpec> {
        TfrtypW::new(self, 12)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crm(&mut self) -> CrmW<IfrSpec> {
        CrmW::new(self, 14)
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline(always)]
    pub fn nbdum(&mut self) -> NbdumW<IfrSpec> {
        NbdumW::new(self, 16)
    }
}
#[doc = "Instruction Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfrSpec;
impl crate::RegisterSpec for IfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifr::R`](R) reader structure"]
impl crate::Readable for IfrSpec {}
#[doc = "`write(|w| ..)` method takes [`ifr::W`](W) writer structure"]
impl crate::Writable for IfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFR to value 0"]
impl crate::Resettable for IfrSpec {}
