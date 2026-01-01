#[doc = "Register `TCCR2B` reader"]
pub struct R(crate::R<TCCR2B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR2B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR2B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR2B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR2B` writer"]
pub struct W(crate::W<TCCR2B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR2B_SPEC>;
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
impl From<crate::W<TCCR2B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR2B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS2` reader - Clock Select bits"]
pub type CS2_R = crate::FieldReader<u8, CS2_A>;
#[doc = "Clock Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS2_A {
    #[doc = "0: No clock source (Timer/Counter stopped)"]
    NO_CLOCK = 0,
    #[doc = "1: Running, No Prescaling"]
    DIRECT = 1,
    #[doc = "2: Running, CLK/8"]
    PRESCALE_8 = 2,
    #[doc = "3: Running, CLK/32"]
    PRESCALE_32 = 3,
    #[doc = "4: Running, CLK/64"]
    PRESCALE_64 = 4,
    #[doc = "5: Running, CLK/128"]
    PRESCALE_128 = 5,
    #[doc = "6: Running, CLK/256"]
    PRESCALE_256 = 6,
    #[doc = "7: Running, CLK/1024"]
    PRESCALE_1024 = 7,
}
impl From<CS2_A> for u8 {
    #[inline(always)]
    fn from(variant: CS2_A) -> Self {
        variant as _
    }
}
impl CS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS2_A {
        match self.bits {
            0 => CS2_A::NO_CLOCK,
            1 => CS2_A::DIRECT,
            2 => CS2_A::PRESCALE_8,
            3 => CS2_A::PRESCALE_32,
            4 => CS2_A::PRESCALE_64,
            5 => CS2_A::PRESCALE_128,
            6 => CS2_A::PRESCALE_256,
            7 => CS2_A::PRESCALE_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS2_A::NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS2_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `PRESCALE_8`"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS2_A::PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `PRESCALE_32`"]
    #[inline(always)]
    pub fn is_prescale_32(&self) -> bool {
        *self == CS2_A::PRESCALE_32
    }
    #[doc = "Checks if the value of the field is `PRESCALE_64`"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS2_A::PRESCALE_64
    }
    #[doc = "Checks if the value of the field is `PRESCALE_128`"]
    #[inline(always)]
    pub fn is_prescale_128(&self) -> bool {
        *self == CS2_A::PRESCALE_128
    }
    #[doc = "Checks if the value of the field is `PRESCALE_256`"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS2_A::PRESCALE_256
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1024`"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS2_A::PRESCALE_1024
    }
}
#[doc = "Field `CS2` writer - Clock Select bits"]
pub type CS2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR2B_SPEC, u8, CS2_A, 3, O>;
impl<'a, const O: u8> CS2_W<'a, O> {
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(CS2_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(CS2_A::DIRECT)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut W {
        self.variant(CS2_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn prescale_32(self) -> &'a mut W {
        self.variant(CS2_A::PRESCALE_32)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut W {
        self.variant(CS2_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn prescale_128(self) -> &'a mut W {
        self.variant(CS2_A::PRESCALE_128)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut W {
        self.variant(CS2_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut W {
        self.variant(CS2_A::PRESCALE_1024)
    }
}
#[doc = "Field `WGM22` reader - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)"]
pub type WGM22_R = crate::BitReader<bool>;
#[doc = "Field `WGM22` writer - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)"]
pub type WGM22_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2B_SPEC, bool, O>;
#[doc = "Field `FOC2B` writer - Force Output Compare B"]
pub type FOC2B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2B_SPEC, bool, O>;
#[doc = "Field `FOC2A` writer - Force Output Compare A"]
pub type FOC2A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)"]
    #[inline(always)]
    pub fn wgm22(&self) -> WGM22_R {
        WGM22_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<0> {
        CS2_W::new(self)
    }
    #[doc = "Bit 3 - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)"]
    #[inline(always)]
    #[must_use]
    pub fn wgm22(&mut self) -> WGM22_W<3> {
        WGM22_W::new(self)
    }
    #[doc = "Bit 6 - Force Output Compare B"]
    #[inline(always)]
    #[must_use]
    pub fn foc2b(&mut self) -> FOC2B_W<6> {
        FOC2B_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare A"]
    #[inline(always)]
    #[must_use]
    pub fn foc2a(&mut self) -> FOC2A_W<7> {
        FOC2A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter2 Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr2b](index.html) module"]
pub struct TCCR2B_SPEC;
impl crate::RegisterSpec for TCCR2B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr2b::R](R) reader structure"]
impl crate::Readable for TCCR2B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr2b::W](W) writer structure"]
impl crate::Writable for TCCR2B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR2B to value 0"]
impl crate::Resettable for TCCR2B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
