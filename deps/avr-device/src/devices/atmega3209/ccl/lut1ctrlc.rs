#[doc = "Register `LUT1CTRLC` reader"]
pub struct R(crate::R<LUT1CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT1CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT1CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT1CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT1CTRLC` writer"]
pub struct W(crate::W<LUT1CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT1CTRLC_SPEC>;
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
impl From<crate::W<LUT1CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT1CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSEL2` reader - LUT Input 2 Source Selection"]
pub type INSEL2_R = crate::FieldReader<u8, INSEL2_A>;
#[doc = "LUT Input 2 Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INSEL2_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event input source A"]
    EVENTA = 3,
    #[doc = "4: Event input source B"]
    EVENTB = 4,
    #[doc = "5: IO pin LUTn-IN2 input source"]
    IO = 5,
    #[doc = "6: AC0 OUT input source"]
    AC0 = 6,
    #[doc = "8: USART2 TXD input source"]
    USART2 = 8,
    #[doc = "9: SPI0 SCK input source"]
    SPI0 = 9,
    #[doc = "10: TCA0 WO2 input source"]
    TCA0 = 10,
    #[doc = "12: TCB2 WO input source"]
    TCB2 = 12,
}
impl From<INSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL2_A) -> Self {
        variant as _
    }
}
impl INSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INSEL2_A> {
        match self.bits {
            0 => Some(INSEL2_A::MASK),
            1 => Some(INSEL2_A::FEEDBACK),
            2 => Some(INSEL2_A::LINK),
            3 => Some(INSEL2_A::EVENTA),
            4 => Some(INSEL2_A::EVENTB),
            5 => Some(INSEL2_A::IO),
            6 => Some(INSEL2_A::AC0),
            8 => Some(INSEL2_A::USART2),
            9 => Some(INSEL2_A::SPI0),
            10 => Some(INSEL2_A::TCA0),
            12 => Some(INSEL2_A::TCB2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL2_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL2_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL2_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENTA`"]
    #[inline(always)]
    pub fn is_eventa(&self) -> bool {
        *self == INSEL2_A::EVENTA
    }
    #[doc = "Checks if the value of the field is `EVENTB`"]
    #[inline(always)]
    pub fn is_eventb(&self) -> bool {
        *self == INSEL2_A::EVENTB
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL2_A::IO
    }
    #[doc = "Checks if the value of the field is `AC0`"]
    #[inline(always)]
    pub fn is_ac0(&self) -> bool {
        *self == INSEL2_A::AC0
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == INSEL2_A::USART2
    }
    #[doc = "Checks if the value of the field is `SPI0`"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == INSEL2_A::SPI0
    }
    #[doc = "Checks if the value of the field is `TCA0`"]
    #[inline(always)]
    pub fn is_tca0(&self) -> bool {
        *self == INSEL2_A::TCA0
    }
    #[doc = "Checks if the value of the field is `TCB2`"]
    #[inline(always)]
    pub fn is_tcb2(&self) -> bool {
        *self == INSEL2_A::TCB2
    }
}
#[doc = "Field `INSEL2` writer - LUT Input 2 Source Selection"]
pub type INSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LUT1CTRLC_SPEC, u8, INSEL2_A, 4, O>;
impl<'a, const O: u8> INSEL2_W<'a, O> {
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL2_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL2_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL2_A::LINK)
    }
    #[doc = "Event input source A"]
    #[inline(always)]
    pub fn eventa(self) -> &'a mut W {
        self.variant(INSEL2_A::EVENTA)
    }
    #[doc = "Event input source B"]
    #[inline(always)]
    pub fn eventb(self) -> &'a mut W {
        self.variant(INSEL2_A::EVENTB)
    }
    #[doc = "IO pin LUTn-IN2 input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL2_A::IO)
    }
    #[doc = "AC0 OUT input source"]
    #[inline(always)]
    pub fn ac0(self) -> &'a mut W {
        self.variant(INSEL2_A::AC0)
    }
    #[doc = "USART2 TXD input source"]
    #[inline(always)]
    pub fn usart2(self) -> &'a mut W {
        self.variant(INSEL2_A::USART2)
    }
    #[doc = "SPI0 SCK input source"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut W {
        self.variant(INSEL2_A::SPI0)
    }
    #[doc = "TCA0 WO2 input source"]
    #[inline(always)]
    pub fn tca0(self) -> &'a mut W {
        self.variant(INSEL2_A::TCA0)
    }
    #[doc = "TCB2 WO input source"]
    #[inline(always)]
    pub fn tcb2(self) -> &'a mut W {
        self.variant(INSEL2_A::TCB2)
    }
}
impl R {
    #[doc = "Bits 0:3 - LUT Input 2 Source Selection"]
    #[inline(always)]
    pub fn insel2(&self) -> INSEL2_R {
        INSEL2_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUT Input 2 Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel2(&mut self) -> INSEL2_W<0> {
        INSEL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT Control 1 C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut1ctrlc](index.html) module"]
pub struct LUT1CTRLC_SPEC;
impl crate::RegisterSpec for LUT1CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lut1ctrlc::R](R) reader structure"]
impl crate::Readable for LUT1CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut1ctrlc::W](W) writer structure"]
impl crate::Writable for LUT1CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT1CTRLC to value 0"]
impl crate::Resettable for LUT1CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
