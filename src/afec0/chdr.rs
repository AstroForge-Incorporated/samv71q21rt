#[doc = "Register `CHDR` writer"]
pub type W = crate::W<ChdrSpec>;
#[doc = "Field `CH0` writer - Channel 0 Disable"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Channel 1 Disable"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Channel 2 Disable"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Channel 3 Disable"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Channel 4 Disable"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Channel 5 Disable"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Channel 6 Disable"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Channel 7 Disable"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Channel 8 Disable"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Channel 9 Disable"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Channel 10 Disable"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Channel 11 Disable"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Disable"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<ChdrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Disable"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<ChdrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Disable"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<ChdrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Disable"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<ChdrSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Disable"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<ChdrSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Disable"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<ChdrSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Disable"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<ChdrSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Disable"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<ChdrSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Disable"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<ChdrSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Disable"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<ChdrSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Disable"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<ChdrSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Disable"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<ChdrSpec> {
        Ch11W::new(self, 11)
    }
}
#[doc = "AFEC Channel Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChdrSpec;
impl crate::RegisterSpec for ChdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chdr::W`](W) writer structure"]
impl crate::Writable for ChdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHDR to value 0"]
impl crate::Resettable for ChdrSpec {}
