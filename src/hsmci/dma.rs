#[doc = "Register `DMA` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "DMA Channel Read and Write Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chksizeselect {
    #[doc = "0: 1 data available"]
    _1 = 0,
    #[doc = "1: 2 data available"]
    _2 = 1,
    #[doc = "2: 4 data available"]
    _4 = 2,
    #[doc = "3: 8 data available"]
    _8 = 3,
    #[doc = "4: 16 data available"]
    _16 = 4,
}
impl From<Chksizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Chksizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chksizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Chksizeselect {}
#[doc = "Field `CHKSIZE` reader - DMA Channel Read and Write Chunk Size"]
pub type ChksizeR = crate::FieldReader<Chksizeselect>;
impl ChksizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chksizeselect> {
        match self.bits {
            0 => Some(Chksizeselect::_1),
            1 => Some(Chksizeselect::_2),
            2 => Some(Chksizeselect::_4),
            3 => Some(Chksizeselect::_8),
            4 => Some(Chksizeselect::_16),
            _ => None,
        }
    }
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chksizeselect::_1
    }
    #[doc = "2 data available"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Chksizeselect::_2
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Chksizeselect::_4
    }
    #[doc = "8 data available"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Chksizeselect::_8
    }
    #[doc = "16 data available"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Chksizeselect::_16
    }
}
#[doc = "Field `CHKSIZE` writer - DMA Channel Read and Write Chunk Size"]
pub type ChksizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Chksizeselect>;
impl<'a, REG> ChksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Chksizeselect::_1)
    }
    #[doc = "2 data available"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Chksizeselect::_2)
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Chksizeselect::_4)
    }
    #[doc = "8 data available"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Chksizeselect::_8)
    }
    #[doc = "16 data available"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Chksizeselect::_16)
    }
}
#[doc = "Field `DMAEN` reader - DMA Hardware Handshaking Enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Hardware Handshaking Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&self) -> ChksizeR {
        ChksizeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&mut self) -> ChksizeW<DmaSpec> {
        ChksizeW::new(self, 4)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<DmaSpec> {
        DmaenW::new(self, 8)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DmaSpec {}
