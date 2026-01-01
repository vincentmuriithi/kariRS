#[doc = "Register `TCCR1B` reader"]
pub struct R(crate::R<TCCR1B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR1B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR1B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR1B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR1B` writer"]
pub struct W(crate::W<TCCR1B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR1B_SPEC>;
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
impl From<crate::W<TCCR1B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR1B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS1` reader - Clock Select Bits"]
pub type CS1_R = crate::FieldReader<u8, CS1_A>;
#[doc = "Clock Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS1_A {
    #[doc = "0: No Clock Source (Stopped)"]
    NO_CLOCK_SOURCE_STOPPED = 0,
    #[doc = "1: Running, No Prescaling"]
    RUNNING_NO_PRESCALING = 1,
    #[doc = "2: Running, CLK/2"]
    RUNNING_CLK_2 = 2,
    #[doc = "3: Running, CLK/4"]
    RUNNING_CLK_4 = 3,
    #[doc = "4: Running, CLK/8"]
    RUNNING_CLK_8 = 4,
    #[doc = "5: Running, CLK/16"]
    RUNNING_CLK_16 = 5,
    #[doc = "6: Running, CLK/32"]
    RUNNING_CLK_32 = 6,
    #[doc = "7: Running, CLK/64"]
    RUNNING_CLK_64 = 7,
    #[doc = "8: Running, CLK/128"]
    RUNNING_CLK_128 = 8,
    #[doc = "9: Running, CLK/256"]
    RUNNING_CLK_256 = 9,
    #[doc = "10: Running, CLK/512"]
    RUNNING_CLK_512 = 10,
    #[doc = "11: Running, CLK/1024"]
    RUNNING_CLK_1024 = 11,
    #[doc = "12: Running, CLK/2048"]
    RUNNING_CLK_2048 = 12,
    #[doc = "13: Running, CLK/4096"]
    RUNNING_CLK_4096 = 13,
    #[doc = "14: Running, CLK/8192"]
    RUNNING_CLK_8192 = 14,
    #[doc = "15: Running, CLK/16384"]
    RUNNING_CLK_16384 = 15,
}
impl From<CS1_A> for u8 {
    #[inline(always)]
    fn from(variant: CS1_A) -> Self {
        variant as _
    }
}
impl CS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS1_A {
        match self.bits {
            0 => CS1_A::NO_CLOCK_SOURCE_STOPPED,
            1 => CS1_A::RUNNING_NO_PRESCALING,
            2 => CS1_A::RUNNING_CLK_2,
            3 => CS1_A::RUNNING_CLK_4,
            4 => CS1_A::RUNNING_CLK_8,
            5 => CS1_A::RUNNING_CLK_16,
            6 => CS1_A::RUNNING_CLK_32,
            7 => CS1_A::RUNNING_CLK_64,
            8 => CS1_A::RUNNING_CLK_128,
            9 => CS1_A::RUNNING_CLK_256,
            10 => CS1_A::RUNNING_CLK_512,
            11 => CS1_A::RUNNING_CLK_1024,
            12 => CS1_A::RUNNING_CLK_2048,
            13 => CS1_A::RUNNING_CLK_4096,
            14 => CS1_A::RUNNING_CLK_8192,
            15 => CS1_A::RUNNING_CLK_16384,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK_SOURCE_STOPPED`"]
    #[inline(always)]
    pub fn is_no_clock_source_stopped(&self) -> bool {
        *self == CS1_A::NO_CLOCK_SOURCE_STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNING_NO_PRESCALING`"]
    #[inline(always)]
    pub fn is_running_no_prescaling(&self) -> bool {
        *self == CS1_A::RUNNING_NO_PRESCALING
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_2`"]
    #[inline(always)]
    pub fn is_running_clk_2(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_2
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_4`"]
    #[inline(always)]
    pub fn is_running_clk_4(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_4
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_8`"]
    #[inline(always)]
    pub fn is_running_clk_8(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_8
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_16`"]
    #[inline(always)]
    pub fn is_running_clk_16(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_16
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_32`"]
    #[inline(always)]
    pub fn is_running_clk_32(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_32
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_64`"]
    #[inline(always)]
    pub fn is_running_clk_64(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_64
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_128`"]
    #[inline(always)]
    pub fn is_running_clk_128(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_128
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_256`"]
    #[inline(always)]
    pub fn is_running_clk_256(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_256
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_512`"]
    #[inline(always)]
    pub fn is_running_clk_512(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_512
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_1024`"]
    #[inline(always)]
    pub fn is_running_clk_1024(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_1024
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_2048`"]
    #[inline(always)]
    pub fn is_running_clk_2048(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_2048
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_4096`"]
    #[inline(always)]
    pub fn is_running_clk_4096(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_4096
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_8192`"]
    #[inline(always)]
    pub fn is_running_clk_8192(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_8192
    }
    #[doc = "Checks if the value of the field is `RUNNING_CLK_16384`"]
    #[inline(always)]
    pub fn is_running_clk_16384(&self) -> bool {
        *self == CS1_A::RUNNING_CLK_16384
    }
}
#[doc = "Field `CS1` writer - Clock Select Bits"]
pub type CS1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR1B_SPEC, u8, CS1_A, 4, O>;
impl<'a, const O: u8> CS1_W<'a, O> {
    #[doc = "No Clock Source (Stopped)"]
    #[inline(always)]
    pub fn no_clock_source_stopped(self) -> &'a mut W {
        self.variant(CS1_A::NO_CLOCK_SOURCE_STOPPED)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn running_no_prescaling(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_NO_PRESCALING)
    }
    #[doc = "Running, CLK/2"]
    #[inline(always)]
    pub fn running_clk_2(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_2)
    }
    #[doc = "Running, CLK/4"]
    #[inline(always)]
    pub fn running_clk_4(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_4)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn running_clk_8(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_8)
    }
    #[doc = "Running, CLK/16"]
    #[inline(always)]
    pub fn running_clk_16(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_16)
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn running_clk_32(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_32)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn running_clk_64(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_64)
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn running_clk_128(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_128)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn running_clk_256(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_256)
    }
    #[doc = "Running, CLK/512"]
    #[inline(always)]
    pub fn running_clk_512(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_512)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn running_clk_1024(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_1024)
    }
    #[doc = "Running, CLK/2048"]
    #[inline(always)]
    pub fn running_clk_2048(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_2048)
    }
    #[doc = "Running, CLK/4096"]
    #[inline(always)]
    pub fn running_clk_4096(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_4096)
    }
    #[doc = "Running, CLK/8192"]
    #[inline(always)]
    pub fn running_clk_8192(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_8192)
    }
    #[doc = "Running, CLK/16384"]
    #[inline(always)]
    pub fn running_clk_16384(self) -> &'a mut W {
        self.variant(CS1_A::RUNNING_CLK_16384)
    }
}
#[doc = "Field `PSR1` reader - Prescaler Reset Timer/Counter1"]
pub type PSR1_R = crate::BitReader<bool>;
#[doc = "Field `PSR1` writer - Prescaler Reset Timer/Counter1"]
pub type PSR1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1B_SPEC, bool, O>;
#[doc = "Field `CTC1` reader - Clear Timer/Counter on Compare Match"]
pub type CTC1_R = crate::BitReader<bool>;
#[doc = "Field `CTC1` writer - Clear Timer/Counter on Compare Match"]
pub type CTC1_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 6 - Prescaler Reset Timer/Counter1"]
    #[inline(always)]
    pub fn psr1(&self) -> PSR1_R {
        PSR1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear Timer/Counter on Compare Match"]
    #[inline(always)]
    pub fn ctc1(&self) -> CTC1_R {
        CTC1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<0> {
        CS1_W::new(self)
    }
    #[doc = "Bit 6 - Prescaler Reset Timer/Counter1"]
    #[inline(always)]
    #[must_use]
    pub fn psr1(&mut self) -> PSR1_W<6> {
        PSR1_W::new(self)
    }
    #[doc = "Bit 7 - Clear Timer/Counter on Compare Match"]
    #[inline(always)]
    #[must_use]
    pub fn ctc1(&mut self) -> CTC1_W<7> {
        CTC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1b](index.html) module"]
pub struct TCCR1B_SPEC;
impl crate::RegisterSpec for TCCR1B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr1b::R](R) reader structure"]
impl crate::Readable for TCCR1B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr1b::W](W) writer structure"]
impl crate::Writable for TCCR1B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1B to value 0"]
impl crate::Resettable for TCCR1B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
