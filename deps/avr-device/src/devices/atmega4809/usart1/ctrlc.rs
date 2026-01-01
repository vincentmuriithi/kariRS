#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NORMAL_CHSIZE` reader - Character Size"]
pub type NORMAL_CHSIZE_R = crate::FieldReader<u8, NORMAL_CHSIZE_A>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NORMAL_CHSIZE_A {
    #[doc = "0: Character size: 5 bit"]
    _5BIT = 0,
    #[doc = "1: Character size: 6 bit"]
    _6BIT = 1,
    #[doc = "2: Character size: 7 bit"]
    _7BIT = 2,
    #[doc = "3: Character size: 8 bit"]
    _8BIT = 3,
    #[doc = "6: Character size: 9 bit read low byte first"]
    _9BITL = 6,
    #[doc = "7: Character size: 9 bit read high byte first"]
    _9BITH = 7,
}
impl From<NORMAL_CHSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: NORMAL_CHSIZE_A) -> Self {
        variant as _
    }
}
impl NORMAL_CHSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NORMAL_CHSIZE_A> {
        match self.bits {
            0 => Some(NORMAL_CHSIZE_A::_5BIT),
            1 => Some(NORMAL_CHSIZE_A::_6BIT),
            2 => Some(NORMAL_CHSIZE_A::_7BIT),
            3 => Some(NORMAL_CHSIZE_A::_8BIT),
            6 => Some(NORMAL_CHSIZE_A::_9BITL),
            7 => Some(NORMAL_CHSIZE_A::_9BITH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_5BIT`"]
    #[inline(always)]
    pub fn is_5bit(&self) -> bool {
        *self == NORMAL_CHSIZE_A::_5BIT
    }
    #[doc = "Checks if the value of the field is `_6BIT`"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == NORMAL_CHSIZE_A::_6BIT
    }
    #[doc = "Checks if the value of the field is `_7BIT`"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == NORMAL_CHSIZE_A::_7BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == NORMAL_CHSIZE_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_9BITL`"]
    #[inline(always)]
    pub fn is_9bitl(&self) -> bool {
        *self == NORMAL_CHSIZE_A::_9BITL
    }
    #[doc = "Checks if the value of the field is `_9BITH`"]
    #[inline(always)]
    pub fn is_9bith(&self) -> bool {
        *self == NORMAL_CHSIZE_A::_9BITH
    }
}
#[doc = "Field `NORMAL_CHSIZE` writer - Character Size"]
pub type NORMAL_CHSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTRLC_SPEC, u8, NORMAL_CHSIZE_A, 3, O>;
impl<'a, const O: u8> NORMAL_CHSIZE_W<'a, O> {
    #[doc = "Character size: 5 bit"]
    #[inline(always)]
    pub fn _5bit(self) -> &'a mut W {
        self.variant(NORMAL_CHSIZE_A::_5BIT)
    }
    #[doc = "Character size: 6 bit"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut W {
        self.variant(NORMAL_CHSIZE_A::_6BIT)
    }
    #[doc = "Character size: 7 bit"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut W {
        self.variant(NORMAL_CHSIZE_A::_7BIT)
    }
    #[doc = "Character size: 8 bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(NORMAL_CHSIZE_A::_8BIT)
    }
    #[doc = "Character size: 9 bit read low byte first"]
    #[inline(always)]
    pub fn _9bitl(self) -> &'a mut W {
        self.variant(NORMAL_CHSIZE_A::_9BITL)
    }
    #[doc = "Character size: 9 bit read high byte first"]
    #[inline(always)]
    pub fn _9bith(self) -> &'a mut W {
        self.variant(NORMAL_CHSIZE_A::_9BITH)
    }
}
#[doc = "Field `MSPI_UCPHA` reader - SPI Master Mode, Clock Phase"]
pub type MSPI_UCPHA_R = crate::BitReader<bool>;
#[doc = "Field `MSPI_UCPHA` writer - SPI Master Mode, Clock Phase"]
pub type MSPI_UCPHA_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
#[doc = "Field `MSPI_UDORD` reader - SPI Master Mode, Data Order"]
pub type MSPI_UDORD_R = crate::BitReader<bool>;
#[doc = "Field `MSPI_UDORD` writer - SPI Master Mode, Data Order"]
pub type MSPI_UDORD_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
#[doc = "Field `NORMAL_SBMODE` reader - Stop Bit Mode"]
pub type NORMAL_SBMODE_R = crate::BitReader<NORMAL_SBMODE_A>;
#[doc = "Stop Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NORMAL_SBMODE_A {
    #[doc = "0: 1 stop bit"]
    _1BIT = 0,
    #[doc = "1: 2 stop bits"]
    _2BIT = 1,
}
impl From<NORMAL_SBMODE_A> for bool {
    #[inline(always)]
    fn from(variant: NORMAL_SBMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl NORMAL_SBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NORMAL_SBMODE_A {
        match self.bits {
            false => NORMAL_SBMODE_A::_1BIT,
            true => NORMAL_SBMODE_A::_2BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_1BIT`"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        *self == NORMAL_SBMODE_A::_1BIT
    }
    #[doc = "Checks if the value of the field is `_2BIT`"]
    #[inline(always)]
    pub fn is_2bit(&self) -> bool {
        *self == NORMAL_SBMODE_A::_2BIT
    }
}
#[doc = "Field `NORMAL_SBMODE` writer - Stop Bit Mode"]
pub type NORMAL_SBMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CTRLC_SPEC, NORMAL_SBMODE_A, O>;
impl<'a, const O: u8> NORMAL_SBMODE_W<'a, O> {
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut W {
        self.variant(NORMAL_SBMODE_A::_1BIT)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _2bit(self) -> &'a mut W {
        self.variant(NORMAL_SBMODE_A::_2BIT)
    }
}
#[doc = "Field `NORMAL_PMODE` reader - Parity Mode"]
pub type NORMAL_PMODE_R = crate::FieldReader<u8, NORMAL_PMODE_A>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NORMAL_PMODE_A {
    #[doc = "0: No Parity"]
    DISABLED = 0,
    #[doc = "2: Even Parity"]
    EVEN = 2,
    #[doc = "3: Odd Parity"]
    ODD = 3,
}
impl From<NORMAL_PMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: NORMAL_PMODE_A) -> Self {
        variant as _
    }
}
impl NORMAL_PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NORMAL_PMODE_A> {
        match self.bits {
            0 => Some(NORMAL_PMODE_A::DISABLED),
            2 => Some(NORMAL_PMODE_A::EVEN),
            3 => Some(NORMAL_PMODE_A::ODD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NORMAL_PMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == NORMAL_PMODE_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == NORMAL_PMODE_A::ODD
    }
}
#[doc = "Field `NORMAL_PMODE` writer - Parity Mode"]
pub type NORMAL_PMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTRLC_SPEC, u8, NORMAL_PMODE_A, 2, O>;
impl<'a, const O: u8> NORMAL_PMODE_W<'a, O> {
    #[doc = "No Parity"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NORMAL_PMODE_A::DISABLED)
    }
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(NORMAL_PMODE_A::EVEN)
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(NORMAL_PMODE_A::ODD)
    }
}
#[doc = "Field `MSPI_CMODE` reader - Communication Mode"]
pub type MSPI_CMODE_R = crate::FieldReader<u8, MSPI_CMODE_A>;
#[doc = "Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSPI_CMODE_A {
    #[doc = "0: Asynchronous Mode"]
    ASYNCHRONOUS = 0,
    #[doc = "1: Synchronous Mode"]
    SYNCHRONOUS = 1,
    #[doc = "2: Infrared Communication"]
    IRCOM = 2,
    #[doc = "3: Master SPI Mode"]
    MSPI = 3,
}
impl From<MSPI_CMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSPI_CMODE_A) -> Self {
        variant as _
    }
}
impl MSPI_CMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSPI_CMODE_A {
        match self.bits {
            0 => MSPI_CMODE_A::ASYNCHRONOUS,
            1 => MSPI_CMODE_A::SYNCHRONOUS,
            2 => MSPI_CMODE_A::IRCOM,
            3 => MSPI_CMODE_A::MSPI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == MSPI_CMODE_A::ASYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == MSPI_CMODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `IRCOM`"]
    #[inline(always)]
    pub fn is_ircom(&self) -> bool {
        *self == MSPI_CMODE_A::IRCOM
    }
    #[doc = "Checks if the value of the field is `MSPI`"]
    #[inline(always)]
    pub fn is_mspi(&self) -> bool {
        *self == MSPI_CMODE_A::MSPI
    }
}
#[doc = "Field `MSPI_CMODE` writer - Communication Mode"]
pub type MSPI_CMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTRLC_SPEC, u8, MSPI_CMODE_A, 2, O>;
impl<'a, const O: u8> MSPI_CMODE_W<'a, O> {
    #[doc = "Asynchronous Mode"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(MSPI_CMODE_A::ASYNCHRONOUS)
    }
    #[doc = "Synchronous Mode"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(MSPI_CMODE_A::SYNCHRONOUS)
    }
    #[doc = "Infrared Communication"]
    #[inline(always)]
    pub fn ircom(self) -> &'a mut W {
        self.variant(MSPI_CMODE_A::IRCOM)
    }
    #[doc = "Master SPI Mode"]
    #[inline(always)]
    pub fn mspi(self) -> &'a mut W {
        self.variant(MSPI_CMODE_A::MSPI)
    }
}
#[doc = "Field `NORMAL_CMODE` reader - Communication Mode"]
pub type NORMAL_CMODE_R = crate::FieldReader<u8, NORMAL_CMODE_A>;
#[doc = "Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NORMAL_CMODE_A {
    #[doc = "0: Asynchronous Mode"]
    ASYNCHRONOUS = 0,
    #[doc = "1: Synchronous Mode"]
    SYNCHRONOUS = 1,
    #[doc = "2: Infrared Communication"]
    IRCOM = 2,
    #[doc = "3: Master SPI Mode"]
    MSPI = 3,
}
impl From<NORMAL_CMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: NORMAL_CMODE_A) -> Self {
        variant as _
    }
}
impl NORMAL_CMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NORMAL_CMODE_A {
        match self.bits {
            0 => NORMAL_CMODE_A::ASYNCHRONOUS,
            1 => NORMAL_CMODE_A::SYNCHRONOUS,
            2 => NORMAL_CMODE_A::IRCOM,
            3 => NORMAL_CMODE_A::MSPI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == NORMAL_CMODE_A::ASYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == NORMAL_CMODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `IRCOM`"]
    #[inline(always)]
    pub fn is_ircom(&self) -> bool {
        *self == NORMAL_CMODE_A::IRCOM
    }
    #[doc = "Checks if the value of the field is `MSPI`"]
    #[inline(always)]
    pub fn is_mspi(&self) -> bool {
        *self == NORMAL_CMODE_A::MSPI
    }
}
#[doc = "Field `NORMAL_CMODE` writer - Communication Mode"]
pub type NORMAL_CMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTRLC_SPEC, u8, NORMAL_CMODE_A, 2, O>;
impl<'a, const O: u8> NORMAL_CMODE_W<'a, O> {
    #[doc = "Asynchronous Mode"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(NORMAL_CMODE_A::ASYNCHRONOUS)
    }
    #[doc = "Synchronous Mode"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(NORMAL_CMODE_A::SYNCHRONOUS)
    }
    #[doc = "Infrared Communication"]
    #[inline(always)]
    pub fn ircom(self) -> &'a mut W {
        self.variant(NORMAL_CMODE_A::IRCOM)
    }
    #[doc = "Master SPI Mode"]
    #[inline(always)]
    pub fn mspi(self) -> &'a mut W {
        self.variant(NORMAL_CMODE_A::MSPI)
    }
}
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn normal_chsize(&self) -> NORMAL_CHSIZE_R {
        NORMAL_CHSIZE_R::new(self.bits & 7)
    }
    #[doc = "Bit 1 - SPI Master Mode, Clock Phase"]
    #[inline(always)]
    pub fn mspi_ucpha(&self) -> MSPI_UCPHA_R {
        MSPI_UCPHA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Master Mode, Data Order"]
    #[inline(always)]
    pub fn mspi_udord(&self) -> MSPI_UDORD_R {
        MSPI_UDORD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Bit Mode"]
    #[inline(always)]
    pub fn normal_sbmode(&self) -> NORMAL_SBMODE_R {
        NORMAL_SBMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Mode"]
    #[inline(always)]
    pub fn normal_pmode(&self) -> NORMAL_PMODE_R {
        NORMAL_PMODE_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Communication Mode"]
    #[inline(always)]
    pub fn mspi_cmode(&self) -> MSPI_CMODE_R {
        MSPI_CMODE_R::new((self.bits >> 6) & 3)
    }
    #[doc = "Bits 6:7 - Communication Mode"]
    #[inline(always)]
    pub fn normal_cmode(&self) -> NORMAL_CMODE_R {
        NORMAL_CMODE_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn normal_chsize(&mut self) -> NORMAL_CHSIZE_W<0> {
        NORMAL_CHSIZE_W::new(self)
    }
    #[doc = "Bit 1 - SPI Master Mode, Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn mspi_ucpha(&mut self) -> MSPI_UCPHA_W<1> {
        MSPI_UCPHA_W::new(self)
    }
    #[doc = "Bit 2 - SPI Master Mode, Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn mspi_udord(&mut self) -> MSPI_UDORD_W<2> {
        MSPI_UDORD_W::new(self)
    }
    #[doc = "Bit 3 - Stop Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn normal_sbmode(&mut self) -> NORMAL_SBMODE_W<3> {
        NORMAL_SBMODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Parity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn normal_pmode(&mut self) -> NORMAL_PMODE_W<4> {
        NORMAL_PMODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mspi_cmode(&mut self) -> MSPI_CMODE_W<6> {
        MSPI_CMODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn normal_cmode(&mut self) -> NORMAL_CMODE_W<6> {
        NORMAL_CMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
