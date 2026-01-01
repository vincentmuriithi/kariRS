#[doc = "Register `TOCPMSA1` reader"]
pub struct R(crate::R<TOCPMSA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCPMSA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCPMSA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCPMSA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCPMSA1` writer"]
pub struct W(crate::W<TOCPMSA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCPMSA1_SPEC>;
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
impl From<crate::W<TOCPMSA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCPMSA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOCC4S` reader - Timer Output Compare Channel 4 Selection Bits"]
pub type TOCC4S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCC4S` writer - Timer Output Compare Channel 4 Selection Bits"]
pub type TOCC4S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TOCPMSA1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TOCC5S` reader - Timer Output Compare Channel 5 Selection Bits"]
pub type TOCC5S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCC5S` writer - Timer Output Compare Channel 5 Selection Bits"]
pub type TOCC5S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TOCPMSA1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TOCC6S` reader - Timer Output Compare Channel 6 Selection Bits"]
pub type TOCC6S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCC6S` writer - Timer Output Compare Channel 6 Selection Bits"]
pub type TOCC6S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TOCPMSA1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TOCC7S` reader - Timer Output Compare Channel 7 Selection Bits"]
pub type TOCC7S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCC7S` writer - Timer Output Compare Channel 7 Selection Bits"]
pub type TOCC7S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TOCPMSA1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Timer Output Compare Channel 4 Selection Bits"]
    #[inline(always)]
    pub fn tocc4s(&self) -> TOCC4S_R {
        TOCC4S_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Timer Output Compare Channel 5 Selection Bits"]
    #[inline(always)]
    pub fn tocc5s(&self) -> TOCC5S_R {
        TOCC5S_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Timer Output Compare Channel 6 Selection Bits"]
    #[inline(always)]
    pub fn tocc6s(&self) -> TOCC6S_R {
        TOCC6S_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Timer Output Compare Channel 7 Selection Bits"]
    #[inline(always)]
    pub fn tocc7s(&self) -> TOCC7S_R {
        TOCC7S_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Output Compare Channel 4 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc4s(&mut self) -> TOCC4S_W<0> {
        TOCC4S_W::new(self)
    }
    #[doc = "Bits 2:3 - Timer Output Compare Channel 5 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc5s(&mut self) -> TOCC5S_W<2> {
        TOCC5S_W::new(self)
    }
    #[doc = "Bits 4:5 - Timer Output Compare Channel 6 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc6s(&mut self) -> TOCC6S_W<4> {
        TOCC6S_W::new(self)
    }
    #[doc = "Bits 6:7 - Timer Output Compare Channel 7 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc7s(&mut self) -> TOCC7S_W<6> {
        TOCC7S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Output Compare Pin Mux Selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocpmsa1](index.html) module"]
pub struct TOCPMSA1_SPEC;
impl crate::RegisterSpec for TOCPMSA1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tocpmsa1::R](R) reader structure"]
impl crate::Readable for TOCPMSA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tocpmsa1::W](W) writer structure"]
impl crate::Writable for TOCPMSA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOCPMSA1 to value 0"]
impl crate::Resettable for TOCPMSA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
