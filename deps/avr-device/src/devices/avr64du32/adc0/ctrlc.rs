#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: VDD"]
    VDD = 0,
    #[doc = "2: VREFA"]
    VREFA = 2,
    #[doc = "4: 1.024V"]
    _1V024 = 4,
    #[doc = "5: 2.048V"]
    _2V048 = 5,
    #[doc = "6: 4.096V"]
    _4V096 = 6,
    #[doc = "7: 2.5V"]
    _2V500 = 7,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::VDD),
            2 => Some(REFSEL_A::VREFA),
            4 => Some(REFSEL_A::_1V024),
            5 => Some(REFSEL_A::_2V048),
            6 => Some(REFSEL_A::_4V096),
            7 => Some(REFSEL_A::_2V500),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REFSEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `VREFA`"]
    #[inline(always)]
    pub fn is_vrefa(&self) -> bool {
        *self == REFSEL_A::VREFA
    }
    #[doc = "Checks if the value of the field is `_1V024`"]
    #[inline(always)]
    pub fn is_1v024(&self) -> bool {
        *self == REFSEL_A::_1V024
    }
    #[doc = "Checks if the value of the field is `_2V048`"]
    #[inline(always)]
    pub fn is_2v048(&self) -> bool {
        *self == REFSEL_A::_2V048
    }
    #[doc = "Checks if the value of the field is `_4V096`"]
    #[inline(always)]
    pub fn is_4v096(&self) -> bool {
        *self == REFSEL_A::_4V096
    }
    #[doc = "Checks if the value of the field is `_2V500`"]
    #[inline(always)]
    pub fn is_2v500(&self) -> bool {
        *self == REFSEL_A::_2V500
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLC_SPEC, u8, REFSEL_A, 3, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::VDD)
    }
    #[doc = "VREFA"]
    #[inline(always)]
    pub fn vrefa(self) -> &'a mut W {
        self.variant(REFSEL_A::VREFA)
    }
    #[doc = "1.024V"]
    #[inline(always)]
    pub fn _1v024(self) -> &'a mut W {
        self.variant(REFSEL_A::_1V024)
    }
    #[doc = "2.048V"]
    #[inline(always)]
    pub fn _2v048(self) -> &'a mut W {
        self.variant(REFSEL_A::_2V048)
    }
    #[doc = "4.096V"]
    #[inline(always)]
    pub fn _4v096(self) -> &'a mut W {
        self.variant(REFSEL_A::_4V096)
    }
    #[doc = "2.5V"]
    #[inline(always)]
    pub fn _2v500(self) -> &'a mut W {
        self.variant(REFSEL_A::_2V500)
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<0> {
        REFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
