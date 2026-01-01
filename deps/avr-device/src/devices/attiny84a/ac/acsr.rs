#[doc = "Register `ACSR` reader"]
pub struct R(crate::R<ACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACSR` writer"]
pub struct W(crate::W<ACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACSR_SPEC>;
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
impl From<crate::W<ACSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACIS` reader - Analog Comparator Interrupt Mode Select bits"]
pub type ACIS_R = crate::FieldReader<u8, ACIS_A>;
#[doc = "Analog Comparator Interrupt Mode Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACIS_A {
    #[doc = "0: Interrupt on Toggle"]
    INTERRUPT_ON_TOGGLE = 0,
    #[doc = "2: Interrupt on Falling Edge"]
    INTERRUPT_ON_FALLING_EDGE = 2,
    #[doc = "3: Interrupt on Rising Edge"]
    INTERRUPT_ON_RISING_EDGE = 3,
}
impl From<ACIS_A> for u8 {
    #[inline(always)]
    fn from(variant: ACIS_A) -> Self {
        variant as _
    }
}
impl ACIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACIS_A {
        match self.bits {
            0 => ACIS_A::INTERRUPT_ON_TOGGLE,
            2 => ACIS_A::INTERRUPT_ON_FALLING_EDGE,
            3 => ACIS_A::INTERRUPT_ON_RISING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_TOGGLE`"]
    #[inline(always)]
    pub fn is_interrupt_on_toggle(&self) -> bool {
        *self == ACIS_A::INTERRUPT_ON_TOGGLE
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_interrupt_on_falling_edge(&self) -> bool {
        *self == ACIS_A::INTERRUPT_ON_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_interrupt_on_rising_edge(&self) -> bool {
        *self == ACIS_A::INTERRUPT_ON_RISING_EDGE
    }
}
#[doc = "Field `ACIS` writer - Analog Comparator Interrupt Mode Select bits"]
pub type ACIS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ACSR_SPEC, u8, ACIS_A, 2, O>;
impl<'a, const O: u8> ACIS_W<'a, O> {
    #[doc = "Interrupt on Toggle"]
    #[inline(always)]
    pub fn interrupt_on_toggle(self) -> &'a mut W {
        self.variant(ACIS_A::INTERRUPT_ON_TOGGLE)
    }
    #[doc = "Interrupt on Falling Edge"]
    #[inline(always)]
    pub fn interrupt_on_falling_edge(self) -> &'a mut W {
        self.variant(ACIS_A::INTERRUPT_ON_FALLING_EDGE)
    }
    #[doc = "Interrupt on Rising Edge"]
    #[inline(always)]
    pub fn interrupt_on_rising_edge(self) -> &'a mut W {
        self.variant(ACIS_A::INTERRUPT_ON_RISING_EDGE)
    }
}
#[doc = "Field `ACIC` reader - Analog Comparator Input Capture Enable"]
pub type ACIC_R = crate::BitReader<bool>;
#[doc = "Field `ACIC` writer - Analog Comparator Input Capture Enable"]
pub type ACIC_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR_SPEC, bool, O>;
#[doc = "Field `ACIE` reader - Analog Comparator Interrupt Enable"]
pub type ACIE_R = crate::BitReader<bool>;
#[doc = "Field `ACIE` writer - Analog Comparator Interrupt Enable"]
pub type ACIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR_SPEC, bool, O>;
#[doc = "Field `ACI` reader - Analog Comparator Interrupt Flag"]
pub type ACI_R = crate::BitReader<bool>;
#[doc = "Field `ACI` writer - Analog Comparator Interrupt Flag"]
pub type ACI_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR_SPEC, bool, O>;
#[doc = "Field `ACO` reader - Analog Compare Output"]
pub type ACO_R = crate::BitReader<bool>;
#[doc = "Field `ACO` writer - Analog Compare Output"]
pub type ACO_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR_SPEC, bool, O>;
#[doc = "Field `ACBG` reader - Analog Comparator Bandgap Select"]
pub type ACBG_R = crate::BitReader<bool>;
#[doc = "Field `ACBG` writer - Analog Comparator Bandgap Select"]
pub type ACBG_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR_SPEC, bool, O>;
#[doc = "Field `ACD` reader - Analog Comparator Disable"]
pub type ACD_R = crate::BitReader<bool>;
#[doc = "Field `ACD` writer - Analog Comparator Disable"]
pub type ACD_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACSR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Analog Comparator Interrupt Mode Select bits"]
    #[inline(always)]
    pub fn acis(&self) -> ACIS_R {
        ACIS_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Analog Comparator Input Capture Enable"]
    #[inline(always)]
    pub fn acic(&self) -> ACIC_R {
        ACIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator Interrupt Enable"]
    #[inline(always)]
    pub fn acie(&self) -> ACIE_R {
        ACIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn aci(&self) -> ACI_R {
        ACI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Compare Output"]
    #[inline(always)]
    pub fn aco(&self) -> ACO_R {
        ACO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator Bandgap Select"]
    #[inline(always)]
    pub fn acbg(&self) -> ACBG_R {
        ACBG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator Disable"]
    #[inline(always)]
    pub fn acd(&self) -> ACD_R {
        ACD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Comparator Interrupt Mode Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn acis(&mut self) -> ACIS_W<0> {
        ACIS_W::new(self)
    }
    #[doc = "Bit 2 - Analog Comparator Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acic(&mut self) -> ACIC_W<2> {
        ACIC_W::new(self)
    }
    #[doc = "Bit 3 - Analog Comparator Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acie(&mut self) -> ACIE_W<3> {
        ACIE_W::new(self)
    }
    #[doc = "Bit 4 - Analog Comparator Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aci(&mut self) -> ACI_W<4> {
        ACI_W::new(self)
    }
    #[doc = "Bit 5 - Analog Compare Output"]
    #[inline(always)]
    #[must_use]
    pub fn aco(&mut self) -> ACO_W<5> {
        ACO_W::new(self)
    }
    #[doc = "Bit 6 - Analog Comparator Bandgap Select"]
    #[inline(always)]
    #[must_use]
    pub fn acbg(&mut self) -> ACBG_W<6> {
        ACBG_W::new(self)
    }
    #[doc = "Bit 7 - Analog Comparator Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acd(&mut self) -> ACD_W<7> {
        ACD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Control And Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acsr](index.html) module"]
pub struct ACSR_SPEC;
impl crate::RegisterSpec for ACSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [acsr::R](R) reader structure"]
impl crate::Readable for ACSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acsr::W](W) writer structure"]
impl crate::Writable for ACSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR to value 0"]
impl crate::Resettable for ACSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
