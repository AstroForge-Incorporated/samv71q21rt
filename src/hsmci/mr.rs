#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWSDIV` reader - Power Saving Divider"]
pub type PwsdivR = crate::FieldReader;
#[doc = "Field `PWSDIV` writer - Power Saving Divider"]
pub type PwsdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDPROOF` reader - Read Proof Enable"]
pub type RdproofR = crate::BitReader;
#[doc = "Field `RDPROOF` writer - Read Proof Enable"]
pub type RdproofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPROOF` reader - Write Proof Enable"]
pub type WrproofR = crate::BitReader;
#[doc = "Field `WRPROOF` writer - Write Proof Enable"]
pub type WrproofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBYTE` reader - Force Byte Transfer"]
pub type FbyteR = crate::BitReader;
#[doc = "Field `FBYTE` writer - Force Byte Transfer"]
pub type FbyteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADV` reader - Padding Value"]
pub type PadvR = crate::BitReader;
#[doc = "Field `PADV` writer - Padding Value"]
pub type PadvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKODD` reader - Clock divider is odd"]
pub type ClkoddR = crate::BitReader;
#[doc = "Field `CLKODD` writer - Clock divider is odd"]
pub type ClkoddW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    pub fn pwsdiv(&self) -> PwsdivR {
        PwsdivR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Read Proof Enable"]
    #[inline(always)]
    pub fn rdproof(&self) -> RdproofR {
        RdproofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write Proof Enable"]
    #[inline(always)]
    pub fn wrproof(&self) -> WrproofR {
        WrproofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    pub fn fbyte(&self) -> FbyteR {
        FbyteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    pub fn padv(&self) -> PadvR {
        PadvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Clock divider is odd"]
    #[inline(always)]
    pub fn clkodd(&self) -> ClkoddR {
        ClkoddR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<MrSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    pub fn pwsdiv(&mut self) -> PwsdivW<MrSpec> {
        PwsdivW::new(self, 8)
    }
    #[doc = "Bit 11 - Read Proof Enable"]
    #[inline(always)]
    pub fn rdproof(&mut self) -> RdproofW<MrSpec> {
        RdproofW::new(self, 11)
    }
    #[doc = "Bit 12 - Write Proof Enable"]
    #[inline(always)]
    pub fn wrproof(&mut self) -> WrproofW<MrSpec> {
        WrproofW::new(self, 12)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    pub fn fbyte(&mut self) -> FbyteW<MrSpec> {
        FbyteW::new(self, 13)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    pub fn padv(&mut self) -> PadvW<MrSpec> {
        PadvW::new(self, 14)
    }
    #[doc = "Bit 16 - Clock divider is odd"]
    #[inline(always)]
    pub fn clkodd(&mut self) -> ClkoddW<MrSpec> {
        ClkoddW::new(self, 16)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {}
