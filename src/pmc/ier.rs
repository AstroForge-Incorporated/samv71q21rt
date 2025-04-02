#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Enable"]
pub type MoscxtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Enable"]
pub type LockaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Enable"]
pub type MckrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKU` writer - UTMI PLL Lock Interrupt Enable"]
pub type LockuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Enable"]
pub type Pckrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Enable"]
pub type Pckrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Enable"]
pub type Pckrdy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY3` writer - Programmable Clock Ready 3 Interrupt Enable"]
pub type Pckrdy3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY4` writer - Programmable Clock Ready 4 Interrupt Enable"]
pub type Pckrdy4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY5` writer - Programmable Clock Ready 5 Interrupt Enable"]
pub type Pckrdy5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY6` writer - Programmable Clock Ready 6 Interrupt Enable"]
pub type Pckrdy6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY7` writer - Programmable Clock Ready 7 Interrupt Enable"]
pub type Pckrdy7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCSELS` writer - Main Clock Source Oscillator Selection Status Interrupt Enable"]
pub type MoscselsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCRCS` writer - Main RC Oscillator Status Interrupt Enable"]
pub type MoscrcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Enable"]
pub type CfdevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT32KERR` writer - 32.768 kHz Crystal Oscillator Error Interrupt Enable"]
pub type Xt32kerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Enable"]
    #[inline(always)]
    pub fn moscxts(&mut self) -> MoscxtsW<IerSpec> {
        MoscxtsW::new(self, 0)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Enable"]
    #[inline(always)]
    pub fn locka(&mut self) -> LockaW<IerSpec> {
        LockaW::new(self, 1)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Enable"]
    #[inline(always)]
    pub fn mckrdy(&mut self) -> MckrdyW<IerSpec> {
        MckrdyW::new(self, 3)
    }
    #[doc = "Bit 6 - UTMI PLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn locku(&mut self) -> LockuW<IerSpec> {
        LockuW::new(self, 6)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy0(&mut self) -> Pckrdy0W<IerSpec> {
        Pckrdy0W::new(self, 8)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy1(&mut self) -> Pckrdy1W<IerSpec> {
        Pckrdy1W::new(self, 9)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy2(&mut self) -> Pckrdy2W<IerSpec> {
        Pckrdy2W::new(self, 10)
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy3(&mut self) -> Pckrdy3W<IerSpec> {
        Pckrdy3W::new(self, 11)
    }
    #[doc = "Bit 12 - Programmable Clock Ready 4 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy4(&mut self) -> Pckrdy4W<IerSpec> {
        Pckrdy4W::new(self, 12)
    }
    #[doc = "Bit 13 - Programmable Clock Ready 5 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy5(&mut self) -> Pckrdy5W<IerSpec> {
        Pckrdy5W::new(self, 13)
    }
    #[doc = "Bit 14 - Programmable Clock Ready 6 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy6(&mut self) -> Pckrdy6W<IerSpec> {
        Pckrdy6W::new(self, 14)
    }
    #[doc = "Bit 15 - Programmable Clock Ready 7 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy7(&mut self) -> Pckrdy7W<IerSpec> {
        Pckrdy7W::new(self, 15)
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status Interrupt Enable"]
    #[inline(always)]
    pub fn moscsels(&mut self) -> MoscselsW<IerSpec> {
        MoscselsW::new(self, 16)
    }
    #[doc = "Bit 17 - Main RC Oscillator Status Interrupt Enable"]
    #[inline(always)]
    pub fn moscrcs(&mut self) -> MoscrcsW<IerSpec> {
        MoscrcsW::new(self, 17)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Enable"]
    #[inline(always)]
    pub fn cfdev(&mut self) -> CfdevW<IerSpec> {
        CfdevW::new(self, 18)
    }
    #[doc = "Bit 21 - 32.768 kHz Crystal Oscillator Error Interrupt Enable"]
    #[inline(always)]
    pub fn xt32kerr(&mut self) -> Xt32kerrW<IerSpec> {
        Xt32kerrW::new(self, 21)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
