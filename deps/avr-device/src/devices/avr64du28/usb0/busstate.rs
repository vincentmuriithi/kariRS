#[doc = "Register `BUSSTATE` reader"]
pub struct R(crate::R<BUSSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSRST` reader - Bus Reset"]
pub type BUSRST_R = crate::BitReader<bool>;
#[doc = "Field `SUSPENDED` reader - Bus Suspended"]
pub type SUSPENDED_R = crate::BitReader<bool>;
#[doc = "Field `DRESUME` reader - Downstram Resume"]
pub type DRESUME_R = crate::BitReader<bool>;
#[doc = "Field `URESUME` reader - Upstream Resume"]
pub type URESUME_R = crate::BitReader<bool>;
#[doc = "Field `WTRSM` reader - Wait Time Resume"]
pub type WTRSM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Bus Reset"]
    #[inline(always)]
    pub fn busrst(&self) -> BUSRST_R {
        BUSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus Suspended"]
    #[inline(always)]
    pub fn suspended(&self) -> SUSPENDED_R {
        SUSPENDED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Downstram Resume"]
    #[inline(always)]
    pub fn dresume(&self) -> DRESUME_R {
        DRESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Upstream Resume"]
    #[inline(always)]
    pub fn uresume(&self) -> URESUME_R {
        URESUME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wait Time Resume"]
    #[inline(always)]
    pub fn wtrsm(&self) -> WTRSM_R {
        WTRSM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Bus State\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busstate](index.html) module"]
pub struct BUSSTATE_SPEC;
impl crate::RegisterSpec for BUSSTATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [busstate::R](R) reader structure"]
impl crate::Readable for BUSSTATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSSTATE to value 0"]
impl crate::Resettable for BUSSTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
