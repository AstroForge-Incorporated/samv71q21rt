#[doc = "Register `US_MR_USART_MODE` reader"]
pub type R = crate::R<UsMrUsartModeSpec>;
#[doc = "Register `US_MR_USART_MODE` writer"]
pub type W = crate::W<UsMrUsartModeSpec>;
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
#[doc = "Field `SYNC` reader - Synchronous Mode Select"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - Synchronous Mode Select"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Parselect {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
    #[doc = "2: Parity forced to 0 (Space)"]
    Space = 2,
    #[doc = "3: Parity forced to 1 (Mark)"]
    Mark = 3,
    #[doc = "4: No parity"]
    No = 4,
    #[doc = "6: Multidrop mode"]
    Multidrop = 6,
}
impl From<Parselect> for u8 {
    #[inline(always)]
    fn from(variant: Parselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Parselect {
    type Ux = u8;
}
impl crate::IsEnum for Parselect {}
#[doc = "Field `PAR` reader - Parity Type"]
pub type ParR = crate::FieldReader<Parselect>;
impl ParR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Parselect> {
        match self.bits {
            0 => Some(Parselect::Even),
            1 => Some(Parselect::Odd),
            2 => Some(Parselect::Space),
            3 => Some(Parselect::Mark),
            4 => Some(Parselect::No),
            6 => Some(Parselect::Multidrop),
            _ => None,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Parselect::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Parselect::Odd
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == Parselect::Space
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == Parselect::Mark
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Parselect::No
    }
    #[doc = "Multidrop mode"]
    #[inline(always)]
    pub fn is_multidrop(&self) -> bool {
        *self == Parselect::Multidrop
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub type ParW<'a, REG> = crate::FieldWriter<'a, REG, 3, Parselect>;
impl<'a, REG> ParW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::Odd)
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline(always)]
    pub fn space(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::Space)
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::Mark)
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::No)
    }
    #[doc = "Multidrop mode"]
    #[inline(always)]
    pub fn multidrop(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::Multidrop)
    }
}
#[doc = "Number of Stop Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nbstopselect {
    #[doc = "0: 1 stop bit"]
    _1Bit = 0,
    #[doc = "1: 1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    _1_5Bit = 1,
    #[doc = "2: 2 stop bits"]
    _2Bit = 2,
}
impl From<Nbstopselect> for u8 {
    #[inline(always)]
    fn from(variant: Nbstopselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nbstopselect {
    type Ux = u8;
}
impl crate::IsEnum for Nbstopselect {}
#[doc = "Field `NBSTOP` reader - Number of Stop Bits"]
pub type NbstopR = crate::FieldReader<Nbstopselect>;
impl NbstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nbstopselect> {
        match self.bits {
            0 => Some(Nbstopselect::_1Bit),
            1 => Some(Nbstopselect::_1_5Bit),
            2 => Some(Nbstopselect::_2Bit),
            _ => None,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        *self == Nbstopselect::_1Bit
    }
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    #[inline(always)]
    pub fn is_1_5_bit(&self) -> bool {
        *self == Nbstopselect::_1_5Bit
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == Nbstopselect::_2Bit
    }
}
#[doc = "Field `NBSTOP` writer - Number of Stop Bits"]
pub type NbstopW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nbstopselect>;
impl<'a, REG> NbstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Nbstopselect::_1Bit)
    }
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    #[inline(always)]
    pub fn _1_5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Nbstopselect::_1_5Bit)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Nbstopselect::_2Bit)
    }
}
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chmodeselect {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: Automatic Echo. Receiver input is connected to the TXD pin."]
    Automatic = 1,
    #[doc = "2: Local Loopback. Transmitter output is connected to the Receiver Input."]
    LocalLoopback = 2,
    #[doc = "3: Remote Loopback. RXD pin is internally connected to the TXD pin."]
    RemoteLoopback = 3,
}
impl From<Chmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Chmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Chmodeselect {}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub type ChmodeR = crate::FieldReader<Chmodeselect>;
impl ChmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chmodeselect {
        match self.bits {
            0 => Chmodeselect::Normal,
            1 => Chmodeselect::Automatic,
            2 => Chmodeselect::LocalLoopback,
            3 => Chmodeselect::RemoteLoopback,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Chmodeselect::Normal
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == Chmodeselect::Automatic
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        *self == Chmodeselect::LocalLoopback
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        *self == Chmodeselect::RemoteLoopback
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub type ChmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chmodeselect, crate::Safe>;
impl<'a, REG> ChmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Chmodeselect::Normal)
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(Chmodeselect::Automatic)
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Chmodeselect::LocalLoopback)
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Chmodeselect::RemoteLoopback)
    }
}
#[doc = "Field `MSBF` reader - Bit Order"]
pub type MsbfR = crate::BitReader;
#[doc = "Field `MSBF` writer - Bit Order"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE9` reader - 9-bit Character Length"]
pub type Mode9R = crate::BitReader;
#[doc = "Field `MODE9` writer - 9-bit Character Length"]
pub type Mode9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKO` reader - Clock Output Select"]
pub type ClkoR = crate::BitReader;
#[doc = "Field `CLKO` writer - Clock Output Select"]
pub type ClkoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER` reader - Oversampling Mode"]
pub type OverR = crate::BitReader;
#[doc = "Field `OVER` writer - Oversampling Mode"]
pub type OverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACK` reader - Inhibit Non Acknowledge"]
pub type InackR = crate::BitReader;
#[doc = "Field `INACK` writer - Inhibit Non Acknowledge"]
pub type InackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub type DsnackR = crate::BitReader;
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub type DsnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VAR_SYNC` reader - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub type VarSyncR = crate::BitReader;
#[doc = "Field `VAR_SYNC` writer - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub type VarSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVDATA` reader - Inverted Data"]
pub type InvdataR = crate::BitReader;
#[doc = "Field `INVDATA` writer - Inverted Data"]
pub type InvdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_ITERATION` reader - Maximum Number of Automatic Iteration"]
pub type MaxIterationR = crate::FieldReader;
#[doc = "Field `MAX_ITERATION` writer - Maximum Number of Automatic Iteration"]
pub type MaxIterationW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FILTER` reader - Receive Line Filter"]
pub type FilterR = crate::BitReader;
#[doc = "Field `FILTER` writer - Receive Line Filter"]
pub type FilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAN` reader - Manchester Encoder/Decoder Enable"]
pub type ManR = crate::BitReader;
#[doc = "Field `MAN` writer - Manchester Encoder/Decoder Enable"]
pub type ManW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSYNC` reader - Manchester Synchronization Mode"]
pub type ModsyncR = crate::BitReader;
#[doc = "Field `MODSYNC` writer - Manchester Synchronization Mode"]
pub type ModsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEBIT` reader - Start Frame Delimiter Selector"]
pub type OnebitR = crate::BitReader;
#[doc = "Field `ONEBIT` writer - Start Frame Delimiter Selector"]
pub type OnebitW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> ParR {
        ParR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    pub fn nbstop(&self) -> NbstopR {
        NbstopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> ChmodeR {
        ChmodeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    pub fn mode9(&self) -> Mode9R {
        Mode9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> ClkoR {
        ClkoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    pub fn over(&self) -> OverR {
        OverR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> InackR {
        InackR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DsnackR {
        DsnackR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    pub fn var_sync(&self) -> VarSyncR {
        VarSyncR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    pub fn invdata(&self) -> InvdataR {
        InvdataR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    pub fn max_iteration(&self) -> MaxIterationR {
        MaxIterationR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FilterR {
        FilterR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    pub fn man(&self) -> ManR {
        ManR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    pub fn modsync(&self) -> ModsyncR {
        ModsyncR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    pub fn onebit(&self) -> OnebitR {
        OnebitR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&mut self) -> UsartModeW<UsMrUsartModeSpec> {
        UsartModeW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&mut self) -> UsclksW<UsMrUsartModeSpec> {
        UsclksW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&mut self) -> ChrlW<UsMrUsartModeSpec> {
        ChrlW::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<UsMrUsartModeSpec> {
        SyncW::new(self, 8)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&mut self) -> ParW<UsMrUsartModeSpec> {
        ParW::new(self, 9)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    pub fn nbstop(&mut self) -> NbstopW<UsMrUsartModeSpec> {
        NbstopW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&mut self) -> ChmodeW<UsMrUsartModeSpec> {
        ChmodeW::new(self, 14)
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MsbfW<UsMrUsartModeSpec> {
        MsbfW::new(self, 16)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    pub fn mode9(&mut self) -> Mode9W<UsMrUsartModeSpec> {
        Mode9W::new(self, 17)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&mut self) -> ClkoW<UsMrUsartModeSpec> {
        ClkoW::new(self, 18)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    pub fn over(&mut self) -> OverW<UsMrUsartModeSpec> {
        OverW::new(self, 19)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    pub fn inack(&mut self) -> InackW<UsMrUsartModeSpec> {
        InackW::new(self, 20)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&mut self) -> DsnackW<UsMrUsartModeSpec> {
        DsnackW::new(self, 21)
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    pub fn var_sync(&mut self) -> VarSyncW<UsMrUsartModeSpec> {
        VarSyncW::new(self, 22)
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    pub fn invdata(&mut self) -> InvdataW<UsMrUsartModeSpec> {
        InvdataW::new(self, 23)
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    pub fn max_iteration(&mut self) -> MaxIterationW<UsMrUsartModeSpec> {
        MaxIterationW::new(self, 24)
    }
    #[doc = "Bit 28 - Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&mut self) -> FilterW<UsMrUsartModeSpec> {
        FilterW::new(self, 28)
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    pub fn man(&mut self) -> ManW<UsMrUsartModeSpec> {
        ManW::new(self, 29)
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    pub fn modsync(&mut self) -> ModsyncW<UsMrUsartModeSpec> {
        ModsyncW::new(self, 30)
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    pub fn onebit(&mut self) -> OnebitW<UsMrUsartModeSpec> {
        OnebitW::new(self, 31)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_mr_usart_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_mr_usart_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsMrUsartModeSpec;
impl crate::RegisterSpec for UsMrUsartModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_mr_usart_mode::R`](R) reader structure"]
impl crate::Readable for UsMrUsartModeSpec {}
#[doc = "`write(|w| ..)` method takes [`us_mr_usart_mode::W`](W) writer structure"]
impl crate::Writable for UsMrUsartModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_MR_USART_MODE to value 0"]
impl crate::Resettable for UsMrUsartModeSpec {}
