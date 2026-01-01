#[doc = "Register `OSCHFSTATUS` reader"]
pub struct R(crate::R<OSCHFSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCHFSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCHFSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCHFSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ATSYNC` reader - Autotune in Lock"]
pub type ATSYNC_R = crate::BitReader<bool>;
#[doc = "Field `ATLOCK` reader - Autotune Synchronized"]
pub type ATLOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Autotune in Lock"]
    #[inline(always)]
    pub fn atsync(&self) -> ATSYNC_R {
        ATSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autotune Synchronized"]
    #[inline(always)]
    pub fn atlock(&self) -> ATLOCK_R {
        ATLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "OSCHF Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oschfstatus](index.html) module"]
pub struct OSCHFSTATUS_SPEC;
impl crate::RegisterSpec for OSCHFSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [oschfstatus::R](R) reader structure"]
impl crate::Readable for OSCHFSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCHFSTATUS to value 0"]
impl crate::Resettable for OSCHFSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
