#[doc = "Register `TCCR4E` reader"]
pub struct R(crate::R<TCCR4E_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR4E_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR4E_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR4E_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR4E` writer"]
pub struct W(crate::W<TCCR4E_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR4E_SPEC>;
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
impl From<crate::W<TCCR4E_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR4E_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC4OE` reader - Output Compare Override Enable bit"]
pub type OC4OE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC4OE` writer - Output Compare Override Enable bit"]
pub type OC4OE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR4E_SPEC, u8, u8, 6, O>;
#[doc = "Field `ENHC4` reader - Enhanced Compare/PWM Mode"]
pub type ENHC4_R = crate::BitReader<bool>;
#[doc = "Field `ENHC4` writer - Enhanced Compare/PWM Mode"]
pub type ENHC4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4E_SPEC, bool, O>;
#[doc = "Field `TLOCK4` reader - Register Update Lock"]
pub type TLOCK4_R = crate::BitReader<bool>;
#[doc = "Field `TLOCK4` writer - Register Update Lock"]
pub type TLOCK4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4E_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Output Compare Override Enable bit"]
    #[inline(always)]
    pub fn oc4oe(&self) -> OC4OE_R {
        OC4OE_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Enhanced Compare/PWM Mode"]
    #[inline(always)]
    pub fn enhc4(&self) -> ENHC4_R {
        ENHC4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Register Update Lock"]
    #[inline(always)]
    pub fn tlock4(&self) -> TLOCK4_R {
        TLOCK4_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Output Compare Override Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn oc4oe(&mut self) -> OC4OE_W<0> {
        OC4OE_W::new(self)
    }
    #[doc = "Bit 6 - Enhanced Compare/PWM Mode"]
    #[inline(always)]
    #[must_use]
    pub fn enhc4(&mut self) -> ENHC4_W<6> {
        ENHC4_W::new(self)
    }
    #[doc = "Bit 7 - Register Update Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tlock4(&mut self) -> TLOCK4_W<7> {
        TLOCK4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter 4 Control Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4e](index.html) module"]
pub struct TCCR4E_SPEC;
impl crate::RegisterSpec for TCCR4E_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr4e::R](R) reader structure"]
impl crate::Readable for TCCR4E_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr4e::W](W) writer structure"]
impl crate::Writable for TCCR4E_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4E to value 0"]
impl crate::Resettable for TCCR4E_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
