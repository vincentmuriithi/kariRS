#[doc = "Register `RAMPZ` reader"]
pub struct R(crate::R<RAMPZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMPZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMPZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMPZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMPZ` writer"]
pub struct W(crate::W<RAMPZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMPZ_SPEC>;
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
impl From<crate::W<RAMPZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMPZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMPZ` reader - Extended Z-Pointer Value"]
pub type RAMPZ_R = crate::FieldReader<u8, RAMPZ_A>;
#[doc = "Extended Z-Pointer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPZ_A {
    #[doc = "0: Default value of Z-pointer MSB's."]
    VAL_0 = 0,
}
impl From<RAMPZ_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPZ_A) -> Self {
        variant as _
    }
}
impl RAMPZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMPZ_A> {
        match self.bits {
            0 => Some(RAMPZ_A::VAL_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0`"]
    #[inline(always)]
    pub fn is_val_0(&self) -> bool {
        *self == RAMPZ_A::VAL_0
    }
}
#[doc = "Field `RAMPZ` writer - Extended Z-Pointer Value"]
pub type RAMPZ_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RAMPZ_SPEC, u8, RAMPZ_A, 2, O>;
impl<'a, const O: u8> RAMPZ_W<'a, O> {
    #[doc = "Default value of Z-pointer MSB's."]
    #[inline(always)]
    pub fn val_0(self) -> &'a mut W {
        self.variant(RAMPZ_A::VAL_0)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, RAMPZ_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - Extended Z-Pointer Value"]
    #[inline(always)]
    pub fn rampz(&self) -> RAMPZ_R {
        RAMPZ_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 2) & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Extended Z-Pointer Value"]
    #[inline(always)]
    #[must_use]
    pub fn rampz(&mut self) -> RAMPZ_W<0> {
        RAMPZ_W::new(self)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<2> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Z-pointer Register for ELPM/SPM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rampz](index.html) module"]
pub struct RAMPZ_SPEC;
impl crate::RegisterSpec for RAMPZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rampz::R](R) reader structure"]
impl crate::Readable for RAMPZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rampz::W](W) writer structure"]
impl crate::Writable for RAMPZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMPZ to value 0"]
impl crate::Resettable for RAMPZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
