#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPIF` reader - Analog Comparator Interrupt Flag"]
pub type CMPIF_R = crate::BitReader<bool>;
#[doc = "Field `CMPIF` writer - Analog Comparator Interrupt Flag"]
pub type CMPIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
#[doc = "Field `CMPSTATE` reader - Analog Comparator State"]
pub type CMPSTATE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn cmpif(&self) -> CMPIF_R {
        CMPIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator State"]
    #[inline(always)]
    pub fn cmpstate(&self) -> CMPSTATE_R {
        CMPSTATE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpif(&mut self) -> CMPIF_W<0> {
        CMPIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
