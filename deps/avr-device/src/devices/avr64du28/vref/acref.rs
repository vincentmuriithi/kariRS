#[doc = "Register `ACREF` reader"]
pub struct R(crate::R<ACREF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACREF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACREF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACREF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACREF` writer"]
pub struct W(crate::W<ACREF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACREF_SPEC>;
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
impl From<crate::W<ACREF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACREF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFSEL` reader - Reference select"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal 1.024V reference"]
    _1V024 = 0,
    #[doc = "1: Internal 2.048V reference"]
    _2V048 = 1,
    #[doc = "2: Internal 4.096V reference"]
    _4V096 = 2,
    #[doc = "3: Internal 2.500V reference"]
    _2V500 = 3,
    #[doc = "5: VDD as reference"]
    VDD = 5,
    #[doc = "6: External reference on VREFA pin"]
    VREFA = 6,
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
            0 => Some(REFSEL_A::_1V024),
            1 => Some(REFSEL_A::_2V048),
            2 => Some(REFSEL_A::_4V096),
            3 => Some(REFSEL_A::_2V500),
            5 => Some(REFSEL_A::VDD),
            6 => Some(REFSEL_A::VREFA),
            _ => None,
        }
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
}
#[doc = "Field `REFSEL` writer - Reference select"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ACREF_SPEC, u8, REFSEL_A, 3, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Internal 1.024V reference"]
    #[inline(always)]
    pub fn _1v024(self) -> &'a mut W {
        self.variant(REFSEL_A::_1V024)
    }
    #[doc = "Internal 2.048V reference"]
    #[inline(always)]
    pub fn _2v048(self) -> &'a mut W {
        self.variant(REFSEL_A::_2V048)
    }
    #[doc = "Internal 4.096V reference"]
    #[inline(always)]
    pub fn _4v096(self) -> &'a mut W {
        self.variant(REFSEL_A::_4V096)
    }
    #[doc = "Internal 2.500V reference"]
    #[inline(always)]
    pub fn _2v500(self) -> &'a mut W {
        self.variant(REFSEL_A::_2V500)
    }
    #[doc = "VDD as reference"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::VDD)
    }
    #[doc = "External reference on VREFA pin"]
    #[inline(always)]
    pub fn vrefa(self) -> &'a mut W {
        self.variant(REFSEL_A::VREFA)
    }
}
#[doc = "Field `ALWAYSON` reader - Always on"]
pub type ALWAYSON_R = crate::BitReader<bool>;
#[doc = "Field `ALWAYSON` writer - Always on"]
pub type ALWAYSON_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACREF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Always on"]
    #[inline(always)]
    pub fn alwayson(&self) -> ALWAYSON_R {
        ALWAYSON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<0> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 7 - Always on"]
    #[inline(always)]
    #[must_use]
    pub fn alwayson(&mut self) -> ALWAYSON_W<7> {
        ALWAYSON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0 Reference\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acref](index.html) module"]
pub struct ACREF_SPEC;
impl crate::RegisterSpec for ACREF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [acref::R](R) reader structure"]
impl crate::Readable for ACREF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acref::W](W) writer structure"]
impl crate::Writable for ACREF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACREF to value 0"]
impl crate::Resettable for ACREF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
