#[doc = "Register `SEQCTRL0` reader"]
pub struct R(crate::R<SEQCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCTRL0` writer"]
pub struct W(crate::W<SEQCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCTRL0_SPEC>;
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
impl From<crate::W<SEQCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQSEL` reader - Sequential Selection"]
pub type SEQSEL_R = crate::FieldReader<u8, SEQSEL_A>;
#[doc = "Sequential Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEQSEL_A {
    #[doc = "0: Sequential logic disabled"]
    DISABLE = 0,
    #[doc = "1: D FlipFlop"]
    DFF = 1,
    #[doc = "2: JK FlipFlop"]
    JK = 2,
    #[doc = "3: D Latch"]
    LATCH = 3,
    #[doc = "4: RS Latch"]
    RS = 4,
}
impl From<SEQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQSEL_A) -> Self {
        variant as _
    }
}
impl SEQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEQSEL_A> {
        match self.bits {
            0 => Some(SEQSEL_A::DISABLE),
            1 => Some(SEQSEL_A::DFF),
            2 => Some(SEQSEL_A::JK),
            3 => Some(SEQSEL_A::LATCH),
            4 => Some(SEQSEL_A::RS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEQSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DFF`"]
    #[inline(always)]
    pub fn is_dff(&self) -> bool {
        *self == SEQSEL_A::DFF
    }
    #[doc = "Checks if the value of the field is `JK`"]
    #[inline(always)]
    pub fn is_jk(&self) -> bool {
        *self == SEQSEL_A::JK
    }
    #[doc = "Checks if the value of the field is `LATCH`"]
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == SEQSEL_A::LATCH
    }
    #[doc = "Checks if the value of the field is `RS`"]
    #[inline(always)]
    pub fn is_rs(&self) -> bool {
        *self == SEQSEL_A::RS
    }
}
#[doc = "Field `SEQSEL` writer - Sequential Selection"]
pub type SEQSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SEQCTRL0_SPEC, u8, SEQSEL_A, 3, O>;
impl<'a, const O: u8> SEQSEL_W<'a, O> {
    #[doc = "Sequential logic disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEQSEL_A::DISABLE)
    }
    #[doc = "D FlipFlop"]
    #[inline(always)]
    pub fn dff(self) -> &'a mut W {
        self.variant(SEQSEL_A::DFF)
    }
    #[doc = "JK FlipFlop"]
    #[inline(always)]
    pub fn jk(self) -> &'a mut W {
        self.variant(SEQSEL_A::JK)
    }
    #[doc = "D Latch"]
    #[inline(always)]
    pub fn latch(self) -> &'a mut W {
        self.variant(SEQSEL_A::LATCH)
    }
    #[doc = "RS Latch"]
    #[inline(always)]
    pub fn rs(self) -> &'a mut W {
        self.variant(SEQSEL_A::RS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sequential Selection"]
    #[inline(always)]
    pub fn seqsel(&self) -> SEQSEL_R {
        SEQSEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sequential Selection"]
    #[inline(always)]
    #[must_use]
    pub fn seqsel(&mut self) -> SEQSEL_W<0> {
        SEQSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequential Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqctrl0](index.html) module"]
pub struct SEQCTRL0_SPEC;
impl crate::RegisterSpec for SEQCTRL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [seqctrl0::R](R) reader structure"]
impl crate::Readable for SEQCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqctrl0::W](W) writer structure"]
impl crate::Writable for SEQCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQCTRL0 to value 0"]
impl crate::Resettable for SEQCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
