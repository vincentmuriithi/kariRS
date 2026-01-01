#[doc = "Register `LINIDR` reader"]
pub struct R(crate::R<LINIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINIDR` writer"]
pub struct W(crate::W<LINIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINIDR_SPEC>;
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
impl From<crate::W<LINIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LID` reader - Identifier bit 5 or Data Length bits"]
pub type LID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LID` writer - Identifier bit 5 or Data Length bits"]
pub type LID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LINIDR_SPEC, u8, u8, 6, O>;
#[doc = "Field `LP` reader - Parity bits"]
pub type LP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP` writer - Parity bits"]
pub type LP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LINIDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5 - Identifier bit 5 or Data Length bits"]
    #[inline(always)]
    pub fn lid(&self) -> LID_R {
        LID_R::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - Parity bits"]
    #[inline(always)]
    pub fn lp(&self) -> LP_R {
        LP_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - Identifier bit 5 or Data Length bits"]
    #[inline(always)]
    #[must_use]
    pub fn lid(&mut self) -> LID_W<0> {
        LID_W::new(self)
    }
    #[doc = "Bits 6:7 - Parity bits"]
    #[inline(always)]
    #[must_use]
    pub fn lp(&mut self) -> LP_W<6> {
        LP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linidr](index.html) module"]
pub struct LINIDR_SPEC;
impl crate::RegisterSpec for LINIDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [linidr::R](R) reader structure"]
impl crate::Readable for LINIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linidr::W](W) writer structure"]
impl crate::Writable for LINIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINIDR to value 0"]
impl crate::Resettable for LINIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
