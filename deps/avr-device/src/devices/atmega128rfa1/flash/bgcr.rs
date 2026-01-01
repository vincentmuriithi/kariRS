#[doc = "Register `BGCR` reader"]
pub struct R(crate::R<BGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGCR` writer"]
pub struct W(crate::W<BGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGCR_SPEC>;
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
impl From<crate::W<BGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGCAL` reader - Coarse Calibration Bits"]
pub type BGCAL_R = crate::FieldReader<u8, BGCAL_A>;
#[doc = "Coarse Calibration Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BGCAL_A {
    #[doc = "0: Setting for highest voltage"]
    SETTING_FOR_HIGHEST_VOLTAGE = 0,
    #[doc = "3: Voltage step up"]
    VOLTAGE_STEP_UP = 3,
    #[doc = "4: Center value"]
    CENTER_VALUE = 4,
    #[doc = "5: Voltage step down"]
    VOLTAGE_STEP_DOWN = 5,
    #[doc = "7: Setting for lowest voltage"]
    SETTING_FOR_LOWEST_VOLTAGE = 7,
}
impl From<BGCAL_A> for u8 {
    #[inline(always)]
    fn from(variant: BGCAL_A) -> Self {
        variant as _
    }
}
impl BGCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BGCAL_A> {
        match self.bits {
            0 => Some(BGCAL_A::SETTING_FOR_HIGHEST_VOLTAGE),
            3 => Some(BGCAL_A::VOLTAGE_STEP_UP),
            4 => Some(BGCAL_A::CENTER_VALUE),
            5 => Some(BGCAL_A::VOLTAGE_STEP_DOWN),
            7 => Some(BGCAL_A::SETTING_FOR_LOWEST_VOLTAGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SETTING_FOR_HIGHEST_VOLTAGE`"]
    #[inline(always)]
    pub fn is_setting_for_highest_voltage(&self) -> bool {
        *self == BGCAL_A::SETTING_FOR_HIGHEST_VOLTAGE
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_STEP_UP`"]
    #[inline(always)]
    pub fn is_voltage_step_up(&self) -> bool {
        *self == BGCAL_A::VOLTAGE_STEP_UP
    }
    #[doc = "Checks if the value of the field is `CENTER_VALUE`"]
    #[inline(always)]
    pub fn is_center_value(&self) -> bool {
        *self == BGCAL_A::CENTER_VALUE
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_STEP_DOWN`"]
    #[inline(always)]
    pub fn is_voltage_step_down(&self) -> bool {
        *self == BGCAL_A::VOLTAGE_STEP_DOWN
    }
    #[doc = "Checks if the value of the field is `SETTING_FOR_LOWEST_VOLTAGE`"]
    #[inline(always)]
    pub fn is_setting_for_lowest_voltage(&self) -> bool {
        *self == BGCAL_A::SETTING_FOR_LOWEST_VOLTAGE
    }
}
#[doc = "Field `BGCAL` writer - Coarse Calibration Bits"]
pub type BGCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BGCR_SPEC, u8, BGCAL_A, 3, O>;
impl<'a, const O: u8> BGCAL_W<'a, O> {
    #[doc = "Setting for highest voltage"]
    #[inline(always)]
    pub fn setting_for_highest_voltage(self) -> &'a mut W {
        self.variant(BGCAL_A::SETTING_FOR_HIGHEST_VOLTAGE)
    }
    #[doc = "Voltage step up"]
    #[inline(always)]
    pub fn voltage_step_up(self) -> &'a mut W {
        self.variant(BGCAL_A::VOLTAGE_STEP_UP)
    }
    #[doc = "Center value"]
    #[inline(always)]
    pub fn center_value(self) -> &'a mut W {
        self.variant(BGCAL_A::CENTER_VALUE)
    }
    #[doc = "Voltage step down"]
    #[inline(always)]
    pub fn voltage_step_down(self) -> &'a mut W {
        self.variant(BGCAL_A::VOLTAGE_STEP_DOWN)
    }
    #[doc = "Setting for lowest voltage"]
    #[inline(always)]
    pub fn setting_for_lowest_voltage(self) -> &'a mut W {
        self.variant(BGCAL_A::SETTING_FOR_LOWEST_VOLTAGE)
    }
}
#[doc = "Field `BGCAL_FINE` reader - Fine Calibration Bits"]
pub type BGCAL_FINE_R = crate::FieldReader<u8, BGCAL_FINE_A>;
#[doc = "Fine Calibration Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BGCAL_FINE_A {
    #[doc = "0: Center value"]
    CENTER_VALUE = 0,
    #[doc = "1: Voltage step up"]
    VOLTAGE_STEP_UP = 1,
    #[doc = "7: Setting for highest voltage"]
    SETTING_FOR_HIGHEST_VOLTAGE = 7,
    #[doc = "8: Voltage step down"]
    VOLTAGE_STEP_DOWN = 8,
    #[doc = "15: Setting for lowest voltage"]
    SETTING_FOR_LOWEST_VOLTAGE = 15,
}
impl From<BGCAL_FINE_A> for u8 {
    #[inline(always)]
    fn from(variant: BGCAL_FINE_A) -> Self {
        variant as _
    }
}
impl BGCAL_FINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BGCAL_FINE_A> {
        match self.bits {
            0 => Some(BGCAL_FINE_A::CENTER_VALUE),
            1 => Some(BGCAL_FINE_A::VOLTAGE_STEP_UP),
            7 => Some(BGCAL_FINE_A::SETTING_FOR_HIGHEST_VOLTAGE),
            8 => Some(BGCAL_FINE_A::VOLTAGE_STEP_DOWN),
            15 => Some(BGCAL_FINE_A::SETTING_FOR_LOWEST_VOLTAGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CENTER_VALUE`"]
    #[inline(always)]
    pub fn is_center_value(&self) -> bool {
        *self == BGCAL_FINE_A::CENTER_VALUE
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_STEP_UP`"]
    #[inline(always)]
    pub fn is_voltage_step_up(&self) -> bool {
        *self == BGCAL_FINE_A::VOLTAGE_STEP_UP
    }
    #[doc = "Checks if the value of the field is `SETTING_FOR_HIGHEST_VOLTAGE`"]
    #[inline(always)]
    pub fn is_setting_for_highest_voltage(&self) -> bool {
        *self == BGCAL_FINE_A::SETTING_FOR_HIGHEST_VOLTAGE
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_STEP_DOWN`"]
    #[inline(always)]
    pub fn is_voltage_step_down(&self) -> bool {
        *self == BGCAL_FINE_A::VOLTAGE_STEP_DOWN
    }
    #[doc = "Checks if the value of the field is `SETTING_FOR_LOWEST_VOLTAGE`"]
    #[inline(always)]
    pub fn is_setting_for_lowest_voltage(&self) -> bool {
        *self == BGCAL_FINE_A::SETTING_FOR_LOWEST_VOLTAGE
    }
}
#[doc = "Field `BGCAL_FINE` writer - Fine Calibration Bits"]
pub type BGCAL_FINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, BGCR_SPEC, u8, BGCAL_FINE_A, 4, O>;
impl<'a, const O: u8> BGCAL_FINE_W<'a, O> {
    #[doc = "Center value"]
    #[inline(always)]
    pub fn center_value(self) -> &'a mut W {
        self.variant(BGCAL_FINE_A::CENTER_VALUE)
    }
    #[doc = "Voltage step up"]
    #[inline(always)]
    pub fn voltage_step_up(self) -> &'a mut W {
        self.variant(BGCAL_FINE_A::VOLTAGE_STEP_UP)
    }
    #[doc = "Setting for highest voltage"]
    #[inline(always)]
    pub fn setting_for_highest_voltage(self) -> &'a mut W {
        self.variant(BGCAL_FINE_A::SETTING_FOR_HIGHEST_VOLTAGE)
    }
    #[doc = "Voltage step down"]
    #[inline(always)]
    pub fn voltage_step_down(self) -> &'a mut W {
        self.variant(BGCAL_FINE_A::VOLTAGE_STEP_DOWN)
    }
    #[doc = "Setting for lowest voltage"]
    #[inline(always)]
    pub fn setting_for_lowest_voltage(self) -> &'a mut W {
        self.variant(BGCAL_FINE_A::SETTING_FOR_LOWEST_VOLTAGE)
    }
}
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::BitReader<bool>;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, const O: u8> = crate::BitWriter<'a, u8, BGCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Coarse Calibration Bits"]
    #[inline(always)]
    pub fn bgcal(&self) -> BGCAL_R {
        BGCAL_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:6 - Fine Calibration Bits"]
    #[inline(always)]
    pub fn bgcal_fine(&self) -> BGCAL_FINE_R {
        BGCAL_FINE_R::new((self.bits >> 3) & 0x0f)
    }
    #[doc = "Bit 7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Coarse Calibration Bits"]
    #[inline(always)]
    #[must_use]
    pub fn bgcal(&mut self) -> BGCAL_W<0> {
        BGCAL_W::new(self)
    }
    #[doc = "Bits 3:6 - Fine Calibration Bits"]
    #[inline(always)]
    #[must_use]
    pub fn bgcal_fine(&mut self) -> BGCAL_FINE_W<3> {
        BGCAL_FINE_W::new(self)
    }
    #[doc = "Bit 7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<7> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference Voltage Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcr](index.html) module"]
pub struct BGCR_SPEC;
impl crate::RegisterSpec for BGCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bgcr::R](R) reader structure"]
impl crate::Readable for BGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgcr::W](W) writer structure"]
impl crate::Writable for BGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BGCR to value 0"]
impl crate::Resettable for BGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
