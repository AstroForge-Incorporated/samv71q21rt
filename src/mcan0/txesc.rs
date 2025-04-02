#[doc = "Register `TXESC` reader"]
pub type R = crate::R<TxescSpec>;
#[doc = "Register `TXESC` writer"]
pub type W = crate::W<TxescSpec>;
#[doc = "Tx Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbdsselect {
    #[doc = "0: 8-byte data field"]
    _8Byte = 0,
    #[doc = "1: 12-byte data field"]
    _12Byte = 1,
    #[doc = "2: 16-byte data field"]
    _16Byte = 2,
    #[doc = "3: 20-byte data field"]
    _20Byte = 3,
    #[doc = "4: 24-byte data field"]
    _24Byte = 4,
    #[doc = "5: 32-byte data field"]
    _32Byte = 5,
    #[doc = "6: 48-byte data field"]
    _48Byte = 6,
    #[doc = "7: 64-byte data field"]
    _64Byte = 7,
}
impl From<Tbdsselect> for u8 {
    #[inline(always)]
    fn from(variant: Tbdsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbdsselect {
    type Ux = u8;
}
impl crate::IsEnum for Tbdsselect {}
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size"]
pub type TbdsR = crate::FieldReader<Tbdsselect>;
impl TbdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbdsselect {
        match self.bits {
            0 => Tbdsselect::_8Byte,
            1 => Tbdsselect::_12Byte,
            2 => Tbdsselect::_16Byte,
            3 => Tbdsselect::_20Byte,
            4 => Tbdsselect::_24Byte,
            5 => Tbdsselect::_32Byte,
            6 => Tbdsselect::_48Byte,
            7 => Tbdsselect::_64Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == Tbdsselect::_8Byte
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        *self == Tbdsselect::_12Byte
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == Tbdsselect::_16Byte
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        *self == Tbdsselect::_20Byte
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        *self == Tbdsselect::_24Byte
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == Tbdsselect::_32Byte
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        *self == Tbdsselect::_48Byte
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == Tbdsselect::_64Byte
    }
}
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size"]
pub type TbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tbdsselect, crate::Safe>;
impl<'a, REG> TbdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::_8Byte)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::_12Byte)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::_16Byte)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::_20Byte)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::_24Byte)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::_32Byte)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::_48Byte)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Tbdsselect::_64Byte)
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TbdsR {
        TbdsR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&mut self) -> TbdsW<TxescSpec> {
        TbdsW::new(self, 0)
    }
}
#[doc = "Transmit Buffer Element Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxescSpec;
impl crate::RegisterSpec for TxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txesc::R`](R) reader structure"]
impl crate::Readable for TxescSpec {}
#[doc = "`write(|w| ..)` method takes [`txesc::W`](W) writer structure"]
impl crate::Writable for TxescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TxescSpec {}
