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
#[doc = "Field `LVL` reader - Bod level"]
pub type LVL_R = crate::FieldReader<u8, LVL_A>;
#[doc = "Bod level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVL_A {
    #[doc = "0: 1.8 V"]
    BODLEVEL0 = 0,
    #[doc = "1: 2.1 V"]
    BODLEVEL1 = 1,
    #[doc = "2: 2.6 V"]
    BODLEVEL2 = 2,
    #[doc = "3: 2.9 V"]
    BODLEVEL3 = 3,
    #[doc = "4: 3.3 V"]
    BODLEVEL4 = 4,
    #[doc = "5: 3.7 V"]
    BODLEVEL5 = 5,
    #[doc = "6: 4.0 V"]
    BODLEVEL6 = 6,
    #[doc = "7: 4.2 V"]
    BODLEVEL7 = 7,
}
impl From<LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVL_A) -> Self {
        variant as _
    }
}
impl LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVL_A {
        match self.bits {
            0 => LVL_A::BODLEVEL0,
            1 => LVL_A::BODLEVEL1,
            2 => LVL_A::BODLEVEL2,
            3 => LVL_A::BODLEVEL3,
            4 => LVL_A::BODLEVEL4,
            5 => LVL_A::BODLEVEL5,
            6 => LVL_A::BODLEVEL6,
            7 => LVL_A::BODLEVEL7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BODLEVEL0`"]
    #[inline(always)]
    pub fn is_bodlevel0(&self) -> bool {
        *self == LVL_A::BODLEVEL0
    }
    #[doc = "Checks if the value of the field is `BODLEVEL1`"]
    #[inline(always)]
    pub fn is_bodlevel1(&self) -> bool {
        *self == LVL_A::BODLEVEL1
    }
    #[doc = "Checks if the value of the field is `BODLEVEL2`"]
    #[inline(always)]
    pub fn is_bodlevel2(&self) -> bool {
        *self == LVL_A::BODLEVEL2
    }
    #[doc = "Checks if the value of the field is `BODLEVEL3`"]
    #[inline(always)]
    pub fn is_bodlevel3(&self) -> bool {
        *self == LVL_A::BODLEVEL3
    }
    #[doc = "Checks if the value of the field is `BODLEVEL4`"]
    #[inline(always)]
    pub fn is_bodlevel4(&self) -> bool {
        *self == LVL_A::BODLEVEL4
    }
    #[doc = "Checks if the value of the field is `BODLEVEL5`"]
    #[inline(always)]
    pub fn is_bodlevel5(&self) -> bool {
        *self == LVL_A::BODLEVEL5
    }
    #[doc = "Checks if the value of the field is `BODLEVEL6`"]
    #[inline(always)]
    pub fn is_bodlevel6(&self) -> bool {
        *self == LVL_A::BODLEVEL6
    }
    #[doc = "Checks if the value of the field is `BODLEVEL7`"]
    #[inline(always)]
    pub fn is_bodlevel7(&self) -> bool {
        *self == LVL_A::BODLEVEL7
    }
}
impl R {
    #[doc = "Bits 0:2 - Bod level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(self.bits & 7)
    }
}
impl W {
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
