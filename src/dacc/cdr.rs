#[doc = "Register `CDR[%s]` writer"]
pub type W = crate::W<CdrSpec>;
#[doc = "Field `DATA0` writer - Data to Convert for channel 0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATA1` writer - Data to Convert for channel 1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data to Convert for channel 0"]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<CdrSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to Convert for channel 1"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<CdrSpec> {
        Data1W::new(self, 16)
    }
}
#[doc = "Conversion Data Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdrSpec;
impl crate::RegisterSpec for CdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDR[%s] to value 0"]
impl crate::Resettable for CdrSpec {}
