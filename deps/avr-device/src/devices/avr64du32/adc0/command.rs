#[doc = "Register `COMMAND` reader"]
pub struct R(crate::R<COMMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMMAND` writer"]
pub struct W(crate::W<COMMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMMAND_SPEC>;
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
impl From<crate::W<COMMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Start Conversion"]
pub type START_R = crate::FieldReader<u8, START_A>;
#[doc = "Start Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum START_A {
    #[doc = "0: Stop/No ongoing conv"]
    STOP = 0,
    #[doc = "1: Start Immediately"]
    IMMEDIATE = 1,
    #[doc = "2: Start after a write to MUXPOS"]
    MUXPOS_WRITE = 2,
    #[doc = "3: Start upon event reception"]
    EVENT_TTRIGGER = 3,
}
impl From<START_A> for u8 {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as _
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            0 => Some(START_A::STOP),
            1 => Some(START_A::IMMEDIATE),
            2 => Some(START_A::MUXPOS_WRITE),
            3 => Some(START_A::EVENT_TTRIGGER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == START_A::STOP
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == START_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `MUXPOS_WRITE`"]
    #[inline(always)]
    pub fn is_muxpos_write(&self) -> bool {
        *self == START_A::MUXPOS_WRITE
    }
    #[doc = "Checks if the value of the field is `EVENT_TTRIGGER`"]
    #[inline(always)]
    pub fn is_event_ttrigger(&self) -> bool {
        *self == START_A::EVENT_TTRIGGER
    }
}
#[doc = "Field `START` writer - Start Conversion"]
pub type START_W<'a, const O: u8> = crate::FieldWriter<'a, u8, COMMAND_SPEC, u8, START_A, 3, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Stop/No ongoing conv"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(START_A::STOP)
    }
    #[doc = "Start Immediately"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(START_A::IMMEDIATE)
    }
    #[doc = "Start after a write to MUXPOS"]
    #[inline(always)]
    pub fn muxpos_write(self) -> &'a mut W {
        self.variant(START_A::MUXPOS_WRITE)
    }
    #[doc = "Start upon event reception"]
    #[inline(always)]
    pub fn event_ttrigger(self) -> &'a mut W {
        self.variant(START_A::EVENT_TTRIGGER)
    }
}
#[doc = "Field `MODE` reader - Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Single 8-bit conv"]
    SINGLE_8BIT = 0,
    #[doc = "1: Single 10-bit conv"]
    SINGLE_10BIT = 1,
    #[doc = "2: Series of 10-bit conv"]
    SERIES = 2,
    #[doc = "3: Burst of 10-bit conv"]
    BURST = 3,
    #[doc = "7: Acc test mode for FuSa"]
    ACCTEST = 7,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::SINGLE_8BIT),
            1 => Some(MODE_A::SINGLE_10BIT),
            2 => Some(MODE_A::SERIES),
            3 => Some(MODE_A::BURST),
            7 => Some(MODE_A::ACCTEST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_8BIT`"]
    #[inline(always)]
    pub fn is_single_8bit(&self) -> bool {
        *self == MODE_A::SINGLE_8BIT
    }
    #[doc = "Checks if the value of the field is `SINGLE_10BIT`"]
    #[inline(always)]
    pub fn is_single_10bit(&self) -> bool {
        *self == MODE_A::SINGLE_10BIT
    }
    #[doc = "Checks if the value of the field is `SERIES`"]
    #[inline(always)]
    pub fn is_series(&self) -> bool {
        *self == MODE_A::SERIES
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == MODE_A::BURST
    }
    #[doc = "Checks if the value of the field is `ACCTEST`"]
    #[inline(always)]
    pub fn is_acctest(&self) -> bool {
        *self == MODE_A::ACCTEST
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, COMMAND_SPEC, u8, MODE_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Single 8-bit conv"]
    #[inline(always)]
    pub fn single_8bit(self) -> &'a mut W {
        self.variant(MODE_A::SINGLE_8BIT)
    }
    #[doc = "Single 10-bit conv"]
    #[inline(always)]
    pub fn single_10bit(self) -> &'a mut W {
        self.variant(MODE_A::SINGLE_10BIT)
    }
    #[doc = "Series of 10-bit conv"]
    #[inline(always)]
    pub fn series(self) -> &'a mut W {
        self.variant(MODE_A::SERIES)
    }
    #[doc = "Burst of 10-bit conv"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(MODE_A::BURST)
    }
    #[doc = "Acc test mode for FuSa"]
    #[inline(always)]
    pub fn acctest(self) -> &'a mut W {
        self.variant(MODE_A::ACCTEST)
    }
}
impl R {
    #[doc = "Bits 0:2 - Start Conversion"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Start Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 4:6 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command](index.html) module"]
pub struct COMMAND_SPEC;
impl crate::RegisterSpec for COMMAND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [command::R](R) reader structure"]
impl crate::Readable for COMMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [command::W](W) writer structure"]
impl crate::Writable for COMMAND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for COMMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
