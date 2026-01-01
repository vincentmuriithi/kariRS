#[doc = "Register `UCSR1C` reader"]
pub struct R(crate::R<UCSR1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSR1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSR1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSR1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSR1C` writer"]
pub struct W(crate::W<UCSR1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSR1C_SPEC>;
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
impl From<crate::W<UCSR1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSR1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCPOL1` reader - Clock Polarity"]
pub type UCPOL1_R = crate::BitReader<UCPOL1_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPOL1_A {
    #[doc = "0: Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    RISING_EDGE = 0,
    #[doc = "1: Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    FALLING_EDGE = 1,
}
impl From<UCPOL1_A> for bool {
    #[inline(always)]
    fn from(variant: UCPOL1_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPOL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPOL1_A {
        match self.bits {
            false => UCPOL1_A::RISING_EDGE,
            true => UCPOL1_A::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == UCPOL1_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == UCPOL1_A::FALLING_EDGE
    }
}
#[doc = "Field `UCPOL1` writer - Clock Polarity"]
pub type UCPOL1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1C_SPEC, UCPOL1_A, O>;
impl<'a, const O: u8> UCPOL1_W<'a, O> {
    #[doc = "Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(UCPOL1_A::RISING_EDGE)
    }
    #[doc = "Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(UCPOL1_A::FALLING_EDGE)
    }
}
#[doc = "Field `UCSZ1` reader - Character Size"]
pub type UCSZ1_R = crate::FieldReader<u8, UCSZ1_A>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCSZ1_A {
    #[doc = "0: Character Size: 5 bit"]
    CHR5 = 0,
    #[doc = "1: Character Size: 6 bit"]
    CHR6 = 1,
    #[doc = "2: Character Size: 7 bit"]
    CHR7 = 2,
    #[doc = "3: Character Size: 8 bit"]
    CHR8 = 3,
}
impl From<UCSZ1_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSZ1_A) -> Self {
        variant as _
    }
}
impl UCSZ1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSZ1_A {
        match self.bits {
            0 => UCSZ1_A::CHR5,
            1 => UCSZ1_A::CHR6,
            2 => UCSZ1_A::CHR7,
            3 => UCSZ1_A::CHR8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHR5`"]
    #[inline(always)]
    pub fn is_chr5(&self) -> bool {
        *self == UCSZ1_A::CHR5
    }
    #[doc = "Checks if the value of the field is `CHR6`"]
    #[inline(always)]
    pub fn is_chr6(&self) -> bool {
        *self == UCSZ1_A::CHR6
    }
    #[doc = "Checks if the value of the field is `CHR7`"]
    #[inline(always)]
    pub fn is_chr7(&self) -> bool {
        *self == UCSZ1_A::CHR7
    }
    #[doc = "Checks if the value of the field is `CHR8`"]
    #[inline(always)]
    pub fn is_chr8(&self) -> bool {
        *self == UCSZ1_A::CHR8
    }
}
#[doc = "Field `UCSZ1` writer - Character Size"]
pub type UCSZ1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UCSR1C_SPEC, u8, UCSZ1_A, 2, O>;
impl<'a, const O: u8> UCSZ1_W<'a, O> {
    #[doc = "Character Size: 5 bit"]
    #[inline(always)]
    pub fn chr5(self) -> &'a mut W {
        self.variant(UCSZ1_A::CHR5)
    }
    #[doc = "Character Size: 6 bit"]
    #[inline(always)]
    pub fn chr6(self) -> &'a mut W {
        self.variant(UCSZ1_A::CHR6)
    }
    #[doc = "Character Size: 7 bit"]
    #[inline(always)]
    pub fn chr7(self) -> &'a mut W {
        self.variant(UCSZ1_A::CHR7)
    }
    #[doc = "Character Size: 8 bit"]
    #[inline(always)]
    pub fn chr8(self) -> &'a mut W {
        self.variant(UCSZ1_A::CHR8)
    }
}
#[doc = "Field `USBS1` reader - Stop Bit Select"]
pub type USBS1_R = crate::BitReader<USBS1_A>;
#[doc = "Stop Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBS1_A {
    #[doc = "0: 1-bit"]
    STOP1 = 0,
    #[doc = "1: 2-bit"]
    STOP2 = 1,
}
impl From<USBS1_A> for bool {
    #[inline(always)]
    fn from(variant: USBS1_A) -> Self {
        variant as u8 != 0
    }
}
impl USBS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBS1_A {
        match self.bits {
            false => USBS1_A::STOP1,
            true => USBS1_A::STOP2,
        }
    }
    #[doc = "Checks if the value of the field is `STOP1`"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == USBS1_A::STOP1
    }
    #[doc = "Checks if the value of the field is `STOP2`"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == USBS1_A::STOP2
    }
}
#[doc = "Field `USBS1` writer - Stop Bit Select"]
pub type USBS1_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR1C_SPEC, USBS1_A, O>;
impl<'a, const O: u8> USBS1_W<'a, O> {
    #[doc = "1-bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(USBS1_A::STOP1)
    }
    #[doc = "2-bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(USBS1_A::STOP2)
    }
}
#[doc = "Field `UPM1` reader - Parity Mode"]
pub type UPM1_R = crate::FieldReader<u8, UPM1_A>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPM1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Enabled, Even Parity"]
    PARITY_EVEN = 2,
    #[doc = "3: Enabled, Odd Parity"]
    PARITY_ODD = 3,
}
impl From<UPM1_A> for u8 {
    #[inline(always)]
    fn from(variant: UPM1_A) -> Self {
        variant as _
    }
}
impl UPM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPM1_A> {
        match self.bits {
            0 => Some(UPM1_A::DISABLED),
            2 => Some(UPM1_A::PARITY_EVEN),
            3 => Some(UPM1_A::PARITY_ODD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPM1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PARITY_EVEN`"]
    #[inline(always)]
    pub fn is_parity_even(&self) -> bool {
        *self == UPM1_A::PARITY_EVEN
    }
    #[doc = "Checks if the value of the field is `PARITY_ODD`"]
    #[inline(always)]
    pub fn is_parity_odd(&self) -> bool {
        *self == UPM1_A::PARITY_ODD
    }
}
#[doc = "Field `UPM1` writer - Parity Mode"]
pub type UPM1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, UCSR1C_SPEC, u8, UPM1_A, 2, O>;
impl<'a, const O: u8> UPM1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPM1_A::DISABLED)
    }
    #[doc = "Enabled, Even Parity"]
    #[inline(always)]
    pub fn parity_even(self) -> &'a mut W {
        self.variant(UPM1_A::PARITY_EVEN)
    }
    #[doc = "Enabled, Odd Parity"]
    #[inline(always)]
    pub fn parity_odd(self) -> &'a mut W {
        self.variant(UPM1_A::PARITY_ODD)
    }
}
#[doc = "Field `UMSEL1` reader - USART Mode Select"]
pub type UMSEL1_R = crate::FieldReader<u8, UMSEL1_A>;
#[doc = "USART Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UMSEL1_A {
    #[doc = "0: Asynchronous USART"]
    USART_ASYNC = 0,
    #[doc = "1: Synchronous USART"]
    USART_SYNC = 1,
    #[doc = "3: Master SPI (MSPIM)"]
    SPI_MASTER = 3,
}
impl From<UMSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: UMSEL1_A) -> Self {
        variant as _
    }
}
impl UMSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UMSEL1_A> {
        match self.bits {
            0 => Some(UMSEL1_A::USART_ASYNC),
            1 => Some(UMSEL1_A::USART_SYNC),
            3 => Some(UMSEL1_A::SPI_MASTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USART_ASYNC`"]
    #[inline(always)]
    pub fn is_usart_async(&self) -> bool {
        *self == UMSEL1_A::USART_ASYNC
    }
    #[doc = "Checks if the value of the field is `USART_SYNC`"]
    #[inline(always)]
    pub fn is_usart_sync(&self) -> bool {
        *self == UMSEL1_A::USART_SYNC
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == UMSEL1_A::SPI_MASTER
    }
}
#[doc = "Field `UMSEL1` writer - USART Mode Select"]
pub type UMSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, UCSR1C_SPEC, u8, UMSEL1_A, 2, O>;
impl<'a, const O: u8> UMSEL1_W<'a, O> {
    #[doc = "Asynchronous USART"]
    #[inline(always)]
    pub fn usart_async(self) -> &'a mut W {
        self.variant(UMSEL1_A::USART_ASYNC)
    }
    #[doc = "Synchronous USART"]
    #[inline(always)]
    pub fn usart_sync(self) -> &'a mut W {
        self.variant(UMSEL1_A::USART_SYNC)
    }
    #[doc = "Master SPI (MSPIM)"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(UMSEL1_A::SPI_MASTER)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn ucpol1(&self) -> UCPOL1_R {
        UCPOL1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    pub fn ucsz1(&self) -> UCSZ1_R {
        UCSZ1_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    pub fn usbs1(&self) -> USBS1_R {
        USBS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Mode"]
    #[inline(always)]
    pub fn upm1(&self) -> UPM1_R {
        UPM1_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - USART Mode Select"]
    #[inline(always)]
    pub fn umsel1(&self) -> UMSEL1_R {
        UMSEL1_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucpol1(&mut self) -> UCPOL1_W<0> {
        UCPOL1_W::new(self)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz1(&mut self) -> UCSZ1_W<1> {
        UCSZ1_W::new(self)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbs1(&mut self) -> USBS1_W<3> {
        USBS1_W::new(self)
    }
    #[doc = "Bits 4:5 - Parity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn upm1(&mut self) -> UPM1_W<4> {
        UPM1_W::new(self)
    }
    #[doc = "Bits 6:7 - USART Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn umsel1(&mut self) -> UMSEL1_W<6> {
        UMSEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART1 Control and Status Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsr1c](index.html) module"]
pub struct UCSR1C_SPEC;
impl crate::RegisterSpec for UCSR1C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsr1c::R](R) reader structure"]
impl crate::Readable for UCSR1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsr1c::W](W) writer structure"]
impl crate::Writable for UCSR1C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR1C to value 0"]
impl crate::Resettable for UCSR1C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
