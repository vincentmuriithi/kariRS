#[doc = "Register `TCCR5B` reader"]
pub struct R(crate::R<TCCR5B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR5B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR5B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR5B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR5B` writer"]
pub struct W(crate::W<TCCR5B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR5B_SPEC>;
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
impl From<crate::W<TCCR5B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR5B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS5` reader - Prescaler source of Timer/Counter 5"]
pub type CS5_R = crate::FieldReader<u8, CS5_A>;
#[doc = "Prescaler source of Timer/Counter 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS5_A {
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
impl From<CS5_A> for u8 {
    #[inline(always)]
    fn from(variant: CS5_A) -> Self {
        variant as _
    }
}
impl CS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS5_A {
        match self.bits {
            0 => CS5_A::NO_CLOCK,
            1 => CS5_A::DIRECT,
            2 => CS5_A::PRESCALE_8,
            3 => CS5_A::PRESCALE_64,
            4 => CS5_A::PRESCALE_256,
            5 => CS5_A::PRESCALE_1024,
            6 => CS5_A::EXT_FALLING,
            7 => CS5_A::EXT_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS5_A::NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS5_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `PRESCALE_8`"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS5_A::PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `PRESCALE_64`"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS5_A::PRESCALE_64
    }
    #[doc = "Checks if the value of the field is `PRESCALE_256`"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS5_A::PRESCALE_256
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1024`"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS5_A::PRESCALE_1024
    }
    #[doc = "Checks if the value of the field is `EXT_FALLING`"]
    #[inline(always)]
    pub fn is_ext_falling(&self) -> bool {
        *self == CS5_A::EXT_FALLING
    }
    #[doc = "Checks if the value of the field is `EXT_RISING`"]
    #[inline(always)]
    pub fn is_ext_rising(&self) -> bool {
        *self == CS5_A::EXT_RISING
    }
}
#[doc = "Field `CS5` writer - Prescaler source of Timer/Counter 5"]
pub type CS5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR5B_SPEC, u8, CS5_A, 3, O>;
impl<'a, const O: u8> CS5_W<'a, O> {
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(CS5_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(CS5_A::DIRECT)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut W {
        self.variant(CS5_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut W {
        self.variant(CS5_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut W {
        self.variant(CS5_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut W {
        self.variant(CS5_A::PRESCALE_1024)
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn ext_falling(self) -> &'a mut W {
        self.variant(CS5_A::EXT_FALLING)
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn ext_rising(self) -> &'a mut W {
        self.variant(CS5_A::EXT_RISING)
    }
}
#[doc = "Field `WGM5` reader - Waveform Generation Mode"]
pub type WGM5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGM5` writer - Waveform Generation Mode"]
pub type WGM5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR5B_SPEC, u8, u8, 2, O>;
#[doc = "Field `ICES5` reader - Input Capture 5 Edge Select"]
pub type ICES5_R = crate::BitReader<bool>;
#[doc = "Field `ICES5` writer - Input Capture 5 Edge Select"]
pub type ICES5_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR5B_SPEC, bool, O>;
#[doc = "Field `ICNC5` reader - Input Capture 5 Noise Canceler"]
pub type ICNC5_R = crate::BitReader<bool>;
#[doc = "Field `ICNC5` writer - Input Capture 5 Noise Canceler"]
pub type ICNC5_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR5B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Prescaler source of Timer/Counter 5"]
    #[inline(always)]
    pub fn cs5(&self) -> CS5_R {
        CS5_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm5(&self) -> WGM5_R {
        WGM5_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Input Capture 5 Edge Select"]
    #[inline(always)]
    pub fn ices5(&self) -> ICES5_R {
        ICES5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture 5 Noise Canceler"]
    #[inline(always)]
    pub fn icnc5(&self) -> ICNC5_R {
        ICNC5_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler source of Timer/Counter 5"]
    #[inline(always)]
    #[must_use]
    pub fn cs5(&mut self) -> CS5_W<0> {
        CS5_W::new(self)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm5(&mut self) -> WGM5_W<3> {
        WGM5_W::new(self)
    }
    #[doc = "Bit 6 - Input Capture 5 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices5(&mut self) -> ICES5_W<6> {
        ICES5_W::new(self)
    }
    #[doc = "Bit 7 - Input Capture 5 Noise Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn icnc5(&mut self) -> ICNC5_W<7> {
        ICNC5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter5 Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr5b](index.html) module"]
pub struct TCCR5B_SPEC;
impl crate::RegisterSpec for TCCR5B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr5b::R](R) reader structure"]
impl crate::Readable for TCCR5B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr5b::W](W) writer structure"]
impl crate::Writable for TCCR5B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR5B to value 0"]
impl crate::Resettable for TCCR5B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
