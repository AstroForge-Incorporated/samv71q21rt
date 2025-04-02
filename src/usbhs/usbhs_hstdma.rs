#[repr(C)]
#[doc = "Host DMA Channel Next Descriptor Address Register"]
#[doc(alias = "USBHS_HSTDMA")]
pub struct UsbhsHstdma {
    hstdmanxtdsc: Hstdmanxtdsc,
    hstdmaaddress: Hstdmaaddress,
    hstdmacontrol: Hstdmacontrol,
    hstdmastatus: Hstdmastatus,
}
impl UsbhsHstdma {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register"]
    #[inline(always)]
    pub const fn hstdmanxtdsc(&self) -> &Hstdmanxtdsc {
        &self.hstdmanxtdsc
    }
    #[doc = "0x04 - Host DMA Channel Address Register"]
    #[inline(always)]
    pub const fn hstdmaaddress(&self) -> &Hstdmaaddress {
        &self.hstdmaaddress
    }
    #[doc = "0x08 - Host DMA Channel Control Register"]
    #[inline(always)]
    pub const fn hstdmacontrol(&self) -> &Hstdmacontrol {
        &self.hstdmacontrol
    }
    #[doc = "0x0c - Host DMA Channel Status Register"]
    #[inline(always)]
    pub const fn hstdmastatus(&self) -> &Hstdmastatus {
        &self.hstdmastatus
    }
}
#[doc = "HSTDMANXTDSC (rw) register accessor: Host DMA Channel Next Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmanxtdsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmanxtdsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc`] module"]
#[doc(alias = "HSTDMANXTDSC")]
pub type Hstdmanxtdsc = crate::Reg<hstdmanxtdsc::HstdmanxtdscSpec>;
#[doc = "Host DMA Channel Next Descriptor Address Register"]
pub mod hstdmanxtdsc;
#[doc = "HSTDMAADDRESS (rw) register accessor: Host DMA Channel Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress`] module"]
#[doc(alias = "HSTDMAADDRESS")]
pub type Hstdmaaddress = crate::Reg<hstdmaaddress::HstdmaaddressSpec>;
#[doc = "Host DMA Channel Address Register"]
pub mod hstdmaaddress;
#[doc = "HSTDMACONTROL (rw) register accessor: Host DMA Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmacontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmacontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol`] module"]
#[doc(alias = "HSTDMACONTROL")]
pub type Hstdmacontrol = crate::Reg<hstdmacontrol::HstdmacontrolSpec>;
#[doc = "Host DMA Channel Control Register"]
pub mod hstdmacontrol;
#[doc = "HSTDMASTATUS (rw) register accessor: Host DMA Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmastatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmastatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus`] module"]
#[doc(alias = "HSTDMASTATUS")]
pub type Hstdmastatus = crate::Reg<hstdmastatus::HstdmastatusSpec>;
#[doc = "Host DMA Channel Status Register"]
pub mod hstdmastatus;
