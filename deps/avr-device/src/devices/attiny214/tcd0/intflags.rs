#[doc = "Register `INTFLAGS` reader"]
pub struct R(crate::R<INTFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGS` writer"]
pub struct W(crate::W<INTFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGS_SPEC>;
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
impl From<crate::W<INTFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Overflow interrupt enable"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - Overflow interrupt enable"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `TRIGA` reader - Trigger A interrupt enable"]
pub type TRIGA_R = crate::BitReader<bool>;
#[doc = "Field `TRIGA` writer - Trigger A interrupt enable"]
pub type TRIGA_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `TRIGB` reader - Trigger B interrupt enable"]
pub type TRIGB_R = crate::BitReader<bool>;
#[doc = "Field `TRIGB` writer - Trigger B interrupt enable"]
pub type TRIGB_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger A interrupt enable"]
    #[inline(always)]
    pub fn triga(&self) -> TRIGA_R {
        TRIGA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger B interrupt enable"]
    #[inline(always)]
    pub fn trigb(&self) -> TRIGB_R {
        TRIGB_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 2 - Trigger A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn triga(&mut self) -> TRIGA_W<2> {
        TRIGA_W::new(self)
    }
    #[doc = "Bit 3 - Trigger B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trigb(&mut self) -> TRIGB_W<3> {
        TRIGB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflags](index.html) module"]
pub struct INTFLAGS_SPEC;
impl crate::RegisterSpec for INTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intflags::R](R) reader structure"]
impl crate::Readable for INTFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflags::W](W) writer structure"]
impl crate::Writable for INTFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGS to value 0"]
impl crate::Resettable for INTFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
