#[doc = "Register `SEQ2R` reader"]
pub type R = crate::R<Seq2rSpec>;
#[doc = "Register `SEQ2R` writer"]
pub type W = crate::W<Seq2rSpec>;
#[doc = "Field `USCH8` reader - User Sequence Number 8"]
pub type Usch8R = crate::FieldReader;
#[doc = "Field `USCH8` writer - User Sequence Number 8"]
pub type Usch8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH9` reader - User Sequence Number 9"]
pub type Usch9R = crate::FieldReader;
#[doc = "Field `USCH9` writer - User Sequence Number 9"]
pub type Usch9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH10` reader - User Sequence Number 10"]
pub type Usch10R = crate::FieldReader;
#[doc = "Field `USCH10` writer - User Sequence Number 10"]
pub type Usch10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH11` reader - User Sequence Number 11"]
pub type Usch11R = crate::FieldReader;
#[doc = "Field `USCH11` writer - User Sequence Number 11"]
pub type Usch11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&self) -> Usch8R {
        Usch8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&self) -> Usch9R {
        Usch9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&self) -> Usch10R {
        Usch10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&self) -> Usch11R {
        Usch11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&mut self) -> Usch8W<Seq2rSpec> {
        Usch8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&mut self) -> Usch9W<Seq2rSpec> {
        Usch9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&mut self) -> Usch10W<Seq2rSpec> {
        Usch10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&mut self) -> Usch11W<Seq2rSpec> {
        Usch11W::new(self, 12)
    }
}
#[doc = "AFEC Channel Sequence 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seq2rSpec;
impl crate::RegisterSpec for Seq2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq2r::R`](R) reader structure"]
impl crate::Readable for Seq2rSpec {}
#[doc = "`write(|w| ..)` method takes [`seq2r::W`](W) writer structure"]
impl crate::Writable for Seq2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQ2R to value 0"]
impl crate::Resettable for Seq2rSpec {}
