#[doc = "Register `DUALCTRL` reader"]
pub struct R(crate::R<DUALCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DUALCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DUALCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DUALCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DUALCTRL` writer"]
pub struct W(crate::W<DUALCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DUALCTRL_SPEC>;
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
impl From<crate::W<DUALCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DUALCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEFAULT_ENABLE` reader - Dual Control Enable"]
pub type DEFAULT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DEFAULT_ENABLE` writer - Dual Control Enable"]
pub type DEFAULT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DUALCTRL_SPEC, bool, O>;
#[doc = "Field `DEFAULT_FMPEN` reader - FM Plus Enable"]
pub type DEFAULT_FMPEN_R = crate::BitReader<bool>;
#[doc = "Field `DEFAULT_FMPEN` writer - FM Plus Enable"]
pub type DEFAULT_FMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, DUALCTRL_SPEC, bool, O>;
#[doc = "Field `DEFAULT_SDAHOLD` reader - SDA Hold Time"]
pub type DEFAULT_SDAHOLD_R = crate::FieldReader<u8, DEFAULT_SDAHOLD_A>;
#[doc = "SDA Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEFAULT_SDAHOLD_A {
    #[doc = "0: SDA hold time off"]
    OFF = 0,
    #[doc = "1: Typical 50ns hold time"]
    _50NS = 1,
    #[doc = "2: Typical 300ns hold time"]
    _300NS = 2,
    #[doc = "3: Typical 500ns hold time"]
    _500NS = 3,
}
impl From<DEFAULT_SDAHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFAULT_SDAHOLD_A) -> Self {
        variant as _
    }
}
impl DEFAULT_SDAHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEFAULT_SDAHOLD_A {
        match self.bits {
            0 => DEFAULT_SDAHOLD_A::OFF,
            1 => DEFAULT_SDAHOLD_A::_50NS,
            2 => DEFAULT_SDAHOLD_A::_300NS,
            3 => DEFAULT_SDAHOLD_A::_500NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DEFAULT_SDAHOLD_A::OFF
    }
    #[doc = "Checks if the value of the field is `_50NS`"]
    #[inline(always)]
    pub fn is_50ns(&self) -> bool {
        *self == DEFAULT_SDAHOLD_A::_50NS
    }
    #[doc = "Checks if the value of the field is `_300NS`"]
    #[inline(always)]
    pub fn is_300ns(&self) -> bool {
        *self == DEFAULT_SDAHOLD_A::_300NS
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == DEFAULT_SDAHOLD_A::_500NS
    }
}
#[doc = "Field `DEFAULT_SDAHOLD` writer - SDA Hold Time"]
pub type DEFAULT_SDAHOLD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, DUALCTRL_SPEC, u8, DEFAULT_SDAHOLD_A, 2, O>;
impl<'a, const O: u8> DEFAULT_SDAHOLD_W<'a, O> {
    #[doc = "SDA hold time off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DEFAULT_SDAHOLD_A::OFF)
    }
    #[doc = "Typical 50ns hold time"]
    #[inline(always)]
    pub fn _50ns(self) -> &'a mut W {
        self.variant(DEFAULT_SDAHOLD_A::_50NS)
    }
    #[doc = "Typical 300ns hold time"]
    #[inline(always)]
    pub fn _300ns(self) -> &'a mut W {
        self.variant(DEFAULT_SDAHOLD_A::_300NS)
    }
    #[doc = "Typical 500ns hold time"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(DEFAULT_SDAHOLD_A::_500NS)
    }
}
impl R {
    #[doc = "Bit 0 - Dual Control Enable"]
    #[inline(always)]
    pub fn default_enable(&self) -> DEFAULT_ENABLE_R {
        DEFAULT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FM Plus Enable"]
    #[inline(always)]
    pub fn default_fmpen(&self) -> DEFAULT_FMPEN_R {
        DEFAULT_FMPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SDA Hold Time"]
    #[inline(always)]
    pub fn default_sdahold(&self) -> DEFAULT_SDAHOLD_R {
        DEFAULT_SDAHOLD_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn default_enable(&mut self) -> DEFAULT_ENABLE_W<0> {
        DEFAULT_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - FM Plus Enable"]
    #[inline(always)]
    #[must_use]
    pub fn default_fmpen(&mut self) -> DEFAULT_FMPEN_W<1> {
        DEFAULT_FMPEN_W::new(self)
    }
    #[doc = "Bits 2:3 - SDA Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn default_sdahold(&mut self) -> DEFAULT_SDAHOLD_W<2> {
        DEFAULT_SDAHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dual Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dualctrl](index.html) module"]
pub struct DUALCTRL_SPEC;
impl crate::RegisterSpec for DUALCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dualctrl::R](R) reader structure"]
impl crate::Readable for DUALCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dualctrl::W](W) writer structure"]
impl crate::Writable for DUALCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUALCTRL to value 0"]
impl crate::Resettable for DUALCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
