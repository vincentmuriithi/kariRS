#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WGMODE` reader - Waveform generation mode"]
pub type WGMODE_R = crate::FieldReader<u8, WGMODE_A>;
#[doc = "Waveform generation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGMODE_A {
    #[doc = "0: One ramp mode"]
    ONERAMP = 0,
    #[doc = "1: Two ramp mode"]
    TWORAMP = 1,
    #[doc = "2: Four ramp mode"]
    FOURRAMP = 2,
    #[doc = "3: Dual slope mode"]
    DS = 3,
}
impl From<WGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WGMODE_A) -> Self {
        variant as _
    }
}
impl WGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WGMODE_A {
        match self.bits {
            0 => WGMODE_A::ONERAMP,
            1 => WGMODE_A::TWORAMP,
            2 => WGMODE_A::FOURRAMP,
            3 => WGMODE_A::DS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONERAMP`"]
    #[inline(always)]
    pub fn is_oneramp(&self) -> bool {
        *self == WGMODE_A::ONERAMP
    }
    #[doc = "Checks if the value of the field is `TWORAMP`"]
    #[inline(always)]
    pub fn is_tworamp(&self) -> bool {
        *self == WGMODE_A::TWORAMP
    }
    #[doc = "Checks if the value of the field is `FOURRAMP`"]
    #[inline(always)]
    pub fn is_fourramp(&self) -> bool {
        *self == WGMODE_A::FOURRAMP
    }
    #[doc = "Checks if the value of the field is `DS`"]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == WGMODE_A::DS
    }
}
#[doc = "Field `WGMODE` writer - Waveform generation mode"]
pub type WGMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CTRLB_SPEC, u8, WGMODE_A, 2, O>;
impl<'a, const O: u8> WGMODE_W<'a, O> {
    #[doc = "One ramp mode"]
    #[inline(always)]
    pub fn oneramp(self) -> &'a mut W {
        self.variant(WGMODE_A::ONERAMP)
    }
    #[doc = "Two ramp mode"]
    #[inline(always)]
    pub fn tworamp(self) -> &'a mut W {
        self.variant(WGMODE_A::TWORAMP)
    }
    #[doc = "Four ramp mode"]
    #[inline(always)]
    pub fn fourramp(self) -> &'a mut W {
        self.variant(WGMODE_A::FOURRAMP)
    }
    #[doc = "Dual slope mode"]
    #[inline(always)]
    pub fn ds(self) -> &'a mut W {
        self.variant(WGMODE_A::DS)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform generation mode"]
    #[inline(always)]
    pub fn wgmode(&self) -> WGMODE_R {
        WGMODE_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform generation mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgmode(&mut self) -> WGMODE_W<0> {
        WGMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
