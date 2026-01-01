#[doc = "Register `DITCTRL` reader"]
pub struct R(crate::R<DITCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DITCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DITCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DITCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DITCTRL` writer"]
pub struct W(crate::W<DITCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DITCTRL_SPEC>;
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
impl From<crate::W<DITCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DITCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHERSEL` reader - dither select"]
pub type DITHERSEL_R = crate::FieldReader<u8, DITHERSEL_A>;
#[doc = "dither select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DITHERSEL_A {
    #[doc = "0: On-time ramp B"]
    ONTIMEB = 0,
    #[doc = "1: On-time ramp A and B"]
    ONTIMEAB = 1,
    #[doc = "2: Dead-time rampB"]
    DEADTIMEB = 2,
    #[doc = "3: Dead-time ramp A and B"]
    DEADTIMEAB = 3,
}
impl From<DITHERSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DITHERSEL_A) -> Self {
        variant as _
    }
}
impl DITHERSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DITHERSEL_A {
        match self.bits {
            0 => DITHERSEL_A::ONTIMEB,
            1 => DITHERSEL_A::ONTIMEAB,
            2 => DITHERSEL_A::DEADTIMEB,
            3 => DITHERSEL_A::DEADTIMEAB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONTIMEB`"]
    #[inline(always)]
    pub fn is_ontimeb(&self) -> bool {
        *self == DITHERSEL_A::ONTIMEB
    }
    #[doc = "Checks if the value of the field is `ONTIMEAB`"]
    #[inline(always)]
    pub fn is_ontimeab(&self) -> bool {
        *self == DITHERSEL_A::ONTIMEAB
    }
    #[doc = "Checks if the value of the field is `DEADTIMEB`"]
    #[inline(always)]
    pub fn is_deadtimeb(&self) -> bool {
        *self == DITHERSEL_A::DEADTIMEB
    }
    #[doc = "Checks if the value of the field is `DEADTIMEAB`"]
    #[inline(always)]
    pub fn is_deadtimeab(&self) -> bool {
        *self == DITHERSEL_A::DEADTIMEAB
    }
}
#[doc = "Field `DITHERSEL` writer - dither select"]
pub type DITHERSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, DITCTRL_SPEC, u8, DITHERSEL_A, 2, O>;
impl<'a, const O: u8> DITHERSEL_W<'a, O> {
    #[doc = "On-time ramp B"]
    #[inline(always)]
    pub fn ontimeb(self) -> &'a mut W {
        self.variant(DITHERSEL_A::ONTIMEB)
    }
    #[doc = "On-time ramp A and B"]
    #[inline(always)]
    pub fn ontimeab(self) -> &'a mut W {
        self.variant(DITHERSEL_A::ONTIMEAB)
    }
    #[doc = "Dead-time rampB"]
    #[inline(always)]
    pub fn deadtimeb(self) -> &'a mut W {
        self.variant(DITHERSEL_A::DEADTIMEB)
    }
    #[doc = "Dead-time ramp A and B"]
    #[inline(always)]
    pub fn deadtimeab(self) -> &'a mut W {
        self.variant(DITHERSEL_A::DEADTIMEAB)
    }
}
impl R {
    #[doc = "Bits 0:1 - dither select"]
    #[inline(always)]
    pub fn dithersel(&self) -> DITHERSEL_R {
        DITHERSEL_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - dither select"]
    #[inline(always)]
    #[must_use]
    pub fn dithersel(&mut self) -> DITHERSEL_W<0> {
        DITHERSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dither Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditctrl](index.html) module"]
pub struct DITCTRL_SPEC;
impl crate::RegisterSpec for DITCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ditctrl::R](R) reader structure"]
impl crate::Readable for DITCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ditctrl::W](W) writer structure"]
impl crate::Writable for DITCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DITCTRL to value 0"]
impl crate::Resettable for DITCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
