#[doc = "Register `MLBC0` reader"]
pub type R = crate::R<Mlbc0Spec>;
#[doc = "Register `MLBC0` writer"]
pub type W = crate::W<Mlbc0Spec>;
#[doc = "Field `MLBEN` reader - MediaLB Enable"]
pub type MlbenR = crate::BitReader;
#[doc = "Field `MLBEN` writer - MediaLB Enable"]
pub type MlbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MLBCLK (MediaLB clock) Speed Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mlbclkselect {
    #[doc = "0: 256xFs (for MLBPEN = 0)"]
    _256Fs = 0,
    #[doc = "1: 512xFs (for MLBPEN = 0)"]
    _512Fs = 1,
    #[doc = "2: 1024xFs (for MLBPEN = 0)"]
    _1024Fs = 2,
}
impl From<Mlbclkselect> for u8 {
    #[inline(always)]
    fn from(variant: Mlbclkselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mlbclkselect {
    type Ux = u8;
}
impl crate::IsEnum for Mlbclkselect {}
#[doc = "Field `MLBCLK` reader - MLBCLK (MediaLB clock) Speed Select"]
pub type MlbclkR = crate::FieldReader<Mlbclkselect>;
impl MlbclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mlbclkselect> {
        match self.bits {
            0 => Some(Mlbclkselect::_256Fs),
            1 => Some(Mlbclkselect::_512Fs),
            2 => Some(Mlbclkselect::_1024Fs),
            _ => None,
        }
    }
    #[doc = "256xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn is_256_fs(&self) -> bool {
        *self == Mlbclkselect::_256Fs
    }
    #[doc = "512xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn is_512_fs(&self) -> bool {
        *self == Mlbclkselect::_512Fs
    }
    #[doc = "1024xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn is_1024_fs(&self) -> bool {
        *self == Mlbclkselect::_1024Fs
    }
}
#[doc = "Field `MLBCLK` writer - MLBCLK (MediaLB clock) Speed Select"]
pub type MlbclkW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mlbclkselect>;
impl<'a, REG> MlbclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _256_fs(self) -> &'a mut crate::W<REG> {
        self.variant(Mlbclkselect::_256Fs)
    }
    #[doc = "512xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _512_fs(self) -> &'a mut crate::W<REG> {
        self.variant(Mlbclkselect::_512Fs)
    }
    #[doc = "1024xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _1024_fs(self) -> &'a mut crate::W<REG> {
        self.variant(Mlbclkselect::_1024Fs)
    }
}
#[doc = "Field `ZERO` reader - Must be Written to 0"]
pub type ZeroR = crate::BitReader;
#[doc = "Field `ZERO` writer - Must be Written to 0"]
pub type ZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MLBLK` reader - MediaLB Lock Status (read-only)"]
pub type MlblkR = crate::BitReader;
#[doc = "Field `MLBLK` writer - MediaLB Lock Status (read-only)"]
pub type MlblkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYRETRY` reader - Asynchronous Tx Packet Retry"]
pub type AsyretryR = crate::BitReader;
#[doc = "Field `ASYRETRY` writer - Asynchronous Tx Packet Retry"]
pub type AsyretryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTLRETRY` reader - Control Tx Packet Retry"]
pub type CtlretryR = crate::BitReader;
#[doc = "Field `CTLRETRY` writer - Control Tx Packet Retry"]
pub type CtlretryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "The number of frames per sub-buffer for synchronous channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fcntselect {
    #[doc = "0: 1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    _1Frame = 0,
    #[doc = "1: 2 frames per sub-buffer"]
    _2Frames = 1,
    #[doc = "2: 4 frames per sub-buffer"]
    _4Frames = 2,
    #[doc = "3: 8 frames per sub-buffer"]
    _8Frames = 3,
    #[doc = "4: 16 frames per sub-buffer"]
    _16Frames = 4,
    #[doc = "5: 32 frames per sub-buffer"]
    _32Frames = 5,
    #[doc = "6: 64 frames per sub-buffer"]
    _64Frames = 6,
}
impl From<Fcntselect> for u8 {
    #[inline(always)]
    fn from(variant: Fcntselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fcntselect {
    type Ux = u8;
}
impl crate::IsEnum for Fcntselect {}
#[doc = "Field `FCNT` reader - The number of frames per sub-buffer for synchronous channels"]
pub type FcntR = crate::FieldReader<Fcntselect>;
impl FcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fcntselect> {
        match self.bits {
            0 => Some(Fcntselect::_1Frame),
            1 => Some(Fcntselect::_2Frames),
            2 => Some(Fcntselect::_4Frames),
            3 => Some(Fcntselect::_8Frames),
            4 => Some(Fcntselect::_16Frames),
            5 => Some(Fcntselect::_32Frames),
            6 => Some(Fcntselect::_64Frames),
            _ => None,
        }
    }
    #[doc = "1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    #[inline(always)]
    pub fn is_1_frame(&self) -> bool {
        *self == Fcntselect::_1Frame
    }
    #[doc = "2 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_2_frames(&self) -> bool {
        *self == Fcntselect::_2Frames
    }
    #[doc = "4 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_4_frames(&self) -> bool {
        *self == Fcntselect::_4Frames
    }
    #[doc = "8 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_8_frames(&self) -> bool {
        *self == Fcntselect::_8Frames
    }
    #[doc = "16 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_16_frames(&self) -> bool {
        *self == Fcntselect::_16Frames
    }
    #[doc = "32 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_32_frames(&self) -> bool {
        *self == Fcntselect::_32Frames
    }
    #[doc = "64 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_64_frames(&self) -> bool {
        *self == Fcntselect::_64Frames
    }
}
#[doc = "Field `FCNT` writer - The number of frames per sub-buffer for synchronous channels"]
pub type FcntW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fcntselect>;
impl<'a, REG> FcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    #[inline(always)]
    pub fn _1_frame(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntselect::_1Frame)
    }
    #[doc = "2 frames per sub-buffer"]
    #[inline(always)]
    pub fn _2_frames(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntselect::_2Frames)
    }
    #[doc = "4 frames per sub-buffer"]
    #[inline(always)]
    pub fn _4_frames(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntselect::_4Frames)
    }
    #[doc = "8 frames per sub-buffer"]
    #[inline(always)]
    pub fn _8_frames(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntselect::_8Frames)
    }
    #[doc = "16 frames per sub-buffer"]
    #[inline(always)]
    pub fn _16_frames(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntselect::_16Frames)
    }
    #[doc = "32 frames per sub-buffer"]
    #[inline(always)]
    pub fn _32_frames(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntselect::_32Frames)
    }
    #[doc = "64 frames per sub-buffer"]
    #[inline(always)]
    pub fn _64_frames(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntselect::_64Frames)
    }
}
impl R {
    #[doc = "Bit 0 - MediaLB Enable"]
    #[inline(always)]
    pub fn mlben(&self) -> MlbenR {
        MlbenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - MLBCLK (MediaLB clock) Speed Select"]
    #[inline(always)]
    pub fn mlbclk(&self) -> MlbclkR {
        MlbclkR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Must be Written to 0"]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - MediaLB Lock Status (read-only)"]
    #[inline(always)]
    pub fn mlblk(&self) -> MlblkR {
        MlblkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Asynchronous Tx Packet Retry"]
    #[inline(always)]
    pub fn asyretry(&self) -> AsyretryR {
        AsyretryR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Control Tx Packet Retry"]
    #[inline(always)]
    pub fn ctlretry(&self) -> CtlretryR {
        CtlretryR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - The number of frames per sub-buffer for synchronous channels"]
    #[inline(always)]
    pub fn fcnt(&self) -> FcntR {
        FcntR::new(((self.bits >> 15) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MediaLB Enable"]
    #[inline(always)]
    pub fn mlben(&mut self) -> MlbenW<Mlbc0Spec> {
        MlbenW::new(self, 0)
    }
    #[doc = "Bits 2:4 - MLBCLK (MediaLB clock) Speed Select"]
    #[inline(always)]
    pub fn mlbclk(&mut self) -> MlbclkW<Mlbc0Spec> {
        MlbclkW::new(self, 2)
    }
    #[doc = "Bit 5 - Must be Written to 0"]
    #[inline(always)]
    pub fn zero(&mut self) -> ZeroW<Mlbc0Spec> {
        ZeroW::new(self, 5)
    }
    #[doc = "Bit 7 - MediaLB Lock Status (read-only)"]
    #[inline(always)]
    pub fn mlblk(&mut self) -> MlblkW<Mlbc0Spec> {
        MlblkW::new(self, 7)
    }
    #[doc = "Bit 12 - Asynchronous Tx Packet Retry"]
    #[inline(always)]
    pub fn asyretry(&mut self) -> AsyretryW<Mlbc0Spec> {
        AsyretryW::new(self, 12)
    }
    #[doc = "Bit 14 - Control Tx Packet Retry"]
    #[inline(always)]
    pub fn ctlretry(&mut self) -> CtlretryW<Mlbc0Spec> {
        CtlretryW::new(self, 14)
    }
    #[doc = "Bits 15:17 - The number of frames per sub-buffer for synchronous channels"]
    #[inline(always)]
    pub fn fcnt(&mut self) -> FcntW<Mlbc0Spec> {
        FcntW::new(self, 15)
    }
}
#[doc = "MediaLB Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mlbc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlbc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mlbc0Spec;
impl crate::RegisterSpec for Mlbc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mlbc0::R`](R) reader structure"]
impl crate::Readable for Mlbc0Spec {}
#[doc = "`write(|w| ..)` method takes [`mlbc0::W`](W) writer structure"]
impl crate::Writable for Mlbc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MLBC0 to value 0"]
impl crate::Resettable for Mlbc0Spec {}
