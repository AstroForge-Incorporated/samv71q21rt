#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    devctrl: Devctrl,
    devisr: Devisr,
    devicr: Devicr,
    devifr: Devifr,
    devimr: Devimr,
    devidr: Devidr,
    devier: Devier,
    devept: Devept,
    devfnum: Devfnum,
    _reserved9: [u8; 0xdc],
    deveptcfg: [Deveptcfg; 10],
    _reserved10: [u8; 0x08],
    _reserved_10_deveptisr: [u8; 0x28],
    _reserved11: [u8; 0x08],
    _reserved_11_devepticr: [u8; 0x28],
    _reserved12: [u8; 0x08],
    _reserved_12_deveptifr: [u8; 0x28],
    _reserved13: [u8; 0x08],
    _reserved_13_deveptimr: [u8; 0x28],
    _reserved14: [u8; 0x08],
    _reserved_14_deveptier: [u8; 0x28],
    _reserved15: [u8; 0x08],
    _reserved_15_deveptidr: [u8; 0x28],
    _reserved16: [u8; 0xc8],
    usbhs_devdma: [UsbhsDevdma; 7],
    _reserved17: [u8; 0x80],
    hstctrl: Hstctrl,
    hstisr: Hstisr,
    hsticr: Hsticr,
    hstifr: Hstifr,
    hstimr: Hstimr,
    hstidr: Hstidr,
    hstier: Hstier,
    hstpip: Hstpip,
    hstfnum: Hstfnum,
    hstaddr1: Hstaddr1,
    hstaddr2: Hstaddr2,
    hstaddr3: Hstaddr3,
    _reserved29: [u8; 0xd0],
    _reserved_29_hstpipcfg: [u8; 0x28],
    _reserved30: [u8; 0x08],
    _reserved_30_hstpipisr: [u8; 0x28],
    _reserved31: [u8; 0x08],
    _reserved_31_hstpipicr: [u8; 0x28],
    _reserved32: [u8; 0x08],
    _reserved_32_hstpipifr: [u8; 0x28],
    _reserved33: [u8; 0x08],
    _reserved_33_hstpipimr: [u8; 0x28],
    _reserved34: [u8; 0x08],
    _reserved_34_hstpipier: [u8; 0x28],
    _reserved35: [u8; 0x08],
    _reserved_35_hstpipidr: [u8; 0x28],
    _reserved36: [u8; 0x08],
    hstpipinrq: [Hstpipinrq; 10],
    _reserved37: [u8; 0x08],
    hstpiperr: [Hstpiperr; 10],
    _reserved38: [u8; 0x68],
    usbhs_hstdma: [UsbhsHstdma; 7],
    _reserved39: [u8; 0x80],
    ctrl: Ctrl,
    sr: Sr,
    scr: Scr,
    sfr: Sfr,
}
impl RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    #[inline(always)]
    pub const fn devctrl(&self) -> &Devctrl {
        &self.devctrl
    }
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    #[inline(always)]
    pub const fn devisr(&self) -> &Devisr {
        &self.devisr
    }
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devicr(&self) -> &Devicr {
        &self.devicr
    }
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    #[inline(always)]
    pub const fn devifr(&self) -> &Devifr {
        &self.devifr
    }
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    #[inline(always)]
    pub const fn devimr(&self) -> &Devimr {
        &self.devimr
    }
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    #[inline(always)]
    pub const fn devidr(&self) -> &Devidr {
        &self.devidr
    }
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    #[inline(always)]
    pub const fn devier(&self) -> &Devier {
        &self.devier
    }
    #[doc = "0x1c - Device Endpoint Register"]
    #[inline(always)]
    pub const fn devept(&self) -> &Devept {
        &self.devept
    }
    #[doc = "0x20 - Device Frame Number Register"]
    #[inline(always)]
    pub const fn devfnum(&self) -> &Devfnum {
        &self.devfnum
    }
    #[doc = "0x100..0x128 - Device Endpoint Configuration Register"]
    #[inline(always)]
    pub const fn deveptcfg(&self, n: usize) -> &Deveptcfg {
        &self.deveptcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x128 - Device Endpoint Configuration Register"]
    #[inline(always)]
    pub fn deveptcfg_iter(&self) -> impl Iterator<Item = &Deveptcfg> {
        self.deveptcfg.iter()
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub const fn deveptisr_intrpt_mode(&self, n: usize) -> &DeveptisrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_intrpt_mode_iter(&self) -> impl Iterator<Item = &DeveptisrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub const fn deveptisr_blk_mode(&self, n: usize) -> &DeveptisrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_blk_mode_iter(&self) -> impl Iterator<Item = &DeveptisrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub const fn deveptisr_iso_mode(&self, n: usize) -> &DeveptisrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_iso_mode_iter(&self) -> impl Iterator<Item = &DeveptisrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub const fn deveptisr_ctrl_mode(&self, n: usize) -> &DeveptisrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub fn deveptisr_ctrl_mode_iter(&self) -> impl Iterator<Item = &DeveptisrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devepticr_intrpt_mode(&self, n: usize) -> &DevepticrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_intrpt_mode_iter(&self) -> impl Iterator<Item = &DevepticrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devepticr_blk_mode(&self, n: usize) -> &DevepticrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_blk_mode_iter(&self) -> impl Iterator<Item = &DevepticrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devepticr_iso_mode(&self, n: usize) -> &DevepticrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_iso_mode_iter(&self) -> impl Iterator<Item = &DevepticrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devepticr_ctrl_mode(&self, n: usize) -> &DevepticrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub fn devepticr_ctrl_mode_iter(&self) -> impl Iterator<Item = &DevepticrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub const fn deveptifr_intrpt_mode(&self, n: usize) -> &DeveptifrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_intrpt_mode_iter(&self) -> impl Iterator<Item = &DeveptifrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub const fn deveptifr_blk_mode(&self, n: usize) -> &DeveptifrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_blk_mode_iter(&self) -> impl Iterator<Item = &DeveptifrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub const fn deveptifr_iso_mode(&self, n: usize) -> &DeveptifrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_iso_mode_iter(&self) -> impl Iterator<Item = &DeveptifrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub const fn deveptifr_ctrl_mode(&self, n: usize) -> &DeveptifrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub fn deveptifr_ctrl_mode_iter(&self) -> impl Iterator<Item = &DeveptifrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub const fn deveptimr_intrpt_mode(&self, n: usize) -> &DeveptimrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_intrpt_mode_iter(&self) -> impl Iterator<Item = &DeveptimrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub const fn deveptimr_blk_mode(&self, n: usize) -> &DeveptimrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_blk_mode_iter(&self) -> impl Iterator<Item = &DeveptimrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub const fn deveptimr_iso_mode(&self, n: usize) -> &DeveptimrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_iso_mode_iter(&self) -> impl Iterator<Item = &DeveptimrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub const fn deveptimr_ctrl_mode(&self, n: usize) -> &DeveptimrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub fn deveptimr_ctrl_mode_iter(&self) -> impl Iterator<Item = &DeveptimrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub const fn deveptier_intrpt_mode(&self, n: usize) -> &DeveptierIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(496)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_intrpt_mode_iter(&self) -> impl Iterator<Item = &DeveptierIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(496)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub const fn deveptier_blk_mode(&self, n: usize) -> &DeveptierBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(496)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_blk_mode_iter(&self) -> impl Iterator<Item = &DeveptierBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(496)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub const fn deveptier_iso_mode(&self, n: usize) -> &DeveptierIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(496)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_iso_mode_iter(&self) -> impl Iterator<Item = &DeveptierIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(496)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub const fn deveptier_ctrl_mode(&self, n: usize) -> &DeveptierCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(496)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub fn deveptier_ctrl_mode_iter(&self) -> impl Iterator<Item = &DeveptierCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(496)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub const fn deveptidr_intrpt_mode(&self, n: usize) -> &DeveptidrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(544)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_intrpt_mode_iter(&self) -> impl Iterator<Item = &DeveptidrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(544)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub const fn deveptidr_blk_mode(&self, n: usize) -> &DeveptidrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(544)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_blk_mode_iter(&self) -> impl Iterator<Item = &DeveptidrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(544)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub const fn deveptidr_iso_mode(&self, n: usize) -> &DeveptidrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(544)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_iso_mode_iter(&self) -> impl Iterator<Item = &DeveptidrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(544)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub const fn deveptidr_ctrl_mode(&self, n: usize) -> &DeveptidrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(544)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub fn deveptidr_ctrl_mode_iter(&self) -> impl Iterator<Item = &DeveptidrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(544)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x310..0x380 - Device DMA Channel Next Descriptor Address Register"]
    #[inline(always)]
    pub const fn usbhs_devdma(&self, n: usize) -> &UsbhsDevdma {
        &self.usbhs_devdma[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x310..0x380 - Device DMA Channel Next Descriptor Address Register"]
    #[inline(always)]
    pub fn usbhs_devdma_iter(&self) -> impl Iterator<Item = &UsbhsDevdma> {
        self.usbhs_devdma.iter()
    }
    #[doc = "0x400 - Host General Control Register"]
    #[inline(always)]
    pub const fn hstctrl(&self) -> &Hstctrl {
        &self.hstctrl
    }
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    #[inline(always)]
    pub const fn hstisr(&self) -> &Hstisr {
        &self.hstisr
    }
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    #[inline(always)]
    pub const fn hsticr(&self) -> &Hsticr {
        &self.hsticr
    }
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    #[inline(always)]
    pub const fn hstifr(&self) -> &Hstifr {
        &self.hstifr
    }
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hstimr(&self) -> &Hstimr {
        &self.hstimr
    }
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    #[inline(always)]
    pub const fn hstidr(&self) -> &Hstidr {
        &self.hstidr
    }
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    #[inline(always)]
    pub const fn hstier(&self) -> &Hstier {
        &self.hstier
    }
    #[doc = "0x41c - Host Pipe Register"]
    #[inline(always)]
    pub const fn hstpip(&self) -> &Hstpip {
        &self.hstpip
    }
    #[doc = "0x420 - Host Frame Number Register"]
    #[inline(always)]
    pub const fn hstfnum(&self) -> &Hstfnum {
        &self.hstfnum
    }
    #[doc = "0x424 - Host Address 1 Register"]
    #[inline(always)]
    pub const fn hstaddr1(&self) -> &Hstaddr1 {
        &self.hstaddr1
    }
    #[doc = "0x428 - Host Address 2 Register"]
    #[inline(always)]
    pub const fn hstaddr2(&self) -> &Hstaddr2 {
        &self.hstaddr2
    }
    #[doc = "0x42c - Host Address 3 Register"]
    #[inline(always)]
    pub const fn hstaddr3(&self) -> &Hstaddr3 {
        &self.hstaddr3
    }
    #[doc = "0x500..0x528 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub const fn hstpipcfg_ctrl_bulk_mode(&self, n: usize) -> &HstpipcfgCtrlBulkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x528 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn hstpipcfg_ctrl_bulk_mode_iter(&self) -> impl Iterator<Item = &HstpipcfgCtrlBulkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x500..0x528 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub const fn hstpipcfg(&self, n: usize) -> &Hstpipcfg {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x528 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub fn hstpipcfg_iter(&self) -> impl Iterator<Item = &Hstpipcfg> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub const fn hstpipisr_intrpt_mode(&self, n: usize) -> &HstpipisrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_intrpt_mode_iter(&self) -> impl Iterator<Item = &HstpipisrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub const fn hstpipisr_blk_mode(&self, n: usize) -> &HstpipisrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_blk_mode_iter(&self) -> impl Iterator<Item = &HstpipisrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub const fn hstpipisr_iso_mode(&self, n: usize) -> &HstpipisrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_iso_mode_iter(&self) -> impl Iterator<Item = &HstpipisrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub const fn hstpipisr_ctrl_mode(&self, n: usize) -> &HstpipisrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub fn hstpipisr_ctrl_mode_iter(&self) -> impl Iterator<Item = &HstpipisrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub const fn hstpipicr_intrpt_mode(&self, n: usize) -> &HstpipicrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1376)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_intrpt_mode_iter(&self) -> impl Iterator<Item = &HstpipicrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1376)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub const fn hstpipicr_blk_mode(&self, n: usize) -> &HstpipicrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1376)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_blk_mode_iter(&self) -> impl Iterator<Item = &HstpipicrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1376)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub const fn hstpipicr_iso_mode(&self, n: usize) -> &HstpipicrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1376)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_iso_mode_iter(&self) -> impl Iterator<Item = &HstpipicrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1376)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub const fn hstpipicr_ctrl_mode(&self, n: usize) -> &HstpipicrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1376)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub fn hstpipicr_ctrl_mode_iter(&self) -> impl Iterator<Item = &HstpipicrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1376)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub const fn hstpipifr_intrpt_mode(&self, n: usize) -> &HstpipifrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1424)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_intrpt_mode_iter(&self) -> impl Iterator<Item = &HstpipifrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1424)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub const fn hstpipifr_blk_mode(&self, n: usize) -> &HstpipifrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1424)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_blk_mode_iter(&self) -> impl Iterator<Item = &HstpipifrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1424)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub const fn hstpipifr_iso_mode(&self, n: usize) -> &HstpipifrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1424)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_iso_mode_iter(&self) -> impl Iterator<Item = &HstpipifrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1424)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub const fn hstpipifr_ctrl_mode(&self, n: usize) -> &HstpipifrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1424)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub fn hstpipifr_ctrl_mode_iter(&self) -> impl Iterator<Item = &HstpipifrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1424)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub const fn hstpipimr_intrpt_mode(&self, n: usize) -> &HstpipimrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1472)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_intrpt_mode_iter(&self) -> impl Iterator<Item = &HstpipimrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1472)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub const fn hstpipimr_blk_mode(&self, n: usize) -> &HstpipimrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1472)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_blk_mode_iter(&self) -> impl Iterator<Item = &HstpipimrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1472)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub const fn hstpipimr_iso_mode(&self, n: usize) -> &HstpipimrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1472)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_iso_mode_iter(&self) -> impl Iterator<Item = &HstpipimrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1472)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub const fn hstpipimr_ctrl_mode(&self, n: usize) -> &HstpipimrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1472)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub fn hstpipimr_ctrl_mode_iter(&self) -> impl Iterator<Item = &HstpipimrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1472)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub const fn hstpipier_intrpt_mode(&self, n: usize) -> &HstpipierIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1520)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_intrpt_mode_iter(&self) -> impl Iterator<Item = &HstpipierIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1520)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub const fn hstpipier_blk_mode(&self, n: usize) -> &HstpipierBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1520)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_blk_mode_iter(&self) -> impl Iterator<Item = &HstpipierBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1520)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub const fn hstpipier_iso_mode(&self, n: usize) -> &HstpipierIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1520)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_iso_mode_iter(&self) -> impl Iterator<Item = &HstpipierIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1520)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub const fn hstpipier_ctrl_mode(&self, n: usize) -> &HstpipierCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1520)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub fn hstpipier_ctrl_mode_iter(&self) -> impl Iterator<Item = &HstpipierCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1520)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub const fn hstpipidr_intrpt_mode(&self, n: usize) -> &HstpipidrIntrptMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1568)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_intrpt_mode_iter(&self) -> impl Iterator<Item = &HstpipidrIntrptMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1568)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub const fn hstpipidr_blk_mode(&self, n: usize) -> &HstpipidrBlkMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1568)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_blk_mode_iter(&self) -> impl Iterator<Item = &HstpipidrBlkMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1568)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub const fn hstpipidr_iso_mode(&self, n: usize) -> &HstpipidrIsoMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1568)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_iso_mode_iter(&self) -> impl Iterator<Item = &HstpipidrIsoMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1568)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub const fn hstpipidr_ctrl_mode(&self, n: usize) -> &HstpipidrCtrlMode {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1568)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub fn hstpipidr_ctrl_mode_iter(&self) -> impl Iterator<Item = &HstpipidrCtrlMode> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1568)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x650..0x678 - Host Pipe IN Request Register"]
    #[inline(always)]
    pub const fn hstpipinrq(&self, n: usize) -> &Hstpipinrq {
        &self.hstpipinrq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x650..0x678 - Host Pipe IN Request Register"]
    #[inline(always)]
    pub fn hstpipinrq_iter(&self) -> impl Iterator<Item = &Hstpipinrq> {
        self.hstpipinrq.iter()
    }
    #[doc = "0x680..0x6a8 - Host Pipe Error Register"]
    #[inline(always)]
    pub const fn hstpiperr(&self, n: usize) -> &Hstpiperr {
        &self.hstpiperr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x680..0x6a8 - Host Pipe Error Register"]
    #[inline(always)]
    pub fn hstpiperr_iter(&self) -> impl Iterator<Item = &Hstpiperr> {
        self.hstpiperr.iter()
    }
    #[doc = "0x710..0x780 - Host DMA Channel Next Descriptor Address Register"]
    #[inline(always)]
    pub const fn usbhs_hstdma(&self, n: usize) -> &UsbhsHstdma {
        &self.usbhs_hstdma[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x710..0x780 - Host DMA Channel Next Descriptor Address Register"]
    #[inline(always)]
    pub fn usbhs_hstdma_iter(&self) -> impl Iterator<Item = &UsbhsHstdma> {
        self.usbhs_hstdma.iter()
    }
    #[doc = "0x800 - General Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x804 - General Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x808 - General Status Clear Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x80c - General Status Set Register"]
    #[inline(always)]
    pub const fn sfr(&self) -> &Sfr {
        &self.sfr
    }
}
#[doc = "DEVCTRL (rw) register accessor: Device General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devctrl`] module"]
#[doc(alias = "DEVCTRL")]
pub type Devctrl = crate::Reg<devctrl::DevctrlSpec>;
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "DEVISR (r) register accessor: Device Global Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devisr`] module"]
#[doc(alias = "DEVISR")]
pub type Devisr = crate::Reg<devisr::DevisrSpec>;
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "DEVICR (w) register accessor: Device Global Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devicr`] module"]
#[doc(alias = "DEVICR")]
pub type Devicr = crate::Reg<devicr::DevicrSpec>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "DEVIFR (w) register accessor: Device Global Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devifr`] module"]
#[doc(alias = "DEVIFR")]
pub type Devifr = crate::Reg<devifr::DevifrSpec>;
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "DEVIMR (r) register accessor: Device Global Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devimr`] module"]
#[doc(alias = "DEVIMR")]
pub type Devimr = crate::Reg<devimr::DevimrSpec>;
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "DEVIDR (w) register accessor: Device Global Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devidr`] module"]
#[doc(alias = "DEVIDR")]
pub type Devidr = crate::Reg<devidr::DevidrSpec>;
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "DEVIER (w) register accessor: Device Global Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devier`] module"]
#[doc(alias = "DEVIER")]
pub type Devier = crate::Reg<devier::DevierSpec>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "DEVEPT (rw) register accessor: Device Endpoint Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devept::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devept::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devept`] module"]
#[doc(alias = "DEVEPT")]
pub type Devept = crate::Reg<devept::DeveptSpec>;
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "DEVFNUM (r) register accessor: Device Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devfnum`] module"]
#[doc(alias = "DEVFNUM")]
pub type Devfnum = crate::Reg<devfnum::DevfnumSpec>;
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "DEVEPTCFG (rw) register accessor: Device Endpoint Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg`] module"]
#[doc(alias = "DEVEPTCFG")]
pub type Deveptcfg = crate::Reg<deveptcfg::DeveptcfgSpec>;
#[doc = "Device Endpoint Configuration Register"]
pub mod deveptcfg;
#[doc = "DEVEPTISR_CTRL_MODE (r) register accessor: Device Endpoint Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptisr_ctrl_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr_ctrl_mode`] module"]
#[doc(alias = "DEVEPTISR_CTRL_MODE")]
pub type DeveptisrCtrlMode = crate::Reg<deveptisr_ctrl_mode::DeveptisrCtrlModeSpec>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_ctrl_mode;
#[doc = "DEVEPTISR_ISO_MODE (r) register accessor: Device Endpoint Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptisr_iso_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr_iso_mode`] module"]
#[doc(alias = "DEVEPTISR_ISO_MODE")]
pub type DeveptisrIsoMode = crate::Reg<deveptisr_iso_mode::DeveptisrIsoModeSpec>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_iso_mode;
#[doc = "DEVEPTISR_BLK_MODE (r) register accessor: Device Endpoint Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptisr_blk_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr_blk_mode`] module"]
#[doc(alias = "DEVEPTISR_BLK_MODE")]
pub type DeveptisrBlkMode = crate::Reg<deveptisr_blk_mode::DeveptisrBlkModeSpec>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_blk_mode;
#[doc = "DEVEPTISR_INTRPT_MODE (r) register accessor: Device Endpoint Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptisr_intrpt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr_intrpt_mode`] module"]
#[doc(alias = "DEVEPTISR_INTRPT_MODE")]
pub type DeveptisrIntrptMode = crate::Reg<deveptisr_intrpt_mode::DeveptisrIntrptModeSpec>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_intrpt_mode;
#[doc = "DEVEPTICR_CTRL_MODE (w) register accessor: Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devepticr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr_ctrl_mode`] module"]
#[doc(alias = "DEVEPTICR_CTRL_MODE")]
pub type DevepticrCtrlMode = crate::Reg<devepticr_ctrl_mode::DevepticrCtrlModeSpec>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_ctrl_mode;
#[doc = "DEVEPTICR_ISO_MODE (w) register accessor: Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devepticr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr_iso_mode`] module"]
#[doc(alias = "DEVEPTICR_ISO_MODE")]
pub type DevepticrIsoMode = crate::Reg<devepticr_iso_mode::DevepticrIsoModeSpec>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_iso_mode;
#[doc = "DEVEPTICR_BLK_MODE (w) register accessor: Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devepticr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr_blk_mode`] module"]
#[doc(alias = "DEVEPTICR_BLK_MODE")]
pub type DevepticrBlkMode = crate::Reg<devepticr_blk_mode::DevepticrBlkModeSpec>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_blk_mode;
#[doc = "DEVEPTICR_INTRPT_MODE (w) register accessor: Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devepticr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr_intrpt_mode`] module"]
#[doc(alias = "DEVEPTICR_INTRPT_MODE")]
pub type DevepticrIntrptMode = crate::Reg<devepticr_intrpt_mode::DevepticrIntrptModeSpec>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_intrpt_mode;
#[doc = "DEVEPTIFR_CTRL_MODE (w) register accessor: Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptifr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr_ctrl_mode`] module"]
#[doc(alias = "DEVEPTIFR_CTRL_MODE")]
pub type DeveptifrCtrlMode = crate::Reg<deveptifr_ctrl_mode::DeveptifrCtrlModeSpec>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_ctrl_mode;
#[doc = "DEVEPTIFR_ISO_MODE (w) register accessor: Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptifr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr_iso_mode`] module"]
#[doc(alias = "DEVEPTIFR_ISO_MODE")]
pub type DeveptifrIsoMode = crate::Reg<deveptifr_iso_mode::DeveptifrIsoModeSpec>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_iso_mode;
#[doc = "DEVEPTIFR_BLK_MODE (w) register accessor: Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptifr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr_blk_mode`] module"]
#[doc(alias = "DEVEPTIFR_BLK_MODE")]
pub type DeveptifrBlkMode = crate::Reg<deveptifr_blk_mode::DeveptifrBlkModeSpec>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_blk_mode;
#[doc = "DEVEPTIFR_INTRPT_MODE (w) register accessor: Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptifr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr_intrpt_mode`] module"]
#[doc(alias = "DEVEPTIFR_INTRPT_MODE")]
pub type DeveptifrIntrptMode = crate::Reg<deveptifr_intrpt_mode::DeveptifrIntrptModeSpec>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_intrpt_mode;
#[doc = "DEVEPTIMR_CTRL_MODE (r) register accessor: Device Endpoint Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptimr_ctrl_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr_ctrl_mode`] module"]
#[doc(alias = "DEVEPTIMR_CTRL_MODE")]
pub type DeveptimrCtrlMode = crate::Reg<deveptimr_ctrl_mode::DeveptimrCtrlModeSpec>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_ctrl_mode;
#[doc = "DEVEPTIMR_ISO_MODE (r) register accessor: Device Endpoint Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptimr_iso_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr_iso_mode`] module"]
#[doc(alias = "DEVEPTIMR_ISO_MODE")]
pub type DeveptimrIsoMode = crate::Reg<deveptimr_iso_mode::DeveptimrIsoModeSpec>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_iso_mode;
#[doc = "DEVEPTIMR_BLK_MODE (r) register accessor: Device Endpoint Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptimr_blk_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr_blk_mode`] module"]
#[doc(alias = "DEVEPTIMR_BLK_MODE")]
pub type DeveptimrBlkMode = crate::Reg<deveptimr_blk_mode::DeveptimrBlkModeSpec>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_blk_mode;
#[doc = "DEVEPTIMR_INTRPT_MODE (r) register accessor: Device Endpoint Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptimr_intrpt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr_intrpt_mode`] module"]
#[doc(alias = "DEVEPTIMR_INTRPT_MODE")]
pub type DeveptimrIntrptMode = crate::Reg<deveptimr_intrpt_mode::DeveptimrIntrptModeSpec>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_intrpt_mode;
#[doc = "DEVEPTIER_CTRL_MODE (w) register accessor: Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptier_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier_ctrl_mode`] module"]
#[doc(alias = "DEVEPTIER_CTRL_MODE")]
pub type DeveptierCtrlMode = crate::Reg<deveptier_ctrl_mode::DeveptierCtrlModeSpec>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_ctrl_mode;
#[doc = "DEVEPTIER_ISO_MODE (w) register accessor: Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptier_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier_iso_mode`] module"]
#[doc(alias = "DEVEPTIER_ISO_MODE")]
pub type DeveptierIsoMode = crate::Reg<deveptier_iso_mode::DeveptierIsoModeSpec>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_iso_mode;
#[doc = "DEVEPTIER_BLK_MODE (w) register accessor: Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptier_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier_blk_mode`] module"]
#[doc(alias = "DEVEPTIER_BLK_MODE")]
pub type DeveptierBlkMode = crate::Reg<deveptier_blk_mode::DeveptierBlkModeSpec>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_blk_mode;
#[doc = "DEVEPTIER_INTRPT_MODE (w) register accessor: Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptier_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier_intrpt_mode`] module"]
#[doc(alias = "DEVEPTIER_INTRPT_MODE")]
pub type DeveptierIntrptMode = crate::Reg<deveptier_intrpt_mode::DeveptierIntrptModeSpec>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_intrpt_mode;
#[doc = "DEVEPTIDR_CTRL_MODE (w) register accessor: Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptidr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr_ctrl_mode`] module"]
#[doc(alias = "DEVEPTIDR_CTRL_MODE")]
pub type DeveptidrCtrlMode = crate::Reg<deveptidr_ctrl_mode::DeveptidrCtrlModeSpec>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_ctrl_mode;
#[doc = "DEVEPTIDR_ISO_MODE (w) register accessor: Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptidr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr_iso_mode`] module"]
#[doc(alias = "DEVEPTIDR_ISO_MODE")]
pub type DeveptidrIsoMode = crate::Reg<deveptidr_iso_mode::DeveptidrIsoModeSpec>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_iso_mode;
#[doc = "DEVEPTIDR_BLK_MODE (w) register accessor: Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptidr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr_blk_mode`] module"]
#[doc(alias = "DEVEPTIDR_BLK_MODE")]
pub type DeveptidrBlkMode = crate::Reg<deveptidr_blk_mode::DeveptidrBlkModeSpec>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_blk_mode;
#[doc = "DEVEPTIDR_INTRPT_MODE (w) register accessor: Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptidr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr_intrpt_mode`] module"]
#[doc(alias = "DEVEPTIDR_INTRPT_MODE")]
pub type DeveptidrIntrptMode = crate::Reg<deveptidr_intrpt_mode::DeveptidrIntrptModeSpec>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_intrpt_mode;
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub use self::usbhs_devdma::UsbhsDevdma;
#[doc = r"Cluster"]
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub mod usbhs_devdma;
#[doc = "HSTCTRL (rw) register accessor: Host General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstctrl`] module"]
#[doc(alias = "HSTCTRL")]
pub type Hstctrl = crate::Reg<hstctrl::HstctrlSpec>;
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "HSTISR (r) register accessor: Host Global Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstisr`] module"]
#[doc(alias = "HSTISR")]
pub type Hstisr = crate::Reg<hstisr::HstisrSpec>;
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "HSTICR (w) register accessor: Host Global Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsticr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsticr`] module"]
#[doc(alias = "HSTICR")]
pub type Hsticr = crate::Reg<hsticr::HsticrSpec>;
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "HSTIFR (w) register accessor: Host Global Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstifr`] module"]
#[doc(alias = "HSTIFR")]
pub type Hstifr = crate::Reg<hstifr::HstifrSpec>;
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "HSTIMR (r) register accessor: Host Global Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstimr`] module"]
#[doc(alias = "HSTIMR")]
pub type Hstimr = crate::Reg<hstimr::HstimrSpec>;
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "HSTIDR (w) register accessor: Host Global Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstidr`] module"]
#[doc(alias = "HSTIDR")]
pub type Hstidr = crate::Reg<hstidr::HstidrSpec>;
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "HSTIER (w) register accessor: Host Global Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstier`] module"]
#[doc(alias = "HSTIER")]
pub type Hstier = crate::Reg<hstier::HstierSpec>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "HSTPIP (rw) register accessor: Host Pipe Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpip`] module"]
#[doc(alias = "HSTPIP")]
pub type Hstpip = crate::Reg<hstpip::HstpipSpec>;
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "HSTFNUM (rw) register accessor: Host Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstfnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstfnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstfnum`] module"]
#[doc(alias = "HSTFNUM")]
pub type Hstfnum = crate::Reg<hstfnum::HstfnumSpec>;
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "HSTADDR1 (rw) register accessor: Host Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstaddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstaddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstaddr1`] module"]
#[doc(alias = "HSTADDR1")]
pub type Hstaddr1 = crate::Reg<hstaddr1::Hstaddr1Spec>;
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "HSTADDR2 (rw) register accessor: Host Address 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstaddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstaddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstaddr2`] module"]
#[doc(alias = "HSTADDR2")]
pub type Hstaddr2 = crate::Reg<hstaddr2::Hstaddr2Spec>;
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "HSTADDR3 (rw) register accessor: Host Address 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstaddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstaddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstaddr3`] module"]
#[doc(alias = "HSTADDR3")]
pub type Hstaddr3 = crate::Reg<hstaddr3::Hstaddr3Spec>;
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "HSTPIPCFG (rw) register accessor: Host Pipe Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg`] module"]
#[doc(alias = "HSTPIPCFG")]
pub type Hstpipcfg = crate::Reg<hstpipcfg::HstpipcfgSpec>;
#[doc = "Host Pipe Configuration Register"]
pub mod hstpipcfg;
#[doc = "HSTPIPCFG_CTRL_BULK_MODE (rw) register accessor: Host Pipe Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipcfg_ctrl_bulk_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipcfg_ctrl_bulk_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg_ctrl_bulk_mode`] module"]
#[doc(alias = "HSTPIPCFG_CTRL_BULK_MODE")]
pub type HstpipcfgCtrlBulkMode = crate::Reg<hstpipcfg_ctrl_bulk_mode::HstpipcfgCtrlBulkModeSpec>;
#[doc = "Host Pipe Configuration Register"]
pub mod hstpipcfg_ctrl_bulk_mode;
#[doc = "HSTPIPISR_CTRL_MODE (r) register accessor: Host Pipe Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipisr_ctrl_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr_ctrl_mode`] module"]
#[doc(alias = "HSTPIPISR_CTRL_MODE")]
pub type HstpipisrCtrlMode = crate::Reg<hstpipisr_ctrl_mode::HstpipisrCtrlModeSpec>;
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_ctrl_mode;
#[doc = "HSTPIPISR_ISO_MODE (r) register accessor: Host Pipe Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipisr_iso_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr_iso_mode`] module"]
#[doc(alias = "HSTPIPISR_ISO_MODE")]
pub type HstpipisrIsoMode = crate::Reg<hstpipisr_iso_mode::HstpipisrIsoModeSpec>;
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_iso_mode;
#[doc = "HSTPIPISR_BLK_MODE (r) register accessor: Host Pipe Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipisr_blk_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr_blk_mode`] module"]
#[doc(alias = "HSTPIPISR_BLK_MODE")]
pub type HstpipisrBlkMode = crate::Reg<hstpipisr_blk_mode::HstpipisrBlkModeSpec>;
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_blk_mode;
#[doc = "HSTPIPISR_INTRPT_MODE (r) register accessor: Host Pipe Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipisr_intrpt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr_intrpt_mode`] module"]
#[doc(alias = "HSTPIPISR_INTRPT_MODE")]
pub type HstpipisrIntrptMode = crate::Reg<hstpipisr_intrpt_mode::HstpipisrIntrptModeSpec>;
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_intrpt_mode;
#[doc = "HSTPIPICR_CTRL_MODE (w) register accessor: Host Pipe Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipicr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr_ctrl_mode`] module"]
#[doc(alias = "HSTPIPICR_CTRL_MODE")]
pub type HstpipicrCtrlMode = crate::Reg<hstpipicr_ctrl_mode::HstpipicrCtrlModeSpec>;
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_ctrl_mode;
#[doc = "HSTPIPICR_ISO_MODE (w) register accessor: Host Pipe Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipicr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr_iso_mode`] module"]
#[doc(alias = "HSTPIPICR_ISO_MODE")]
pub type HstpipicrIsoMode = crate::Reg<hstpipicr_iso_mode::HstpipicrIsoModeSpec>;
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_iso_mode;
#[doc = "HSTPIPICR_BLK_MODE (w) register accessor: Host Pipe Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipicr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr_blk_mode`] module"]
#[doc(alias = "HSTPIPICR_BLK_MODE")]
pub type HstpipicrBlkMode = crate::Reg<hstpipicr_blk_mode::HstpipicrBlkModeSpec>;
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_blk_mode;
#[doc = "HSTPIPICR_INTRPT_MODE (w) register accessor: Host Pipe Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipicr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr_intrpt_mode`] module"]
#[doc(alias = "HSTPIPICR_INTRPT_MODE")]
pub type HstpipicrIntrptMode = crate::Reg<hstpipicr_intrpt_mode::HstpipicrIntrptModeSpec>;
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_intrpt_mode;
#[doc = "HSTPIPIFR_CTRL_MODE (w) register accessor: Host Pipe Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipifr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr_ctrl_mode`] module"]
#[doc(alias = "HSTPIPIFR_CTRL_MODE")]
pub type HstpipifrCtrlMode = crate::Reg<hstpipifr_ctrl_mode::HstpipifrCtrlModeSpec>;
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_ctrl_mode;
#[doc = "HSTPIPIFR_ISO_MODE (w) register accessor: Host Pipe Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipifr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr_iso_mode`] module"]
#[doc(alias = "HSTPIPIFR_ISO_MODE")]
pub type HstpipifrIsoMode = crate::Reg<hstpipifr_iso_mode::HstpipifrIsoModeSpec>;
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_iso_mode;
#[doc = "HSTPIPIFR_BLK_MODE (w) register accessor: Host Pipe Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipifr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr_blk_mode`] module"]
#[doc(alias = "HSTPIPIFR_BLK_MODE")]
pub type HstpipifrBlkMode = crate::Reg<hstpipifr_blk_mode::HstpipifrBlkModeSpec>;
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_blk_mode;
#[doc = "HSTPIPIFR_INTRPT_MODE (w) register accessor: Host Pipe Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipifr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr_intrpt_mode`] module"]
#[doc(alias = "HSTPIPIFR_INTRPT_MODE")]
pub type HstpipifrIntrptMode = crate::Reg<hstpipifr_intrpt_mode::HstpipifrIntrptModeSpec>;
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_intrpt_mode;
#[doc = "HSTPIPIMR_CTRL_MODE (r) register accessor: Host Pipe Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipimr_ctrl_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr_ctrl_mode`] module"]
#[doc(alias = "HSTPIPIMR_CTRL_MODE")]
pub type HstpipimrCtrlMode = crate::Reg<hstpipimr_ctrl_mode::HstpipimrCtrlModeSpec>;
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_ctrl_mode;
#[doc = "HSTPIPIMR_ISO_MODE (r) register accessor: Host Pipe Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipimr_iso_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr_iso_mode`] module"]
#[doc(alias = "HSTPIPIMR_ISO_MODE")]
pub type HstpipimrIsoMode = crate::Reg<hstpipimr_iso_mode::HstpipimrIsoModeSpec>;
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_iso_mode;
#[doc = "HSTPIPIMR_BLK_MODE (r) register accessor: Host Pipe Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipimr_blk_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr_blk_mode`] module"]
#[doc(alias = "HSTPIPIMR_BLK_MODE")]
pub type HstpipimrBlkMode = crate::Reg<hstpipimr_blk_mode::HstpipimrBlkModeSpec>;
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_blk_mode;
#[doc = "HSTPIPIMR_INTRPT_MODE (r) register accessor: Host Pipe Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipimr_intrpt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr_intrpt_mode`] module"]
#[doc(alias = "HSTPIPIMR_INTRPT_MODE")]
pub type HstpipimrIntrptMode = crate::Reg<hstpipimr_intrpt_mode::HstpipimrIntrptModeSpec>;
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_intrpt_mode;
#[doc = "HSTPIPIER_CTRL_MODE (w) register accessor: Host Pipe Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipier_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier_ctrl_mode`] module"]
#[doc(alias = "HSTPIPIER_CTRL_MODE")]
pub type HstpipierCtrlMode = crate::Reg<hstpipier_ctrl_mode::HstpipierCtrlModeSpec>;
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_ctrl_mode;
#[doc = "HSTPIPIER_ISO_MODE (w) register accessor: Host Pipe Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipier_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier_iso_mode`] module"]
#[doc(alias = "HSTPIPIER_ISO_MODE")]
pub type HstpipierIsoMode = crate::Reg<hstpipier_iso_mode::HstpipierIsoModeSpec>;
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_iso_mode;
#[doc = "HSTPIPIER_BLK_MODE (w) register accessor: Host Pipe Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipier_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier_blk_mode`] module"]
#[doc(alias = "HSTPIPIER_BLK_MODE")]
pub type HstpipierBlkMode = crate::Reg<hstpipier_blk_mode::HstpipierBlkModeSpec>;
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_blk_mode;
#[doc = "HSTPIPIER_INTRPT_MODE (w) register accessor: Host Pipe Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipier_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier_intrpt_mode`] module"]
#[doc(alias = "HSTPIPIER_INTRPT_MODE")]
pub type HstpipierIntrptMode = crate::Reg<hstpipier_intrpt_mode::HstpipierIntrptModeSpec>;
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_intrpt_mode;
#[doc = "HSTPIPIDR_CTRL_MODE (w) register accessor: Host Pipe Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipidr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr_ctrl_mode`] module"]
#[doc(alias = "HSTPIPIDR_CTRL_MODE")]
pub type HstpipidrCtrlMode = crate::Reg<hstpipidr_ctrl_mode::HstpipidrCtrlModeSpec>;
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_ctrl_mode;
#[doc = "HSTPIPIDR_ISO_MODE (w) register accessor: Host Pipe Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipidr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr_iso_mode`] module"]
#[doc(alias = "HSTPIPIDR_ISO_MODE")]
pub type HstpipidrIsoMode = crate::Reg<hstpipidr_iso_mode::HstpipidrIsoModeSpec>;
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_iso_mode;
#[doc = "HSTPIPIDR_BLK_MODE (w) register accessor: Host Pipe Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipidr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr_blk_mode`] module"]
#[doc(alias = "HSTPIPIDR_BLK_MODE")]
pub type HstpipidrBlkMode = crate::Reg<hstpipidr_blk_mode::HstpipidrBlkModeSpec>;
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_blk_mode;
#[doc = "HSTPIPIDR_INTRPT_MODE (w) register accessor: Host Pipe Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipidr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr_intrpt_mode`] module"]
#[doc(alias = "HSTPIPIDR_INTRPT_MODE")]
pub type HstpipidrIntrptMode = crate::Reg<hstpipidr_intrpt_mode::HstpipidrIntrptModeSpec>;
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_intrpt_mode;
#[doc = "HSTPIPINRQ (rw) register accessor: Host Pipe IN Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipinrq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipinrq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq`] module"]
#[doc(alias = "HSTPIPINRQ")]
pub type Hstpipinrq = crate::Reg<hstpipinrq::HstpipinrqSpec>;
#[doc = "Host Pipe IN Request Register"]
pub mod hstpipinrq;
#[doc = "HSTPIPERR (rw) register accessor: Host Pipe Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpiperr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpiperr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr`] module"]
#[doc(alias = "HSTPIPERR")]
pub type Hstpiperr = crate::Reg<hstpiperr::HstpiperrSpec>;
#[doc = "Host Pipe Error Register"]
pub mod hstpiperr;
#[doc = "Host DMA Channel Next Descriptor Address Register"]
pub use self::usbhs_hstdma::UsbhsHstdma;
#[doc = r"Cluster"]
#[doc = "Host DMA Channel Next Descriptor Address Register"]
pub mod usbhs_hstdma;
#[doc = "CTRL (rw) register accessor: General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: General Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "General Status Register"]
pub mod sr;
#[doc = "SCR (w) register accessor: General Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "SFR (w) register accessor: General Status Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr`] module"]
#[doc(alias = "SFR")]
pub type Sfr = crate::Reg<sfr::SfrSpec>;
#[doc = "General Status Set Register"]
pub mod sfr;
