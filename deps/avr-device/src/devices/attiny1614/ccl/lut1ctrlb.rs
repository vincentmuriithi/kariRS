#[doc = "Register `LUT1CTRLB` reader"]
pub struct R(crate::R<LUT1CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT1CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT1CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT1CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT1CTRLB` writer"]
pub struct W(crate::W<LUT1CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT1CTRLB_SPEC>;
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
impl From<crate::W<LUT1CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT1CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSEL0` reader - LUT Input 0 Source Selection"]
pub type INSEL0_R = crate::FieldReader<u8, INSEL0_A>;
#[doc = "LUT Input 0 Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INSEL0_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event input source 0"]
    EVENT0 = 3,
    #[doc = "4: Event input source 1"]
    EVENT1 = 4,
    #[doc = "5: IO pin LUTn-IN0 input source"]
    IO = 5,
    #[doc = "6: AC0 OUT input source"]
    AC0 = 6,
    #[doc = "7: TCB0 WO input source"]
    TCB0 = 7,
    #[doc = "8: TCA0 WO0 input source"]
    TCA0 = 8,
    #[doc = "9: TCD0 WOA input source"]
    TCD0 = 9,
    #[doc = "10: USART0 XCK input source"]
    USART0 = 10,
    #[doc = "11: SPI0 SCK source"]
    SPI0 = 11,
    #[doc = "12: AC1 OUT input source"]
    AC1 = 12,
    #[doc = "13: TCB1 WO input source"]
    TCB1 = 13,
    #[doc = "14: AC2 OUT input source"]
    AC2 = 14,
}
impl From<INSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL0_A) -> Self {
        variant as _
    }
}
impl INSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INSEL0_A> {
        match self.bits {
            0 => Some(INSEL0_A::MASK),
            1 => Some(INSEL0_A::FEEDBACK),
            2 => Some(INSEL0_A::LINK),
            3 => Some(INSEL0_A::EVENT0),
            4 => Some(INSEL0_A::EVENT1),
            5 => Some(INSEL0_A::IO),
            6 => Some(INSEL0_A::AC0),
            7 => Some(INSEL0_A::TCB0),
            8 => Some(INSEL0_A::TCA0),
            9 => Some(INSEL0_A::TCD0),
            10 => Some(INSEL0_A::USART0),
            11 => Some(INSEL0_A::SPI0),
            12 => Some(INSEL0_A::AC1),
            13 => Some(INSEL0_A::TCB1),
            14 => Some(INSEL0_A::AC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL0_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL0_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL0_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT0`"]
    #[inline(always)]
    pub fn is_event0(&self) -> bool {
        *self == INSEL0_A::EVENT0
    }
    #[doc = "Checks if the value of the field is `EVENT1`"]
    #[inline(always)]
    pub fn is_event1(&self) -> bool {
        *self == INSEL0_A::EVENT1
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL0_A::IO
    }
    #[doc = "Checks if the value of the field is `AC0`"]
    #[inline(always)]
    pub fn is_ac0(&self) -> bool {
        *self == INSEL0_A::AC0
    }
    #[doc = "Checks if the value of the field is `TCB0`"]
    #[inline(always)]
    pub fn is_tcb0(&self) -> bool {
        *self == INSEL0_A::TCB0
    }
    #[doc = "Checks if the value of the field is `TCA0`"]
    #[inline(always)]
    pub fn is_tca0(&self) -> bool {
        *self == INSEL0_A::TCA0
    }
    #[doc = "Checks if the value of the field is `TCD0`"]
    #[inline(always)]
    pub fn is_tcd0(&self) -> bool {
        *self == INSEL0_A::TCD0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == INSEL0_A::USART0
    }
    #[doc = "Checks if the value of the field is `SPI0`"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == INSEL0_A::SPI0
    }
    #[doc = "Checks if the value of the field is `AC1`"]
    #[inline(always)]
    pub fn is_ac1(&self) -> bool {
        *self == INSEL0_A::AC1
    }
    #[doc = "Checks if the value of the field is `TCB1`"]
    #[inline(always)]
    pub fn is_tcb1(&self) -> bool {
        *self == INSEL0_A::TCB1
    }
    #[doc = "Checks if the value of the field is `AC2`"]
    #[inline(always)]
    pub fn is_ac2(&self) -> bool {
        *self == INSEL0_A::AC2
    }
}
#[doc = "Field `INSEL0` writer - LUT Input 0 Source Selection"]
pub type INSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LUT1CTRLB_SPEC, u8, INSEL0_A, 4, O>;
impl<'a, const O: u8> INSEL0_W<'a, O> {
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL0_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL0_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL0_A::LINK)
    }
    #[doc = "Event input source 0"]
    #[inline(always)]
    pub fn event0(self) -> &'a mut W {
        self.variant(INSEL0_A::EVENT0)
    }
    #[doc = "Event input source 1"]
    #[inline(always)]
    pub fn event1(self) -> &'a mut W {
        self.variant(INSEL0_A::EVENT1)
    }
    #[doc = "IO pin LUTn-IN0 input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL0_A::IO)
    }
    #[doc = "AC0 OUT input source"]
    #[inline(always)]
    pub fn ac0(self) -> &'a mut W {
        self.variant(INSEL0_A::AC0)
    }
    #[doc = "TCB0 WO input source"]
    #[inline(always)]
    pub fn tcb0(self) -> &'a mut W {
        self.variant(INSEL0_A::TCB0)
    }
    #[doc = "TCA0 WO0 input source"]
    #[inline(always)]
    pub fn tca0(self) -> &'a mut W {
        self.variant(INSEL0_A::TCA0)
    }
    #[doc = "TCD0 WOA input source"]
    #[inline(always)]
    pub fn tcd0(self) -> &'a mut W {
        self.variant(INSEL0_A::TCD0)
    }
    #[doc = "USART0 XCK input source"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut W {
        self.variant(INSEL0_A::USART0)
    }
    #[doc = "SPI0 SCK source"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut W {
        self.variant(INSEL0_A::SPI0)
    }
    #[doc = "AC1 OUT input source"]
    #[inline(always)]
    pub fn ac1(self) -> &'a mut W {
        self.variant(INSEL0_A::AC1)
    }
    #[doc = "TCB1 WO input source"]
    #[inline(always)]
    pub fn tcb1(self) -> &'a mut W {
        self.variant(INSEL0_A::TCB1)
    }
    #[doc = "AC2 OUT input source"]
    #[inline(always)]
    pub fn ac2(self) -> &'a mut W {
        self.variant(INSEL0_A::AC2)
    }
}
#[doc = "Field `INSEL1` reader - LUT Input 1 Source Selection"]
pub type INSEL1_R = crate::FieldReader<u8, INSEL1_A>;
#[doc = "LUT Input 1 Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INSEL1_A {
    #[doc = "0: Masked input"]
    MASK = 0,
    #[doc = "1: Feedback input source"]
    FEEDBACK = 1,
    #[doc = "2: Linked LUT input source"]
    LINK = 2,
    #[doc = "3: Event input source 0"]
    EVENT0 = 3,
    #[doc = "4: Event input source 1"]
    EVENT1 = 4,
    #[doc = "5: IO pin LUTn-N1 input source"]
    IO = 5,
    #[doc = "6: AC0 OUT input source"]
    AC0 = 6,
    #[doc = "7: TCB0 WO input source"]
    TCB0 = 7,
    #[doc = "8: TCA0 WO1 input source"]
    TCA0 = 8,
    #[doc = "9: TCD0 WOB input source"]
    TCD0 = 9,
    #[doc = "10: USART0 TXD input source"]
    USART0 = 10,
    #[doc = "11: SPI0 MOSI input source"]
    SPI0 = 11,
    #[doc = "12: AC1 OUT input source"]
    AC1 = 12,
    #[doc = "13: TCB1WO input source"]
    TCB1 = 13,
    #[doc = "14: AC2 OUT input source"]
    AC2 = 14,
}
impl From<INSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL1_A) -> Self {
        variant as _
    }
}
impl INSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INSEL1_A> {
        match self.bits {
            0 => Some(INSEL1_A::MASK),
            1 => Some(INSEL1_A::FEEDBACK),
            2 => Some(INSEL1_A::LINK),
            3 => Some(INSEL1_A::EVENT0),
            4 => Some(INSEL1_A::EVENT1),
            5 => Some(INSEL1_A::IO),
            6 => Some(INSEL1_A::AC0),
            7 => Some(INSEL1_A::TCB0),
            8 => Some(INSEL1_A::TCA0),
            9 => Some(INSEL1_A::TCD0),
            10 => Some(INSEL1_A::USART0),
            11 => Some(INSEL1_A::SPI0),
            12 => Some(INSEL1_A::AC1),
            13 => Some(INSEL1_A::TCB1),
            14 => Some(INSEL1_A::AC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INSEL1_A::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL1_A::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == INSEL1_A::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT0`"]
    #[inline(always)]
    pub fn is_event0(&self) -> bool {
        *self == INSEL1_A::EVENT0
    }
    #[doc = "Checks if the value of the field is `EVENT1`"]
    #[inline(always)]
    pub fn is_event1(&self) -> bool {
        *self == INSEL1_A::EVENT1
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == INSEL1_A::IO
    }
    #[doc = "Checks if the value of the field is `AC0`"]
    #[inline(always)]
    pub fn is_ac0(&self) -> bool {
        *self == INSEL1_A::AC0
    }
    #[doc = "Checks if the value of the field is `TCB0`"]
    #[inline(always)]
    pub fn is_tcb0(&self) -> bool {
        *self == INSEL1_A::TCB0
    }
    #[doc = "Checks if the value of the field is `TCA0`"]
    #[inline(always)]
    pub fn is_tca0(&self) -> bool {
        *self == INSEL1_A::TCA0
    }
    #[doc = "Checks if the value of the field is `TCD0`"]
    #[inline(always)]
    pub fn is_tcd0(&self) -> bool {
        *self == INSEL1_A::TCD0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == INSEL1_A::USART0
    }
    #[doc = "Checks if the value of the field is `SPI0`"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == INSEL1_A::SPI0
    }
    #[doc = "Checks if the value of the field is `AC1`"]
    #[inline(always)]
    pub fn is_ac1(&self) -> bool {
        *self == INSEL1_A::AC1
    }
    #[doc = "Checks if the value of the field is `TCB1`"]
    #[inline(always)]
    pub fn is_tcb1(&self) -> bool {
        *self == INSEL1_A::TCB1
    }
    #[doc = "Checks if the value of the field is `AC2`"]
    #[inline(always)]
    pub fn is_ac2(&self) -> bool {
        *self == INSEL1_A::AC2
    }
}
#[doc = "Field `INSEL1` writer - LUT Input 1 Source Selection"]
pub type INSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LUT1CTRLB_SPEC, u8, INSEL1_A, 4, O>;
impl<'a, const O: u8> INSEL1_W<'a, O> {
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL1_A::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL1_A::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL1_A::LINK)
    }
    #[doc = "Event input source 0"]
    #[inline(always)]
    pub fn event0(self) -> &'a mut W {
        self.variant(INSEL1_A::EVENT0)
    }
    #[doc = "Event input source 1"]
    #[inline(always)]
    pub fn event1(self) -> &'a mut W {
        self.variant(INSEL1_A::EVENT1)
    }
    #[doc = "IO pin LUTn-N1 input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL1_A::IO)
    }
    #[doc = "AC0 OUT input source"]
    #[inline(always)]
    pub fn ac0(self) -> &'a mut W {
        self.variant(INSEL1_A::AC0)
    }
    #[doc = "TCB0 WO input source"]
    #[inline(always)]
    pub fn tcb0(self) -> &'a mut W {
        self.variant(INSEL1_A::TCB0)
    }
    #[doc = "TCA0 WO1 input source"]
    #[inline(always)]
    pub fn tca0(self) -> &'a mut W {
        self.variant(INSEL1_A::TCA0)
    }
    #[doc = "TCD0 WOB input source"]
    #[inline(always)]
    pub fn tcd0(self) -> &'a mut W {
        self.variant(INSEL1_A::TCD0)
    }
    #[doc = "USART0 TXD input source"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut W {
        self.variant(INSEL1_A::USART0)
    }
    #[doc = "SPI0 MOSI input source"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut W {
        self.variant(INSEL1_A::SPI0)
    }
    #[doc = "AC1 OUT input source"]
    #[inline(always)]
    pub fn ac1(self) -> &'a mut W {
        self.variant(INSEL1_A::AC1)
    }
    #[doc = "TCB1WO input source"]
    #[inline(always)]
    pub fn tcb1(self) -> &'a mut W {
        self.variant(INSEL1_A::TCB1)
    }
    #[doc = "AC2 OUT input source"]
    #[inline(always)]
    pub fn ac2(self) -> &'a mut W {
        self.variant(INSEL1_A::AC2)
    }
}
impl R {
    #[doc = "Bits 0:3 - LUT Input 0 Source Selection"]
    #[inline(always)]
    pub fn insel0(&self) -> INSEL0_R {
        INSEL0_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - LUT Input 1 Source Selection"]
    #[inline(always)]
    pub fn insel1(&self) -> INSEL1_R {
        INSEL1_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUT Input 0 Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel0(&mut self) -> INSEL0_W<0> {
        INSEL0_W::new(self)
    }
    #[doc = "Bits 4:7 - LUT Input 1 Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel1(&mut self) -> INSEL1_W<4> {
        INSEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT Control 1 B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut1ctrlb](index.html) module"]
pub struct LUT1CTRLB_SPEC;
impl crate::RegisterSpec for LUT1CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lut1ctrlb::R](R) reader structure"]
impl crate::Readable for LUT1CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut1ctrlb::W](W) writer structure"]
impl crate::Writable for LUT1CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT1CTRLB to value 0"]
impl crate::Resettable for LUT1CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
