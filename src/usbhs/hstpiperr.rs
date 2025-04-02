#[doc = "Register `HSTPIPERR[%s]` reader"]
pub type R = crate::R<HstpiperrSpec>;
#[doc = "Register `HSTPIPERR[%s]` writer"]
pub type W = crate::W<HstpiperrSpec>;
#[doc = "Field `DATATGL` reader - Data Toggle Error"]
pub type DatatglR = crate::BitReader;
#[doc = "Field `DATATGL` writer - Data Toggle Error"]
pub type DatatglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAPID` reader - Data PID Error"]
pub type DatapidR = crate::BitReader;
#[doc = "Field `DATAPID` writer - Data PID Error"]
pub type DatapidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID` reader - Data PID Error"]
pub type PidR = crate::BitReader;
#[doc = "Field `PID` writer - Data PID Error"]
pub type PidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Time-Out Error"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Time-Out Error"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC16` reader - CRC16 Error"]
pub type Crc16R = crate::BitReader;
#[doc = "Field `CRC16` writer - CRC16 Error"]
pub type Crc16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTER` reader - Error Counter"]
pub type CounterR = crate::FieldReader;
#[doc = "Field `COUNTER` writer - Error Counter"]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatgl(&self) -> DatatglR {
        DatatglR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    pub fn datapid(&self) -> DatapidR {
        DatapidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data PID Error"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    pub fn crc16(&self) -> Crc16R {
        Crc16R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatgl(&mut self) -> DatatglW<HstpiperrSpec> {
        DatatglW::new(self, 0)
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    pub fn datapid(&mut self) -> DatapidW<HstpiperrSpec> {
        DatapidW::new(self, 1)
    }
    #[doc = "Bit 2 - Data PID Error"]
    #[inline(always)]
    pub fn pid(&mut self) -> PidW<HstpiperrSpec> {
        PidW::new(self, 2)
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<HstpiperrSpec> {
        TimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    pub fn crc16(&mut self) -> Crc16W<HstpiperrSpec> {
        Crc16W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    pub fn counter(&mut self) -> CounterW<HstpiperrSpec> {
        CounterW::new(self, 5)
    }
}
#[doc = "Host Pipe Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpiperr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpiperr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpiperrSpec;
impl crate::RegisterSpec for HstpiperrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpiperr::R`](R) reader structure"]
impl crate::Readable for HstpiperrSpec {}
#[doc = "`write(|w| ..)` method takes [`hstpiperr::W`](W) writer structure"]
impl crate::Writable for HstpiperrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTPIPERR[%s] to value 0"]
impl crate::Resettable for HstpiperrSpec {}
