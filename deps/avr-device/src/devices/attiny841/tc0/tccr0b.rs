#[doc = "Register `TCCR0B` reader"]
pub struct R(crate::R<TCCR0B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR0B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR0B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR0B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR0B` writer"]
pub struct W(crate::W<TCCR0B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR0B_SPEC>;
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
impl From<crate::W<TCCR0B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR0B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS0` reader - Clock Select bits"]
pub type CS0_R = crate::FieldReader<u8, CS0_A>;
#[doc = "Clock Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS0_A {
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
impl From<CS0_A> for u8 {
    #[inline(always)]
    fn from(variant: CS0_A) -> Self {
        variant as _
    }
}
impl CS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS0_A {
        match self.bits {
            0 => CS0_A::NO_CLOCK,
            1 => CS0_A::DIRECT,
            2 => CS0_A::PRESCALE_8,
            3 => CS0_A::PRESCALE_64,
            4 => CS0_A::PRESCALE_256,
            5 => CS0_A::PRESCALE_1024,
            6 => CS0_A::EXT_FALLING,
            7 => CS0_A::EXT_RISING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS0_A::NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS0_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `PRESCALE_8`"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS0_A::PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `PRESCALE_64`"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS0_A::PRESCALE_64
    }
    #[doc = "Checks if the value of the field is `PRESCALE_256`"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS0_A::PRESCALE_256
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1024`"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS0_A::PRESCALE_1024
    }
    #[doc = "Checks if the value of the field is `EXT_FALLING`"]
    #[inline(always)]
    pub fn is_ext_falling(&self) -> bool {
        *self == CS0_A::EXT_FALLING
    }
    #[doc = "Checks if the value of the field is `EXT_RISING`"]
    #[inline(always)]
    pub fn is_ext_rising(&self) -> bool {
        *self == CS0_A::EXT_RISING
    }
}
#[doc = "Field `CS0` writer - Clock Select bits"]
pub type CS0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR0B_SPEC, u8, CS0_A, 3, O>;
impl<'a, const O: u8> CS0_W<'a, O> {
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(CS0_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(CS0_A::DIRECT)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut W {
        self.variant(CS0_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut W {
        self.variant(CS0_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut W {
        self.variant(CS0_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut W {
        self.variant(CS0_A::PRESCALE_1024)
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn ext_falling(self) -> &'a mut W {
        self.variant(CS0_A::EXT_FALLING)
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn ext_rising(self) -> &'a mut W {
        self.variant(CS0_A::EXT_RISING)
    }
}
#[doc = "Field `WGM02` reader - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)"]
pub type WGM02_R = crate::BitReader<bool>;
#[doc = "Field `WGM02` writer - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)"]
pub type WGM02_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0B_SPEC, bool, O>;
#[doc = "Field `FOC0B` writer - Force Output Compare B"]
pub type FOC0B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0B_SPEC, bool, O>;
#[doc = "Field `FOC0A` writer - Force Output Compare A"]
pub type FOC0A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)"]
    #[inline(always)]
    pub fn wgm02(&self) -> WGM02_R {
        WGM02_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs0(&mut self) -> CS0_W<0> {
        CS0_W::new(self)
    }
    #[doc = "Bit 3 - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)"]
    #[inline(always)]
    #[must_use]
    pub fn wgm02(&mut self) -> WGM02_W<3> {
        WGM02_W::new(self)
    }
    #[doc = "Bit 6 - Force Output Compare B"]
    #[inline(always)]
    #[must_use]
    pub fn foc0b(&mut self) -> FOC0B_W<6> {
        FOC0B_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare A"]
    #[inline(always)]
    #[must_use]
    pub fn foc0a(&mut self) -> FOC0A_W<7> {
        FOC0A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr0b](index.html) module"]
pub struct TCCR0B_SPEC;
impl crate::RegisterSpec for TCCR0B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr0b::R](R) reader structure"]
impl crate::Readable for TCCR0B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr0b::W](W) writer structure"]
impl crate::Writable for TCCR0B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0B to value 0"]
impl crate::Resettable for TCCR0B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
