#[doc = "Register `MCLKTIMEBASE` reader"]
pub struct R(crate::R<MCLKTIMEBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKTIMEBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKTIMEBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKTIMEBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKTIMEBASE` writer"]
pub struct W(crate::W<MCLKTIMEBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKTIMEBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MCLKTIMEBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKTIMEBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEBASE` reader - Timebase"]
pub type TIMEBASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMEBASE` writer - Timebase"]
pub type TIMEBASE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, MCLKTIMEBASE_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Timebase"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Timebase"]
    #[inline(always)]
    #[must_use]
    pub fn timebase(&mut self) -> TIMEBASE_W<0> {
        TIMEBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timebase\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclktimebase](index.html) module"]
pub struct MCLKTIMEBASE_SPEC;
impl crate::RegisterSpec for MCLKTIMEBASE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mclktimebase::R](R) reader structure"]
impl crate::Readable for MCLKTIMEBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclktimebase::W](W) writer structure"]
impl crate::Writable for MCLKTIMEBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKTIMEBASE to value 0"]
impl crate::Resettable for MCLKTIMEBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
