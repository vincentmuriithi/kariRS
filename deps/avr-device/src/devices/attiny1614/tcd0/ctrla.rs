#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `SYNCPRES` reader - Syncronization prescaler"]
pub type SYNCPRES_R = crate::FieldReader<u8, SYNCPRES_A>;
#[doc = "Syncronization prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCPRES_A {
    #[doc = "0: Selevted clock source divided by 1"]
    DIV1 = 0,
    #[doc = "1: Selevted clock source divided by 2"]
    DIV2 = 1,
    #[doc = "2: Selevted clock source divided by 4"]
    DIV4 = 2,
    #[doc = "3: Selevted clock source divided by 8"]
    DIV8 = 3,
}
impl From<SYNCPRES_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCPRES_A) -> Self {
        variant as _
    }
}
impl SYNCPRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCPRES_A {
        match self.bits {
            0 => SYNCPRES_A::DIV1,
            1 => SYNCPRES_A::DIV2,
            2 => SYNCPRES_A::DIV4,
            3 => SYNCPRES_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SYNCPRES_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SYNCPRES_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SYNCPRES_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SYNCPRES_A::DIV8
    }
}
#[doc = "Field `SYNCPRES` writer - Syncronization prescaler"]
pub type SYNCPRES_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTRLA_SPEC, u8, SYNCPRES_A, 2, O>;
impl<'a, const O: u8> SYNCPRES_W<'a, O> {
    #[doc = "Selevted clock source divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(SYNCPRES_A::DIV1)
    }
    #[doc = "Selevted clock source divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(SYNCPRES_A::DIV2)
    }
    #[doc = "Selevted clock source divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(SYNCPRES_A::DIV4)
    }
    #[doc = "Selevted clock source divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(SYNCPRES_A::DIV8)
    }
}
#[doc = "Field `CNTPRES` reader - counter prescaler"]
pub type CNTPRES_R = crate::FieldReader<u8, CNTPRES_A>;
#[doc = "counter prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTPRES_A {
    #[doc = "0: Sync clock divided by 1"]
    DIV1 = 0,
    #[doc = "1: Sync clock divided by 4"]
    DIV4 = 1,
    #[doc = "2: Sync clock divided by 32"]
    DIV32 = 2,
}
impl From<CNTPRES_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTPRES_A) -> Self {
        variant as _
    }
}
impl CNTPRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTPRES_A> {
        match self.bits {
            0 => Some(CNTPRES_A::DIV1),
            1 => Some(CNTPRES_A::DIV4),
            2 => Some(CNTPRES_A::DIV32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CNTPRES_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CNTPRES_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CNTPRES_A::DIV32
    }
}
#[doc = "Field `CNTPRES` writer - counter prescaler"]
pub type CNTPRES_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLA_SPEC, u8, CNTPRES_A, 2, O>;
impl<'a, const O: u8> CNTPRES_W<'a, O> {
    #[doc = "Sync clock divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CNTPRES_A::DIV1)
    }
    #[doc = "Sync clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CNTPRES_A::DIV4)
    }
    #[doc = "Sync clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CNTPRES_A::DIV32)
    }
}
#[doc = "Field `CLKSEL` reader - clock select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: 20 MHz oscillator"]
    _20MHZ = 0,
    #[doc = "2: External clock"]
    EXTCLK = 2,
    #[doc = "3: System clock"]
    SYSCLK = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::_20MHZ),
            2 => Some(CLKSEL_A::EXTCLK),
            3 => Some(CLKSEL_A::SYSCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_20MHZ`"]
    #[inline(always)]
    pub fn is_20mhz(&self) -> bool {
        *self == CLKSEL_A::_20MHZ
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == CLKSEL_A::EXTCLK
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CLKSEL_A::SYSCLK
    }
}
#[doc = "Field `CLKSEL` writer - clock select"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLA_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "20 MHz oscillator"]
    #[inline(always)]
    pub fn _20mhz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_20MHZ)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::EXTCLK)
    }
    #[doc = "System clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::SYSCLK)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Syncronization prescaler"]
    #[inline(always)]
    pub fn syncpres(&self) -> SYNCPRES_R {
        SYNCPRES_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 3:4 - counter prescaler"]
    #[inline(always)]
    pub fn cntpres(&self) -> CNTPRES_R {
        CNTPRES_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bits 5:6 - clock select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits >> 5) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:2 - Syncronization prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn syncpres(&mut self) -> SYNCPRES_W<1> {
        SYNCPRES_W::new(self)
    }
    #[doc = "Bits 3:4 - counter prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn cntpres(&mut self) -> CNTPRES_W<3> {
        CNTPRES_W::new(self)
    }
    #[doc = "Bits 5:6 - clock select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<5> {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
