#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gtype: Gtype,
    gcfg: Gcfg,
    gwac: Gwac,
    gie: Gie,
    gid: Gid,
    gim: Gim,
    gis: Gis,
    ge: Ge,
    gd: Gd,
    gs: Gs,
    grs: Grs,
    gws: Gws,
    grws: Grws,
    grwr: Grwr,
    gswr: Gswr,
    gsws: Gsws,
    gswf: Gswf,
    _reserved17: [u8; 0x0c],
    xdmac_chid: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Global Type Register"]
    #[inline(always)]
    pub const fn gtype(&self) -> &Gtype {
        &self.gtype
    }
    #[doc = "0x04 - Global Configuration Register"]
    #[inline(always)]
    pub const fn gcfg(&self) -> &Gcfg {
        &self.gcfg
    }
    #[doc = "0x08 - Global Weighted Arbiter Configuration Register"]
    #[inline(always)]
    pub const fn gwac(&self) -> &Gwac {
        &self.gwac
    }
    #[doc = "0x0c - Global Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gie(&self) -> &Gie {
        &self.gie
    }
    #[doc = "0x10 - Global Interrupt Disable Register"]
    #[inline(always)]
    pub const fn gid(&self) -> &Gid {
        &self.gid
    }
    #[doc = "0x14 - Global Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gim(&self) -> &Gim {
        &self.gim
    }
    #[doc = "0x18 - Global Interrupt Status Register"]
    #[inline(always)]
    pub const fn gis(&self) -> &Gis {
        &self.gis
    }
    #[doc = "0x1c - Global Channel Enable Register"]
    #[inline(always)]
    pub const fn ge(&self) -> &Ge {
        &self.ge
    }
    #[doc = "0x20 - Global Channel Disable Register"]
    #[inline(always)]
    pub const fn gd(&self) -> &Gd {
        &self.gd
    }
    #[doc = "0x24 - Global Channel Status Register"]
    #[inline(always)]
    pub const fn gs(&self) -> &Gs {
        &self.gs
    }
    #[doc = "0x28 - Global Channel Read Suspend Register"]
    #[inline(always)]
    pub const fn grs(&self) -> &Grs {
        &self.grs
    }
    #[doc = "0x2c - Global Channel Write Suspend Register"]
    #[inline(always)]
    pub const fn gws(&self) -> &Gws {
        &self.gws
    }
    #[doc = "0x30 - Global Channel Read Write Suspend Register"]
    #[inline(always)]
    pub const fn grws(&self) -> &Grws {
        &self.grws
    }
    #[doc = "0x34 - Global Channel Read Write Resume Register"]
    #[inline(always)]
    pub const fn grwr(&self) -> &Grwr {
        &self.grwr
    }
    #[doc = "0x38 - Global Channel Software Request Register"]
    #[inline(always)]
    pub const fn gswr(&self) -> &Gswr {
        &self.gswr
    }
    #[doc = "0x3c - Global Channel Software Request Status Register"]
    #[inline(always)]
    pub const fn gsws(&self) -> &Gsws {
        &self.gsws
    }
    #[doc = "0x40 - Global Channel Software Flush Request Register"]
    #[inline(always)]
    pub const fn gswf(&self) -> &Gswf {
        &self.gswf
    }
    #[doc = "0x50..0x590 - Channel Interrupt Enable Register"]
    #[inline(always)]
    pub const fn xdmac_chid(&self, n: usize) -> &XdmacChid {
        #[allow(clippy::no_effect)]
        [(); 24][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x590 - Channel Interrupt Enable Register"]
    #[inline(always)]
    pub fn xdmac_chid_iter(&self) -> impl Iterator<Item = &XdmacChid> {
        (0..24).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(64 * n)
                .cast()
        })
    }
}
#[doc = "GTYPE (r) register accessor: Global Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtype`] module"]
#[doc(alias = "GTYPE")]
pub type Gtype = crate::Reg<gtype::GtypeSpec>;
#[doc = "Global Type Register"]
pub mod gtype;
#[doc = "GCFG (rw) register accessor: Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcfg`] module"]
#[doc(alias = "GCFG")]
pub type Gcfg = crate::Reg<gcfg::GcfgSpec>;
#[doc = "Global Configuration Register"]
pub mod gcfg;
#[doc = "GWAC (rw) register accessor: Global Weighted Arbiter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gwac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gwac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gwac`] module"]
#[doc(alias = "GWAC")]
pub type Gwac = crate::Reg<gwac::GwacSpec>;
#[doc = "Global Weighted Arbiter Configuration Register"]
pub mod gwac;
#[doc = "GIE (w) register accessor: Global Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gie::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gie`] module"]
#[doc(alias = "GIE")]
pub type Gie = crate::Reg<gie::GieSpec>;
#[doc = "Global Interrupt Enable Register"]
pub mod gie;
#[doc = "GID (w) register accessor: Global Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gid::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gid`] module"]
#[doc(alias = "GID")]
pub type Gid = crate::Reg<gid::GidSpec>;
#[doc = "Global Interrupt Disable Register"]
pub mod gid;
#[doc = "GIM (r) register accessor: Global Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gim`] module"]
#[doc(alias = "GIM")]
pub type Gim = crate::Reg<gim::GimSpec>;
#[doc = "Global Interrupt Mask Register"]
pub mod gim;
#[doc = "GIS (r) register accessor: Global Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gis`] module"]
#[doc(alias = "GIS")]
pub type Gis = crate::Reg<gis::GisSpec>;
#[doc = "Global Interrupt Status Register"]
pub mod gis;
#[doc = "GE (w) register accessor: Global Channel Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ge::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ge`] module"]
#[doc(alias = "GE")]
pub type Ge = crate::Reg<ge::GeSpec>;
#[doc = "Global Channel Enable Register"]
pub mod ge;
#[doc = "GD (w) register accessor: Global Channel Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gd`] module"]
#[doc(alias = "GD")]
pub type Gd = crate::Reg<gd::GdSpec>;
#[doc = "Global Channel Disable Register"]
pub mod gd;
#[doc = "GS (r) register accessor: Global Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gs`] module"]
#[doc(alias = "GS")]
pub type Gs = crate::Reg<gs::GsSpec>;
#[doc = "Global Channel Status Register"]
pub mod gs;
#[doc = "GRS (rw) register accessor: Global Channel Read Suspend Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grs`] module"]
#[doc(alias = "GRS")]
pub type Grs = crate::Reg<grs::GrsSpec>;
#[doc = "Global Channel Read Suspend Register"]
pub mod grs;
#[doc = "GWS (rw) register accessor: Global Channel Write Suspend Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gws::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gws::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gws`] module"]
#[doc(alias = "GWS")]
pub type Gws = crate::Reg<gws::GwsSpec>;
#[doc = "Global Channel Write Suspend Register"]
pub mod gws;
#[doc = "GRWS (w) register accessor: Global Channel Read Write Suspend Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grws::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grws`] module"]
#[doc(alias = "GRWS")]
pub type Grws = crate::Reg<grws::GrwsSpec>;
#[doc = "Global Channel Read Write Suspend Register"]
pub mod grws;
#[doc = "GRWR (w) register accessor: Global Channel Read Write Resume Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grwr`] module"]
#[doc(alias = "GRWR")]
pub type Grwr = crate::Reg<grwr::GrwrSpec>;
#[doc = "Global Channel Read Write Resume Register"]
pub mod grwr;
#[doc = "GSWR (w) register accessor: Global Channel Software Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gswr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gswr`] module"]
#[doc(alias = "GSWR")]
pub type Gswr = crate::Reg<gswr::GswrSpec>;
#[doc = "Global Channel Software Request Register"]
pub mod gswr;
#[doc = "GSWS (r) register accessor: Global Channel Software Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gsws::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsws`] module"]
#[doc(alias = "GSWS")]
pub type Gsws = crate::Reg<gsws::GswsSpec>;
#[doc = "Global Channel Software Request Status Register"]
pub mod gsws;
#[doc = "GSWF (w) register accessor: Global Channel Software Flush Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gswf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gswf`] module"]
#[doc(alias = "GSWF")]
pub type Gswf = crate::Reg<gswf::GswfSpec>;
#[doc = "Global Channel Software Flush Request Register"]
pub mod gswf;
#[doc = "Channel Interrupt Enable Register"]
pub use self::xdmac_chid::XdmacChid;
#[doc = r"Cluster"]
#[doc = "Channel Interrupt Enable Register"]
pub mod xdmac_chid;
