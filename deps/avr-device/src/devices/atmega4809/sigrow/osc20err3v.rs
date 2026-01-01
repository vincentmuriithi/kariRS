#[doc = "Register `OSC20ERR3V` reader"]
pub struct R(crate::R<OSC20ERR3V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC20ERR3V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC20ERR3V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC20ERR3V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "OSC20 error at 3V\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc20err3v](index.html) module"]
pub struct OSC20ERR3V_SPEC;
impl crate::RegisterSpec for OSC20ERR3V_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc20err3v::R](R) reader structure"]
impl crate::Readable for OSC20ERR3V_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSC20ERR3V to value 0"]
impl crate::Resettable for OSC20ERR3V_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
