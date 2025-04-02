#[doc = "Register `MIEN` reader"]
pub type R = crate::R<MienSpec>;
#[doc = "Register `MIEN` writer"]
pub type W = crate::W<MienSpec>;
#[doc = "Field `ISOC_PE` reader - Isochronous Rx Protocol Error Enable"]
pub type IsocPeR = crate::BitReader;
#[doc = "Field `ISOC_PE` writer - Isochronous Rx Protocol Error Enable"]
pub type IsocPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOC_BUFO` reader - Isochronous Rx Buffer Overflow Enable"]
pub type IsocBufoR = crate::BitReader;
#[doc = "Field `ISOC_BUFO` writer - Isochronous Rx Buffer Overflow Enable"]
pub type IsocBufoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_PE` reader - Synchronous Protocol Error Enable"]
pub type SyncPeR = crate::BitReader;
#[doc = "Field `SYNC_PE` writer - Synchronous Protocol Error Enable"]
pub type SyncPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARX_DONE` reader - Asynchronous Rx Done Enable"]
pub type ArxDoneR = crate::BitReader;
#[doc = "Field `ARX_DONE` writer - Asynchronous Rx Done Enable"]
pub type ArxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARX_PE` reader - Asynchronous Rx Protocol Error Enable"]
pub type ArxPeR = crate::BitReader;
#[doc = "Field `ARX_PE` writer - Asynchronous Rx Protocol Error Enable"]
pub type ArxPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARX_BREAK` reader - Asynchronous Rx Break Enable"]
pub type ArxBreakR = crate::BitReader;
#[doc = "Field `ARX_BREAK` writer - Asynchronous Rx Break Enable"]
pub type ArxBreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATX_DONE` reader - Asynchronous Tx Packet Done Enable"]
pub type AtxDoneR = crate::BitReader;
#[doc = "Field `ATX_DONE` writer - Asynchronous Tx Packet Done Enable"]
pub type AtxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATX_PE` reader - Asynchronous Tx Protocol Error Enable"]
pub type AtxPeR = crate::BitReader;
#[doc = "Field `ATX_PE` writer - Asynchronous Tx Protocol Error Enable"]
pub type AtxPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATX_BREAK` reader - Asynchronous Tx Break Enable"]
pub type AtxBreakR = crate::BitReader;
#[doc = "Field `ATX_BREAK` writer - Asynchronous Tx Break Enable"]
pub type AtxBreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRX_DONE` reader - Control Rx Packet Done Enable"]
pub type CrxDoneR = crate::BitReader;
#[doc = "Field `CRX_DONE` writer - Control Rx Packet Done Enable"]
pub type CrxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRX_PE` reader - Control Rx Protocol Error Enable"]
pub type CrxPeR = crate::BitReader;
#[doc = "Field `CRX_PE` writer - Control Rx Protocol Error Enable"]
pub type CrxPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRX_BREAK` reader - Control Rx Break Enable"]
pub type CrxBreakR = crate::BitReader;
#[doc = "Field `CRX_BREAK` writer - Control Rx Break Enable"]
pub type CrxBreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTX_DONE` reader - Control Tx Packet Done Enable"]
pub type CtxDoneR = crate::BitReader;
#[doc = "Field `CTX_DONE` writer - Control Tx Packet Done Enable"]
pub type CtxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTX_PE` reader - Control Tx Protocol Error Enable"]
pub type CtxPeR = crate::BitReader;
#[doc = "Field `CTX_PE` writer - Control Tx Protocol Error Enable"]
pub type CtxPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTX_BREAK` reader - Control Tx Break Enable"]
pub type CtxBreakR = crate::BitReader;
#[doc = "Field `CTX_BREAK` writer - Control Tx Break Enable"]
pub type CtxBreakW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Isochronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn isoc_pe(&self) -> IsocPeR {
        IsocPeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Isochronous Rx Buffer Overflow Enable"]
    #[inline(always)]
    pub fn isoc_bufo(&self) -> IsocBufoR {
        IsocBufoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronous Protocol Error Enable"]
    #[inline(always)]
    pub fn sync_pe(&self) -> SyncPeR {
        SyncPeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Asynchronous Rx Done Enable"]
    #[inline(always)]
    pub fn arx_done(&self) -> ArxDoneR {
        ArxDoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Asynchronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn arx_pe(&self) -> ArxPeR {
        ArxPeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Asynchronous Rx Break Enable"]
    #[inline(always)]
    pub fn arx_break(&self) -> ArxBreakR {
        ArxBreakR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Asynchronous Tx Packet Done Enable"]
    #[inline(always)]
    pub fn atx_done(&self) -> AtxDoneR {
        AtxDoneR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Asynchronous Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn atx_pe(&self) -> AtxPeR {
        AtxPeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Asynchronous Tx Break Enable"]
    #[inline(always)]
    pub fn atx_break(&self) -> AtxBreakR {
        AtxBreakR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Control Rx Packet Done Enable"]
    #[inline(always)]
    pub fn crx_done(&self) -> CrxDoneR {
        CrxDoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Control Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn crx_pe(&self) -> CrxPeR {
        CrxPeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Control Rx Break Enable"]
    #[inline(always)]
    pub fn crx_break(&self) -> CrxBreakR {
        CrxBreakR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Control Tx Packet Done Enable"]
    #[inline(always)]
    pub fn ctx_done(&self) -> CtxDoneR {
        CtxDoneR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Control Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn ctx_pe(&self) -> CtxPeR {
        CtxPeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Control Tx Break Enable"]
    #[inline(always)]
    pub fn ctx_break(&self) -> CtxBreakR {
        CtxBreakR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Isochronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn isoc_pe(&mut self) -> IsocPeW<MienSpec> {
        IsocPeW::new(self, 0)
    }
    #[doc = "Bit 1 - Isochronous Rx Buffer Overflow Enable"]
    #[inline(always)]
    pub fn isoc_bufo(&mut self) -> IsocBufoW<MienSpec> {
        IsocBufoW::new(self, 1)
    }
    #[doc = "Bit 16 - Synchronous Protocol Error Enable"]
    #[inline(always)]
    pub fn sync_pe(&mut self) -> SyncPeW<MienSpec> {
        SyncPeW::new(self, 16)
    }
    #[doc = "Bit 17 - Asynchronous Rx Done Enable"]
    #[inline(always)]
    pub fn arx_done(&mut self) -> ArxDoneW<MienSpec> {
        ArxDoneW::new(self, 17)
    }
    #[doc = "Bit 18 - Asynchronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn arx_pe(&mut self) -> ArxPeW<MienSpec> {
        ArxPeW::new(self, 18)
    }
    #[doc = "Bit 19 - Asynchronous Rx Break Enable"]
    #[inline(always)]
    pub fn arx_break(&mut self) -> ArxBreakW<MienSpec> {
        ArxBreakW::new(self, 19)
    }
    #[doc = "Bit 20 - Asynchronous Tx Packet Done Enable"]
    #[inline(always)]
    pub fn atx_done(&mut self) -> AtxDoneW<MienSpec> {
        AtxDoneW::new(self, 20)
    }
    #[doc = "Bit 21 - Asynchronous Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn atx_pe(&mut self) -> AtxPeW<MienSpec> {
        AtxPeW::new(self, 21)
    }
    #[doc = "Bit 22 - Asynchronous Tx Break Enable"]
    #[inline(always)]
    pub fn atx_break(&mut self) -> AtxBreakW<MienSpec> {
        AtxBreakW::new(self, 22)
    }
    #[doc = "Bit 24 - Control Rx Packet Done Enable"]
    #[inline(always)]
    pub fn crx_done(&mut self) -> CrxDoneW<MienSpec> {
        CrxDoneW::new(self, 24)
    }
    #[doc = "Bit 25 - Control Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn crx_pe(&mut self) -> CrxPeW<MienSpec> {
        CrxPeW::new(self, 25)
    }
    #[doc = "Bit 26 - Control Rx Break Enable"]
    #[inline(always)]
    pub fn crx_break(&mut self) -> CrxBreakW<MienSpec> {
        CrxBreakW::new(self, 26)
    }
    #[doc = "Bit 27 - Control Tx Packet Done Enable"]
    #[inline(always)]
    pub fn ctx_done(&mut self) -> CtxDoneW<MienSpec> {
        CtxDoneW::new(self, 27)
    }
    #[doc = "Bit 28 - Control Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn ctx_pe(&mut self) -> CtxPeW<MienSpec> {
        CtxPeW::new(self, 28)
    }
    #[doc = "Bit 29 - Control Tx Break Enable"]
    #[inline(always)]
    pub fn ctx_break(&mut self) -> CtxBreakW<MienSpec> {
        CtxBreakW::new(self, 29)
    }
}
#[doc = "MediaLB Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MienSpec;
impl crate::RegisterSpec for MienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mien::R`](R) reader structure"]
impl crate::Readable for MienSpec {}
#[doc = "`write(|w| ..)` method takes [`mien::W`](W) writer structure"]
impl crate::Writable for MienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIEN to value 0"]
impl crate::Resettable for MienSpec {}
