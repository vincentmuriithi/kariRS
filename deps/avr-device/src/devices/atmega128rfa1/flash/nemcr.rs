#[doc = "Register `NEMCR` reader"]
pub struct R(crate::R<NEMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NEMCR` writer"]
pub struct W(crate::W<NEMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NEMCR_SPEC>;
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
impl From<crate::W<NEMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NEMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AEAM` reader - Address for Extended Address Mode of Extra Rows"]
pub type AEAM_R = crate::FieldReader<u8, AEAM_A>;
#[doc = "Address for Extended Address Mode of Extra Rows\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AEAM_A {
    #[doc = "0: Factory Row"]
    FACTORY_ROW = 0,
    #[doc = "1: User Row 1"]
    USER_ROW_1 = 1,
    #[doc = "2: User Row 2"]
    USER_ROW_2 = 2,
    #[doc = "3: User Row 3"]
    USER_ROW_3 = 3,
}
impl From<AEAM_A> for u8 {
    #[inline(always)]
    fn from(variant: AEAM_A) -> Self {
        variant as _
    }
}
impl AEAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEAM_A {
        match self.bits {
            0 => AEAM_A::FACTORY_ROW,
            1 => AEAM_A::USER_ROW_1,
            2 => AEAM_A::USER_ROW_2,
            3 => AEAM_A::USER_ROW_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FACTORY_ROW`"]
    #[inline(always)]
    pub fn is_factory_row(&self) -> bool {
        *self == AEAM_A::FACTORY_ROW
    }
    #[doc = "Checks if the value of the field is `USER_ROW_1`"]
    #[inline(always)]
    pub fn is_user_row_1(&self) -> bool {
        *self == AEAM_A::USER_ROW_1
    }
    #[doc = "Checks if the value of the field is `USER_ROW_2`"]
    #[inline(always)]
    pub fn is_user_row_2(&self) -> bool {
        *self == AEAM_A::USER_ROW_2
    }
    #[doc = "Checks if the value of the field is `USER_ROW_3`"]
    #[inline(always)]
    pub fn is_user_row_3(&self) -> bool {
        *self == AEAM_A::USER_ROW_3
    }
}
#[doc = "Field `AEAM` writer - Address for Extended Address Mode of Extra Rows"]
pub type AEAM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, NEMCR_SPEC, u8, AEAM_A, 2, O>;
impl<'a, const O: u8> AEAM_W<'a, O> {
    #[doc = "Factory Row"]
    #[inline(always)]
    pub fn factory_row(self) -> &'a mut W {
        self.variant(AEAM_A::FACTORY_ROW)
    }
    #[doc = "User Row 1"]
    #[inline(always)]
    pub fn user_row_1(self) -> &'a mut W {
        self.variant(AEAM_A::USER_ROW_1)
    }
    #[doc = "User Row 2"]
    #[inline(always)]
    pub fn user_row_2(self) -> &'a mut W {
        self.variant(AEAM_A::USER_ROW_2)
    }
    #[doc = "User Row 3"]
    #[inline(always)]
    pub fn user_row_3(self) -> &'a mut W {
        self.variant(AEAM_A::USER_ROW_3)
    }
}
#[doc = "Field `ENEAM` reader - Enable Extended Address Mode for Extra Rows"]
pub type ENEAM_R = crate::BitReader<bool>;
#[doc = "Field `ENEAM` writer - Enable Extended Address Mode for Extra Rows"]
pub type ENEAM_W<'a, const O: u8> = crate::BitWriter<'a, u8, NEMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:5 - Address for Extended Address Mode of Extra Rows"]
    #[inline(always)]
    pub fn aeam(&self) -> AEAM_R {
        AEAM_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Enable Extended Address Mode for Extra Rows"]
    #[inline(always)]
    pub fn eneam(&self) -> ENEAM_R {
        ENEAM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Address for Extended Address Mode of Extra Rows"]
    #[inline(always)]
    #[must_use]
    pub fn aeam(&mut self) -> AEAM_W<4> {
        AEAM_W::new(self)
    }
    #[doc = "Bit 6 - Enable Extended Address Mode for Extra Rows"]
    #[inline(always)]
    #[must_use]
    pub fn eneam(&mut self) -> ENEAM_W<6> {
        ENEAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Extended-Mode Control-Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nemcr](index.html) module"]
pub struct NEMCR_SPEC;
impl crate::RegisterSpec for NEMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nemcr::R](R) reader structure"]
impl crate::Readable for NEMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nemcr::W](W) writer structure"]
impl crate::Writable for NEMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NEMCR to value 0"]
impl crate::Resettable for NEMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
