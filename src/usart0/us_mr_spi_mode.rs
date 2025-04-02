#[doc = "Register `US_MR_SPI_MODE` reader"]
pub type R = crate::R<UsMrSpiModeSpec>;
#[doc = "Register `US_MR_SPI_MODE` writer"]
pub type W = crate::W<UsMrSpiModeSpec>;
#[doc = "USART Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsartModeselect {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: RS485"]
    Rs485 = 1,
    #[doc = "2: Hardware handshaking"]
    HwHandshaking = 2,
    #[doc = "3: Modem"]
    Modem = 3,
    #[doc = "4: IS07816 Protocol: T = 0"]
    Is07816T0 = 4,
    #[doc = "6: IS07816 Protocol: T = 1"]
    Is07816T1 = 6,
    #[doc = "8: IrDA"]
    Irda = 8,
    #[doc = "9: LON"]
    Lon = 9,
    #[doc = "10: LIN Master mode"]
    LinMaster = 10,
    #[doc = "11: LIN Slave mode"]
    LinSlave = 11,
    #[doc = "14: SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    SpiMaster = 14,
    #[doc = "15: SPI Slave mode"]
    SpiSlave = 15,
}
impl From<UsartModeselect> for u8 {
    #[inline(always)]
    fn from(variant: UsartModeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsartModeselect {
    type Ux = u8;
}
impl crate::IsEnum for UsartModeselect {}
#[doc = "Field `USART_MODE` reader - USART Mode of Operation"]
pub type UsartModeR = crate::FieldReader<UsartModeselect>;
impl UsartModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UsartModeselect> {
        match self.bits {
            0 => Some(UsartModeselect::Normal),
            1 => Some(UsartModeselect::Rs485),
            2 => Some(UsartModeselect::HwHandshaking),
            3 => Some(UsartModeselect::Modem),
            4 => Some(UsartModeselect::Is07816T0),
            6 => Some(UsartModeselect::Is07816T1),
            8 => Some(UsartModeselect::Irda),
            9 => Some(UsartModeselect::Lon),
            10 => Some(UsartModeselect::LinMaster),
            11 => Some(UsartModeselect::LinSlave),
            14 => Some(UsartModeselect::SpiMaster),
            15 => Some(UsartModeselect::SpiSlave),
            _ => None,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == UsartModeselect::Normal
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == UsartModeselect::Rs485
    }
    #[doc = "Hardware handshaking"]
    #[inline(always)]
    pub fn is_hw_handshaking(&self) -> bool {
        *self == UsartModeselect::HwHandshaking
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == UsartModeselect::Modem
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is_is07816_t_0(&self) -> bool {
        *self == UsartModeselect::Is07816T0
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is_is07816_t_1(&self) -> bool {
        *self == UsartModeselect::Is07816T1
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == UsartModeselect::Irda
    }
    #[doc = "LON"]
    #[inline(always)]
    pub fn is_lon(&self) -> bool {
        *self == UsartModeselect::Lon
    }
    #[doc = "LIN Master mode"]
    #[inline(always)]
    pub fn is_lin_master(&self) -> bool {
        *self == UsartModeselect::LinMaster
    }
    #[doc = "LIN Slave mode"]
    #[inline(always)]
    pub fn is_lin_slave(&self) -> bool {
        *self == UsartModeselect::LinSlave
    }
    #[doc = "SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == UsartModeselect::SpiMaster
    }
    #[doc = "SPI Slave mode"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == UsartModeselect::SpiSlave
    }
}
#[doc = "Field `USART_MODE` writer - USART Mode of Operation"]
pub type UsartModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, UsartModeselect>;
impl<'a, REG> UsartModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::Normal)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::Rs485)
    }
    #[doc = "Hardware handshaking"]
    #[inline(always)]
    pub fn hw_handshaking(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::HwHandshaking)
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn modem(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::Modem)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is07816_t_0(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::Is07816T0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is07816_t_1(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::Is07816T1)
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::Irda)
    }
    #[doc = "LON"]
    #[inline(always)]
    pub fn lon(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::Lon)
    }
    #[doc = "LIN Master mode"]
    #[inline(always)]
    pub fn lin_master(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::LinMaster)
    }
    #[doc = "LIN Slave mode"]
    #[inline(always)]
    pub fn lin_slave(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::LinSlave)
    }
    #[doc = "SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::SpiMaster)
    }
    #[doc = "SPI Slave mode"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(UsartModeselect::SpiSlave)
    }
}
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usclksselect {
    #[doc = "0: Peripheral clock is selected"]
    Mck = 0,
    #[doc = "1: Peripheral clock divided (DIV = 8) is selected"]
    Div = 1,
    #[doc = "2: PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    Pck = 2,
    #[doc = "3: Serial clock (SCK) is selected"]
    Sck = 3,
}
impl From<Usclksselect> for u8 {
    #[inline(always)]
    fn from(variant: Usclksselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usclksselect {
    type Ux = u8;
}
impl crate::IsEnum for Usclksselect {}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub type UsclksR = crate::FieldReader<Usclksselect>;
impl UsclksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usclksselect {
        match self.bits {
            0 => Usclksselect::Mck,
            1 => Usclksselect::Div,
            2 => Usclksselect::Pck,
            3 => Usclksselect::Sck,
            _ => unreachable!(),
        }
    }
    #[doc = "Peripheral clock is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Usclksselect::Mck
    }
    #[doc = "Peripheral clock divided (DIV = 8) is selected"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == Usclksselect::Div
    }
    #[doc = "PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    #[inline(always)]
    pub fn is_pck(&self) -> bool {
        *self == Usclksselect::Pck
    }
    #[doc = "Serial clock (SCK) is selected"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == Usclksselect::Sck
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub type UsclksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usclksselect, crate::Safe>;
impl<'a, REG> UsclksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Usclksselect::Mck)
    }
    #[doc = "Peripheral clock divided (DIV = 8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(Usclksselect::Div)
    }
    #[doc = "PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    #[inline(always)]
    pub fn pck(self) -> &'a mut crate::W<REG> {
        self.variant(Usclksselect::Pck)
    }
    #[doc = "Serial clock (SCK) is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut crate::W<REG> {
        self.variant(Usclksselect::Sck)
    }
}
#[doc = "Character Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chrlselect {
    #[doc = "0: Character length is 5 bits"]
    _5Bit = 0,
    #[doc = "1: Character length is 6 bits"]
    _6Bit = 1,
    #[doc = "2: Character length is 7 bits"]
    _7Bit = 2,
    #[doc = "3: Character length is 8 bits"]
    _8Bit = 3,
}
impl From<Chrlselect> for u8 {
    #[inline(always)]
    fn from(variant: Chrlselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chrlselect {
    type Ux = u8;
}
impl crate::IsEnum for Chrlselect {}
#[doc = "Field `CHRL` reader - Character Length"]
pub type ChrlR = crate::FieldReader<Chrlselect>;
impl ChrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chrlselect {
        match self.bits {
            0 => Chrlselect::_5Bit,
            1 => Chrlselect::_6Bit,
            2 => Chrlselect::_7Bit,
            3 => Chrlselect::_8Bit,
            _ => unreachable!(),
        }
    }
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == Chrlselect::_5Bit
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == Chrlselect::_6Bit
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == Chrlselect::_7Bit
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Chrlselect::_8Bit
    }
}
#[doc = "Field `CHRL` writer - Character Length"]
pub type ChrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chrlselect, crate::Safe>;
impl<'a, REG> ChrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chrlselect::_5Bit)
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chrlselect::_6Bit)
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chrlselect::_7Bit)
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chrlselect::_8Bit)
    }
}
#[doc = "Field `CPHA` reader - SPI Clock Phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - SPI Clock Phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - SPI Clock Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - SPI Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKO` reader - Clock Output Select"]
pub type ClkoR = crate::BitReader;
#[doc = "Field `CLKO` writer - Clock Output Select"]
pub type ClkoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRDBT` reader - Wait Read Data Before Transfer"]
pub type WrdbtR = crate::BitReader;
#[doc = "Field `WRDBT` writer - Wait Read Data Before Transfer"]
pub type WrdbtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&self) -> UsartModeR {
        UsartModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> UsclksR {
        UsclksR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&self) -> ChrlR {
        ChrlR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> ClkoR {
        ClkoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&self) -> WrdbtR {
        WrdbtR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&mut self) -> UsartModeW<UsMrSpiModeSpec> {
        UsartModeW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&mut self) -> UsclksW<UsMrSpiModeSpec> {
        UsclksW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&mut self) -> ChrlW<UsMrSpiModeSpec> {
        ChrlW::new(self, 6)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<UsMrSpiModeSpec> {
        CphaW::new(self, 8)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<UsMrSpiModeSpec> {
        CpolW::new(self, 16)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&mut self) -> ClkoW<UsMrSpiModeSpec> {
        ClkoW::new(self, 18)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&mut self) -> WrdbtW<UsMrSpiModeSpec> {
        WrdbtW::new(self, 20)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_mr_spi_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_mr_spi_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsMrSpiModeSpec;
impl crate::RegisterSpec for UsMrSpiModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_mr_spi_mode::R`](R) reader structure"]
impl crate::Readable for UsMrSpiModeSpec {}
#[doc = "`write(|w| ..)` method takes [`us_mr_spi_mode::W`](W) writer structure"]
impl crate::Writable for UsMrSpiModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_MR_SPI_MODE to value 0"]
impl crate::Resettable for UsMrSpiModeSpec {}
