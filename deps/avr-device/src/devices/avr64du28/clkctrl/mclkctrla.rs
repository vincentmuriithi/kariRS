#[doc = "Register `MCLKCTRLA` reader"]
pub struct R(crate::R<MCLKCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKCTRLA` writer"]
pub struct W(crate::W<MCLKCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKCTRLA_SPEC>;
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
impl From<crate::W<MCLKCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - Clock select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "Clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Internal high-frequency oscillator"]
    OSCHF = 0,
    #[doc = "1: Internal 32.768 kHz oscillator"]
    OSC32K = 1,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    XOSC32K = 2,
    #[doc = "3: External clock"]
    EXTCLK = 3,
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
            0 => Some(CLKSEL_A::OSCHF),
            1 => Some(CLKSEL_A::OSC32K),
            2 => Some(CLKSEL_A::XOSC32K),
            3 => Some(CLKSEL_A::EXTCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OSCHF`"]
    #[inline(always)]
    pub fn is_oschf(&self) -> bool {
        *self == CLKSEL_A::OSCHF
    }
    #[doc = "Checks if the value of the field is `OSC32K`"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == CLKSEL_A::OSC32K
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == CLKSEL_A::XOSC32K
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == CLKSEL_A::EXTCLK
    }
}
#[doc = "Field `CLKSEL` writer - Clock select"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MCLKCTRLA_SPEC, u8, CLKSEL_A, 3, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "Internal high-frequency oscillator"]
    #[inline(always)]
    pub fn oschf(self) -> &'a mut W {
        self.variant(CLKSEL_A::OSCHF)
    }
    #[doc = "Internal 32.768 kHz oscillator"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut W {
        self.variant(CLKSEL_A::OSC32K)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(CLKSEL_A::XOSC32K)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::EXTCLK)
    }
}
#[doc = "Field `CLKOUT` reader - System clock out"]
pub type CLKOUT_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUT` writer - System clock out"]
pub type CLKOUT_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - System clock out"]
    #[inline(always)]
    pub fn clkout(&self) -> CLKOUT_R {
        CLKOUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - System clock out"]
    #[inline(always)]
    #[must_use]
    pub fn clkout(&mut self) -> CLKOUT_W<7> {
        CLKOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkctrla](index.html) module"]
pub struct MCLKCTRLA_SPEC;
impl crate::RegisterSpec for MCLKCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mclkctrla::R](R) reader structure"]
impl crate::Readable for MCLKCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkctrla::W](W) writer structure"]
impl crate::Writable for MCLKCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKCTRLA to value 0"]
impl crate::Resettable for MCLKCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
