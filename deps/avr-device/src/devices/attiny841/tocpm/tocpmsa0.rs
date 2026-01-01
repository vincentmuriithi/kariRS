#[doc = "Register `TOCPMSA0` reader"]
pub struct R(crate::R<TOCPMSA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCPMSA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCPMSA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCPMSA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCPMSA0` writer"]
pub struct W(crate::W<TOCPMSA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCPMSA0_SPEC>;
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
impl From<crate::W<TOCPMSA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCPMSA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOCC0S` reader - Timer Output Compare Channel 0 Selection Bits"]
pub type TOCC0S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCC0S` writer - Timer Output Compare Channel 0 Selection Bits"]
pub type TOCC0S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TOCPMSA0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TOCC1S` reader - Timer Output Compare Channel 1 Selection Bits"]
pub type TOCC1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCC1S` writer - Timer Output Compare Channel 1 Selection Bits"]
pub type TOCC1S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TOCPMSA0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TOCC2S` reader - Timer Output Compare Channel 2 Selection Bits"]
pub type TOCC2S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCC2S` writer - Timer Output Compare Channel 2 Selection Bits"]
pub type TOCC2S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TOCPMSA0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TOCC3S` reader - Timer Output Compare Channel 3 Selection Bits"]
pub type TOCC3S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCC3S` writer - Timer Output Compare Channel 3 Selection Bits"]
pub type TOCC3S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TOCPMSA0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Timer Output Compare Channel 0 Selection Bits"]
    #[inline(always)]
    pub fn tocc0s(&self) -> TOCC0S_R {
        TOCC0S_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Timer Output Compare Channel 1 Selection Bits"]
    #[inline(always)]
    pub fn tocc1s(&self) -> TOCC1S_R {
        TOCC1S_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Timer Output Compare Channel 2 Selection Bits"]
    #[inline(always)]
    pub fn tocc2s(&self) -> TOCC2S_R {
        TOCC2S_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Timer Output Compare Channel 3 Selection Bits"]
    #[inline(always)]
    pub fn tocc3s(&self) -> TOCC3S_R {
        TOCC3S_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Output Compare Channel 0 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc0s(&mut self) -> TOCC0S_W<0> {
        TOCC0S_W::new(self)
    }
    #[doc = "Bits 2:3 - Timer Output Compare Channel 1 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc1s(&mut self) -> TOCC1S_W<2> {
        TOCC1S_W::new(self)
    }
    #[doc = "Bits 4:5 - Timer Output Compare Channel 2 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc2s(&mut self) -> TOCC2S_W<4> {
        TOCC2S_W::new(self)
    }
    #[doc = "Bits 6:7 - Timer Output Compare Channel 3 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc3s(&mut self) -> TOCC3S_W<6> {
        TOCC3S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Output Compare Pin Mux Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocpmsa0](index.html) module"]
pub struct TOCPMSA0_SPEC;
impl crate::RegisterSpec for TOCPMSA0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tocpmsa0::R](R) reader structure"]
impl crate::Readable for TOCPMSA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tocpmsa0::W](W) writer structure"]
impl crate::Writable for TOCPMSA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOCPMSA0 to value 0"]
impl crate::Resettable for TOCPMSA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
