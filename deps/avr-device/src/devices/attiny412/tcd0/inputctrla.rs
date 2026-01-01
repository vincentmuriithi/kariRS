#[doc = "Register `INPUTCTRLA` reader"]
pub struct R(crate::R<INPUTCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUTCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUTCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUTCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUTCTRLA` writer"]
pub struct W(crate::W<INPUTCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTCTRLA_SPEC>;
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
impl From<crate::W<INPUTCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTMODE` reader - Input mode"]
pub type INPUTMODE_R = crate::FieldReader<u8, INPUTMODE_A>;
#[doc = "Input mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUTMODE_A {
    #[doc = "0: Input has no actions"]
    NONE = 0,
    #[doc = "1: Stop output, jump to opposite compare cycle and wait"]
    JMPWAIT = 1,
    #[doc = "2: Stop output, execute opposite compare cycle and wait"]
    EXECWAIT = 2,
    #[doc = "3: stop output, execute opposite compare cycle while fault active"]
    EXECFAULT = 3,
    #[doc = "4: Stop all outputs, maintain frequency"]
    FREQ = 4,
    #[doc = "5: Stop all outputs, execute dead time while fault active"]
    EXECDT = 5,
    #[doc = "6: Stop all outputs, jump to next compare cycle and wait"]
    WAIT = 6,
    #[doc = "7: Stop all outputs, wait for software action"]
    WAITSW = 7,
    #[doc = "8: Stop output on edge, jump to next compare cycle"]
    EDGETRIG = 8,
    #[doc = "9: Stop output on edge, maintain frequency"]
    EDGETRIGFREQ = 9,
    #[doc = "10: Stop output at level, maintain frequency"]
    LVLTRIGFREQ = 10,
}
impl From<INPUTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTMODE_A) -> Self {
        variant as _
    }
}
impl INPUTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUTMODE_A> {
        match self.bits {
            0 => Some(INPUTMODE_A::NONE),
            1 => Some(INPUTMODE_A::JMPWAIT),
            2 => Some(INPUTMODE_A::EXECWAIT),
            3 => Some(INPUTMODE_A::EXECFAULT),
            4 => Some(INPUTMODE_A::FREQ),
            5 => Some(INPUTMODE_A::EXECDT),
            6 => Some(INPUTMODE_A::WAIT),
            7 => Some(INPUTMODE_A::WAITSW),
            8 => Some(INPUTMODE_A::EDGETRIG),
            9 => Some(INPUTMODE_A::EDGETRIGFREQ),
            10 => Some(INPUTMODE_A::LVLTRIGFREQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == INPUTMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `JMPWAIT`"]
    #[inline(always)]
    pub fn is_jmpwait(&self) -> bool {
        *self == INPUTMODE_A::JMPWAIT
    }
    #[doc = "Checks if the value of the field is `EXECWAIT`"]
    #[inline(always)]
    pub fn is_execwait(&self) -> bool {
        *self == INPUTMODE_A::EXECWAIT
    }
    #[doc = "Checks if the value of the field is `EXECFAULT`"]
    #[inline(always)]
    pub fn is_execfault(&self) -> bool {
        *self == INPUTMODE_A::EXECFAULT
    }
    #[doc = "Checks if the value of the field is `FREQ`"]
    #[inline(always)]
    pub fn is_freq(&self) -> bool {
        *self == INPUTMODE_A::FREQ
    }
    #[doc = "Checks if the value of the field is `EXECDT`"]
    #[inline(always)]
    pub fn is_execdt(&self) -> bool {
        *self == INPUTMODE_A::EXECDT
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == INPUTMODE_A::WAIT
    }
    #[doc = "Checks if the value of the field is `WAITSW`"]
    #[inline(always)]
    pub fn is_waitsw(&self) -> bool {
        *self == INPUTMODE_A::WAITSW
    }
    #[doc = "Checks if the value of the field is `EDGETRIG`"]
    #[inline(always)]
    pub fn is_edgetrig(&self) -> bool {
        *self == INPUTMODE_A::EDGETRIG
    }
    #[doc = "Checks if the value of the field is `EDGETRIGFREQ`"]
    #[inline(always)]
    pub fn is_edgetrigfreq(&self) -> bool {
        *self == INPUTMODE_A::EDGETRIGFREQ
    }
    #[doc = "Checks if the value of the field is `LVLTRIGFREQ`"]
    #[inline(always)]
    pub fn is_lvltrigfreq(&self) -> bool {
        *self == INPUTMODE_A::LVLTRIGFREQ
    }
}
#[doc = "Field `INPUTMODE` writer - Input mode"]
pub type INPUTMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, INPUTCTRLA_SPEC, u8, INPUTMODE_A, 4, O>;
impl<'a, const O: u8> INPUTMODE_W<'a, O> {
    #[doc = "Input has no actions"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(INPUTMODE_A::NONE)
    }
    #[doc = "Stop output, jump to opposite compare cycle and wait"]
    #[inline(always)]
    pub fn jmpwait(self) -> &'a mut W {
        self.variant(INPUTMODE_A::JMPWAIT)
    }
    #[doc = "Stop output, execute opposite compare cycle and wait"]
    #[inline(always)]
    pub fn execwait(self) -> &'a mut W {
        self.variant(INPUTMODE_A::EXECWAIT)
    }
    #[doc = "stop output, execute opposite compare cycle while fault active"]
    #[inline(always)]
    pub fn execfault(self) -> &'a mut W {
        self.variant(INPUTMODE_A::EXECFAULT)
    }
    #[doc = "Stop all outputs, maintain frequency"]
    #[inline(always)]
    pub fn freq(self) -> &'a mut W {
        self.variant(INPUTMODE_A::FREQ)
    }
    #[doc = "Stop all outputs, execute dead time while fault active"]
    #[inline(always)]
    pub fn execdt(self) -> &'a mut W {
        self.variant(INPUTMODE_A::EXECDT)
    }
    #[doc = "Stop all outputs, jump to next compare cycle and wait"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut W {
        self.variant(INPUTMODE_A::WAIT)
    }
    #[doc = "Stop all outputs, wait for software action"]
    #[inline(always)]
    pub fn waitsw(self) -> &'a mut W {
        self.variant(INPUTMODE_A::WAITSW)
    }
    #[doc = "Stop output on edge, jump to next compare cycle"]
    #[inline(always)]
    pub fn edgetrig(self) -> &'a mut W {
        self.variant(INPUTMODE_A::EDGETRIG)
    }
    #[doc = "Stop output on edge, maintain frequency"]
    #[inline(always)]
    pub fn edgetrigfreq(self) -> &'a mut W {
        self.variant(INPUTMODE_A::EDGETRIGFREQ)
    }
    #[doc = "Stop output at level, maintain frequency"]
    #[inline(always)]
    pub fn lvltrigfreq(self) -> &'a mut W {
        self.variant(INPUTMODE_A::LVLTRIGFREQ)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input mode"]
    #[inline(always)]
    pub fn inputmode(&self) -> INPUTMODE_R {
        INPUTMODE_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input mode"]
    #[inline(always)]
    #[must_use]
    pub fn inputmode(&mut self) -> INPUTMODE_W<0> {
        INPUTMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputctrla](index.html) module"]
pub struct INPUTCTRLA_SPEC;
impl crate::RegisterSpec for INPUTCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [inputctrla::R](R) reader structure"]
impl crate::Readable for INPUTCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inputctrla::W](W) writer structure"]
impl crate::Writable for INPUTCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTCTRLA to value 0"]
impl crate::Resettable for INPUTCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
