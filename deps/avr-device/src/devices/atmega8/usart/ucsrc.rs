#[doc = "Register `UCSRC` reader"]
pub struct R(crate::R<UCSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSRC` writer"]
pub struct W(crate::W<UCSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSRC_SPEC>;
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
impl From<crate::W<UCSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCPOL` reader - Clock Polarity"]
pub type UCPOL_R = crate::BitReader<UCPOL_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPOL_A {
    #[doc = "0: Transmit on Rising XCK Edge, Receive on Falling XCK Edge"]
    RISING_EDGE = 0,
    #[doc = "1: Transmit on Falling XCK Edge, Receive on Rising XCK Edge"]
    FALLING_EDGE = 1,
}
impl From<UCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: UCPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPOL_A {
        match self.bits {
            false => UCPOL_A::RISING_EDGE,
            true => UCPOL_A::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == UCPOL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == UCPOL_A::FALLING_EDGE
    }
}
#[doc = "Field `UCPOL` writer - Clock Polarity"]
pub type UCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRC_SPEC, UCPOL_A, O>;
impl<'a, const O: u8> UCPOL_W<'a, O> {
    #[doc = "Transmit on Rising XCK Edge, Receive on Falling XCK Edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(UCPOL_A::RISING_EDGE)
    }
    #[doc = "Transmit on Falling XCK Edge, Receive on Rising XCK Edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(UCPOL_A::FALLING_EDGE)
    }
}
#[doc = "Field `UCSZ` reader - Character Size"]
pub type UCSZ_R = crate::FieldReader<u8, UCSZ_A>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCSZ_A {
    #[doc = "0: Character Size: 5 bit"]
    CHR5 = 0,
    #[doc = "1: Character Size: 6 bit"]
    CHR6 = 1,
    #[doc = "2: Character Size: 7 bit"]
    CHR7 = 2,
    #[doc = "3: Character Size: 8 bit"]
    CHR8 = 3,
}
impl From<UCSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSZ_A) -> Self {
        variant as _
    }
}
impl UCSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSZ_A {
        match self.bits {
            0 => UCSZ_A::CHR5,
            1 => UCSZ_A::CHR6,
            2 => UCSZ_A::CHR7,
            3 => UCSZ_A::CHR8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHR5`"]
    #[inline(always)]
    pub fn is_chr5(&self) -> bool {
        *self == UCSZ_A::CHR5
    }
    #[doc = "Checks if the value of the field is `CHR6`"]
    #[inline(always)]
    pub fn is_chr6(&self) -> bool {
        *self == UCSZ_A::CHR6
    }
    #[doc = "Checks if the value of the field is `CHR7`"]
    #[inline(always)]
    pub fn is_chr7(&self) -> bool {
        *self == UCSZ_A::CHR7
    }
    #[doc = "Checks if the value of the field is `CHR8`"]
    #[inline(always)]
    pub fn is_chr8(&self) -> bool {
        *self == UCSZ_A::CHR8
    }
}
#[doc = "Field `UCSZ` writer - Character Size"]
pub type UCSZ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UCSRC_SPEC, u8, UCSZ_A, 2, O>;
impl<'a, const O: u8> UCSZ_W<'a, O> {
    #[doc = "Character Size: 5 bit"]
    #[inline(always)]
    pub fn chr5(self) -> &'a mut W {
        self.variant(UCSZ_A::CHR5)
    }
    #[doc = "Character Size: 6 bit"]
    #[inline(always)]
    pub fn chr6(self) -> &'a mut W {
        self.variant(UCSZ_A::CHR6)
    }
    #[doc = "Character Size: 7 bit"]
    #[inline(always)]
    pub fn chr7(self) -> &'a mut W {
        self.variant(UCSZ_A::CHR7)
    }
    #[doc = "Character Size: 8 bit"]
    #[inline(always)]
    pub fn chr8(self) -> &'a mut W {
        self.variant(UCSZ_A::CHR8)
    }
}
#[doc = "Field `USBS` reader - Stop Bit Select"]
pub type USBS_R = crate::BitReader<USBS_A>;
#[doc = "Stop Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBS_A {
    #[doc = "0: 1-bit"]
    STOP1 = 0,
    #[doc = "1: 2-bit"]
    STOP2 = 1,
}
impl From<USBS_A> for bool {
    #[inline(always)]
    fn from(variant: USBS_A) -> Self {
        variant as u8 != 0
    }
}
impl USBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBS_A {
        match self.bits {
            false => USBS_A::STOP1,
            true => USBS_A::STOP2,
        }
    }
    #[doc = "Checks if the value of the field is `STOP1`"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == USBS_A::STOP1
    }
    #[doc = "Checks if the value of the field is `STOP2`"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == USBS_A::STOP2
    }
}
#[doc = "Field `USBS` writer - Stop Bit Select"]
pub type USBS_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRC_SPEC, USBS_A, O>;
impl<'a, const O: u8> USBS_W<'a, O> {
    #[doc = "1-bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(USBS_A::STOP1)
    }
    #[doc = "2-bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(USBS_A::STOP2)
    }
}
#[doc = "Field `UPM` reader - Parity Mode Bits"]
pub type UPM_R = crate::FieldReader<u8, UPM_A>;
#[doc = "Parity Mode Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPM_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Enabled, Even Parity"]
    PARITY_EVEN = 2,
    #[doc = "3: Enabled, Odd Parity"]
    PARITY_ODD = 3,
}
impl From<UPM_A> for u8 {
    #[inline(always)]
    fn from(variant: UPM_A) -> Self {
        variant as _
    }
}
impl UPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPM_A> {
        match self.bits {
            0 => Some(UPM_A::DISABLED),
            2 => Some(UPM_A::PARITY_EVEN),
            3 => Some(UPM_A::PARITY_ODD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PARITY_EVEN`"]
    #[inline(always)]
    pub fn is_parity_even(&self) -> bool {
        *self == UPM_A::PARITY_EVEN
    }
    #[doc = "Checks if the value of the field is `PARITY_ODD`"]
    #[inline(always)]
    pub fn is_parity_odd(&self) -> bool {
        *self == UPM_A::PARITY_ODD
    }
}
#[doc = "Field `UPM` writer - Parity Mode Bits"]
pub type UPM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, UCSRC_SPEC, u8, UPM_A, 2, O>;
impl<'a, const O: u8> UPM_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPM_A::DISABLED)
    }
    #[doc = "Enabled, Even Parity"]
    #[inline(always)]
    pub fn parity_even(self) -> &'a mut W {
        self.variant(UPM_A::PARITY_EVEN)
    }
    #[doc = "Enabled, Odd Parity"]
    #[inline(always)]
    pub fn parity_odd(self) -> &'a mut W {
        self.variant(UPM_A::PARITY_ODD)
    }
}
#[doc = "Field `UMSEL` reader - USART Mode Select"]
pub type UMSEL_R = crate::BitReader<UMSEL_A>;
#[doc = "USART Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UMSEL_A {
    #[doc = "0: Asynchronous USART"]
    USART_ASYNC = 0,
    #[doc = "1: Synchronous USART"]
    USART_SYNC = 1,
}
impl From<UMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: UMSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl UMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UMSEL_A {
        match self.bits {
            false => UMSEL_A::USART_ASYNC,
            true => UMSEL_A::USART_SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `USART_ASYNC`"]
    #[inline(always)]
    pub fn is_usart_async(&self) -> bool {
        *self == UMSEL_A::USART_ASYNC
    }
    #[doc = "Checks if the value of the field is `USART_SYNC`"]
    #[inline(always)]
    pub fn is_usart_sync(&self) -> bool {
        *self == UMSEL_A::USART_SYNC
    }
}
#[doc = "Field `UMSEL` writer - USART Mode Select"]
pub type UMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRC_SPEC, UMSEL_A, O>;
impl<'a, const O: u8> UMSEL_W<'a, O> {
    #[doc = "Asynchronous USART"]
    #[inline(always)]
    pub fn usart_async(self) -> &'a mut W {
        self.variant(UMSEL_A::USART_ASYNC)
    }
    #[doc = "Synchronous USART"]
    #[inline(always)]
    pub fn usart_sync(self) -> &'a mut W {
        self.variant(UMSEL_A::USART_SYNC)
    }
}
#[doc = "Field `URSEL` reader - Register Select"]
pub type URSEL_R = crate::BitReader<bool>;
#[doc = "Field `URSEL` writer - Register Select"]
pub type URSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSRC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn ucpol(&self) -> UCPOL_R {
        UCPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    pub fn ucsz(&self) -> UCSZ_R {
        UCSZ_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    pub fn usbs(&self) -> USBS_R {
        USBS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Mode Bits"]
    #[inline(always)]
    pub fn upm(&self) -> UPM_R {
        UPM_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - USART Mode Select"]
    #[inline(always)]
    pub fn umsel(&self) -> UMSEL_R {
        UMSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Register Select"]
    #[inline(always)]
    pub fn ursel(&self) -> URSEL_R {
        URSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucpol(&mut self) -> UCPOL_W<0> {
        UCPOL_W::new(self)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz(&mut self) -> UCSZ_W<1> {
        UCSZ_W::new(self)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbs(&mut self) -> USBS_W<3> {
        USBS_W::new(self)
    }
    #[doc = "Bits 4:5 - Parity Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn upm(&mut self) -> UPM_W<4> {
        UPM_W::new(self)
    }
    #[doc = "Bit 6 - USART Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn umsel(&mut self) -> UMSEL_W<6> {
        UMSEL_W::new(self)
    }
    #[doc = "Bit 7 - Register Select"]
    #[inline(always)]
    #[must_use]
    pub fn ursel(&mut self) -> URSEL_W<7> {
        URSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control and Status Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsrc](index.html) module"]
pub struct UCSRC_SPEC;
impl crate::RegisterSpec for UCSRC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsrc::R](R) reader structure"]
impl crate::Readable for UCSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsrc::W](W) writer structure"]
impl crate::Writable for UCSRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSRC to value 0"]
impl crate::Resettable for UCSRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
