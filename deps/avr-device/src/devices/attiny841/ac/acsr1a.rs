#[doc = "Register `ACSR1A` reader"]
pub struct R(crate::R<ACSR1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACSR1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACSR1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACSR1A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACSR1A` writer"]
pub struct W(crate::W<ACSR1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACSR1A_SPEC>;
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
impl From<crate::W<ACSR1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACSR1A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACIS1` reader - Analog Comparator Interrupt Mode Select"]
pub type ACIS1_R = crate::FieldReader<u8, ACIS1_A>;
#[doc = "Analog Comparator Interrupt Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACIS1_A {
    #[doc = "0: Interrupt on Toggle"]
    ON_TOGGLE = 0,
    #[doc = "2: Interrupt on Falling Edge"]
    ON_FALLING_EDGE = 2,
    #[doc = "3: Interrupt on Rising Edge"]
    ON_RISING_EDGE = 3,
}
impl From<ACIS1_A> for u8 {
    #[inline(always)]
    fn from(variant: ACIS1_A) -> Self {
        variant as _
    }
}
impl ACIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACIS1_A> {
        match self.bits {
            0 => Some(ACIS1_A::ON_TOGGLE),
            2 => Some(ACIS1_A::ON_FALLING_EDGE),
            3 => Some(ACIS1_A::ON_RISING_EDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ON_TOGGLE`"]
    #[inline(always)]
    pub fn is_on_toggle(&self) -> bool {
        *self == ACIS1_A::ON_TOGGLE
    }
    #[doc = "Checks if the value of the field is `ON_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_on_falling_edge(&self) -> bool {
        *self == ACIS1_A::ON_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `ON_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_on_rising_edge(&self) -> bool {
        *self == ACIS1_A::ON_RISING_EDGE
    }
}
#[doc = "Field `ACIS1` writer - Analog Comparator Interrupt Mode Select"]
pub type ACIS1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ACSR1A_SPEC, u8, ACIS1_A, 2, O>;
impl<'a, const O: u8> ACIS1_W<'a, O> {
    #[doc = "Interrupt on Toggle"]
    #[inline(always)]
    pub fn on_toggle(self) -> &'a mut W {
        self.variant(ACIS1_A::ON_TOGGLE)
    }
    #[doc = "Interrupt on Falling Edge"]
    #[inline(always)]
    pub fn on_falling_edge(self) -> &'a mut W {
        self.variant(ACIS1_A::ON_FALLING_EDGE)
    }
    #[doc = "Interrupt on Rising Edge"]
    #[inline(always)]
    pub fn on_rising_edge(self) -> &'a mut W {
        self.variant(ACIS1_A::ON_RISING_EDGE)
    }
}
#[doc = "Field `ACIC1` reader - Analog Comparator 1 Input Capture Enable"]
pub type ACIC1_R = crate::BitReader<bool>;
#[doc = "Field `ACIC1` writer - Analog Comparator 1 Input Capture Enable"]
pub type ACIC1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1A_SPEC, bool, O>;
#[doc = "Field `ACIE1` reader - Analog Comparator 1 Interrupt Enable"]
pub type ACIE1_R = crate::BitReader<bool>;
#[doc = "Field `ACIE1` writer - Analog Comparator 1 Interrupt Enable"]
pub type ACIE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1A_SPEC, bool, O>;
#[doc = "Field `ACI1` reader - Analog Comparator 1 Interrupt Flag"]
pub type ACI1_R = crate::BitReader<bool>;
#[doc = "Field `ACI1` writer - Analog Comparator 1 Interrupt Flag"]
pub type ACI1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1A_SPEC, bool, O>;
#[doc = "Field `ACO1` reader - Analog Comparator 1 Output"]
pub type ACO1_R = crate::BitReader<bool>;
#[doc = "Field `ACO1` writer - Analog Comparator 1 Output"]
pub type ACO1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1A_SPEC, bool, O>;
#[doc = "Field `ACBG1` reader - Analog Comparator 1 Bandgap Select"]
pub type ACBG1_R = crate::BitReader<bool>;
#[doc = "Field `ACBG1` writer - Analog Comparator 1 Bandgap Select"]
pub type ACBG1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1A_SPEC, bool, O>;
#[doc = "Field `ACD1` reader - Analog Comparator 1 Disable"]
pub type ACD1_R = crate::BitReader<bool>;
#[doc = "Field `ACD1` writer - Analog Comparator 1 Disable"]
pub type ACD1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR1A_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Analog Comparator Interrupt Mode Select"]
    #[inline(always)]
    pub fn acis1(&self) -> ACIS1_R {
        ACIS1_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Analog Comparator 1 Input Capture Enable"]
    #[inline(always)]
    pub fn acic1(&self) -> ACIC1_R {
        ACIC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator 1 Interrupt Enable"]
    #[inline(always)]
    pub fn acie1(&self) -> ACIE1_R {
        ACIE1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator 1 Interrupt Flag"]
    #[inline(always)]
    pub fn aci1(&self) -> ACI1_R {
        ACI1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Comparator 1 Output"]
    #[inline(always)]
    pub fn aco1(&self) -> ACO1_R {
        ACO1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator 1 Bandgap Select"]
    #[inline(always)]
    pub fn acbg1(&self) -> ACBG1_R {
        ACBG1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 1 Disable"]
    #[inline(always)]
    pub fn acd1(&self) -> ACD1_R {
        ACD1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Comparator Interrupt Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn acis1(&mut self) -> ACIS1_W<0> {
        ACIS1_W::new(self)
    }
    #[doc = "Bit 2 - Analog Comparator 1 Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acic1(&mut self) -> ACIC1_W<2> {
        ACIC1_W::new(self)
    }
    #[doc = "Bit 3 - Analog Comparator 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acie1(&mut self) -> ACIE1_W<3> {
        ACIE1_W::new(self)
    }
    #[doc = "Bit 4 - Analog Comparator 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aci1(&mut self) -> ACI1_W<4> {
        ACI1_W::new(self)
    }
    #[doc = "Bit 5 - Analog Comparator 1 Output"]
    #[inline(always)]
    #[must_use]
    pub fn aco1(&mut self) -> ACO1_W<5> {
        ACO1_W::new(self)
    }
    #[doc = "Bit 6 - Analog Comparator 1 Bandgap Select"]
    #[inline(always)]
    #[must_use]
    pub fn acbg1(&mut self) -> ACBG1_W<6> {
        ACBG1_W::new(self)
    }
    #[doc = "Bit 7 - Analog Comparator 1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acd1(&mut self) -> ACD1_W<7> {
        ACD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator 1 Control And Status Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acsr1a](index.html) module"]
pub struct ACSR1A_SPEC;
impl crate::RegisterSpec for ACSR1A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [acsr1a::R](R) reader structure"]
impl crate::Readable for ACSR1A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acsr1a::W](W) writer structure"]
impl crate::Writable for ACSR1A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR1A to value 0"]
impl crate::Resettable for ACSR1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
