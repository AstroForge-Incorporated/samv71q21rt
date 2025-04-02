#[doc = "Register `OCR` reader"]
pub type R = crate::R<OcrSpec>;
#[doc = "Register `OCR` writer"]
pub type W = crate::W<OcrSpec>;
#[doc = "Field `CAL4` reader - Main RC Oscillator Calibration Bits for 4 MHz"]
pub type Cal4R = crate::FieldReader;
#[doc = "Field `CAL4` writer - Main RC Oscillator Calibration Bits for 4 MHz"]
pub type Cal4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL4` reader - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
pub type Sel4R = crate::BitReader;
#[doc = "Field `SEL4` writer - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
pub type Sel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL8` reader - Main RC Oscillator Calibration Bits for 8 MHz"]
pub type Cal8R = crate::FieldReader;
#[doc = "Field `CAL8` writer - Main RC Oscillator Calibration Bits for 8 MHz"]
pub type Cal8W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL8` reader - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
pub type Sel8R = crate::BitReader;
#[doc = "Field `SEL8` writer - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
pub type Sel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL12` reader - Main RC Oscillator Calibration Bits for 12 MHz"]
pub type Cal12R = crate::FieldReader;
#[doc = "Field `CAL12` writer - Main RC Oscillator Calibration Bits for 12 MHz"]
pub type Cal12W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL12` reader - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
pub type Sel12R = crate::BitReader;
#[doc = "Field `SEL12` writer - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
pub type Sel12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&self) -> Cal4R {
        Cal4R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&self) -> Sel4R {
        Sel4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&self) -> Cal8R {
        Cal8R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&self) -> Sel8R {
        Sel8R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&self) -> Cal12R {
        Cal12R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&self) -> Sel12R {
        Sel12R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&mut self) -> Cal4W<OcrSpec> {
        Cal4W::new(self, 0)
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&mut self) -> Sel4W<OcrSpec> {
        Sel4W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&mut self) -> Cal8W<OcrSpec> {
        Cal8W::new(self, 8)
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&mut self) -> Sel8W<OcrSpec> {
        Sel8W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&mut self) -> Cal12W<OcrSpec> {
        Cal12W::new(self, 16)
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&mut self) -> Sel12W<OcrSpec> {
        Sel12W::new(self, 23)
    }
}
#[doc = "Oscillator Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcrSpec;
impl crate::RegisterSpec for OcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocr::R`](R) reader structure"]
impl crate::Readable for OcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ocr::W`](W) writer structure"]
impl crate::Writable for OcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCR to value 0"]
impl crate::Resettable for OcrSpec {}
