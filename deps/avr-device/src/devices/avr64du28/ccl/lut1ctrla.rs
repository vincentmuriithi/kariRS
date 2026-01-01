#[doc = "Register `LUT1CTRLA` reader"]
pub struct R(crate::R<LUT1CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT1CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT1CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT1CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT1CTRLA` writer"]
pub struct W(crate::W<LUT1CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT1CTRLA_SPEC>;
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
impl From<crate::W<LUT1CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT1CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - LUT Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - LUT Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LUT1CTRLA_SPEC, bool, O>;
#[doc = "Field `CLKSRC` reader - Clock Source Selection"]
pub type CLKSRC_R = crate::FieldReader<u8, CLKSRC_A>;
#[doc = "Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSRC_A {
    #[doc = "0: Peripheral Clock"]
    CLKPER = 0,
    #[doc = "1: Selection by INSEL2"]
    IN2 = 1,
    #[doc = "4: Internal High-Frequency Oscillator"]
    OSCHF = 4,
    #[doc = "5: 32.768 kHz oscillator"]
    OSC32K = 5,
    #[doc = "6: 32.768 kHz oscillator divided by 32"]
    OSC1K = 6,
}
impl From<CLKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as _
    }
}
impl CLKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSRC_A> {
        match self.bits {
            0 => Some(CLKSRC_A::CLKPER),
            1 => Some(CLKSRC_A::IN2),
            4 => Some(CLKSRC_A::OSCHF),
            5 => Some(CLKSRC_A::OSC32K),
            6 => Some(CLKSRC_A::OSC1K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLKPER`"]
    #[inline(always)]
    pub fn is_clkper(&self) -> bool {
        *self == CLKSRC_A::CLKPER
    }
    #[doc = "Checks if the value of the field is `IN2`"]
    #[inline(always)]
    pub fn is_in2(&self) -> bool {
        *self == CLKSRC_A::IN2
    }
    #[doc = "Checks if the value of the field is `OSCHF`"]
    #[inline(always)]
    pub fn is_oschf(&self) -> bool {
        *self == CLKSRC_A::OSCHF
    }
    #[doc = "Checks if the value of the field is `OSC32K`"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == CLKSRC_A::OSC32K
    }
    #[doc = "Checks if the value of the field is `OSC1K`"]
    #[inline(always)]
    pub fn is_osc1k(&self) -> bool {
        *self == CLKSRC_A::OSC1K
    }
}
#[doc = "Field `CLKSRC` writer - Clock Source Selection"]
pub type CLKSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LUT1CTRLA_SPEC, u8, CLKSRC_A, 3, O>;
impl<'a, const O: u8> CLKSRC_W<'a, O> {
    #[doc = "Peripheral Clock"]
    #[inline(always)]
    pub fn clkper(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKPER)
    }
    #[doc = "Selection by INSEL2"]
    #[inline(always)]
    pub fn in2(self) -> &'a mut W {
        self.variant(CLKSRC_A::IN2)
    }
    #[doc = "Internal High-Frequency Oscillator"]
    #[inline(always)]
    pub fn oschf(self) -> &'a mut W {
        self.variant(CLKSRC_A::OSCHF)
    }
    #[doc = "32.768 kHz oscillator"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut W {
        self.variant(CLKSRC_A::OSC32K)
    }
    #[doc = "32.768 kHz oscillator divided by 32"]
    #[inline(always)]
    pub fn osc1k(self) -> &'a mut W {
        self.variant(CLKSRC_A::OSC1K)
    }
}
#[doc = "Field `FILTSEL` reader - Filter Selection"]
pub type FILTSEL_R = crate::FieldReader<u8, FILTSEL_A>;
#[doc = "Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTSEL_A {
    #[doc = "0: Filter disabled"]
    DISABLE = 0,
    #[doc = "1: Synchronizer enabled"]
    SYNCH = 1,
    #[doc = "2: Filter enabled"]
    FILTER = 2,
}
impl From<FILTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTSEL_A) -> Self {
        variant as _
    }
}
impl FILTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FILTSEL_A> {
        match self.bits {
            0 => Some(FILTSEL_A::DISABLE),
            1 => Some(FILTSEL_A::SYNCH),
            2 => Some(FILTSEL_A::FILTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FILTSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `SYNCH`"]
    #[inline(always)]
    pub fn is_synch(&self) -> bool {
        *self == FILTSEL_A::SYNCH
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == FILTSEL_A::FILTER
    }
}
#[doc = "Field `FILTSEL` writer - Filter Selection"]
pub type FILTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, LUT1CTRLA_SPEC, u8, FILTSEL_A, 2, O>;
impl<'a, const O: u8> FILTSEL_W<'a, O> {
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FILTSEL_A::DISABLE)
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn synch(self) -> &'a mut W {
        self.variant(FILTSEL_A::SYNCH)
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn filter(self) -> &'a mut W {
        self.variant(FILTSEL_A::FILTER)
    }
}
#[doc = "Field `OUTEN` reader - Output Enable"]
pub type OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `OUTEN` writer - Output Enable"]
pub type OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, LUT1CTRLA_SPEC, bool, O>;
#[doc = "Field `EDGEDET` reader - Edge Detection Enable"]
pub type EDGEDET_R = crate::BitReader<EDGEDET_A>;
#[doc = "Edge Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGEDET_A {
    #[doc = "0: Edge detector is disabled"]
    DIS = 0,
    #[doc = "1: Edge detector is enabled"]
    EN = 1,
}
impl From<EDGEDET_A> for bool {
    #[inline(always)]
    fn from(variant: EDGEDET_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGEDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGEDET_A {
        match self.bits {
            false => EDGEDET_A::DIS,
            true => EDGEDET_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EDGEDET_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EDGEDET_A::EN
    }
}
#[doc = "Field `EDGEDET` writer - Edge Detection Enable"]
pub type EDGEDET_W<'a, const O: u8> = crate::BitWriter<'a, u8, LUT1CTRLA_SPEC, EDGEDET_A, O>;
impl<'a, const O: u8> EDGEDET_W<'a, O> {
    #[doc = "Edge detector is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EDGEDET_A::DIS)
    }
    #[doc = "Edge detector is enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EDGEDET_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - LUT Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Clock Source Selection"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&self) -> FILTSEL_R {
        FILTSEL_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Output Enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge Detection Enable"]
    #[inline(always)]
    pub fn edgedet(&self) -> EDGEDET_R {
        EDGEDET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LUT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:3 - Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> CLKSRC_W<1> {
        CLKSRC_W::new(self)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn filtsel(&mut self) -> FILTSEL_W<4> {
        FILTSEL_W::new(self)
    }
    #[doc = "Bit 6 - Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<6> {
        OUTEN_W::new(self)
    }
    #[doc = "Bit 7 - Edge Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edgedet(&mut self) -> EDGEDET_W<7> {
        EDGEDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT 1 Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut1ctrla](index.html) module"]
pub struct LUT1CTRLA_SPEC;
impl crate::RegisterSpec for LUT1CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lut1ctrla::R](R) reader structure"]
impl crate::Readable for LUT1CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut1ctrla::W](W) writer structure"]
impl crate::Writable for LUT1CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT1CTRLA to value 0"]
impl crate::Resettable for LUT1CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
