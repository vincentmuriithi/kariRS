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
#[doc = "Field `CLKSEL` reader - clock select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: 20MHz oscillator"]
    OSC20M = 0,
    #[doc = "1: 32KHz oscillator"]
    OSCULP32K = 1,
    #[doc = "2: 32.768kHz crystal oscillator"]
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
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::OSC20M,
            1 => CLKSEL_A::OSCULP32K,
            2 => CLKSEL_A::XOSC32K,
            3 => CLKSEL_A::EXTCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OSC20M`"]
    #[inline(always)]
    pub fn is_osc20m(&self) -> bool {
        *self == CLKSEL_A::OSC20M
    }
    #[doc = "Checks if the value of the field is `OSCULP32K`"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == CLKSEL_A::OSCULP32K
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
#[doc = "Field `CLKSEL` writer - clock select"]
pub type CLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, MCLKCTRLA_SPEC, u8, CLKSEL_A, 2, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "20MHz oscillator"]
    #[inline(always)]
    pub fn osc20m(self) -> &'a mut W {
        self.variant(CLKSEL_A::OSC20M)
    }
    #[doc = "32KHz oscillator"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut W {
        self.variant(CLKSEL_A::OSCULP32K)
    }
    #[doc = "32.768kHz crystal oscillator"]
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
    #[doc = "Bits 0:1 - clock select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(self.bits & 3)
    }
    #[doc = "Bit 7 - System clock out"]
    #[inline(always)]
    pub fn clkout(&self) -> CLKOUT_R {
        CLKOUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - clock select"]
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
