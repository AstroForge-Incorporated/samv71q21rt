#[repr(C)]
#[doc = "Device DMA Channel Next Descriptor Address Register"]
#[doc(alias = "USBHS_DEVDMA")]
pub struct UsbhsDevdma {
    devdmanxtdsc: Devdmanxtdsc,
    devdmaaddress: Devdmaaddress,
    devdmacontrol: Devdmacontrol,
    devdmastatus: Devdmastatus,
}
impl UsbhsDevdma {
    #[doc = "0x00 - Device DMA Channel Next Descriptor Address Register"]
    #[inline(always)]
    pub const fn devdmanxtdsc(&self) -> &Devdmanxtdsc {
        &self.devdmanxtdsc
    }
    #[doc = "0x04 - Device DMA Channel Address Register"]
    #[inline(always)]
    pub const fn devdmaaddress(&self) -> &Devdmaaddress {
        &self.devdmaaddress
    }
    #[doc = "0x08 - Device DMA Channel Control Register"]
    #[inline(always)]
    pub const fn devdmacontrol(&self) -> &Devdmacontrol {
        &self.devdmacontrol
    }
    #[doc = "0x0c - Device DMA Channel Status Register"]
    #[inline(always)]
    pub const fn devdmastatus(&self) -> &Devdmastatus {
        &self.devdmastatus
    }
}
#[doc = "DEVDMANXTDSC (rw) register accessor: Device DMA Channel Next Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmanxtdsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmanxtdsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc`] module"]
#[doc(alias = "DEVDMANXTDSC")]
pub type Devdmanxtdsc = crate::Reg<devdmanxtdsc::DevdmanxtdscSpec>;
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub mod devdmanxtdsc;
#[doc = "DEVDMAADDRESS (rw) register accessor: Device DMA Channel Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress`] module"]
#[doc(alias = "DEVDMAADDRESS")]
pub type Devdmaaddress = crate::Reg<devdmaaddress::DevdmaaddressSpec>;
#[doc = "Device DMA Channel Address Register"]
pub mod devdmaaddress;
#[doc = "DEVDMACONTROL (rw) register accessor: Device DMA Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmacontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmacontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol`] module"]
#[doc(alias = "DEVDMACONTROL")]
pub type Devdmacontrol = crate::Reg<devdmacontrol::DevdmacontrolSpec>;
#[doc = "Device DMA Channel Control Register"]
pub mod devdmacontrol;
#[doc = "DEVDMASTATUS (rw) register accessor: Device DMA Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmastatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmastatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus`] module"]
#[doc(alias = "DEVDMASTATUS")]
pub type Devdmastatus = crate::Reg<devdmastatus::DevdmastatusSpec>;
#[doc = "Device DMA Channel Status Register"]
pub mod devdmastatus;
