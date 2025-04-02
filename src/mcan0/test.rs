#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Loop Back Mode (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbckselect {
    #[doc = "0: Reset value. Loop Back mode is disabled."]
    Disabled = 0,
    #[doc = "1: Loop Back mode is enabled (see Section 6.1.9)."]
    Enabled = 1,
}
impl From<Lbckselect> for bool {
    #[inline(always)]
    fn from(variant: Lbckselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCK` reader - Loop Back Mode (read/write)"]
pub type LbckR = crate::BitReader<Lbckselect>;
impl LbckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbckselect {
        match self.bits {
            false => Lbckselect::Disabled,
            true => Lbckselect::Enabled,
        }
    }
    #[doc = "Reset value. Loop Back mode is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lbckselect::Disabled
    }
    #[doc = "Loop Back mode is enabled (see Section 6.1.9)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lbckselect::Enabled
    }
}
#[doc = "Field `LBCK` writer - Loop Back Mode (read/write)"]
pub type LbckW<'a, REG> = crate::BitWriter<'a, REG, Lbckselect>;
impl<'a, REG> LbckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset value. Loop Back mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbckselect::Disabled)
    }
    #[doc = "Loop Back mode is enabled (see Section 6.1.9)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbckselect::Enabled)
    }
}
#[doc = "Control of Transmit Pin (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txselect {
    #[doc = "0: Reset value, CANTX controlled by the CAN Core, updated at the end of the CAN bit time."]
    Reset = 0,
    #[doc = "1: Sample Point can be monitored at pin CANTX."]
    SamplePointMonitoring = 1,
    #[doc = "2: Dominant ('0') level at pin CANTX."]
    Dominant = 2,
    #[doc = "3: Recessive ('1') at pin CANTX."]
    Recessive = 3,
}
impl From<Txselect> for u8 {
    #[inline(always)]
    fn from(variant: Txselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txselect {
    type Ux = u8;
}
impl crate::IsEnum for Txselect {}
#[doc = "Field `TX` reader - Control of Transmit Pin (read/write)"]
pub type TxR = crate::FieldReader<Txselect>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txselect {
        match self.bits {
            0 => Txselect::Reset,
            1 => Txselect::SamplePointMonitoring,
            2 => Txselect::Dominant,
            3 => Txselect::Recessive,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset value, CANTX controlled by the CAN Core, updated at the end of the CAN bit time."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Txselect::Reset
    }
    #[doc = "Sample Point can be monitored at pin CANTX."]
    #[inline(always)]
    pub fn is_sample_point_monitoring(&self) -> bool {
        *self == Txselect::SamplePointMonitoring
    }
    #[doc = "Dominant ('0') level at pin CANTX."]
    #[inline(always)]
    pub fn is_dominant(&self) -> bool {
        *self == Txselect::Dominant
    }
    #[doc = "Recessive ('1') at pin CANTX."]
    #[inline(always)]
    pub fn is_recessive(&self) -> bool {
        *self == Txselect::Recessive
    }
}
#[doc = "Field `TX` writer - Control of Transmit Pin (read/write)"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txselect, crate::Safe>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset value, CANTX controlled by the CAN Core, updated at the end of the CAN bit time."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Txselect::Reset)
    }
    #[doc = "Sample Point can be monitored at pin CANTX."]
    #[inline(always)]
    pub fn sample_point_monitoring(self) -> &'a mut crate::W<REG> {
        self.variant(Txselect::SamplePointMonitoring)
    }
    #[doc = "Dominant ('0') level at pin CANTX."]
    #[inline(always)]
    pub fn dominant(self) -> &'a mut crate::W<REG> {
        self.variant(Txselect::Dominant)
    }
    #[doc = "Recessive ('1') at pin CANTX."]
    #[inline(always)]
    pub fn recessive(self) -> &'a mut crate::W<REG> {
        self.variant(Txselect::Recessive)
    }
}
#[doc = "Field `RX` reader - Receive Pin (read-only)"]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - Receive Pin (read-only)"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Loop Back Mode (read/write)"]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin (read/write)"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Pin (read-only)"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back Mode (read/write)"]
    #[inline(always)]
    pub fn lbck(&mut self) -> LbckW<TestSpec> {
        LbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin (read/write)"]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<TestSpec> {
        TxW::new(self, 5)
    }
    #[doc = "Bit 7 - Receive Pin (read-only)"]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<TestSpec> {
        RxW::new(self, 7)
    }
}
#[doc = "Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}
