#[doc = "Register `TCCR4B` reader"]
pub struct R(crate::R<TCCR4B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR4B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR4B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR4B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR4B` writer"]
pub struct W(crate::W<TCCR4B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR4B_SPEC>;
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
impl From<crate::W<TCCR4B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR4B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS4` reader - Clock Select bits"]
pub type CS4_R = crate::FieldReader<u8, CS4_A>;
#[doc = "Clock Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS4_A {
    #[doc = "0: No clock source (Timer/Counter stopped)"]
    NO_CLOCK = 0,
    #[doc = "1: Running, No Prescaling"]
    DIRECT = 1,
    #[doc = "2: Running, CLK/8"]
    PRESCALE_8 = 2,
    #[doc = "3: Running, CLK/64"]
    PRESCALE_64 = 3,
    #[doc = "4: Running, CLK/256"]
    PRESCALE_256 = 4,
    #[doc = "5: Running, CLK/1024"]
    PRESCALE_1024 = 5,
    #[doc = "6: Running, ExtClk Tx Falling Edge"]
    EXT_FALLING = 6,
    #[doc = "7: Running, ExtClk Tx Rising Edge"]
    EXT_RISING = 7,
}
impl From<CS4_A> for u8 {
    #[inline(always)]
    fn from(variant: CS4_A) -> Self {
        variant as _
    }
}
impl CS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS4_A {
        match self.bits {
            0 => CS4_A::NO_CLOCK,
            1 => CS4_A::DIRECT,
            2 => CS4_A::PRESCALE_8,
            3 => CS4_A::PRESCALE_64,
            4 => CS4_A::PRESCALE_256,
            5 => CS4_A::PRESCALE_1024,
            6 => CS4_A::EXT_FALLING,
            7 => CS4_A::EXT_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS4_A::NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS4_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `PRESCALE_8`"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS4_A::PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `PRESCALE_64`"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS4_A::PRESCALE_64
    }
    #[doc = "Checks if the value of the field is `PRESCALE_256`"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS4_A::PRESCALE_256
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1024`"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS4_A::PRESCALE_1024
    }
    #[doc = "Checks if the value of the field is `EXT_FALLING`"]
    #[inline(always)]
    pub fn is_ext_falling(&self) -> bool {
        *self == CS4_A::EXT_FALLING
    }
    #[doc = "Checks if the value of the field is `EXT_RISING`"]
    #[inline(always)]
    pub fn is_ext_rising(&self) -> bool {
        *self == CS4_A::EXT_RISING
    }
}
#[doc = "Field `CS4` writer - Clock Select bits"]
pub type CS4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR4B_SPEC, u8, CS4_A, 3, O>;
impl<'a, const O: u8> CS4_W<'a, O> {
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(CS4_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(CS4_A::DIRECT)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut W {
        self.variant(CS4_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut W {
        self.variant(CS4_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut W {
        self.variant(CS4_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut W {
        self.variant(CS4_A::PRESCALE_1024)
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn ext_falling(self) -> &'a mut W {
        self.variant(CS4_A::EXT_FALLING)
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn ext_rising(self) -> &'a mut W {
        self.variant(CS4_A::EXT_RISING)
    }
}
#[doc = "Field `WGM4` reader - Waveform Generation Mode"]
pub type WGM4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGM4` writer - Waveform Generation Mode"]
pub type WGM4_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCCR4B_SPEC, u8, u8, 2, O>;
#[doc = "Field `ICES4` reader - Input Capture Edge Select"]
pub type ICES4_R = crate::BitReader<bool>;
#[doc = "Field `ICES4` writer - Input Capture Edge Select"]
pub type ICES4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4B_SPEC, bool, O>;
#[doc = "Field `ICNC4` reader - Input Capture Noise Canceler"]
pub type ICNC4_R = crate::BitReader<bool>;
#[doc = "Field `ICNC4` writer - Input Capture Noise Canceler"]
pub type ICNC4_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR4B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    pub fn cs4(&self) -> CS4_R {
        CS4_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm4(&self) -> WGM4_R {
        WGM4_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn ices4(&self) -> ICES4_R {
        ICES4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture Noise Canceler"]
    #[inline(always)]
    pub fn icnc4(&self) -> ICNC4_R {
        ICNC4_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs4(&mut self) -> CS4_W<0> {
        CS4_W::new(self)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm4(&mut self) -> WGM4_W<3> {
        WGM4_W::new(self)
    }
    #[doc = "Bit 6 - Input Capture Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices4(&mut self) -> ICES4_W<6> {
        ICES4_W::new(self)
    }
    #[doc = "Bit 7 - Input Capture Noise Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn icnc4(&mut self) -> ICNC4_W<7> {
        ICNC4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter4 Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4b](index.html) module"]
pub struct TCCR4B_SPEC;
impl crate::RegisterSpec for TCCR4B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr4b::R](R) reader structure"]
impl crate::Readable for TCCR4B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr4b::W](W) writer structure"]
impl crate::Writable for TCCR4B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4B to value 0"]
impl crate::Resettable for TCCR4B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
