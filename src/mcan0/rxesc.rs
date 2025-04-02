#[doc = "Register `RXESC` reader"]
pub type R = crate::R<RxescSpec>;
#[doc = "Register `RXESC` writer"]
pub type W = crate::W<RxescSpec>;
#[doc = "Receive FIFO 0 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F0dsselect {
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
impl From<F0dsselect> for u8 {
    #[inline(always)]
    fn from(variant: F0dsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for F0dsselect {
    type Ux = u8;
}
impl crate::IsEnum for F0dsselect {}
#[doc = "Field `F0DS` reader - Receive FIFO 0 Data Field Size"]
pub type F0dsR = crate::FieldReader<F0dsselect>;
impl F0dsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F0dsselect {
        match self.bits {
            0 => F0dsselect::_8Byte,
            1 => F0dsselect::_12Byte,
            2 => F0dsselect::_16Byte,
            3 => F0dsselect::_20Byte,
            4 => F0dsselect::_24Byte,
            5 => F0dsselect::_32Byte,
            6 => F0dsselect::_48Byte,
            7 => F0dsselect::_64Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == F0dsselect::_8Byte
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        *self == F0dsselect::_12Byte
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == F0dsselect::_16Byte
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        *self == F0dsselect::_20Byte
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        *self == F0dsselect::_24Byte
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == F0dsselect::_32Byte
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        *self == F0dsselect::_48Byte
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == F0dsselect::_64Byte
    }
}
#[doc = "Field `F0DS` writer - Receive FIFO 0 Data Field Size"]
pub type F0dsW<'a, REG> = crate::FieldWriter<'a, REG, 3, F0dsselect, crate::Safe>;
impl<'a, REG> F0dsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::_8Byte)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::_12Byte)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::_16Byte)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::_20Byte)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::_24Byte)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::_32Byte)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::_48Byte)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F0dsselect::_64Byte)
    }
}
#[doc = "Receive FIFO 1 Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F1dsselect {
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
impl From<F1dsselect> for u8 {
    #[inline(always)]
    fn from(variant: F1dsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for F1dsselect {
    type Ux = u8;
}
impl crate::IsEnum for F1dsselect {}
#[doc = "Field `F1DS` reader - Receive FIFO 1 Data Field Size"]
pub type F1dsR = crate::FieldReader<F1dsselect>;
impl F1dsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F1dsselect {
        match self.bits {
            0 => F1dsselect::_8Byte,
            1 => F1dsselect::_12Byte,
            2 => F1dsselect::_16Byte,
            3 => F1dsselect::_20Byte,
            4 => F1dsselect::_24Byte,
            5 => F1dsselect::_32Byte,
            6 => F1dsselect::_48Byte,
            7 => F1dsselect::_64Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == F1dsselect::_8Byte
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        *self == F1dsselect::_12Byte
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == F1dsselect::_16Byte
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        *self == F1dsselect::_20Byte
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        *self == F1dsselect::_24Byte
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == F1dsselect::_32Byte
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        *self == F1dsselect::_48Byte
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == F1dsselect::_64Byte
    }
}
#[doc = "Field `F1DS` writer - Receive FIFO 1 Data Field Size"]
pub type F1dsW<'a, REG> = crate::FieldWriter<'a, REG, 3, F1dsselect, crate::Safe>;
impl<'a, REG> F1dsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::_8Byte)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::_12Byte)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::_16Byte)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::_20Byte)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::_24Byte)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::_32Byte)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::_48Byte)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(F1dsselect::_64Byte)
    }
}
#[doc = "Receive Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rbdsselect {
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
impl From<Rbdsselect> for u8 {
    #[inline(always)]
    fn from(variant: Rbdsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rbdsselect {
    type Ux = u8;
}
impl crate::IsEnum for Rbdsselect {}
#[doc = "Field `RBDS` reader - Receive Buffer Data Field Size"]
pub type RbdsR = crate::FieldReader<Rbdsselect>;
impl RbdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbdsselect {
        match self.bits {
            0 => Rbdsselect::_8Byte,
            1 => Rbdsselect::_12Byte,
            2 => Rbdsselect::_16Byte,
            3 => Rbdsselect::_20Byte,
            4 => Rbdsselect::_24Byte,
            5 => Rbdsselect::_32Byte,
            6 => Rbdsselect::_48Byte,
            7 => Rbdsselect::_64Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == Rbdsselect::_8Byte
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        *self == Rbdsselect::_12Byte
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == Rbdsselect::_16Byte
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        *self == Rbdsselect::_20Byte
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        *self == Rbdsselect::_24Byte
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == Rbdsselect::_32Byte
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        *self == Rbdsselect::_48Byte
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == Rbdsselect::_64Byte
    }
}
#[doc = "Field `RBDS` writer - Receive Buffer Data Field Size"]
pub type RbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rbdsselect, crate::Safe>;
impl<'a, REG> RbdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::_8Byte)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::_12Byte)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::_16Byte)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::_20Byte)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::_24Byte)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::_32Byte)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::_48Byte)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Rbdsselect::_64Byte)
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO 0 Data Field Size"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0dsR {
        F0dsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Receive FIFO 1 Data Field Size"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1dsR {
        F1dsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Receive Buffer Data Field Size"]
    #[inline(always)]
    pub fn rbds(&self) -> RbdsR {
        RbdsR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive FIFO 0 Data Field Size"]
    #[inline(always)]
    pub fn f0ds(&mut self) -> F0dsW<RxescSpec> {
        F0dsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Receive FIFO 1 Data Field Size"]
    #[inline(always)]
    pub fn f1ds(&mut self) -> F1dsW<RxescSpec> {
        F1dsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Receive Buffer Data Field Size"]
    #[inline(always)]
    pub fn rbds(&mut self) -> RbdsW<RxescSpec> {
        RbdsW::new(self, 8)
    }
}
#[doc = "Receive Buffer / FIFO Element Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxescSpec;
impl crate::RegisterSpec for RxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxesc::R`](R) reader structure"]
impl crate::Readable for RxescSpec {}
#[doc = "`write(|w| ..)` method takes [`rxesc::W`](W) writer structure"]
impl crate::Writable for RxescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RxescSpec {}
