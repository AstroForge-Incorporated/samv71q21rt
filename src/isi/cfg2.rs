#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Field `IM_VSIZE` reader - Vertical Size of the Image Sensor \\[0..2047\\]"]
pub type ImVsizeR = crate::FieldReader<u16>;
#[doc = "Field `IM_VSIZE` writer - Vertical Size of the Image Sensor \\[0..2047\\]"]
pub type ImVsizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `GS_MODE` reader - Grayscale Pixel Format Mode"]
pub type GsModeR = crate::BitReader;
#[doc = "Field `GS_MODE` writer - Grayscale Pixel Format Mode"]
pub type GsModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGB_MODE` reader - RGB Input Mode"]
pub type RgbModeR = crate::BitReader;
#[doc = "Field `RGB_MODE` writer - RGB Input Mode"]
pub type RgbModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRAYSCALE` reader - Grayscale Mode Format Enable"]
pub type GrayscaleR = crate::BitReader;
#[doc = "Field `GRAYSCALE` writer - Grayscale Mode Format Enable"]
pub type GrayscaleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGB_SWAP` reader - RGB Format Swap Mode"]
pub type RgbSwapR = crate::BitReader;
#[doc = "Field `RGB_SWAP` writer - RGB Format Swap Mode"]
pub type RgbSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COL_SPACE` reader - Color Space for the Image Data"]
pub type ColSpaceR = crate::BitReader;
#[doc = "Field `COL_SPACE` writer - Color Space for the Image Data"]
pub type ColSpaceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM_HSIZE` reader - Horizontal Size of the Image Sensor \\[0..2047\\]"]
pub type ImHsizeR = crate::FieldReader<u16>;
#[doc = "Field `IM_HSIZE` writer - Horizontal Size of the Image Sensor \\[0..2047\\]"]
pub type ImHsizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "YCrCb Format Swap Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YccSwapselect {
    #[doc = "0: Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    Default = 0,
    #[doc = "1: Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    Mode1 = 1,
    #[doc = "2: Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    Mode2 = 2,
    #[doc = "3: Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    Mode3 = 3,
}
impl From<YccSwapselect> for u8 {
    #[inline(always)]
    fn from(variant: YccSwapselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for YccSwapselect {
    type Ux = u8;
}
impl crate::IsEnum for YccSwapselect {}
#[doc = "Field `YCC_SWAP` reader - YCrCb Format Swap Mode"]
pub type YccSwapR = crate::FieldReader<YccSwapselect>;
impl YccSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> YccSwapselect {
        match self.bits {
            0 => YccSwapselect::Default,
            1 => YccSwapselect::Mode1,
            2 => YccSwapselect::Mode2,
            3 => YccSwapselect::Mode3,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == YccSwapselect::Default
    }
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == YccSwapselect::Mode1
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == YccSwapselect::Mode2
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == YccSwapselect::Mode3
    }
}
#[doc = "Field `YCC_SWAP` writer - YCrCb Format Swap Mode"]
pub type YccSwapW<'a, REG> = crate::FieldWriter<'a, REG, 2, YccSwapselect, crate::Safe>;
impl<'a, REG> YccSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(YccSwapselect::Default)
    }
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(YccSwapselect::Mode1)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(YccSwapselect::Mode2)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(YccSwapselect::Mode3)
    }
}
#[doc = "RGB Pixel Mapping Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RgbCfgselect {
    #[doc = "0: Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    Default = 0,
    #[doc = "1: Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    Mode1 = 1,
    #[doc = "2: Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    Mode2 = 2,
    #[doc = "3: Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    Mode3 = 3,
}
impl From<RgbCfgselect> for u8 {
    #[inline(always)]
    fn from(variant: RgbCfgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RgbCfgselect {
    type Ux = u8;
}
impl crate::IsEnum for RgbCfgselect {}
#[doc = "Field `RGB_CFG` reader - RGB Pixel Mapping Configuration"]
pub type RgbCfgR = crate::FieldReader<RgbCfgselect>;
impl RgbCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgbCfgselect {
        match self.bits {
            0 => RgbCfgselect::Default,
            1 => RgbCfgselect::Mode1,
            2 => RgbCfgselect::Mode2,
            3 => RgbCfgselect::Mode3,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == RgbCfgselect::Default
    }
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == RgbCfgselect::Mode1
    }
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == RgbCfgselect::Mode2
    }
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == RgbCfgselect::Mode3
    }
}
#[doc = "Field `RGB_CFG` writer - RGB Pixel Mapping Configuration"]
pub type RgbCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, RgbCfgselect, crate::Safe>;
impl<'a, REG> RgbCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(RgbCfgselect::Default)
    }
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(RgbCfgselect::Mode1)
    }
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(RgbCfgselect::Mode2)
    }
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(RgbCfgselect::Mode3)
    }
}
impl R {
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&self) -> ImVsizeR {
        ImVsizeR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&self) -> GsModeR {
        GsModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&self) -> RgbModeR {
        RgbModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&self) -> GrayscaleR {
        GrayscaleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&self) -> RgbSwapR {
        RgbSwapR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&self) -> ColSpaceR {
        ColSpaceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&self) -> ImHsizeR {
        ImHsizeR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&self) -> YccSwapR {
        YccSwapR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&self) -> RgbCfgR {
        RgbCfgR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&mut self) -> ImVsizeW<Cfg2Spec> {
        ImVsizeW::new(self, 0)
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&mut self) -> GsModeW<Cfg2Spec> {
        GsModeW::new(self, 11)
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&mut self) -> RgbModeW<Cfg2Spec> {
        RgbModeW::new(self, 12)
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&mut self) -> GrayscaleW<Cfg2Spec> {
        GrayscaleW::new(self, 13)
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&mut self) -> RgbSwapW<Cfg2Spec> {
        RgbSwapW::new(self, 14)
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&mut self) -> ColSpaceW<Cfg2Spec> {
        ColSpaceW::new(self, 15)
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&mut self) -> ImHsizeW<Cfg2Spec> {
        ImHsizeW::new(self, 16)
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&mut self) -> YccSwapW<Cfg2Spec> {
        YccSwapW::new(self, 28)
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&mut self) -> RgbCfgW<Cfg2Spec> {
        RgbCfgW::new(self, 30)
    }
}
#[doc = "ISI Configuration 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for Cfg2Spec {}
