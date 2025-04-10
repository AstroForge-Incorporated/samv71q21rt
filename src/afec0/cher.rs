#[doc = "Register `CHER` writer"]
pub type W = crate::W<CherSpec>;
#[doc = "Field `CH0` writer - Channel 0 Enable"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Channel 1 Enable"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Channel 2 Enable"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Channel 3 Enable"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Channel 4 Enable"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Channel 5 Enable"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Channel 6 Enable"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Channel 7 Enable"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Channel 8 Enable"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Channel 9 Enable"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Channel 10 Enable"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Channel 11 Enable"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<CherSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<CherSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Enable"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<CherSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Enable"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<CherSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Enable"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<CherSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Enable"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<CherSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Enable"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<CherSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Enable"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<CherSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Enable"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<CherSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Enable"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<CherSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Enable"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<CherSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Enable"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<CherSpec> {
        Ch11W::new(self, 11)
    }
}
#[doc = "AFEC Channel Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cher::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CherSpec;
impl crate::RegisterSpec for CherSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cher::W`](W) writer structure"]
impl crate::Writable for CherSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHER to value 0"]
impl crate::Resettable for CherSpec {}
