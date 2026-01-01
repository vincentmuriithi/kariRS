#[doc = "Register `UDADDR` reader"]
pub struct R(crate::R<UDADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDADDR` writer"]
pub struct W(crate::W<UDADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDADDR_SPEC>;
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
impl From<crate::W<UDADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UADD` reader - No Description."]
pub type UADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UADD` writer - No Description."]
pub type UADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UDADDR_SPEC, u8, u8, 7, O>;
#[doc = "Field `ADDEN` reader - No Description."]
pub type ADDEN_R = crate::BitReader<bool>;
#[doc = "Field `ADDEN` writer - No Description."]
pub type ADDEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, UDADDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - No Description."]
    #[inline(always)]
    pub fn uadd(&self) -> UADD_R {
        UADD_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn uadd(&mut self) -> UADD_W<0> {
        UADD_W::new(self)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn adden(&mut self) -> ADDEN_W<7> {
        ADDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udaddr](index.html) module"]
pub struct UDADDR_SPEC;
impl crate::RegisterSpec for UDADDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [udaddr::R](R) reader structure"]
impl crate::Readable for UDADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udaddr::W](W) writer structure"]
impl crate::Writable for UDADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDADDR to value 0"]
impl crate::Resettable for UDADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
