#[doc = "Register `TWSR` reader"]
pub struct R(crate::R<TWSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWSR` writer"]
pub struct W(crate::W<TWSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWSR_SPEC>;
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
impl From<crate::W<TWSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWPS` reader - TWI Prescaler Bits"]
pub type TWPS_R = crate::FieldReader<u8, TWPS_A>;
#[doc = "TWI Prescaler Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWPS_A {
    #[doc = "0: Prescaler Value 1"]
    PRESCALER_1 = 0,
    #[doc = "1: Prescaler Value 4"]
    PRESCALER_4 = 1,
    #[doc = "2: Prescaler Value 16"]
    PRESCALER_16 = 2,
    #[doc = "3: Prescaler Value 64"]
    PRESCALER_64 = 3,
}
impl From<TWPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TWPS_A) -> Self {
        variant as _
    }
}
impl TWPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWPS_A {
        match self.bits {
            0 => TWPS_A::PRESCALER_1,
            1 => TWPS_A::PRESCALER_4,
            2 => TWPS_A::PRESCALER_16,
            3 => TWPS_A::PRESCALER_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER_1`"]
    #[inline(always)]
    pub fn is_prescaler_1(&self) -> bool {
        *self == TWPS_A::PRESCALER_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER_4`"]
    #[inline(always)]
    pub fn is_prescaler_4(&self) -> bool {
        *self == TWPS_A::PRESCALER_4
    }
    #[doc = "Checks if the value of the field is `PRESCALER_16`"]
    #[inline(always)]
    pub fn is_prescaler_16(&self) -> bool {
        *self == TWPS_A::PRESCALER_16
    }
    #[doc = "Checks if the value of the field is `PRESCALER_64`"]
    #[inline(always)]
    pub fn is_prescaler_64(&self) -> bool {
        *self == TWPS_A::PRESCALER_64
    }
}
#[doc = "Field `TWPS` writer - TWI Prescaler Bits"]
pub type TWPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TWSR_SPEC, u8, TWPS_A, 2, O>;
impl<'a, const O: u8> TWPS_W<'a, O> {
    #[doc = "Prescaler Value 1"]
    #[inline(always)]
    pub fn prescaler_1(self) -> &'a mut W {
        self.variant(TWPS_A::PRESCALER_1)
    }
    #[doc = "Prescaler Value 4"]
    #[inline(always)]
    pub fn prescaler_4(self) -> &'a mut W {
        self.variant(TWPS_A::PRESCALER_4)
    }
    #[doc = "Prescaler Value 16"]
    #[inline(always)]
    pub fn prescaler_16(self) -> &'a mut W {
        self.variant(TWPS_A::PRESCALER_16)
    }
    #[doc = "Prescaler Value 64"]
    #[inline(always)]
    pub fn prescaler_64(self) -> &'a mut W {
        self.variant(TWPS_A::PRESCALER_64)
    }
}
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::BitReader<bool>;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWSR_SPEC, bool, O>;
#[doc = "Field `TWS` reader - TWI Status"]
pub type TWS_R = crate::FieldReader<u8, TWS_A>;
#[doc = "TWI Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWS_A {
    #[doc = "0: Bus error due to illegal START or STOP condition."]
    BUS_ERROR_DUE_TO_ILLEGAL_START_OR_STOP_CONDITION = 0,
    #[doc = "8: A START condition has been transmitted."]
    A_START_CONDITION_HAS_BEEN_TRANSMITTED = 8,
    #[doc = "16: A repeated START condition has been transmitted."]
    A_REPEATED_START_CONDITION_HAS_BEEN_TRANSMITTED = 16,
    #[doc = "24: SLA+W has been transmitted; ACK has been received."]
    SLA_W_HAS_BEEN_TRANSMITTED_ACK_HAS_BEEN_RECEIVED = 24,
}
impl From<TWS_A> for u8 {
    #[inline(always)]
    fn from(variant: TWS_A) -> Self {
        variant as _
    }
}
impl TWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TWS_A> {
        match self.bits {
            0 => Some(TWS_A::BUS_ERROR_DUE_TO_ILLEGAL_START_OR_STOP_CONDITION),
            8 => Some(TWS_A::A_START_CONDITION_HAS_BEEN_TRANSMITTED),
            16 => Some(TWS_A::A_REPEATED_START_CONDITION_HAS_BEEN_TRANSMITTED),
            24 => Some(TWS_A::SLA_W_HAS_BEEN_TRANSMITTED_ACK_HAS_BEEN_RECEIVED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR_DUE_TO_ILLEGAL_START_OR_STOP_CONDITION`"]
    #[inline(always)]
    pub fn is_bus_error_due_to_illegal_start_or_stop_condition(&self) -> bool {
        *self == TWS_A::BUS_ERROR_DUE_TO_ILLEGAL_START_OR_STOP_CONDITION
    }
    #[doc = "Checks if the value of the field is `A_START_CONDITION_HAS_BEEN_TRANSMITTED`"]
    #[inline(always)]
    pub fn is_a_start_condition_has_been_transmitted(&self) -> bool {
        *self == TWS_A::A_START_CONDITION_HAS_BEEN_TRANSMITTED
    }
    #[doc = "Checks if the value of the field is `A_REPEATED_START_CONDITION_HAS_BEEN_TRANSMITTED`"]
    #[inline(always)]
    pub fn is_a_repeated_start_condition_has_been_transmitted(&self) -> bool {
        *self == TWS_A::A_REPEATED_START_CONDITION_HAS_BEEN_TRANSMITTED
    }
    #[doc = "Checks if the value of the field is `SLA_W_HAS_BEEN_TRANSMITTED_ACK_HAS_BEEN_RECEIVED`"]
    #[inline(always)]
    pub fn is_sla_w_has_been_transmitted_ack_has_been_received(&self) -> bool {
        *self == TWS_A::SLA_W_HAS_BEEN_TRANSMITTED_ACK_HAS_BEEN_RECEIVED
    }
}
impl R {
    #[doc = "Bits 0:1 - TWI Prescaler Bits"]
    #[inline(always)]
    pub fn twps(&self) -> TWPS_R {
        TWPS_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - TWI Status"]
    #[inline(always)]
    pub fn tws(&self) -> TWS_R {
        TWS_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:1 - TWI Prescaler Bits"]
    #[inline(always)]
    #[must_use]
    pub fn twps(&mut self) -> TWPS_W<0> {
        TWPS_W::new(self)
    }
    #[doc = "Bit 2 - Reserved Bit"]
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
#[doc = "TWI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twsr](index.html) module"]
pub struct TWSR_SPEC;
impl crate::RegisterSpec for TWSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twsr::R](R) reader structure"]
impl crate::Readable for TWSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twsr::W](W) writer structure"]
impl crate::Writable for TWSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSR to value 0"]
impl crate::Resettable for TWSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
