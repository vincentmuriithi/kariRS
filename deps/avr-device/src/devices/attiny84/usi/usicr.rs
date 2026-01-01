#[doc = "Register `USICR` reader"]
pub struct R(crate::R<USICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USICR` writer"]
pub struct W(crate::W<USICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USICR_SPEC>;
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
impl From<crate::W<USICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USITC` writer - Toggle Clock Port Pin"]
pub type USITC_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICR_SPEC, bool, O>;
#[doc = "Field `USICLK` writer - Clock Strobe"]
pub type USICLK_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICR_SPEC, bool, O>;
#[doc = "Field `USICS` reader - USI Clock Source Select Bits"]
pub type USICS_R = crate::FieldReader<u8, USICS_A>;
#[doc = "USI Clock Source Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USICS_A {
    #[doc = "0: No Clock/Software clock strobe"]
    NO_CLOCK = 0,
    #[doc = "1: Timer/Counter0 Compare Match"]
    TC0 = 1,
    #[doc = "2: External, positive edge"]
    EXT_POS = 2,
    #[doc = "3: External, negative edge"]
    EXT_NEG = 3,
}
impl From<USICS_A> for u8 {
    #[inline(always)]
    fn from(variant: USICS_A) -> Self {
        variant as _
    }
}
impl USICS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USICS_A {
        match self.bits {
            0 => USICS_A::NO_CLOCK,
            1 => USICS_A::TC0,
            2 => USICS_A::EXT_POS,
            3 => USICS_A::EXT_NEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == USICS_A::NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `TC0`"]
    #[inline(always)]
    pub fn is_tc0(&self) -> bool {
        *self == USICS_A::TC0
    }
    #[doc = "Checks if the value of the field is `EXT_POS`"]
    #[inline(always)]
    pub fn is_ext_pos(&self) -> bool {
        *self == USICS_A::EXT_POS
    }
    #[doc = "Checks if the value of the field is `EXT_NEG`"]
    #[inline(always)]
    pub fn is_ext_neg(&self) -> bool {
        *self == USICS_A::EXT_NEG
    }
}
#[doc = "Field `USICS` writer - USI Clock Source Select Bits"]
pub type USICS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, USICR_SPEC, u8, USICS_A, 2, O>;
impl<'a, const O: u8> USICS_W<'a, O> {
    #[doc = "No Clock/Software clock strobe"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(USICS_A::NO_CLOCK)
    }
    #[doc = "Timer/Counter0 Compare Match"]
    #[inline(always)]
    pub fn tc0(self) -> &'a mut W {
        self.variant(USICS_A::TC0)
    }
    #[doc = "External, positive edge"]
    #[inline(always)]
    pub fn ext_pos(self) -> &'a mut W {
        self.variant(USICS_A::EXT_POS)
    }
    #[doc = "External, negative edge"]
    #[inline(always)]
    pub fn ext_neg(self) -> &'a mut W {
        self.variant(USICS_A::EXT_NEG)
    }
}
#[doc = "Field `USIWM` reader - USI Wire Mode Bits"]
pub type USIWM_R = crate::FieldReader<u8, USIWM_A>;
#[doc = "USI Wire Mode Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USIWM_A {
    #[doc = "0: All detectors disabled. Port pins operates as normal."]
    DISABLED = 0,
    #[doc = "1: Three-wire mode. Uses DO, DI, and USCK pins."]
    THREE_WIRE = 1,
    #[doc = "2: Two-wire mode (Slave). Uses SDA (DI) and SCL (USCK) pins."]
    TWO_WIRE_SLAVE = 2,
    #[doc = "3: Two-wire mode (Master). Uses SDA and SCL pins."]
    TWO_WIRE_MASTER = 3,
}
impl From<USIWM_A> for u8 {
    #[inline(always)]
    fn from(variant: USIWM_A) -> Self {
        variant as _
    }
}
impl USIWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIWM_A {
        match self.bits {
            0 => USIWM_A::DISABLED,
            1 => USIWM_A::THREE_WIRE,
            2 => USIWM_A::TWO_WIRE_SLAVE,
            3 => USIWM_A::TWO_WIRE_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USIWM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `THREE_WIRE`"]
    #[inline(always)]
    pub fn is_three_wire(&self) -> bool {
        *self == USIWM_A::THREE_WIRE
    }
    #[doc = "Checks if the value of the field is `TWO_WIRE_SLAVE`"]
    #[inline(always)]
    pub fn is_two_wire_slave(&self) -> bool {
        *self == USIWM_A::TWO_WIRE_SLAVE
    }
    #[doc = "Checks if the value of the field is `TWO_WIRE_MASTER`"]
    #[inline(always)]
    pub fn is_two_wire_master(&self) -> bool {
        *self == USIWM_A::TWO_WIRE_MASTER
    }
}
#[doc = "Field `USIWM` writer - USI Wire Mode Bits"]
pub type USIWM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, USICR_SPEC, u8, USIWM_A, 2, O>;
impl<'a, const O: u8> USIWM_W<'a, O> {
    #[doc = "All detectors disabled. Port pins operates as normal."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USIWM_A::DISABLED)
    }
    #[doc = "Three-wire mode. Uses DO, DI, and USCK pins."]
    #[inline(always)]
    pub fn three_wire(self) -> &'a mut W {
        self.variant(USIWM_A::THREE_WIRE)
    }
    #[doc = "Two-wire mode (Slave). Uses SDA (DI) and SCL (USCK) pins."]
    #[inline(always)]
    pub fn two_wire_slave(self) -> &'a mut W {
        self.variant(USIWM_A::TWO_WIRE_SLAVE)
    }
    #[doc = "Two-wire mode (Master). Uses SDA and SCL pins."]
    #[inline(always)]
    pub fn two_wire_master(self) -> &'a mut W {
        self.variant(USIWM_A::TWO_WIRE_MASTER)
    }
}
#[doc = "Field `USIOIE` reader - Counter Overflow Interrupt Enable"]
pub type USIOIE_R = crate::BitReader<bool>;
#[doc = "Field `USIOIE` writer - Counter Overflow Interrupt Enable"]
pub type USIOIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICR_SPEC, bool, O>;
#[doc = "Field `USISIE` reader - Start Condition Interrupt Enable"]
pub type USISIE_R = crate::BitReader<bool>;
#[doc = "Field `USISIE` writer - Start Condition Interrupt Enable"]
pub type USISIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:3 - USI Clock Source Select Bits"]
    #[inline(always)]
    pub fn usics(&self) -> USICS_R {
        USICS_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - USI Wire Mode Bits"]
    #[inline(always)]
    pub fn usiwm(&self) -> USIWM_R {
        USIWM_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn usioie(&self) -> USIOIE_R {
        USIOIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start Condition Interrupt Enable"]
    #[inline(always)]
    pub fn usisie(&self) -> USISIE_R {
        USISIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Toggle Clock Port Pin"]
    #[inline(always)]
    #[must_use]
    pub fn usitc(&mut self) -> USITC_W<0> {
        USITC_W::new(self)
    }
    #[doc = "Bit 1 - Clock Strobe"]
    #[inline(always)]
    #[must_use]
    pub fn usiclk(&mut self) -> USICLK_W<1> {
        USICLK_W::new(self)
    }
    #[doc = "Bits 2:3 - USI Clock Source Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn usics(&mut self) -> USICS_W<2> {
        USICS_W::new(self)
    }
    #[doc = "Bits 4:5 - USI Wire Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn usiwm(&mut self) -> USIWM_W<4> {
        USIWM_W::new(self)
    }
    #[doc = "Bit 6 - Counter Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usioie(&mut self) -> USIOIE_W<6> {
        USIOIE_W::new(self)
    }
    #[doc = "Bit 7 - Start Condition Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usisie(&mut self) -> USISIE_W<7> {
        USISIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usicr](index.html) module"]
pub struct USICR_SPEC;
impl crate::RegisterSpec for USICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usicr::R](R) reader structure"]
impl crate::Readable for USICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usicr::W](W) writer structure"]
impl crate::Writable for USICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USICR to value 0"]
impl crate::Resettable for USICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
