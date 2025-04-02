#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `CMDRDY` reader - Command Ready (cleared by writing in HSMCI_CMDR)"]
pub type CmdrdyR = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading HSMCI_RDR)"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Ready (cleared by writing in HSMCI_TDR)"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `BLKE` reader - Data Block Ended (cleared on read)"]
pub type BlkeR = crate::BitReader;
#[doc = "Field `DTIP` reader - Data Transfer in Progress (cleared at the end of CRC16 calculation)"]
pub type DtipR = crate::BitReader;
#[doc = "Field `NOTBUSY` reader - HSMCI Not Busy"]
pub type NotbusyR = crate::BitReader;
#[doc = "Field `SDIOIRQA` reader - SDIO Interrupt for Slot A (cleared on read)"]
pub type SdioirqaR = crate::BitReader;
#[doc = "Field `SDIOWAIT` reader - SDIO Read Wait Operation Status"]
pub type SdiowaitR = crate::BitReader;
#[doc = "Field `CSRCV` reader - CE-ATA Completion Signal Received (cleared on read)"]
pub type CsrcvR = crate::BitReader;
#[doc = "Field `RINDE` reader - Response Index Error (cleared by writing in HSMCI_CMDR)"]
pub type RindeR = crate::BitReader;
#[doc = "Field `RDIRE` reader - Response Direction Error (cleared by writing in HSMCI_CMDR)"]
pub type RdireR = crate::BitReader;
#[doc = "Field `RCRCE` reader - Response CRC Error (cleared by writing in HSMCI_CMDR)"]
pub type RcrceR = crate::BitReader;
#[doc = "Field `RENDE` reader - Response End Bit Error (cleared by writing in HSMCI_CMDR)"]
pub type RendeR = crate::BitReader;
#[doc = "Field `RTOE` reader - Response Time-out Error (cleared by writing in HSMCI_CMDR)"]
pub type RtoeR = crate::BitReader;
#[doc = "Field `DCRCE` reader - Data CRC Error (cleared on read)"]
pub type DcrceR = crate::BitReader;
#[doc = "Field `DTOE` reader - Data Time-out Error (cleared on read)"]
pub type DtoeR = crate::BitReader;
#[doc = "Field `CSTOE` reader - Completion Signal Time-out Error (cleared on read)"]
pub type CstoeR = crate::BitReader;
#[doc = "Field `BLKOVRE` reader - DMA Block Overrun Error (cleared on read)"]
pub type BlkovreR = crate::BitReader;
#[doc = "Field `FIFOEMPTY` reader - FIFO empty flag"]
pub type FifoemptyR = crate::BitReader;
#[doc = "Field `XFRDONE` reader - Transfer Done flag"]
pub type XfrdoneR = crate::BitReader;
#[doc = "Field `ACKRCV` reader - Boot Operation Acknowledge Received (cleared on read)"]
pub type AckrcvR = crate::BitReader;
#[doc = "Field `ACKRCVE` reader - Boot Operation Acknowledge Error (cleared on read)"]
pub type AckrcveR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
pub type OvreR = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
pub type UnreR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command Ready (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn cmdrdy(&self) -> CmdrdyR {
        CmdrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Ready (cleared by reading HSMCI_RDR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Ready (cleared by writing in HSMCI_TDR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Block Ended (cleared on read)"]
    #[inline(always)]
    pub fn blke(&self) -> BlkeR {
        BlkeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transfer in Progress (cleared at the end of CRC16 calculation)"]
    #[inline(always)]
    pub fn dtip(&self) -> DtipR {
        DtipR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSMCI Not Busy"]
    #[inline(always)]
    pub fn notbusy(&self) -> NotbusyR {
        NotbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A (cleared on read)"]
    #[inline(always)]
    pub fn sdioirqa(&self) -> SdioirqaR {
        SdioirqaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status"]
    #[inline(always)]
    pub fn sdiowait(&self) -> SdiowaitR {
        SdiowaitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CE-ATA Completion Signal Received (cleared on read)"]
    #[inline(always)]
    pub fn csrcv(&self) -> CsrcvR {
        CsrcvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Response Index Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rinde(&self) -> RindeR {
        RindeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Response Direction Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rdire(&self) -> RdireR {
        RdireR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Response CRC Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rcrce(&self) -> RcrceR {
        RcrceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Response End Bit Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rende(&self) -> RendeR {
        RendeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Response Time-out Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rtoe(&self) -> RtoeR {
        RtoeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error (cleared on read)"]
    #[inline(always)]
    pub fn dcrce(&self) -> DcrceR {
        DcrceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Time-out Error (cleared on read)"]
    #[inline(always)]
    pub fn dtoe(&self) -> DtoeR {
        DtoeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Completion Signal Time-out Error (cleared on read)"]
    #[inline(always)]
    pub fn cstoe(&self) -> CstoeR {
        CstoeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA Block Overrun Error (cleared on read)"]
    #[inline(always)]
    pub fn blkovre(&self) -> BlkovreR {
        BlkovreR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - FIFO empty flag"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FifoemptyR {
        FifoemptyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transfer Done flag"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XfrdoneR {
        XfrdoneR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Boot Operation Acknowledge Received (cleared on read)"]
    #[inline(always)]
    pub fn ackrcv(&self) -> AckrcvR {
        AckrcvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Boot Operation Acknowledge Error (cleared on read)"]
    #[inline(always)]
    pub fn ackrcve(&self) -> AckrcveR {
        AckrcveR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Overrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Underrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
