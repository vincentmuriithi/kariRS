#[doc = "Register `SFD_VALUE` reader"]
pub struct R(crate::R<SFD_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFD_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFD_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFD_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFD_VALUE` writer"]
pub struct W(crate::W<SFD_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFD_VALUE_SPEC>;
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
impl From<crate::W<SFD_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFD_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFD_VALUE` reader - Start of Frame Delimiter Value"]
pub type SFD_VALUE_R = crate::FieldReader<u8, SFD_VALUE_A>;
#[doc = "Start of Frame Delimiter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFD_VALUE_A {
    #[doc = "167: IEEE 802.15.4 compliant value of the SFD"]
    IEEE_SFD = 167,
}
impl From<SFD_VALUE_A> for u8 {
    #[inline(always)]
    fn from(variant: SFD_VALUE_A) -> Self {
        variant as _
    }
}
impl SFD_VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SFD_VALUE_A> {
        match self.bits {
            167 => Some(SFD_VALUE_A::IEEE_SFD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IEEE_SFD`"]
    #[inline(always)]
    pub fn is_ieee_sfd(&self) -> bool {
        *self == SFD_VALUE_A::IEEE_SFD
    }
}
#[doc = "Field `SFD_VALUE` writer - Start of Frame Delimiter Value"]
pub type SFD_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, SFD_VALUE_SPEC, u8, SFD_VALUE_A, 8, O>;
impl<'a, const O: u8> SFD_VALUE_W<'a, O> {
    #[doc = "IEEE 802.15.4 compliant value of the SFD"]
    #[inline(always)]
    pub fn ieee_sfd(self) -> &'a mut W {
        self.variant(SFD_VALUE_A::IEEE_SFD)
    }
}
impl R {
    #[doc = "Bits 0:7 - Start of Frame Delimiter Value"]
    #[inline(always)]
    pub fn sfd_value(&self) -> SFD_VALUE_R {
        SFD_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start of Frame Delimiter Value"]
    #[inline(always)]
    #[must_use]
    pub fn sfd_value(&mut self) -> SFD_VALUE_W<0> {
        SFD_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start of Frame Delimiter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfd_value](index.html) module"]
pub struct SFD_VALUE_SPEC;
impl crate::RegisterSpec for SFD_VALUE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sfd_value::R](R) reader structure"]
impl crate::Readable for SFD_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfd_value::W](W) writer structure"]
impl crate::Writable for SFD_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFD_VALUE to value 0"]
impl crate::Resettable for SFD_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
